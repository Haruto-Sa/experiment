use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000").expect("failed to bind 0.0.0.0:3000");
    println!("Webゲーム向けページを http://localhost:3000 で公開中");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(error) = handle_connection(stream) {
                    eprintln!("connection error: {error}");
                }
            }
            Err(error) => eprintln!("incoming stream failed: {error}"),
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0_u8; 1024];
    let _ = stream.read(&mut buffer)?;

    let body = PAGE.as_bytes();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );

    stream.write_all(response.as_bytes())?;
    stream.write_all(body)?;
    stream.flush()?;
    Ok(())
}

const PAGE: &str = r#"<!doctype html>
<html lang="ja">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Rust Web Game Studio | 全体包括ページ</title>
    <style>
      :root {
        color-scheme: dark;
        --bg: #0b1020;
        --panel: #141b33;
        --line: #2b3868;
        --text: #e9ecff;
        --sub: #aeb8e6;
        --accent: #7aa2ff;
        --accent-2: #61d3b8;
      }

      * {
        box-sizing: border-box;
      }

      body {
        margin: 0;
        font-family: "Inter", "Noto Sans JP", system-ui, sans-serif;
        background: radial-gradient(circle at 20% 0%, #1a2650 0%, var(--bg) 35%);
        color: var(--text);
        line-height: 1.65;
      }

      .container {
        max-width: 1100px;
        margin: 0 auto;
        padding: 24px;
      }

      .hero {
        padding: 40px 24px;
        border: 1px solid var(--line);
        border-radius: 18px;
        background: linear-gradient(145deg, rgba(122, 162, 255, 0.12), rgba(97, 211, 184, 0.08));
      }

      h1 {
        margin-top: 0;
        font-size: clamp(1.8rem, 3vw, 2.7rem);
      }

      h2 {
        margin: 0 0 12px;
        font-size: 1.25rem;
      }

      p {
        color: var(--sub);
      }

      .grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
        gap: 16px;
        margin-top: 20px;
      }

      .card {
        border: 1px solid var(--line);
        border-radius: 14px;
        padding: 18px;
        background: var(--panel);
      }

      .pill {
        display: inline-block;
        padding: 4px 10px;
        border-radius: 999px;
        border: 1px solid var(--line);
        color: var(--accent);
        font-size: 0.8rem;
        margin-bottom: 10px;
      }

      .two-col {
        margin-top: 24px;
        display: grid;
        grid-template-columns: 1.2fr 1fr;
        gap: 16px;
      }

      .todo li {
        margin-bottom: 8px;
      }

      .board {
        margin-top: 10px;
        display: grid;
        grid-template-columns: repeat(4, 1fr);
        gap: 8px;
      }

      .tile {
        border: 1px solid var(--line);
        border-radius: 10px;
        min-height: 72px;
        display: flex;
        justify-content: center;
        align-items: center;
        color: var(--accent-2);
      }

      footer {
        margin: 24px 0 8px;
        color: var(--sub);
        font-size: 0.9rem;
      }

      @media (max-width: 820px) {
        .two-col {
          grid-template-columns: 1fr;
        }
      }
    </style>
  </head>
  <body>
    <main class="container">
      <section class="hero">
        <span class="pill">Rust-based Web Game Blueprint</span>
        <h1>Webゲーム制作のための全体包括ページ</h1>
        <p>
          このページは、Rustを中心にWebゲームを作るための「入口」です。ゲーム設計、技術選定、開発フロー、
          MVP（最小構成）までをひと目で把握できます。
        </p>
      </section>

      <section class="grid" aria-label="overview">
        <article class="card">
          <h2>1. 推奨構成</h2>
          <p>
            描画はCanvas/WebGL、コアロジックはRust。WASM化してブラウザへ配信し、
            APIやランキングはRustサーバーで統一。
          </p>
        </article>
        <article class="card">
          <h2>2. 開発ステップ</h2>
          <p>
            企画 → ゲームループ実装 → 入力/当たり判定 → UI/HUD → セーブ/スコア連携 → デプロイ
            の順で進めると破綻しにくいです。
          </p>
        </article>
        <article class="card">
          <h2>3. 主要技術</h2>
          <p>
            Rust / wasm-bindgen / web-sys / Bevy(将来拡張) / サーバーAPI。
            まずはシンプル構成で速度より保守性を優先します。
          </p>
        </article>
      </section>

      <section class="two-col" aria-label="mvp-plan">
        <article class="card">
          <h2>MVP TODO</h2>
          <ol class="todo">
            <li>1画面・1目的の小ゲームを決める（例: 回避 or クリック）</li>
            <li>Rustで状態管理（スコア、タイマー、HP）を実装</li>
            <li>WASMで描画更新を接続</li>
            <li>ゲームオーバーとリトライ導線を用意</li>
            <li>最低限の効果音と演出を追加</li>
          </ol>
        </article>

        <article class="card">
          <h2>画面レイアウト例（4x4）</h2>
          <p>UIイメージ用のダミーグリッドです。次のステップで実プレイ画面へ置き換えます。</p>
          <div class="board" role="presentation">
            <div class="tile">HUD</div><div class="tile">MAP</div><div class="tile">MAP</div><div class="tile">LOG</div>
            <div class="tile">MAP</div><div class="tile">MAP</div><div class="tile">MAP</div><div class="tile">LOG</div>
            <div class="tile">MAP</div><div class="tile">MAP</div><div class="tile">MAP</div><div class="tile">LOG</div>
            <div class="tile">BTN</div><div class="tile">BTN</div><div class="tile">BTN</div><div class="tile">STATUS</div>
          </div>
        </article>
      </section>

      <footer>
        RustサーバーがHTMLを直接返す構成です。次回はWASMゲーム本体と接続します。
      </footer>
    </main>
  </body>
</html>
"#;
