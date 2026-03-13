# rust_showcase

Rustの特徴（所有権、列挙型によるエラー処理、trait、イテレータ、スレッド並列処理）を短いコードで体験できる、テキスト解析CLIです。

## ディレクトリ構成

```
.
├── Cargo.toml
├── README.md
└── src
    ├── lib.rs   # 解析ロジック（再利用可能なライブラリ）
    └── main.rs  # CLIエントリポイント
```

## できること

入力テキストに対して次を計算します。

- 文字数
- 単語数
- 行数
- 単語頻度（上位5件を表示）

## 使い方

```bash
cargo run -- "Rust is fast. Rust is safe."
```

## 実装メモ

- `Analyzer` traitで責務を抽象化
- `AnalyzeError`（`enum`）で明示的な失敗を表現
- `thread::scope`で借用を保ったまま並列集計
- イテレータチェーンで頻度表を構築

## テスト

```bash
cargo test
```
