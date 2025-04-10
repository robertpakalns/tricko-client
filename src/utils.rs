use ico::IconDir;
use std::io::Cursor;
use tao::{dpi::PhysicalPosition, event_loop::EventLoop, window::Icon};

pub fn get_scale_factor() -> f64 {
    if let Some(monitor) = EventLoop::new().primary_monitor() {
        return monitor.scale_factor();
    }
    1.0
}

pub fn center_window(window_width: i32, window_height: i32) -> PhysicalPosition<i32> {
    let scale_factor = get_scale_factor();

    if let Some(monitor) = EventLoop::new().primary_monitor() {
        let screen_size = monitor.size();

        let scaled_window_width: i32 = (window_width as f64 * scale_factor) as i32;
        let scaled_window_height: i32 = (window_height as f64 * scale_factor) as i32;

        let x: i32 = (screen_size.width as i32 - scaled_window_width) / 2;
        let y: i32 = (screen_size.height as i32 - scaled_window_height) / 2;

        return PhysicalPosition::new(x, y);
    }

    PhysicalPosition::new(0, 0)
}

pub fn load_icon() -> Icon {
    let icon_data = include_bytes!("../frontend/icon.ico");
    let icon = IconDir::read(Cursor::new(icon_data))
        .expect("Failed to parse .ico file")
        .entries()[0]
        .decode()
        .expect("Failed to decode .ico");

    let width: u32 = icon.width();
    let height: u32 = icon.height();
    let rgba = icon.rgba_data().to_vec();

    Icon::from_rgba(rgba, width, height).expect("Failed to create icon")
}
