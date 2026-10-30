use disco::discord::DiscordClient;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use tokio::time::{Duration, timeout};

async fn mock_webhook(status: &str) -> (DiscordClient, JoinHandle<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}/webhook", listener.local_addr().unwrap());
    let response = format!("HTTP/1.1 {status}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
    let request = tokio::spawn(async move {
        timeout(Duration::from_secs(5), async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut request = Vec::new();
            let mut buffer = [0; 4096];
            loop {
                let count = stream.read(&mut buffer).await.unwrap();
                assert_ne!(count, 0, "request ended before body was received");
                request.extend_from_slice(&buffer[..count]);
                if let Some(end) = request.windows(4).position(|part| part == b"\r\n\r\n") {
                    let headers = String::from_utf8_lossy(&request[..end]);
                    let length: usize = headers
                        .lines()
                        .find_map(|line| {
                            let (name, value) = line.split_once(':')?;
                            name.eq_ignore_ascii_case("content-length")
                                .then(|| value.trim().parse().unwrap())
                        })
                        .unwrap();
                    if request.len() >= end + 4 + length {
                        break;
                    }
                }
            }
            stream.write_all(response.as_bytes()).await.unwrap();
            String::from_utf8(request).unwrap()
        })
        .await
        .expect("webhook request timed out")
    });
    (DiscordClient::new(url), request)
}

#[tokio::test]
async fn text_is_sent_as_json() {
    let (client, request) = mock_webhook("204 No Content").await;
    client.send_text("hello\n世界").await.unwrap();
    let request = request.await.unwrap();
    assert!(request.starts_with("POST /webhook HTTP/1.1\r\n"));
    let (_, body) = request.split_once("\r\n\r\n").unwrap();
    let body: serde_json::Value = serde_json::from_str(body).unwrap();
    assert_eq!(body["content"], "hello\n世界");
}

#[tokio::test]
async fn file_and_comment_are_sent_as_multipart() {
    let (client, request) = mock_webhook("204 No Content").await;
    client
        .send_file("log.txt", b"file contents".to_vec(), "comment")
        .await
        .unwrap();
    let request = request.await.unwrap();
    assert!(request.contains("multipart/form-data; boundary="));
    assert!(request.contains("filename=\"log.txt\""));
    assert!(request.contains("file contents"));
    assert!(request.contains("name=\"payload_json\""));
    assert!(request.contains(r#"{"content":"comment"}"#));
}

#[tokio::test]
async fn webhook_errors_are_reported() {
    let (client, request) = mock_webhook("429 Too Many Requests").await;
    assert!(
        client
            .send_text("hello")
            .await
            .unwrap_err()
            .to_string()
            .contains("429")
    );
    request.await.unwrap();
    let (client, request) = mock_webhook("400 Bad Request").await;
    assert!(
        client
            .send_file("log.txt", vec![], "")
            .await
            .unwrap_err()
            .to_string()
            .contains("400")
    );
    request.await.unwrap();
}
