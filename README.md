#aideaTool for Rust Actix

## フォルダ構成

```
├── Cargo.toml
└── src/
    ├── main.rs
    └── controller/
        ├── mod.rs
        └── health_check.rs
    └── services/
        ├── mod.rs
        └── health_check.rs
    └── structs/
        ├── mod.rs
        └── health_check.rs
```

- main  
  rust のエントリーポイント  
  `async fn main()`の関数内で web サーバーの設定を行っている  
  書く module の宣言等を行っている  
  直接 main.rs に関数を記載することもできる

- controller  
  ルーティングで指定された URL に対してリクエストを受け取り  
  そのリクエストに応じた処理を行う  
  main.rs の記載を長くしないためにルーティングを分散させる

- services  
  ビジネスロジックを記載する  
  controller と処理を分散させる

- structs  
  型の定義を記載する  
  controller,services で共通した struct を使う
