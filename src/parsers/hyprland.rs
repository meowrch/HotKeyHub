use crate::models::Keybind;
use crate::utils::resolve_path;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

/// Context for parsing Hyprland configurations
pub struct HyprContext {
    pub variables: HashMap<String, String>,
    pub processed_files: HashSet<PathBuf>,
}

impl HyprContext {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            processed_files: HashSet::new(),
        }
    }
}

/// Normalize key name for comparison (handle variations like CAPS vs Caps_Lock)
fn normalize_key(key: &str) -> String {
    let upper = key.to_uppercase();
    // Remove underscores and common suffixes
    upper.replace('_', "").replace("LOCK", "").replace("KEY", "")
}

/// Recursively parses Hyprland configuration with source directive support
pub fn parse_hyprland_recursive(path: PathBuf, ctx: &mut HyprContext, binds: &mut Vec<Keybind>) {
    if !path.exists() || ctx.processed_files.contains(&path) {
        return;
    }

    ctx.processed_files.insert(path.clone());
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(_) => return,
    };
    let reader = BufReader::new(file);

    let re_bind = Regex::new(r"^\s*bind[a-z]*\s*=\s*(.+)$").unwrap();
    let re_var = Regex::new(r"^\s*\$([a-zA-Z0-9_]+)\s*=\s*(.+)$").unwrap();
    let re_source = Regex::new(r"^\s*source\s*=\s*(.+)$").unwrap();

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some(caps) = re_var.captures(line) {
            ctx.variables
                .insert(caps[1].to_string(), caps[2].trim().to_string());
            continue;
        }

        if let Some(caps) = re_source.captures(line) {
            let raw_path = caps[1].trim();
            if let Some(p) = resolve_path(raw_path) {
                if let Ok(paths) = glob::glob(p.to_str().unwrap()) {
                    for entry in paths {
                        if let Ok(path) = entry {
                            parse_hyprland_recursive(path, ctx, binds);
                        }
                    }
                } else {
                    parse_hyprland_recursive(p, ctx, binds);
                }
            }
            continue;
        }

        if let Some(caps) = re_bind.captures(line) {
            let content = caps[1].split('#').next().unwrap_or("").trim();
            let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();
            if parts.len() < 3 {
                continue;
            }

            let mut raw_mods = parts[0].to_string();
            for (v_key, v_val) in &ctx.variables {
                raw_mods = raw_mods.replace(&format!("${}", v_key), v_val);
            }

            let mut mods_list: Vec<String> = raw_mods
                .split(|c| c == ' ' || c == '+' || c == '_')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            let key = parts[1].to_string();
            
            // If modifier and key are the same (normalized comparison)
            // This handles cases like: bindr = CAPS, Caps_Lock or bind = ESC, Escape
            if mods_list.len() == 1 {
                let norm_mod = normalize_key(&mods_list[0]);
                let norm_key = normalize_key(&key);
                
                if norm_mod == norm_key {
                    mods_list.clear();
                }
            }
            
            let dispatcher = parts[2];
            let arg = if parts.len() > 3 {
                parts[3..].join(", ")
            } else {
                String::new()
            };

            let display_cmd = if arg.is_empty() {
                dispatcher.to_string()
            } else {
                format!("{} {}", dispatcher, arg)
            };

            binds.push(Keybind {
                mods: mods_list,
                key,
                command: display_cmd,
            });
        }
    }
}
