pub mod bash {
    use anyhow::Result;
    use std::process::Command;
    pub fn eval_path(path: &str) -> Result<String> {
        let bash_to_exec = format!("echo {}", path);
        let bash_output = Command::new("bash").arg("-c").arg(bash_to_exec).output()?;
        let after_sub = String::from_utf8(bash_output.stdout)?;
        Ok(after_sub.trim().to_string())
    }
}

pub mod log {
    use chrono::Local;
    use env_logger::Env;
    use std::io::Write;

    pub fn init_env_logger() {
        env_logger::Builder::from_env(Env::default().default_filter_or("info"))
            .format(|buf, record| {
                writeln!(
                    buf,
                    "{} [{}] {}",
                    Local::now().format("%Y-%m-%dT%H:%M:%S"),
                    record.level(),
                    record.args()
                )
            })
            .init();
    }
}
