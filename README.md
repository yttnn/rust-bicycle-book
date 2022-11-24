# rust-bicycle-book
## Memo
- rust-analyzerはrootに`Cargo.toml`がないと`rust-analyzer failed to discover workspace`を吐き、`root scanned 0/0`でずっとぐるぐるしてしまう
  - `.vscode/settings.json`に`Cargo.toml`のファイルパスを書いてあげるとよい