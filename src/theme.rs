use crate::models::Theme;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

/// Загружает тему из конфигурационного файла
pub fn load_theme() -> Theme {
    let mut theme = Theme::default();
    let config_path = dirs::config_dir()
        .map(|p| p.join("HotkeyHub/theme.conf"))
        .unwrap_or_else(|| PathBuf::from("theme.conf"));

    if let Ok(file) = File::open(&config_path) {
        let reader = BufReader::new(file);
        let re = Regex::new(r"^\s*([a-z_]+)\s*=\s*(.+)$").unwrap();

        for line in reader.lines().filter_map(|l| l.ok()) {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some(caps) = re.captures(line) {
                let key = &caps[1];
                let val = caps[2].trim().to_string();
                match key {
                    "background" => theme.background = val,
                    "background_alt" => theme.background_alt = val,
                    "accent" => theme.accent = val,
                    "text" => theme.text = val,
                    "border" | "border_color" => theme.border = val,
                    _ => {}
                }
            }
        }
    }
    theme
}

/// Генерирует CSS строку на основе темы
pub fn generate_css(theme: &Theme) -> String {
    format!(
        r#"
        window {{ background: {bg}; color: {text}; font-family: 'JetBrains Mono', sans-serif; }}
        tab {{ background: {bg}; color: {text}; padding: 5px; }}
        tab:checked {{ color: {text}; border-bottom: 2px solid {accent}; }}
        .tab-container {{ padding: 20px; }}
        
        .search-bar {{
            background: {bg_alt};
            color: {text};
            border: 1px solid {border}; 
            border-radius: 8px;
        }}
        
        .bind-card {{
            background: {bg_alt};
            padding: 12px;
            border-radius: 8px;
            border: 1px solid {border};
        }}
        
        .keycap {{
            background: {bg}; 
            color: {text};
            font-weight: 800;
            padding: 4px 10px;
            border-radius: 6px;
            border-bottom: 3px solid {bg}; 
            margin-right: 2px;
            margin-bottom: 2px;
            min-width: 24px;
            text-align: center;
            border: 1px solid alpha({text}, 0.2);
        }}
        
        .keycap-super {{
            background: {accent};
            color: {bg};
            border-bottom-color: shade({accent}, 0.8);
        }}
        
        .command {{ color: alpha({text}, 0.7); margin-top: 8px; font-size: 0.9em; }}
        .plus {{ color: alpha({text}, 0.5); margin: 0 4px; font-weight: bold; }}
        "#,
        bg = theme.background,
        bg_alt = theme.background_alt,
        accent = theme.accent,
        text = theme.text,
        border = theme.border
    )
}
