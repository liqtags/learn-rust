# Learn Rust by Example

This repository contains a collection of practical examples demonstrating various aspects of Rust programming, with a focus on web development and backend services. Each example is designed to be self-contained and demonstrates specific concepts and best practices.

## Examples

### 1. Database Migrations
A database schema management system that demonstrates:
- SQL migration files
- Up and down migrations
- Table creation and modification
- Index management
- Foreign key constraints
- Database versioning

[View Example](migrations/)

### 2. Todo List Application
A simple RESTful API that demonstrates:
- Database operations with SQLx
- RESTful API endpoints
- Error handling
- Request/Response serialization
- Application state management
- CRUD operations
- Database relationships

[View Example](usecases/todo_list/)

### 3. Authentication API
A JWT-based authentication system that shows:
- User registration and login
- JWT token generation and validation
- Password hashing with bcrypt
- Protected routes
- Error handling
- Secure password storage
- Token-based authentication

[View Example](usecases/auth_api/)

### 4. WebSocket Chat
A real-time chat application demonstrating:
- WebSocket server implementation
- Real-time message broadcasting
- Connection management
- Simple chat interface
- Error handling
- Asynchronous communication
- State management

[View Example](usecases/websocket_chat/)

### 5. File Upload System
A file handling system that showcases:
- File upload handling
- Secure file storage
- File metadata tracking
- File download functionality
- Simple web interface
- Multipart form processing
- Stream-based file handling

[View Example](usecases/file_upload/)

## Key Concepts Covered

### Database Management
- SQL migrations
- Schema versioning
- Table creation and modification
- Index management
- Foreign key constraints
- Database relationships

### Web Development
- RESTful API design
- WebSocket implementation
- File handling
- Authentication and authorization
- Error handling
- State management

### Security
- Password hashing
- JWT token management
- Secure file storage
- Protected routes
- Input validation

### Asynchronous Programming
- Async/await patterns
- WebSocket connections
- File streaming
- Database operations
- Error handling

### Best Practices
- Clean code organization
- Error handling patterns
- Documentation
- Testing
- Security considerations

## Getting Started

### Prerequisites
- Rust and Cargo installed
- PostgreSQL database (for examples that use databases)
- Modern web browser (for web-based examples)

### Running the Examples

1. Clone the repository:
```bash
git clone https://github.com/liqtags/learn-rust.git
cd learn-rust
```

2. Navigate to the example directory:
```bash
cd usecases/example_name
```

3. Build and run:
```bash
cargo run
```

## Project Structure

```
learn-rust/
├── migrations/
│   ├── 20240321000000_create_users_table.sql
│   └── 20240321000000_create_users_table.down.sql
├── usecases/
│   ├── todo_list/
│   ├── auth_api/
│   ├── websocket_chat/
│   └── file_upload/
└── README.md
```

## Acknowledgments

- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [SQLx](https://github.com/launchbadge/sqlx) - Database toolkit
- [Tokio](https://github.com/tokio-rs/tokio) - Async runtime
- [Serde](https://github.com/serde-rs/serde) - Serialization framework
- [jsonwebtoken](https://github.com/Keats/jsonwebtoken) - JWT implementation
- [bcrypt](https://github.com/Keats/rust-bcrypt) - Password hashing
