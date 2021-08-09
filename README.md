# disco

Disco is a tool for sending messages to discord.

## Installing

TBW

## Configuration

```bash
DISCORD_WEBHOOK_URL="https://discord.com/api/webhooks/xxxx/yyyy"
mkdir -p ~/.config/disco
echo "{\"webhook_url\":\"${DISCORD_WEBHOOK_URL}\"}" > ~/.config/disco/config
```

## Usage

Pipe command output.

```bash
$ echo "hello" | disco cat
```

Pipe command output as a text snippet.

```bash
$ echo "hello" | disco cat --snippet
```

Send file with comment.

```bash
$ disco send-file REAMD.md --comment "comment"
```
