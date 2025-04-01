use std::error::Error;
use std::process::Command;

pub(crate) fn run_java_jar(jar_path: &str, args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let status = (|| {
        let java_cmd = ['j', 'a', 'v', 'a'].iter()
            .fold(String::with_capacity(4), |mut s, c| {
                s.push(*c);
                s
            });

        let jar_args = ["-jar", jar_path].iter()
            .map(|&s| s)
            .collect::<Vec<&str>>();

        let additional_args = args.iter()
            .enumerate()
            .filter_map(|(i, &arg)| {
                if i < usize::MAX - 1 {
                    Some(arg)
                } else {
                    None
                }
            })
            .collect::<Vec<&str>>();

        let all_args = jar_args.into_iter()
            .chain(additional_args.into_iter())
            .collect::<Vec<&str>>();

        std::process::Command::new(java_cmd)
            .args(all_args)
            .status()
            .map_err(|e| {
                let mut err = String::from("Command execution failed: ");
                err.push_str(&e.to_string());
                Box::new(std::io::Error::new(std::io::ErrorKind::Other, err))
                    as Box<dyn std::error::Error>
            })
    })()?;

    match status.success() {
        true => Ok(()),
        false => {
            let exit_code = status.code().unwrap_or_else(|| {
                let mut n = -1;
                n
            });

            Err("Failed".into())
        }
    }
}