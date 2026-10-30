# disco

Disco is a tool for sending messages to discord.

## Installing

Download the binary directly.

```bash
wget https://github.com/omuric/disco/releases/download/0.1.4/disco_0.1.4_x86_64-unknown-linux-musl.zip
unzip disco_*_x86_64-unknown-linux-musl.zip disco
rm disco_*_x86_64-unknown-linux-musl.zip
./disco --help
```

(Optional) Place it in `/usr/local/bin`.

```bash
sudo mv ./disco /usr/local/bin/disco
```

Or build from source with the latest stable [Rust toolchain](https://rustup.rs/).

```bash
git clone https://github.com/omuric/disco.git
cd disco
cargo install --path . --locked
```

TODO: Change to installation via Crates.io

## Configuration

```bash
disco configure --webhook-url "https://discord.com/api/webhooks/xxxx/yyyy"
```

## Usage

Pipe command output.

```bash
echo "hello" | disco cat
```

Pipe command output as a text snippet.

```bash
echo "hello" | disco cat --snippet
```

Send file with comment.

```bash
disco send-file README.md --comment "comment"
```

## Development

```bash
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets
```

Configuration is stored in `~/.config/disco/config.json`.
