use std::process::{Command, exit};
use winres::WindowsResource;

fn main() {
    if !cfg!(target_os = "windows") {
        return;
    }

    let mut res = WindowsResource::new();
    res.set("ProductName", "Tricko Client")
        .set("FileDescription", "Tricko Client")
        .set_icon("./frontend/icon.ico")
        .set("ProductVersion", env!("CARGO_PKG_VERSION"))
        .set("FileVersion", env!("CARGO_PKG_VERSION"));

    if let Err(_) = res.compile() {
        exit(1);
    }

    let output = Command::new("wix")
        .arg("build")
        .arg("build.wxs")
        .arg("-out")
        .arg("./dist/tricko-client-win.msi")
        .output()
        .expect("Failed to execute wix build");

    if !output.status.success() {
        exit(1);
    }
}
