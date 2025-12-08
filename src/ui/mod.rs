pub mod tab_builder;
pub mod widgets;

use crate::models::RunMode;
use crate::parsers::{hyprland::HyprContext, parse_hyprland_recursive, parse_sxhkd};
use crate::theme::{generate_css, load_theme};
use gtk4::prelude::*;
use gtk4::{
    gdk, glib, Application, ApplicationWindow, CssProvider, Label, Notebook, ScrolledWindow,
    SearchEntry,
};
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::SystemTime;

/// Builds the main application UI
pub fn build_ui(app: &Application, run_mode: &RunMode) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Keybinds Viewer")
        .default_width(1100)
        .default_height(800)
        .build();

    // Theme and CSS Provider
    let provider = CssProvider::new();
    let initial_theme = load_theme();
    provider.load_from_data(&generate_css(&initial_theme));

    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().unwrap(),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // Watch for theme changes
    let provider_weak = provider.downgrade();
    let config_path = dirs::config_dir()
        .map(|p| p.join("hypr-keys/theme.conf"))
        .unwrap_or_else(|| PathBuf::from("theme.conf"));

    // Track last modified time
    let last_mtime = Rc::new(std::cell::RefCell::new(SystemTime::now()));

    // Simple polling for config changes
    glib::timeout_add_seconds_local(1, move || {
        if let Ok(metadata) = fs::metadata(&config_path) {
            if let Ok(modified) = metadata.modified() {
                let mut last = last_mtime.borrow_mut();
                if modified > *last {
                    *last = modified;
                    if let Some(prov) = provider_weak.upgrade() {
                        let new_theme = load_theme();
                        prov.load_from_data(&generate_css(&new_theme));
                        println!("Theme reloaded!");
                    }
                }
            }
        }
        glib::ControlFlow::Continue
    });

    // Content
    let notebook = Notebook::builder().tab_pos(gtk4::PositionType::Top).build();

    // Store search entries and scrolled windows for keyboard shortcuts
    let search_entries: Rc<std::cell::RefCell<Vec<SearchEntry>>> =
        Rc::new(std::cell::RefCell::new(Vec::new()));
    let scrolled_windows: Rc<std::cell::RefCell<Vec<ScrolledWindow>>> =
        Rc::new(std::cell::RefCell::new(Vec::new()));

    match run_mode {
        RunMode::All => {
            // 1. Hyprland Tab
            if let Some(config_home) = dirs::config_dir() {
                let mut ctx = HyprContext::new();
                let mut hypr_binds = Vec::new();
                parse_hyprland_recursive(
                    config_home.join("hypr/hyprland.conf"),
                    &mut ctx,
                    &mut hypr_binds,
                );
                let (hypr_page, search, scrolled) = tab_builder::create_tab_page(hypr_binds);
                search_entries.borrow_mut().push(search);
                scrolled_windows.borrow_mut().push(scrolled);
                notebook.append_page(&hypr_page, Some(&Label::new(Some("Hyprland"))));

                // 2. Sxhkd Tabs (Auto Detection)
                let bspwm_sxhkd = config_home.join("bspwm/sxhkdrc");
                let normal_sxhkd = config_home.join("sxhkd/sxhkdrc");

                // If found in bspwm folder
                if bspwm_sxhkd.exists() {
                    let binds = parse_sxhkd(bspwm_sxhkd);
                    let (page, search, scrolled) = tab_builder::create_tab_page(binds);
                    search_entries.borrow_mut().push(search);
                    scrolled_windows.borrow_mut().push(scrolled);
                    notebook.append_page(&page, Some(&Label::new(Some("Sxhkd (bspwm)"))));
                }

                // If found in sxhkd folder
                if normal_sxhkd.exists() {
                    let binds = parse_sxhkd(normal_sxhkd);
                    let (page, search, scrolled) = tab_builder::create_tab_page(binds);
                    search_entries.borrow_mut().push(search);
                    scrolled_windows.borrow_mut().push(scrolled);
                    notebook.append_page(&page, Some(&Label::new(Some("Sxhkd"))));
                }
            }
        }
        RunMode::SingleHyprland(path) => {
            notebook.set_show_tabs(false);
            let mut ctx = HyprContext::new();
            let mut binds = Vec::new();
            parse_hyprland_recursive(path.clone(), &mut ctx, &mut binds);
            let (page, search, scrolled) = tab_builder::create_tab_page(binds);
            search_entries.borrow_mut().push(search);
            scrolled_windows.borrow_mut().push(scrolled);
            notebook.append_page(&page, Some(&Label::new(None)));
        }
        RunMode::SingleSxhkd(path) => {
            notebook.set_show_tabs(false);
            let binds = parse_sxhkd(path.clone());
            let (page, search, scrolled) = tab_builder::create_tab_page(binds);
            search_entries.borrow_mut().push(search);
            scrolled_windows.borrow_mut().push(scrolled);
            notebook.append_page(&page, Some(&Label::new(None)));
        }
    }

    // Auto-focus first search entry on startup
    if let Some(first_search) = search_entries.borrow().first() {
        let search_clone = first_search.clone();
        glib::idle_add_local_once(move || {
            search_clone.grab_focus();
        });
    }

    // Keyboard shortcuts
    let key_controller = gtk4::EventControllerKey::new();
    let window_weak = window.downgrade();
    let notebook_weak = notebook.downgrade();
    let search_entries_clone = search_entries.clone();
    let scrolled_windows_clone = scrolled_windows.clone();

    key_controller.connect_key_pressed(move |_, keyval, _keycode, state| {
        let ctrl_pressed = state.contains(gdk::ModifierType::CONTROL_MASK);
        let alt_pressed = state.contains(gdk::ModifierType::ALT_MASK);

        // Ctrl+F: Focus search
        if ctrl_pressed && keyval == gdk::Key::f {
            if let Some(notebook) = notebook_weak.upgrade() {
                if let Some(current_page) = notebook.current_page() {
                    if let Some(search) = search_entries_clone.borrow().get(current_page as usize) {
                        search.grab_focus();
                        return glib::Propagation::Stop;
                    }
                }
            }
        }

        // Esc: Unfocus search (focus window instead)
        if keyval == gdk::Key::Escape {
            if let Some(window) = window_weak.upgrade() {
                gtk4::prelude::RootExt::set_focus(&window, None::<&gtk4::Widget>);
                return glib::Propagation::Stop;
            }
        }

        // Q: Quit application
        if keyval == gdk::Key::q || keyval == gdk::Key::Q {
            if let Some(window) = window_weak.upgrade() {
                window.close();
                return glib::Propagation::Stop;
            }
        }

        // Alt+1-9: Switch tabs
        if alt_pressed {
            let tab_index = match keyval {
                gdk::Key::_1 => Some(0),
                gdk::Key::_2 => Some(1),
                gdk::Key::_3 => Some(2),
                gdk::Key::_4 => Some(3),
                gdk::Key::_5 => Some(4),
                gdk::Key::_6 => Some(5),
                gdk::Key::_7 => Some(6),
                gdk::Key::_8 => Some(7),
                gdk::Key::_9 => Some(8),
                _ => None,
            };

            if let Some(idx) = tab_index {
                if let Some(notebook) = notebook_weak.upgrade() {
                    if idx < notebook.n_pages() as usize {
                        notebook.set_current_page(Some(idx as u32));
                        return glib::Propagation::Stop;
                    }
                }
            }
        }

        // PgUp/PgDn: Scroll
        if keyval == gdk::Key::Page_Up || keyval == gdk::Key::Page_Down {
            if let Some(notebook) = notebook_weak.upgrade() {
                if let Some(current_page) = notebook.current_page() {
                    if let Some(scrolled) = scrolled_windows_clone.borrow().get(current_page as usize)
                    {
                        let vadj = scrolled.vadjustment();
                        let page_size = vadj.page_size();
                        let current = vadj.value();
                        let new_value = if keyval == gdk::Key::Page_Up {
                            (current - page_size).max(vadj.lower())
                        } else {
                            (current + page_size).min(vadj.upper() - page_size)
                        };
                        vadj.set_value(new_value);
                        return glib::Propagation::Stop;
                    }
                }
            }
        }

        glib::Propagation::Proceed
    });

    window.add_controller(key_controller);
    window.set_child(Some(&notebook));
    window.present();
}
