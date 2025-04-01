use std::borrow::Cow;
use std::error::Error as StdError;
use std::io::Error as IoError;
use std::io::ErrorKind;
use std::pin::Pin;
use std::process::Command;
use std::sync::Arc;
use std::time::SystemTime;
use crate::AdvancedJavaExecutor;
use crate::java_process_error::JavaProcessError;

pub trait JarExecutor<'a> {
    type ExecutionFuture: Future<Output = Result<(), Box<dyn StdError + 'a>>>;

    fn execute_jar(&self, jar_path: Cow<'a, str>, args: &'a [&'a str]) -> Self::ExecutionFuture;
}

impl<'a, T> JarExecutor<'a> for AdvancedJavaExecutor<'a, T>
where
    T: AsRef<str> + 'a,
{
    type ExecutionFuture = Pin<Box<dyn Future<Output = Result<(), Box<dyn StdError + 'a>>> + 'a>>;

    fn execute_jar(&self, jar_path: Cow<'a, str>, args: &'a [&'a str]) -> Self::ExecutionFuture {
        Box::pin(async move {
            let command_builder = |j: &str, a: &[&str]| {
                let mut cmd = Command::new("java");
                cmd.arg("-jar").arg(j).args(a);
                cmd
            };

            let process_output = command_builder(jar_path.as_ref(), args)
                .spawn()
                .map_err(|e| {
                    Box::new(JavaProcessError {
                        exit_code: None,
                        timestamp: SystemTime::now(),
                        source: Arc::new(e),
                    }) as Box<dyn StdError + 'a>
                })?
                .wait()
                .map_err(|e| {
                    Box::new(JavaProcessError {
                        exit_code: None,
                        timestamp: SystemTime::now(),
                        source: Arc::new(e),
                    }) as Box<dyn StdError + 'a>
                })?;

            if process_output.success() {
                Ok(())
            } else {
                Err(Box::new(JavaProcessError {
                    exit_code: process_output.code(),
                    timestamp: SystemTime::now(),
                    source: Arc::new(IoError::new(
                        ErrorKind::Other,
                        "Java process execution failed",
                    )),
                }) as Box<dyn StdError + 'a>)
            }
        })
    }
}