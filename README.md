# rust-bicycle-book
## Memo
- rust-analyzerはrootに`Cargo.toml`がないと`rust-analyzer failed to discover workspace`を吐き、`root scanned 0/0`でずっとぐるぐるしてしまう
  - `.vscode/settings.json`に`Cargo.toml`のファイルパスを書いてあげるとよい
- chapter03
  - step3 (次3-5-7から)
    - `the trait bound T: Ord is not satisfied`を解決するために、secondから`T: Ord`を修正した
      - なぜ動いたかがよくわからない
      - トレイトの理解がひつようそう
    - クロージャがイマイチわからない
      - `&`がついたりつかなかったり。。