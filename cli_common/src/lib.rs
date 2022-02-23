use anyhow::Result;
use std::process::Command;

fn print_a() {
    println!("a");
}

pub fn bash_eval_path(path: &str) -> Result<String> {
    let bash_to_exec = format!("echo {}", path);
    let bash_output = Command::new("bash").arg("-c").arg(bash_to_exec).output()?;
    let after_sub = String::from_utf8(bash_output.stdout)?;
    Ok(after_sub.trim().to_string())
}

#[test]
fn it_works() {
    print_a();
}
