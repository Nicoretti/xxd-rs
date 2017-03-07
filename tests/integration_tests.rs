use std::process::Command;

fn is_command_available(command: String) -> bool {
    // TODO NiCo: remove output on terminal for this check
    let mut result = Command::new(command)
                         .arg("-v")
                         .status();
    match result {
        Ok(status) => true,
        Err(_) => false,
    }
}

// Attention: if this test fails all other test will/should do so too
#[test]
fn original_xxd_is_available() {
    assert!(is_command_available("xxd".to_string()));
}

#[test]
fn xxd_rs_is_available() {
    assert!(is_command_available("xxd-rs".to_string()));
}
