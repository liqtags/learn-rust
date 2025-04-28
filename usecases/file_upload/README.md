# File Upload Application

This example demonstrates how to build a file upload and storage system in Rust. It includes:

- File upload handling
- Secure file storage
- File metadata tracking
- File download functionality
- Simple web interface

## Features

- Multiple file upload support
- Progress tracking
- File metadata storage
- Secure file naming
- Download functionality
- Simple and intuitive UI

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

1. **File Upload**
   - Files are uploaded through a multipart form
   - Each file gets a unique UUID
   - Files are stored in the `uploads` directory
   - Metadata is tracked for each file

2. **File Storage**
   - Files are stored with unique identifiers
   - Original filenames are preserved in metadata
   - Content types are tracked
   - File sizes are recorded

3. **File Download**
   - Files can be downloaded by their ID
   - Stream-based file serving
   - Proper error handling
   - Content type preservation

## Usage

1. Open the web interface
2. Select one or more files to upload
3. Click Upload to start the process
4. View uploaded files in the list
5. Click Download to retrieve a file

## Technical Details

- Uses Axum's multipart support
- Implements streaming file handling
- Provides clean error handling
- Includes a simple but functional UI
- Handles file metadata

## Security Features

- Unique file identifiers
- Secure file storage
- Content type validation
- File size tracking
- Error handling for invalid uploads

## Error Handling

The application handles various error cases:
- Invalid file uploads
- Missing files
- Storage errors
- Download failures
- Invalid file types 