# disco

Disco is a tool for sending messages to discord.

## Installing

TBW

## Configuration

```bash
$ disco configure --webhook-url "https://discord.com/api/webhooks/xxxx/yyyy"
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
