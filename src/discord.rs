use anyhow::{anyhow, Result};
use reqwest::multipart;
use serde_derive::Serialize;

pub struct DiscordClient {
    webhook_url: String,
    client: reqwest::Client,
}

#[derive(Serialize)]
struct CreateMessageParams {
    content: String,
}

impl DiscordClient {
    pub fn new(webhook_url: String) -> Self {
        DiscordClient {
            webhook_url,
            client: Default::default(),
        }
    }

    pub async fn send_text<T>(&self, text: T) -> Result<()>
    where
        T: Into<String>,
    {
        let params = CreateMessageParams {
            content: text.into(),
        };
        let response = self
            .client
            .post(self.webhook_url.as_str())
            .json(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "Failed send message. (status={})",
                response.status()
            ));
        }
        Ok(())
    }

    pub async fn send_file<N, M>(&self, name: N, bytes: Vec<u8>, comment: M) -> Result<()>
    where
        N: Into<String>,
        M: Into<String>,
    {
        let params = CreateMessageParams {
            content: comment.into(),
        };
        let params = serde_json::to_string(&params)?;
        let payload_json = multipart::Part::text(params).mime_str("application/json")?;
        let file = multipart::Part::bytes(bytes).file_name(name.into());
        let form = multipart::Form::new()
            .part("file", file)
            .part("payload_json", payload_json);

        let response = self
            .client
            .post(self.webhook_url.as_str())
            .multipart(form)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed send file. (status={})", response.status()));
        }

        Ok(())
    }
}
