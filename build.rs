use std::process::exit;
use winres::WindowsResource;

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = WindowsResource::new();
        res.set("ProductName", "Tricko Client")
            .set("FileDescription", "Tricko Client")
            .set_icon("./frontend/icon.ico")
            .set("ProductVersion", env!("CARGO_PKG_VERSION"))
            .set("FileVersion", env!("CARGO_PKG_VERSION"));

        if let Err(e) = res.compile() {
            eprintln!("{}", e);
            exit(1);
        }
    }
}
