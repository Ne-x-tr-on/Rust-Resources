use axum::{
    routing::get,
    Router,
    extract::Path,
    response::{Html, IntoResponse},
};
use std::time::Duration;
use tokio::sync::Mutex;
use std::{sync::Arc, io::Write};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Open serial connection to Arduino on COM4
    let serial_port = serialport::new("COM4", 9600)
        .timeout(Duration::from_secs(1))
        .open()
        .expect("⚠️ Failed to open COM4 — check your Arduino port");

    // Give Arduino 2 seconds to reset after opening serial
    tokio::time::sleep(Duration::from_millis(2000)).await;

    println!("✅ Connected to Arduino on COM4");

    let shared_port = Arc::new(Mutex::new(serial_port));

    let app = Router::new()
        // Web dashboard
        .route("/", get(dashboard))
        // LED control endpoint
        .route("/led/:state", get({
            let port = shared_port.clone();
            move |Path(state): Path<String>| async move {
                let mut port = port.lock().await;
                if state == "on" {
                    writeln!(port, "ON").unwrap();
                    println!("💡 Sent: ON");
                    "💡 LED ON".into_response()
                } else if state == "off" {
                    writeln!(port, "OFF").unwrap();
                    println!("💤 Sent: OFF");
                    "💤 LED OFF".into_response()
                } else {
                    println!("❌ Invalid command received: {}", state);
                    "❌ Invalid command".into_response()
                }
            }
        }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://localhost:3000");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// HTML dashboard
async fn dashboard() -> Html<&'static str> {
    Html(r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Arduino LED Control</title>
            <style>
                body {
                    background-color: #0f172a;
                    color: white;
                    font-family: Arial, sans-serif;
                    text-align: center;
                    margin-top: 10%;
                }
                h1 {
                    font-size: 2rem;
                    margin-bottom: 30px;
                }
                button {
                    font-size: 1.2rem;
                    padding: 15px 30px;
                    border: none;
                    border-radius: 10px;
                    cursor: pointer;
                    margin: 10px;
                    transition: 0.3s;
                }
                .on {
                    background-color: #22c55e;
                    color: white;
                }
                .on:hover {
                    background-color: #16a34a;
                }
                .off {
                    background-color: #ef4444;
                    color: white;
                }
                .off:hover {
                    background-color: #dc2626;
                }
            </style>
        </head>
        <body>
            <h1>💡 Arduino LED Control Panel</h1>
            <button class="on" onclick="toggleLED('on')">Turn ON</button>
            <button class="off" onclick="toggleLED('off')">Turn OFF</button>

            <script>
                async function toggleLED(state) {
                    const response = await fetch(`/led/${state}`);
                    const text = await response.text();
                    alert(text);
                }
            </script>
        </body>
        </html>
    "#)
}
