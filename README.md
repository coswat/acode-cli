![Acode Plugin Cli](https://github-production-user-asset-6210df.s3.amazonaws.com/97345827/266290195-9d09fa68-0044-4b1d-96ee-5df5e426a9af.jpeg)

# Acode Plugin CLI 

This is a Acode plugin cli tool to create plugins and easily work with it

## Installation

using cargo

```bash
git clone https://github.com/coswat/acode-cli.git
cd acode-cli
cargo build --release
mv target/release/acode-cli ~/.local/bin/
```

using curl

```bash
curl -LJO https://github.com/coswat/acode-cli/releases/download/v1.0.5/acode-cli && mv acode-cli ~/.local/bin/
```

## Usage

run it using `acode-cli` command

```bash
acode-cli --help
```