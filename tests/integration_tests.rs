use std::process::{Command, Stdio};

fn is_command_available(command: String) -> bool {
    match Command::new(command)
        .arg("-v")
        .stdout(Stdio::null())
        .status()
    {
        Ok(status) => status.success(),
        _ => false,
    }
}

#[test]
fn original_xxd_is_available() {
    //assert!(is_command_available("xxd".to_string()));
}
