# Todo List Application

This is a simple Todo List application that demonstrates how to build a RESTful API using Rust, Axum, and PostgreSQL. It includes examples of:

- Database operations with SQLx
- RESTful API endpoints
- Error handling
- Request/Response serialization
- Application state management

## Prerequisites

- Rust and Cargo installed
- PostgreSQL database
- Environment variables set up

## Setup

1. Create a `.env` file in the project root with your database connection string:
```
DATABASE_URL=postgres://username:password@localhost:5432/your_database
```

2. Make sure you have the users table created (from the previous migration)

3. Build and run the application:
```bash
cargo run
```

## API Endpoints

- `GET /todos` - List all todos
- `POST /todos` - Create a new todo
- `GET /todos/:id` - Get a specific todo
- `PUT /todos/:id` - Update a todo
- `DELETE /todos/:id` - Delete a todo

## Example Usage

Create a new todo:
```bash
curl -X POST http://localhost:3000/todos \
  -H "Content-Type: application/json" \
  -d '{"title": "Learn Rust", "user_id": 1}'
```

List all todos:
```bash
curl http://localhost:3000/todos
```

Update a todo:
```bash
curl -X PUT http://localhost:3000/todos/1 \
  -H "Content-Type: application/json" \
  -d '{"completed": true}'
```

Delete a todo:
```bash
curl -X DELETE http://localhost:3000/todos/1
``` 