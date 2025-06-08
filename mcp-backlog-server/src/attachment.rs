use backlog_api_client::{DownloadedFile, bytes};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64_STANDARD};
use rmcp::{Error as McpError, model::Content};

pub struct SerializableRawAttachment {
    filename: String,
    content_type: String,
    content: SerializableRawAttachmentContent,
}

pub enum SerializableRawAttachmentContent {
    Image(bytes::Bytes),
    Text(String),
    Raw(bytes::Bytes),
}

impl SerializableRawAttachment {
    pub fn text(file: DownloadedFile) -> Result<Self, McpError> {
        let filename = file.filename.clone();
        let content_type = file.content_type.clone();
        let content = SerializableRawAttachmentContent::text(file)?;
        Ok(Self {
            filename,
            content_type,
            content,
        })
    }

    pub fn image(file: DownloadedFile) -> Result<Self, McpError> {
        let filename = file.filename.clone();
        let content_type = file.content_type.clone();
        let content = SerializableRawAttachmentContent::image(file)?;
        Ok(Self {
            filename,
            content_type,
            content,
        })
    }

    pub fn raw(file: DownloadedFile) -> Self {
        let filename = file.filename.clone();
        let content_type = file.content_type.clone();
        let content = SerializableRawAttachmentContent::raw(file);
        Self {
            filename,
            content_type,
            content,
        }
    }
}

impl SerializableRawAttachmentContent {
    fn text(file: DownloadedFile) -> Result<Self, McpError> {
        let content = ensure_text_type(&file)?;
        Ok(Self::Text(content))
    }

    fn image(file: DownloadedFile) -> Result<Self, McpError> {
        ensure_image_type(&file.content_type, &file.filename)?;
        Ok(Self::Image(file.bytes))
    }

    fn raw(file: DownloadedFile) -> Self {
        Self::Raw(file.bytes)
    }
}

impl TryFrom<SerializableRawAttachment> for Content {
    type Error = McpError;
    fn try_from(file: SerializableRawAttachment) -> Result<Self, Self::Error> {
        match file.content {
            SerializableRawAttachmentContent::Text(text) => Ok(Content::text(text)),
            SerializableRawAttachmentContent::Image(bytes) => Ok(Content::image(
                BASE64_STANDARD.encode(bytes),
                file.content_type,
            )),
            SerializableRawAttachmentContent::Raw(bytes) => Ok(Content::json(serde_json::json!({
                "filename": file.filename,
                "content_type": file.content_type,
                "content": BASE64_STANDARD.encode(bytes),
            }))?),
        }
    }
}

fn ensure_image_type(content_type: &str, filename_for_error_message: &str) -> Result<(), McpError> {
    if !content_type.starts_with("image/") {
        return Err(McpError::invalid_request(
            format!(
                "Attachment '{}' is not an image. Reported content type: {}",
                filename_for_error_message, content_type
            ),
            None,
        ));
    }
    Ok(())
}

fn ensure_text_type(downloaded_file: &DownloadedFile) -> Result<String, McpError> {
    match String::from_utf8(downloaded_file.bytes.to_vec()) {
        Ok(text_content) => Ok(text_content),
        Err(_) => Err(McpError::invalid_request(
            format!(
                "Attachment '{}' is not a valid UTF-8 text file.",
                downloaded_file.filename
            ),
            None,
        )),
    }
}
