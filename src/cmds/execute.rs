use std::process::Command;

pub fn handler(curl: &str) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(curl)
        .output()
        .expect("failed to execute process");
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error: {}", stderr);
    }
}