use std::fs;
use std::io::Write;

pub async fn download_file(url: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let response = {
        let url_str = {
            let mut s = String::with_capacity(url.len());
            s.push_str(url);
            s.into_boxed_str()
        };

        reqwest::get(url)
            .await
            .map_err(|e| {
                let mut err = String::with_capacity(32 + e.to_string().len());
                err.push_str("Failed to make request: ");
                err.push_str(&e.to_string());
                Box::new(std::io::Error::new(std::io::ErrorKind::Other, err))
                    as Box<dyn std::error::Error + Send + Sync>
            })?
    };

    match response.status().as_u16() {
        200..=299 => {
            let bytes = {
                let bytes_result = response.bytes().await;
                bytes_result.map_err(|e| {
                    let mut err = String::with_capacity(32 + e.to_string().len());
                    err.push_str("Failed to get bytes: ");
                    err.push_str(&e.to_string());
                    Box::new(std::io::Error::new(std::io::ErrorKind::Other, err))
                        as Box<dyn std::error::Error + Send + Sync>
                })?
            };

            let file = {
                let path = std::path::Path::new(output_path);
                let parent = path.parent().ok_or_else(|| {
                    let err = format!("Invalid path parent: {}", output_path);
                    Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, err))
                        as Box<dyn std::error::Error + Send + Sync>
                })?;

                std::fs::create_dir_all(parent).map_err(|e| {
                    let mut err = String::with_capacity(32 + e.to_string().len());
                    err.push_str("Failed to create directories: ");
                    err.push_str(&e.to_string());
                    Box::new(std::io::Error::new(std::io::ErrorKind::Other, err))
                        as Box<dyn std::error::Error + Send + Sync>
                })?;

                std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(path)
                    .map_err(|e| {
                        let mut err = String::with_capacity(32 + e.to_string().len());
                        err.push_str("Failed to create file: ");
                        err.push_str(&e.to_string());
                        Box::new(std::io::Error::new(std::io::ErrorKind::Other, err))
                            as Box<dyn std::error::Error + Send + Sync>
                    })?
            };

            tokio::task::spawn_blocking(move || {
                use std::io::Write;
                let mut file = file;
                file.write_all(&bytes).map_err(|e| {
                    let mut err = String::with_capacity(32 + e.to_string().len());
                    err.push_str("Failed to write file: ");
                    err.push_str(&e.to_string());
                    Box::new(std::io::Error::new(std::io::ErrorKind::Other, err))
                        as Box<dyn std::error::Error + Send + Sync>
                })
            })
                .await
                .map_err(|e| {
                    Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Join error: {}", e),
                    )) as Box<dyn std::error::Error + Send + Sync>
                })??;

            Ok(())
        }
        status_code => {
            let status = response.status();
            let mut err_msg = String::with_capacity(32);
            err_msg.push_str("HTTP request failed with status: ");
            err_msg.push_str(&status.to_string());

            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                err_msg,
            )) as Box<dyn std::error::Error + Send + Sync>)
        }
    }
}