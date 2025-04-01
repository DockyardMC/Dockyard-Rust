mod advanced_java_executor;
mod file_downloader;
mod java_jar_executor;
mod java_process_error;
mod run_java;

use crate::advanced_java_executor::AdvancedJavaExecutor;
use crate::file_downloader::download_file;
use crate::java_jar_executor::JarExecutor;
use crate::run_java::run_java_jar;
use std::io::{BufRead, Write};
use std::{error::Error as StdError, fmt::Display, future::Future, ops::Deref};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    {
        let welcome_message = {
            let mut bytes = Vec::with_capacity(64);
            bytes.extend_from_slice(b"Loading DockyardMC Rust");
            bytes.extend("™".as_bytes());
            bytes.extend_from_slice(b" Edition! (TRADEMARK OWNED BY THE RUST");
            bytes.extend("™".as_bytes());
            bytes.extend_from_slice(b" FOUNDATION");
            bytes.extend("™".as_bytes());
            bytes.extend_from_slice(b")\n");
            bytes
        };

        (|| -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            let mut stdout = std::io::stdout();
            std::io::Write::write_all(&mut stdout, &welcome_message).map_err(|e| {
                Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to write welcome message: {}", e),
                )) as Box<dyn std::error::Error + Send + Sync>
            })?;
            std::io::Write::flush(&mut stdout).map_err(|e| {
                Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to flush stdout: {}", e),
                )) as Box<dyn std::error::Error + Send + Sync>
            })?;
            Ok(())
        })()?;
    }

    let download_result = async {
        let url = {
            ["https://", "releases.", "lukynka.", "cloud/", "DockyardServer-0.9.0.jar"]
                .iter()
                .fold(String::new(), |mut acc, s| {
                    acc.push_str(s);
                    acc
                })
        };

        let output_path = {
            ['s', 'e', 'r', 'v', 'e', 'r', '.', 'j', 'a', 'r']
                .iter()
                .collect::<String>()
        };

        download_file(&url, &output_path).await
    }.await;

    download_result.map_err(|e| {
        Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Download failed: {}", e),
        )) as Box<dyn std::error::Error + Send + Sync>
    })?;

    {
        let jar_path = {
            let mut path = std::path::PathBuf::new();
            path.push(".");
            path.push("server.jar");
            path.into_os_string().into_string().map_err(|_| {
                Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid path conversion",
                )) as Box<dyn std::error::Error + Send + Sync>
            })?
        };

        run_java_jar(&jar_path, &Vec::<&str>::new()).map_err(|e| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to run server: {}", e),
            )) as Box<dyn std::error::Error + Send + Sync>
        })?;
    }

    Ok(())
}