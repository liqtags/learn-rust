# Authentication API

This example demonstrates how to implement JWT-based authentication in a Rust API using Axum. It includes:

- User registration and login
- JWT token generation and validation
- Password hashing with bcrypt
- Protected routes
- Error handling

## Prerequisites

- Rust and Cargo installed
- PostgreSQL database
- Environment variables set up

## Setup

1. Create a `.env` file in the project root with your configuration:
```
DATABASE_URL=postgres://username:password@localhost:5432/your_database
JWT_SECRET=your-secret-key-here
```

2. Make sure you have the users table created (from the previous migration)

3. Build and run the application:
```bash
cargo run
```

## API Endpoints

- `POST /auth/register` - Register a new user
- `POST /auth/login` - Login and get JWT token
- `GET /auth/me` - Get current user info (protected route)

## Example Usage

Register a new user:
```bash
curl -X POST http://localhost:3000/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "john_doe",
    "email": "john@example.com",
    "password": "secure_password"
  }'
```

Login:
```bash
curl -X POST http://localhost:3000/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "john_doe",
    "password": "secure_password"
  }'
```

Get current user info (protected route):
```bash
curl http://localhost:3000/auth/me \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

## Security Features

- Passwords are hashed using bcrypt
- JWT tokens expire after 24 hours
- Protected routes require valid JWT token
- Secure password storage
- Input validation

## Error Handling

The API returns appropriate HTTP status codes and error messages:
- 400 Bad Request - Invalid input
- 401 Unauthorized - Invalid credentials or missing token
- 500 Internal Server Error - Server-side errors 