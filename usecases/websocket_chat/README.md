# WebSocket Chat Application

This example demonstrates how to build a real-time chat application using WebSocket connections in Rust. It includes:

- WebSocket server implementation
- Real-time message broadcasting
- Simple chat interface
- Connection management
- Error handling

## Features

- Real-time message delivery
- Multiple client support
- Clean connection handling
- Simple and intuitive UI
- Timestamp tracking
- Username support

## Prerequisites

- Rust and Cargo installed
- Modern web browser

## Setup

1. Build and run the application:
```bash
cargo run
```

2. Open your web browser and navigate to:
```
http://localhost:3000
```

## How It Works

1. **WebSocket Connection**
   - Clients connect to the WebSocket endpoint at `/ws`
   - Each connection gets a unique ID
   - Messages are broadcast to all connected clients

2. **Message Flow**
   - Client sends a message through WebSocket
   - Server receives and validates the message
   - Message is broadcast to all connected clients
   - Clients receive and display the message

3. **Connection Management**
   - New connections are tracked
   - Disconnected clients are automatically cleaned up
   - Each client has its own broadcast channel

## Usage

1. Enter your username in the input field
2. Type your message in the message input
3. Press Enter or click Send to broadcast your message
4. Messages from other users will appear in real-time

## Technical Details

- Uses Axum's WebSocket support
- Implements broadcast channels for message distribution
- Handles connection lifecycle
- Provides clean error handling
- Includes a simple but functional UI

## Error Handling

The application handles various error cases:
- Connection failures
- Message parsing errors
- Disconnection cleanup
- Invalid message formats 