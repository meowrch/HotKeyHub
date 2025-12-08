use regex::Regex;
use std::path::PathBuf;

/// Разрешает путь с подстановкой переменных окружения и тильды
pub fn resolve_path(path_str: &str) -> Option<PathBuf> {
    let mut expanded = path_str.to_string();

    if let Some(home) = dirs::home_dir() {
        let home_str = home.to_string_lossy();
        expanded = expanded.replace("$HOME", &home_str);
        if expanded.starts_with("~") {
            expanded = expanded.replacen("~", &home_str, 1);
        }
    }

    // Переменные окружения
    let re_env = Regex::new(r"\$([A-Za-z_][A-Za-z0-9_]*)").unwrap();
    while let Some(caps) = re_env.captures(&expanded.clone()) {
        let var_name = &caps[1];
        if let Ok(val) = std::env::var(var_name) {
            expanded = expanded.replace(&format!("${}", var_name), &val);
        } else {
            break;
        }
    }

    Some(PathBuf::from(expanded))
}
