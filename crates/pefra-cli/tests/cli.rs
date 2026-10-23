use std::process::Command;

#[test]
fn help_and_version_are_available() {
    let binary = env!("CARGO_BIN_EXE_pefra");
    let help = Command::new(binary).arg("--help").output().unwrap();
    assert!(help.status.success());
    assert!(String::from_utf8_lossy(&help.stdout).contains("Usage: pefra"));

    let version = Command::new(binary).arg("--version").output().unwrap();
    assert!(version.status.success());
    assert_eq!(
        String::from_utf8_lossy(&version.stdout).trim(),
        format!("pefra {}", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn unsupported_command_fails() {
    let output = Command::new(env!("CARGO_BIN_EXE_pefra"))
        .arg("run")
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("Unsupported argument: run"));
}
