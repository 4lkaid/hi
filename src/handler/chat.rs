use axum::{
    extract::ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade},
    response::{Html, IntoResponse},
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::{
    collections::HashSet,
    sync::{Mutex, OnceLock},
};
use tokio::sync::broadcast;

pub struct AppState {
    pub user_set: Mutex<HashSet<String>>,
    pub tx: broadcast::Sender<String>,
}

static APP_STATE: OnceLock<AppState> = OnceLock::new();

pub async fn websocket_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    let state = APP_STATE.get_or_init(|| {
        let user_set = Mutex::new(HashSet::new());
        let (tx, _rx) = broadcast::channel(100);
        AppState { user_set, tx }
    });
    ws.on_upgrade(|socket| websocket(socket, state))
}

async fn websocket(stream: WebSocket, state: &AppState) {
    let (mut sender, mut receiver) = stream.split();
    let mut username = String::new();
    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(name) = message {
            check_username(state, &mut username, name.as_str());
            if !username.is_empty() {
                break;
            } else {
                let _ = sender
                    .send(Message::Text(Utf8Bytes::from_static(
                        "Username already taken.",
                    )))
                    .await;
                return;
            }
        }
    }

    let mut rx = state.tx.subscribe();
    let msg = format!("{username} joined.");
    let _ = state.tx.send(msg);
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::text(msg)).await.is_err() {
                break;
            }
        }
    });

    let tx = state.tx.clone();
    let name = username.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            let _ = tx.send(format!("{name}: {text}"));
        }
    });

    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    };

    let msg = format!("{username} left.");
    let _ = state.tx.send(msg);
    state.user_set.lock().unwrap().remove(&username);
}

fn check_username(state: &AppState, string: &mut String, name: &str) {
    let mut user_set = state.user_set.lock().unwrap();

    if !user_set.contains(name) {
        user_set.insert(name.to_owned());
        string.push_str(name);
    }
}

pub async fn index() -> Html<&'static str> {
    Html(include_str!("../../chat.html"))
}
