use std::sync::Arc;

use pyo3::{types::PyString, Py, PyAny, Python};

use axum::{
    body::Body,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Extension, Router,
};
use axum_typed_websockets::{Message, WebSocket, WebSocketUpgrade};
use serde::{Deserialize, Serialize};

use crate::{
    bridge::{Action, Bridge},
    components::RenderComponent,
};
use crate::{info, s};

pub async fn create_app(host: String, port: String, pyhandler: Py<PyAny>) {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/zenx.png", get(logo))
        .route("/essentials/:filename", get(essentials))
        .route("/ws", get(handler))
        .layer(Extension(Arc::new(pyhandler)));

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<String> {
    info!("/");
    Html(file("index.html"))
}

async fn essentials(
    axum::extract::Path(filename): axum::extract::Path<String>,
) -> impl IntoResponse {
    info!("/essentials");

    match filename.as_str() {
        "app.js" => Ok((
            [(axum::http::header::CONTENT_TYPE, "text/javascript")],
            file("app.js"),
        )),
        "styles.css" => Ok((
            [(axum::http::header::CONTENT_TYPE, "text/css")],
            file("styles.css"),
        )),
        _ => Err((StatusCode::NOT_FOUND, format!("error!"))),
    }
}

async fn logo() -> impl IntoResponse {
    let file = match tokio::fs::File::open("public/zenx.png").await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };

    let stream = tokio_util::io::ReaderStream::new(file);
    let body = Body::from_stream(stream);
    Ok(([(axum::http::header::CONTENT_TYPE, "image/png")], body))
}

/// Gets a file from the `public/` directory.
fn file(s: &str) -> String {
    std::fs::read_to_string(format!("public/{}", s)).unwrap()
}

async fn handler(
    ws: WebSocketUpgrade<ServerMsg, ClientMsg>,
    Extension(ph): Extension<Arc<Py<PyAny>>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| ws_handler(socket, ph))
}

async fn ws_handler(mut socket: WebSocket<ServerMsg, ClientMsg>, ph: Arc<Py<PyAny>>) {
    Python::with_gil(|py| {
        if let Ok(result) = ph.call1(
            py,
            (serde_json::to_string(&Bridge {
                action: Action::Outline,
            })
            .unwrap(),),
        ) {
            let r: Py<PyString> = result.extract(py).unwrap();
            let value: serde_json::Value = serde_json::from_str(r.to_str(py).unwrap()).unwrap();
            info!(format!("(ws) 🐍  Python: {}", value.as_str().unwrap()));
        } else {
            info!("(ws) 🐍  Python: error");
        }
    });

    socket
        .send(Message::Item(ServerMsg {
            d: ServerMsgType::Render {
                components: vec![
                    RenderComponent {
                        id_selector: s!("def:app"),
                        text_content: s!("Hello, Zenx!"),
                        tag: s!("h1"),
                    },
                    RenderComponent {
                        id_selector: s!("def:app"),
                        text_content: s!(
                            "Zenx is a simple web framework, based on Rust and Python."
                        ),
                        tag: s!("p"),
                    },
                ],
            },
        }))
        .await
        .ok();

    while let Ok(Some(msg)) =
        tokio::time::timeout(std::time::Duration::from_secs(30), socket.recv()).await
    {
        match msg {
            Ok(Message::Item(ClientMsg { d })) => match d {
                ClientMsgType::Ping => {
                    info!("(ws) ❤️  beat");
                    socket
                        .send(Message::Item(ServerMsg {
                            d: ServerMsgType::Pong,
                        }))
                        .await
                        .ok();
                }
                ClientMsgType::Rendered(vaults) => {
                    info!("(ws) rendered");
                    println!("{:?}", vaults);
                }
            },
            Ok(Message::Close(_)) => {
                info!("(ws) client disconnected");
                break;
            }
            Ok(_) => {}
            Err(err) => {
                eprintln!("(ws) got error: {}", err);
                break;
            }
        }
    }

    info!("(ws) disconnected.");
}

#[derive(Debug, Serialize)]
enum ServerMsgType {
    #[serde(rename = "pong")]
    Pong,
    #[serde(rename = "render")]
    Render { components: Vec<RenderComponent> },
}

#[derive(Debug, Serialize)]
struct ServerMsg {
    d: ServerMsgType,
}

#[derive(Debug, Deserialize)]
enum ClientMsgType {
    #[serde(rename = "ping")]
    Ping,

    /// The rendered component.
    /// Example:
    /// ```rust
    /// ClientMsgType::Rendered(
    ///     vec![s!("1c2dc526-0d49..."), s!("0277a2ab-9be1...")],
    /// )
    /// ```
    #[serde(rename = "rendered")]
    Rendered(Vec<String>),
}

#[derive(Debug, Deserialize)]
struct ClientMsg {
    d: ClientMsgType,
}
