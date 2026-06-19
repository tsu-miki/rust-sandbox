# axum-sample

axum で最小構成の Web サーバーを作るサンプルです。
`GET /v1/systems/ping` にアクセスすると `pong` が返ります。

```bash
cargo run
# 別ターミナルで
curl http://127.0.0.1:3000/v1/systems/ping   # => pong
```

使用バージョン: axum 0.8 / tokio 1（`Cargo.toml` 参照）。

---

# axum でプロジェクトを作るときに見るべき公式ドキュメント・参考資料

axum で開発するときに参照すべき公式ドキュメントと、参考になる公式 example をまとめた日本語ガイドです。

## 1. 公式ドキュメント（まずここを見る）

| 資料 | URL | 内容 |
|------|-----|------|
| axum API ドキュメント (docs.rs) | https://docs.rs/axum/latest/axum/ | 型・関数のリファレンス。トップページの解説が事実上のチュートリアルになっており最初に通読推奨 |
| axum GitHub リポジトリ | https://github.com/tokio-rs/axum | 本体。README・CHANGELOG・Issue/Discussion |
| axum CHANGELOG | https://github.com/tokio-rs/axum/blob/main/axum/CHANGELOG.md | バージョン間の破壊的変更。アップグレード時に必読 |
| crates.io: axum | https://crates.io/crates/axum | 最新リリースバージョンの確認 |

### 関連クレート（axum は薄く、周辺と組み合わせて使う）

| 資料 | URL | 内容 |
|------|-----|------|
| Tokio 公式サイト / チュートリアル | https://tokio.rs/tokio/tutorial | 非同期ランタイムの基礎。axum の前提知識 |
| tower | https://docs.rs/tower/latest/tower/ | ミドルウェアの基盤（`Service` / `Layer`） |
| tower-http | https://docs.rs/tower-http/latest/tower_http/ | CORS・圧縮・トレース・静的ファイル配信など実用ミドルウェア集 |
| hyper | https://hyper.rs/ | axum が内部で使う HTTP 実装 |
| serde | https://serde.rs/ | JSON などのシリアライズ/デシリアライズ（`Json` エクストラクタで使用） |

## 2. 参考にすべき公式 example

すべて axum 公式リポジトリの `examples/` 配下にあります。
一覧: https://github.com/tokio-rs/axum/tree/main/examples

