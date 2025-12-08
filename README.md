<div align="center">

# ⌨️ HotkeyHub

### *Your keybinds in one window*
**Essential for newcomers in a new environment**

<br>

[![Issues](https://img.shields.io/github/issues/meowrch/HotKeyHub?color=ffb29b&labelColor=1C2325&style=for-the-badge)](https://github.com/meowrch/HotKeyHub/issues)
[![Stars](https://img.shields.io/github/stars/meowrch/HotKeyHub?color=fab387&labelColor=1C2325&style=for-the-badge)](https://github.com/meowrch/HotKeyHub/stargazers)
[![License](https://img.shields.io/github/license/meowrch/HotKeyHub?color=FCA2AA&labelColor=1C2325&style=for-the-badge)](./LICENSE)

[![README RU](https://img.shields.io/badge/README-RU-blue?color=cba6f7&labelColor=1C2325&style=for-the-badge)](./README.ru.md)
[![README ENG](https://img.shields.io/badge/README-ENG-blue?color=C9CBFF&labelColor=C9CBFF&style=for-the-badge)](./README.md)

[🚀 Quick Start](#quick-start) - [✨ Features](#features) - [🔧 Supported Formats](#supported-formats) - [📚 FAQ](#faq)

</div>

***

## 🎯 What is HotkeyHub?
HotkeyHub is an application that displays your keybindings.
It can help newcomers get comfortable with new dotfiles. \
Just add the `super + /` keybinding to your WM and a convenient Cheat Sheet will always be at hand!


## <a name="features"></a>✨ Key Features

### 📋 Multiple Configuration Support

HotkeyHub [automatically detects WM](#supported-formats) and parses keybindings:
- **Hyprland** (`~/.config/hypr/hyprland.conf`)
- **SXHKD** (`~/.config/sxhkd/sxhkdrc` or `~/.config/bspwm/sxhkdrc`)

Each configuration is displayed in a separate tab!

### 🔍 Smart Search

- Search by **modifiers** (Super, Ctrl, Alt)
- Search by **keys** (Return, Space)
- Search by **commands** (kitty, firefox, rofi)
- **Real-time** search

## <a name="quick-start"></a>🚀 Quick Start

### Installation

#### Arch Linux (AUR)

```
# Coming soon to AUR
yay -S hotkeyhub-bin
```

#### Build from Source

```
# 1. Install dependencies
sudo pacman -S rust gtk4 base-devel

# 2. Clone the repository
git clone https://github.com/meowrch/HotkeyHub.git
cd HotkeyHub

# 3. Build
cargo build --release

# 4. Install
sudo cp target/release/hotkeyhub /usr/bin/
```

### First Launch

```
# Run the application
hotkeyhub

# Or run for a specific config
hotkeyhub --hyprland ~/.config/hypr/hyprland.conf
hotkeyhub --sxhkd ~/.config/sxhkd/sxhkdrc
```

### Add to Your WM

**Hyprland** (`~/.config/hypr/hyprland.conf`):
```
bind = $mainMod, SLASH, exec, hotkeyhub  # Super + /
```

**BSPWM** (`~/.config/sxhkd/sxhkdrc`):
```
super + slash
    hotkeyhub
```

## <a name="shortcuts"></a>⌨️ Keyboard Shortcuts

| Keys | Action |
|---------|----------|
| **Ctrl + F** | Focus search field |
| **Alt + 1-9** | Switch between tabs |
| **PgUp / PgDn** | Scroll list |
| **Q** | Quit application |

> [!TIP]
> On startup, the cursor is automatically in the search field — start typing right away!

## <a name="themes"></a>🎨 Theme Customization

HotkeyHub supports custom themes via `~/.config/HotkeyHub/theme.conf`:

```
# Theme colors (HEX format)
background = #1e1e2e
background_alt = #313244
accent = #89b4fa
text = #cdd6f4
border = #45475a
```

> [!NOTE]
> The theme updates automatically when the file changes — no restart needed!

## <a name="supported-formats"></a>🔧 Supported Formats

### Hyprland

```
# Simple binds
bind = $mainMod, Return, exec, kitty

# With modifiers
bind = $mainMod+Shift, Q, killactive

# Special keys
bind = , XF86AudioRaiseVolume, exec, wpctl set-volume @DEFAULT_AUDIO_SINK@ 5%+

# Code keys
bind = $mainMod, code:60, exec, rofimoji  # code:60 = period

# Mouse
bindm = $mainMod, mouse:272, movewindow   # LMB
bindm = $mainMod, mouse:273, resizewindow # RMB
```

### SXHKD

```
# Simple binds
super + Return
    kitty

# Multiple variants
super + {_,shift + }{Left,Right,Up,Down}
    bspc node -{f,s} {west,east,north,south}

# XF86 keys
XF86Audio{RaiseVolume,LowerVolume,Mute}
    wpctl set-volume @DEFAULT_AUDIO_SINK@ {5%+,5%-,toggle}
```

## <a name="faq"></a>📚 FAQ

### Why aren't some binds displayed?
Make sure the syntax in your config is correct. HotkeyHub skips lines with parsing errors.

### Can support for i3/Sway be added?
Yes! Open a [Feature Request](https://github.com/meowrch/HotkeyHub/issues) — we'll add support.

### How do I change the font?
Edit the CSS in the code or create an issue requesting font settings to be added to `theme.conf`.

### Does it work on Wayland?
Yes, HotkeyHub uses GTK4, which is fully compatible with Wayland.

> [!TIP]
> **Not working?** \
> [Open an issue](https://github.com/meowrch/HotkeyHub/issues) with a description of the problem.

## 🤝 Contributing

Want to improve HotkeyHub? We'd love your contribution!

1. Fork the repository
2. Create a branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 🗺️ Roadmap

- [ ] i3/Sway support
- [ ] Niri support
- [ ] Export binds to PDF/PNG
- [ ] Awesome WM support

## ☕ Support the Project

<div align="center">

**Like HotkeyHub?** Help the project grow! 🚀

| 💎 Cryptocurrency | 📬 Address |
|:---:|:---|
| **TON** | `UQB9qNTcAazAbFoeobeDPMML9MG73DUCAFTpVanQnLk3BHg3` |
| **Ethereum** | `0x56e8bf8Ec07b6F2d6aEdA7Bd8814DB5A72164b13` |
| **Bitcoin** | `bc1qt5urnw7esunf0v7e9az0jhatxrdd0smem98gdn` |
| **Tron** | `TBTZ5RRMfGQQ8Vpf8i5N8DZhNxSum2rzAs` |

<br>

*Every donation motivates us to continue development! ❤️*

</div>

---

<div align="center">

**Made with ❤️ for the Linux community**

</div>
