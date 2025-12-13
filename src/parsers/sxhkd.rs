use crate::models::Keybind;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

/// Expands braces in string (e.g., {a,b,c} -> ["a", "b", "c"])
/// Handles multiple brace sets and filters out underscore placeholders
fn expand_braces(input: &str) -> Vec<String> {
    let re = Regex::new(r"\{([^{}]+)\}").unwrap();
    
    let mut results = vec![input.to_string()];
    let mut changed = true;
    
    // Keep expanding until no more braces
    while changed {
        changed = false;
        let mut new_results = Vec::new();
        
        for item in &results {
            if let Some(caps) = re.captures(item) {
                changed = true;
                let content = &caps[1];
                let prefix = &item[..caps.get(0).unwrap().start()];
                let suffix = &item[caps.get(0).unwrap().end()..];

                let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();
                for part in parts {
                    // Skip underscore placeholders during expansion
                    if part != "_" {
                        new_results.push(format!("{}{}{}", prefix, part, suffix));
                    }
                }
            } else {
                new_results.push(item.clone());
            }
        }
        results = new_results;
    }
    
    results
}

/// Parses SXHKD configuration
pub fn parse_sxhkd(path: PathBuf) -> Vec<Keybind> {
    let mut binds = Vec::new();
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(_) => return binds,
    };
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();

    let mut i = 0;
    while i < lines.len() {
        let mut line = lines[i].trim().to_string();
        i += 1;

        // Check for description comment
        let mut description: Option<String> = None;
        if line.starts_with("# Description:") {
            description = Some(line.trim_start_matches("# Description:").trim().to_string());
            // Move to next line (the actual keybind)
            if i < lines.len() {
                line = lines[i].trim().to_string();
                i += 1;
            } else {
                continue;
            }
        }

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Handle line continuations
        while line.ends_with('\\') {
            line.pop();
            if i < lines.len() {
                line.push_str(lines[i].trim());
                i += 1;
            }
        }

        let triggers_raw = line;
        let mut cmd_lines = Vec::new();

        // Collect command lines (indented lines following the trigger)
        while i < lines.len() {
            let cmd_line = lines[i].trim();
            if cmd_line.is_empty() && cmd_lines.is_empty() {
                i += 1;
                continue;
            }
            if cmd_line.is_empty() || (cmd_line.starts_with('#') && !cmd_line.starts_with("#!")) {
                break;
            }
            if !lines[i].starts_with(' ') && !lines[i].starts_with('\t') && !cmd_lines.is_empty() {
                break;
            }
            cmd_lines.push(cmd_line.trim_end_matches('\\').trim().to_string());
            i += 1;
        }

        if cmd_lines.is_empty() {
            continue;
        }

        let full_cmd_raw = cmd_lines.join(" ");
        
        // Expand braces in both triggers and commands
        let expanded_triggers = expand_braces(&triggers_raw);
        let expanded_cmds = expand_braces(&full_cmd_raw);

        for (idx, trigger) in expanded_triggers.iter().enumerate() {
            let cmd = if idx < expanded_cmds.len() {
                &expanded_cmds[idx]
            } else {
                &expanded_cmds[0]
            };

            // Parse trigger into modifiers and key
            let parts: Vec<&str> = trigger.split('+').map(|s| s.trim()).collect();
            if parts.is_empty() {
                continue;
            }

            let key = parts.last().unwrap().to_string();
            
            // Filter out empty and underscore modifiers
            let mods: Vec<String> = parts[..parts.len() - 1]
                .iter()
                .filter(|s| !s.is_empty() && **s != "_")
                .map(|s| s.to_string())
                .collect();

            // Skip if key is empty or underscore
            if key.is_empty() || key == "_" {
                continue;
            }

            binds.push(Keybind {
                mods,
                key,
                command: cmd.clone(),
                description: description.clone(),
            });
        }
    }
    binds
}
