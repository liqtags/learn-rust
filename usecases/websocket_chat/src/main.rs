use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

// Message types
#[derive(Debug, Serialize, Deserialize, Clone)]
struct ChatMessage {
    username: String,
    content: String,
    timestamp: chrono::DateTime<chrono::Utc>,
}

// Application state
struct AppState {
    // Broadcast channel for sending messages to all connected clients
    tx: broadcast::Sender<ChatMessage>,
    // Store active connections
    connections: Arc<Mutex<HashMap<String, broadcast::Sender<ChatMessage>>>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Create broadcast channel
    let (tx, _) = broadcast::channel::<ChatMessage>(100);

    // Create application state
    let state = Arc::new(AppState {
        tx,
        connections: Arc::new(Mutex::new(HashMap::new())),
    });

    // Build our application with routes
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state);

    // Run it
    let addr = "127.0.0.1:3000";
    info!("Starting server on {}", addr);
    axum::Server::bind(&addr.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

// WebSocket handler
async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

// Handle WebSocket connection
async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();

    // Create a unique ID for this connection
    let connection_id = uuid::Uuid::new_v4().to_string();

    // Create a new broadcast channel for this connection
    let (tx, mut rx) = broadcast::channel::<ChatMessage>(100);
    {
        let mut connections = state.connections.lock().unwrap();
        connections.insert(connection_id.clone(), tx);
    }

    // Spawn a task to forward messages from the broadcast channel to the WebSocket
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let message = serde_json::to_string(&msg).unwrap();
            if sender.send(Message::Text(message)).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming messages
    while let Some(Ok(message)) = receiver.next().await {
        match message {
            Message::Text(text) => {
                if let Ok(chat_message) = serde_json::from_str::<ChatMessage>(&text) {
                    // Broadcast the message to all connected clients
                    let _ = state.tx.send(chat_message);
                }
            }
            Message::Close(_) => break,
            _ => {}
        }
    }

    // Clean up
    send_task.abort();
    let mut connections = state.connections.lock().unwrap();
    connections.remove(&connection_id);
}

// HTML page for the chat interface
async fn chat_page() -> impl IntoResponse {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>WebSocket Chat</title>
        <style>
            body { font-family: Arial, sans-serif; margin: 0; padding: 20px; }
            #chat { height: 400px; overflow-y: auto; border: 1px solid #ccc; padding: 10px; margin-bottom: 10px; }
            #message { width: 80%; padding: 5px; }
            #send { padding: 5px 10px; }
            .message { margin-bottom: 10px; }
            .username { font-weight: bold; color: #2196F3; }
            .timestamp { color: #666; font-size: 0.8em; }
        </style>
    </head>
    <body>
        <h1>WebSocket Chat</h1>
        <div id="chat"></div>
        <input type="text" id="username" placeholder="Your username" style="width: 200px; margin-right: 10px;">
        <input type="text" id="message" placeholder="Type a message...">
        <button id="send">Send</button>

        <script>
            const chat = document.getElementById('chat');
            const messageInput = document.getElementById('message');
            const usernameInput = document.getElementById('username');
            const sendButton = document.getElementById('send');
            const ws = new WebSocket('ws://localhost:3000/ws');

            ws.onmessage = (event) => {
                const message = JSON.parse(event.data);
                const messageDiv = document.createElement('div');
                messageDiv.className = 'message';
                messageDiv.innerHTML = `
                    <span class="username">${message.username}</span>
                    <span class="timestamp">${new Date(message.timestamp).toLocaleString()}</span>
                    <div>${message.content}</div>
                `;
                chat.appendChild(messageDiv);
                chat.scrollTop = chat.scrollHeight;
            };

            function sendMessage() {
                const username = usernameInput.value.trim();
                const content = messageInput.value.trim();
                
                if (username && content) {
                    const message = {
                        username,
                        content,
                        timestamp: new Date().toISOString()
                    };
                    ws.send(JSON.stringify(message));
                    messageInput.value = '';
                }
            }

            sendButton.onclick = sendMessage;
            messageInput.onkeypress = (e) => {
                if (e.key === 'Enter') sendMessage();
            };
        </script>
    </body>
    </html>
    "#;

    axum::response::Html(html)
} 