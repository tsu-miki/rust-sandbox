# rust-sandbox

Rust学習用のサンドボックスリポジトリ。

## 🦀 Rust Daily 10min

`docs/` は毎朝10分程度で解ける Rust の練習問題を出題する静的Webアプリです（GitHub Pages想定）。
クイズ・出力予想・バグ探し・リファクタリングなど複数の形式の問題を日替わりで出題し、
スマホのブラウザから毎朝開いて解く用途を想定しています。

- 問題データ: `docs/problems.json`
- アプリ本体: `docs/index.html`, `docs/app.js`, `docs/style.css`
- ローカル確認: `cd docs && python3 -m http.server 8000` の後 `http://localhost:8000` を開く
- 公開する場合: リポジトリの Settings > Pages で Source を「Deploy from a branch」、
  ブランチを `main`、フォルダを `/docs` に設定してください。

## その他のディレクトリ

- `ownership/`: 所有権・借用に関する学習用サンプル
- `axum-sample/`: axum を使った最小限のAPIサンプル
