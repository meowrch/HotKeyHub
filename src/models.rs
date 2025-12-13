use std::path::PathBuf;

/// Структура для хранения информации о горячей клавише
#[derive(Debug, Clone)]
pub struct Keybind {
    pub mods: Vec<String>,
    pub key: String,
    pub command: String,
    pub description: Option<String>,
}

/// Режим работы приложения
#[derive(Debug, Clone)]
pub enum RunMode {
    All,
    SingleHyprland(PathBuf),
    SingleSxhkd(PathBuf),
}

/// Тема оформления приложения
#[derive(Debug, Clone)]
pub struct Theme {
    pub background: String,
    pub background_alt: String,
    pub accent: String,
    pub text: String,
    pub border: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: "#1e1e2e".to_string(),
            background_alt: "#313244".to_string(),
            accent: "#89b4fa".to_string(),
            text: "#cdd6f4".to_string(),
            border: "#45475a".to_string(),
        }
    }
}
