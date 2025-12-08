use gtk4::{Label, prelude::*};

/// Returns display text for key and whether it's a Super modifier
pub fn get_modifier_display(mod_name: &str) -> (&str, bool) {
    // Handle code:XX format
    if mod_name.starts_with("code:") {
        if mod_name == "code:60" {
            return (".", false);
        }
        return (mod_name, false);
    }

    // Handle mouse:XXX format
    if mod_name.starts_with("mouse:") {
        return match mod_name {
            "mouse:272" => ("● LMB", false),
            "mouse:273" => ("● RMB", false),
            _ => (mod_name, false),
        };
    }

    // Handle buttonX format (sxhkd)
    if mod_name.starts_with("button") {
        return match mod_name {
            "button4" => ("⇧ Scroll", false),
            "button5" => ("⇩ Scroll", false),
            _ => (mod_name, false),
        };
    }

    // Handle XF86 keys
    if mod_name.starts_with("XF86") {
        return match mod_name {
            // Audio controls
            "XF86AudioRaiseVolume" => ("♫ Vol+", false),
            "XF86AudioLowerVolume" => ("♫ Vol-", false),
            "XF86AudioMute" => ("♬ Mute", false),
            "XF86AudioMicMute" => (" Mute", false),
            
            // Media controls
            "XF86AudioPlay" => ("▶ Play", false),
            "XF86AudioPause" => ("⏸ Pause", false),
            "XF86AudioNext" => ("󰒭 Next", false),
            "XF86AudioPrev" => ("󰒮 Prev", false),
            "XF86AudioStop" => ("■ Stop", false),
            
            // Brightness
            "XF86MonBrightnessUp" => ("☀ Bri+", false),
            "XF86MonBrightnessDown" => ("☼ Bri-", false),
            
            _ => (mod_name, false),
        };
    }

    // Arrow keys and modifiers
    match mod_name.to_uppercase().as_str() {
        "UP" => ("↑", false),
        "DOWN" => ("↓", false),
        "LEFT" => ("←", false),
        "RIGHT" => ("→", false),
        
        // Modifiers
        "SUPER" | "WIN" | "LOGO" | "MOD4" => ("⌘", true),
        "CTRL" | "CONTROL" => ("Ctrl", false),
        "ALT" | "MOD1" => ("Alt", false),
        "SHIFT" => ("󰘶 Shift", false),
        
        // Special keys
        "ENTER" | "RETURN" => ("󰌑", false),
        "SPACE" => ("󱁐", false),
        "TAB" => ("󰌥", false),
        "ESC" | "ESCAPE" => ("Esc", false),
        "BS" | "BACKSPACE" => ("󰁮 ", false),
        "DEL" | "DELETE" => ("󰧧 Del", false),
        "PERIOD" => (".", false),
        "CAPS" | "CAPS_LOCK" => ("󰌎 CapsLock", false),
        
        // Mouse events (hyprland)
        "MOUSE_DOWN" => ("⇩ Scroll", false),
        "MOUSE_UP" => ("⇧ Scroll", false),
        
        _ => (mod_name, false),
    }
}

/// Creates a keycap widget with appropriate styling
pub fn create_keycap(text: &str, is_super: bool) -> Label {
    let label = Label::new(Some(text));
    label.add_css_class("keycap");
    if is_super {
        label.add_css_class("keycap-super");
    }
    label
}