### 入門・基礎
| example | 用途 |
|---------|------|
| [hello-world](https://github.com/tokio-rs/axum/tree/main/examples/hello-world) | 最小構成。まずこれ |
| [readme](https://github.com/tokio-rs/axum/tree/main/examples/readme) | 公式 README に載っている基本形 |
| [routes-and-handlers-close-together](https://github.com/tokio-rs/axum/tree/main/examples/routes-and-handlers-close-together) | ルートとハンドラの整理方法 |
| [versioning](https://github.com/tokio-rs/axum/tree/main/examples/versioning) | `/v1` のような API バージョニング（本サンプルの構成に関連） |

### リクエスト/レスポンス処理
| example | 用途 |
|---------|------|
| [form](https://github.com/tokio-rs/axum/tree/main/examples/form) | フォーム入力の受け取り |
| [multipart-form](https://github.com/tokio-rs/axum/tree/main/examples/multipart-form) | ファイルアップロード等のマルチパート |
| [parse-body-based-on-content-type](https://github.com/tokio-rs/axum/tree/main/examples/parse-body-based-on-content-type) | Content-Type に応じたボディのパース |
| [global-404-handler](https://github.com/tokio-rs/axum/tree/main/examples/global-404-handler) | 404 などのフォールバック |
| [validator](https://github.com/tokio-rs/axum/tree/main/examples/validator) | 入力バリデーション |

### エラーハンドリング
| example | 用途 |
|---------|------|
| [error-handling](https://github.com/tokio-rs/axum/tree/main/examples/error-handling) | エラー処理の定石 |
| [anyhow-error-response](https://github.com/tokio-rs/axum/tree/main/examples/anyhow-error-response) | `anyhow` を使ったエラー返却 |
| [customize-extractor-error](https://github.com/tokio-rs/axum/tree/main/examples/customize-extractor-error) | エクストラクタのエラーをカスタマイズ |

### ミドルウェア・横断的関心事
| example | 用途 |
|---------|------|
| [tracing-aka-logging](https://github.com/tokio-rs/axum/tree/main/examples/tracing-aka-logging) | ログ/トレーシング |
| [cors](https://github.com/tokio-rs/axum/tree/main/examples/cors) | CORS 設定 |
| [compression](https://github.com/tokio-rs/axum/tree/main/examples/compression) | レスポンス圧縮 |
| [request-id](https://github.com/tokio-rs/axum/tree/main/examples/request-id) | リクエスト ID 付与 |
| [print-request-response](https://github.com/tokio-rs/axum/tree/main/examples/print-request-response) | 自作ミドルウェアの基本形 |

### 状態管理・DI・DB
| example | 用途 |
|---------|------|
| [dependency-injection](https://github.com/tokio-rs/axum/tree/main/examples/dependency-injection) | 状態(State)・依存性の注入 |
| [key-value-store](https://github.com/tokio-rs/axum/tree/main/examples/key-value-store) | インメモリ状態を共有する例 |
| [todos](https://github.com/tokio-rs/axum/tree/main/examples/todos) | CRUD API の総合例 |
| [sqlx-postgres](https://github.com/tokio-rs/axum/tree/main/examples/sqlx-postgres) | SQLx + PostgreSQL |
| [diesel-async-postgres](https://github.com/tokio-rs/axum/tree/main/examples/diesel-async-postgres) | Diesel(async) + PostgreSQL |
| [tokio-redis](https://github.com/tokio-rs/axum/tree/main/examples/tokio-redis) | Redis 連携 |

### 認証
| example | 用途 |
|---------|------|
| [jwt](https://github.com/tokio-rs/axum/tree/main/examples/jwt) | JWT 認証 |
| [oauth](https://github.com/tokio-rs/axum/tree/main/examples/oauth) | OAuth 認証 |

### リアルタイム通信
| example | 用途 |
|---------|------|
| [websockets](https://github.com/tokio-rs/axum/tree/main/examples/websockets) | WebSocket |
| [sse](https://github.com/tokio-rs/axum/tree/main/examples/sse) | Server-Sent Events |
| [chat](https://github.com/tokio-rs/axum/tree/main/examples/chat) | WebSocket を使ったチャット |

### 静的配信・テンプレート
| example | 用途 |
|---------|------|
| [static-file-server](https://github.com/tokio-rs/axum/tree/main/examples/static-file-server) | 静的ファイル配信 |
| [templates](https://github.com/tokio-rs/axum/tree/main/examples/templates) | HTML テンプレート（askama） |
| [templates-minijinja](https://github.com/tokio-rs/axum/tree/main/examples/templates-minijinja) | HTML テンプレート（minijinja） |

### テスト・運用
| example | 用途 |
|---------|------|
| [testing](https://github.com/tokio-rs/axum/tree/main/examples/testing) | ハンドラ/ルーターのテスト |
| [testing-websockets](https://github.com/tokio-rs/axum/tree/main/examples/testing-websockets) | WebSocket のテスト |
| [graceful-shutdown](https://github.com/tokio-rs/axum/tree/main/examples/graceful-shutdown) | グレースフルシャットダウン |
| [tls-rustls](https://github.com/tokio-rs/axum/tree/main/examples/tls-rustls) | TLS(HTTPS) 対応 |
| [prometheus-metrics](https://github.com/tokio-rs/axum/tree/main/examples/prometheus-metrics) | Prometheus メトリクス |

## 3. おすすめの読む順番

1. Tokio チュートリアルで非同期の基礎を押さえる
2. axum docs.rs トップの解説を読む
3. `hello-world` → `todos` の順に example を写経
4. 必要に応じてミドルウェア（`tracing` / `cors`）・DB（`sqlx-postgres`）・認証（`jwt`）の example を参照
5. アップグレード時は CHANGELOG を必ず確認

> 注: example は最新版（main ブランチ）を指しています。使用中の axum バージョンに合わせる場合は、リポジトリのタグ（例: `axum-v0.8.x`）を切り替えて参照してください。
