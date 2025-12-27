mod models;
mod parsers;
mod theme;
mod ui;
mod utils;

use gtk4::prelude::*;
use gtk4::Application;
use models::RunMode;
use std::path::PathBuf;

/// Парсит аргументы командной строки
fn parse_args() -> RunMode {
    let args: Vec<String> = std::env::args().collect();
    let mut mode = RunMode::All;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--hyprland" => {
                if i + 1 < args.len() {
                    mode = RunMode::SingleHyprland(PathBuf::from(&args[i + 1]));
                    i += 1;
                }
            }
            "--sxhkd" => {
                if i + 1 < args.len() {
                    mode = RunMode::SingleSxhkd(PathBuf::from(&args[i + 1]));
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    mode
}

fn main() {
    let mode = parse_args();

    let app = Application::builder()
        .application_id("com.meowrch.HotkeyHub")
        .build();
    
    app.connect_activate(move |app| {
        ui::build_ui(app, &mode);
    });

    app.run_with_args::<&str>(&[]);
}
