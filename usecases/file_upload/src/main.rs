use axum::{
    body::Bytes,
    extract::{multipart::Multipart, Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::{
    path::PathBuf,
    sync::Arc,
};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use uuid::Uuid;

// File metadata
#[derive(Debug, Serialize, Deserialize)]
struct FileMetadata {
    id: String,
    filename: String,
    content_type: String,
    size: u64,
    uploaded_at: chrono::DateTime<chrono::Utc>,
}

// Application state
struct AppState {
    upload_dir: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Create upload directory if it doesn't exist
    let upload_dir = PathBuf::from("uploads");
    fs::create_dir_all(&upload_dir).await?;

    // Create application state
    let state = Arc::new(AppState { upload_dir });

    // Build our application with routes
    let app = Router::new()
        .route("/", get(upload_page))
        .route("/upload", post(upload_file))
        .route("/files/:id", get(get_file))
        .with_state(state);

    // Run it
    let addr = "127.0.0.1:3000";
    info!("Starting server on {}", addr);
    axum::Server::bind(&addr.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

// Upload page handler
async fn upload_page() -> impl IntoResponse {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>File Upload</title>
        <style>
            body { font-family: Arial, sans-serif; margin: 0; padding: 20px; }
            .container { max-width: 800px; margin: 0 auto; }
            .upload-form { margin: 20px 0; padding: 20px; border: 1px solid #ccc; }
            .file-list { margin-top: 20px; }
            .file-item { margin: 10px 0; padding: 10px; border: 1px solid #eee; }
            .progress { margin-top: 10px; }
            .progress-bar { height: 20px; background-color: #f0f0f0; }
            .progress-bar-fill { height: 100%; background-color: #4CAF50; width: 0%; }
        </style>
    </head>
    <body>
        <div class="container">
            <h1>File Upload</h1>
            <div class="upload-form">
                <form id="uploadForm">
                    <input type="file" id="fileInput" multiple>
                    <button type="submit">Upload</button>
                </form>
                <div class="progress">
                    <div class="progress-bar">
                        <div class="progress-bar-fill"></div>
                    </div>
                </div>
            </div>
            <div class="file-list" id="fileList"></div>
        </div>

        <script>
            const form = document.getElementById('uploadForm');
            const fileInput = document.getElementById('fileInput');
            const fileList = document.getElementById('fileList');
            const progressBar = document.querySelector('.progress-bar-fill');

            form.onsubmit = async (e) => {
                e.preventDefault();
                const files = fileInput.files;
                
                for (const file of files) {
                    const formData = new FormData();
                    formData.append('file', file);

                    try {
                        const response = await fetch('/upload', {
                            method: 'POST',
                            body: formData
                        });

                        if (response.ok) {
                            const data = await response.json();
                            addFileToList(data);
                        } else {
                            console.error('Upload failed:', await response.text());
                        }
                    } catch (error) {
                        console.error('Error:', error);
                    }
                }
            };

            function addFileToList(file) {
                const div = document.createElement('div');
                div.className = 'file-item';
                div.innerHTML = `
                    <div>Filename: ${file.filename}</div>
                    <div>Size: ${formatSize(file.size)}</div>
                    <div>Uploaded: ${new Date(file.uploaded_at).toLocaleString()}</div>
                    <a href="/files/${file.id}" target="_blank">Download</a>
                `;
                fileList.appendChild(div);
            }

            function formatSize(bytes) {
                const units = ['B', 'KB', 'MB', 'GB'];
                let size = bytes;
                let unitIndex = 0;
                while (size >= 1024 && unitIndex < units.length - 1) {
                    size /= 1024;
                    unitIndex++;
                }
                return `${size.toFixed(2)} ${units[unitIndex]}`;
            }
        </script>
    </body>
    </html>
    "#;

    Html(html)
}

// File upload handler
async fn upload_file(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut file_metadata = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        (StatusCode::BAD_REQUEST, format!("Failed to process multipart form: {}", e))
    })? {
        let filename = field.file_name()
            .map(ToString::to_string)
            .ok_or((StatusCode::BAD_REQUEST, "No filename provided".to_string()))?;

        let content_type = field.content_type()
            .map(ToString::to_string)
            .unwrap_or_else(|| "application/octet-stream".to_string());

        let data = field.bytes().await.map_err(|e| {
            (StatusCode::BAD_REQUEST, format!("Failed to read file data: {}", e))
        })?;

        // Generate unique filename
        let file_id = Uuid::new_v4().to_string();
        let file_path = state.upload_dir.join(&file_id);

        // Save file
        let mut file = File::create(&file_path).await.map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create file: {}", e))
        })?;

        file.write_all(&data).await.map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to write file: {}", e))
        })?;

        // Create metadata
        file_metadata = Some(FileMetadata {
            id: file_id,
            filename,
            content_type,
            size: data.len() as u64,
            uploaded_at: chrono::Utc::now(),
        });
    }

    let metadata = file_metadata.ok_or((StatusCode::BAD_REQUEST, "No file uploaded".to_string()))?;
    Ok(axum::Json(metadata))
}

// File download handler
async fn get_file(
    State(state): State<Arc<AppState>>,
    Path(file_id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let file_path = state.upload_dir.join(&file_id);
    
    if !file_path.exists() {
        return Err((StatusCode::NOT_FOUND, "File not found".to_string()));
    }

    let file = File::open(&file_path).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to open file: {}", e))
    })?;

    Ok(axum::body::StreamBody::new(file))
} 