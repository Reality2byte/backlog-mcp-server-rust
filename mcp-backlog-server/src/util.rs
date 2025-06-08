use backlog_api_client::DownloadedFile;
use rmcp::Error as McpError;
use std::cmp::Ordering;

pub(crate) enum MatchResult<T> {
    Exact(T),
    Suggestion(Vec<String>),
    None,
}

pub(crate) fn find_by_name_from_array<T: Clone>(
    array: &[T],
    name: &str,
    name_getter: impl Fn(&T) -> &String,
) -> MatchResult<T> {
    let preprocessed_name = |name: &str| name.to_lowercase().replace([' ', '　'], "");

    let name = preprocessed_name(name);

    for m in array {
        if preprocessed_name(name_getter(m)) == name {
            return MatchResult::Exact(m.clone());
        }
    }

    let get_prefix_length = |s1: &str, s2: &str| {
        s1.chars()
            .zip(s2.chars())
            .take_while(|(c1, c2)| c1 == c2)
            .count()
    };

    let mut candidates = Vec::new();
    for m in array {
        let dist = strsim::levenshtein(&name, &preprocessed_name(name_getter(m)));
        if dist <= 2 {
            let prefix_len = get_prefix_length(&name, name_getter(m));
            candidates.push((name_getter(m), dist, prefix_len));
        }
    }

    if candidates.is_empty() {
        MatchResult::None
    } else {
        candidates.sort_by(|a, b| match a.1.cmp(&b.1) {
            Ordering::Equal => b.2.cmp(&a.2),
            other => other,
        });
        MatchResult::Suggestion(
            candidates
                .into_iter()
                .map(|(name, _, _)| name.clone())
                .collect(),
        )
    }
}

pub fn ensure_image_type(
    content_type: &str,
    filename_for_error_message: &str,
) -> Result<(), McpError> {
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

pub fn ensure_text_type(downloaded_file: &DownloadedFile) -> Result<String, McpError> {
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
