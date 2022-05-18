use std::process::Command;

fn main() {
    if cfg!(windows) {
        build_windows();
    } else if cfg!(unix) {
        build_linux();
    }
}

fn build_windows() {
    Command::new("cmd")
        .arg("move")
        .arg(r"target\release\spm-helper.exe")
        .arg(r"%USERPROFILE%\Desktop\spm-helper.exe")
        .output()
        .expect("Failed to execute command");
}

fn build_linux() {
    Command::new("sudo")
        .arg("mv")
        .arg(r"target/release/spm-helper")
        .arg(r"/usr/bin/spm-helper")
        .output()
        .expect("Failed to execute command");
}
