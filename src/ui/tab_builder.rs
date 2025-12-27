use crate::models::Keybind;
use crate::ui::widgets::{create_keycap, get_modifier_display};
use gtk4::prelude::*;
use gtk4::{
    gdk, glib, Box as GtkBox, FlowBox, FlowBoxChild, Label, Orientation, Popover,
    ScrolledWindow, SearchEntry, SelectionMode,
};
use std::cell::RefCell;
use std::rc::Rc;

/// Creates a tab page with keybinds table and search
pub fn create_tab_page(binds: Vec<Keybind>) -> (GtkBox, SearchEntry, ScrolledWindow) {
    let container = GtkBox::new(Orientation::Vertical, 0);
    container.add_css_class("tab-container");

    let search_entry = SearchEntry::builder()
        .placeholder_text("Search keybinds, commands, descriptions...")
        .margin_bottom(15)
        .build();
    search_entry.add_css_class("search-bar");

    container.append(&search_entry);

    let scrolled = ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vexpand(true)
        .build();

    let flow_box = FlowBox::builder()
        .valign(gtk4::Align::Start)
        .selection_mode(SelectionMode::Single)
        .min_children_per_line(1)
        .max_children_per_line(20)
        .row_spacing(10)
        .column_spacing(10)
        .build();

    // Store popovers for each card
    let popovers: Rc<RefCell<Vec<Option<Popover>>>> = Rc::new(RefCell::new(Vec::new()));

    for bind in binds {
        let card = GtkBox::new(Orientation::Vertical, 6);
        card.add_css_class("bind-card");

        // Main row with keys and info icon on the same level
        let main_row = GtkBox::new(Orientation::Horizontal, 0);
        main_row.set_hexpand(true);
        main_row.set_valign(gtk4::Align::Center);

        // Keys box on the left
        let keys_box = GtkBox::new(Orientation::Horizontal, 0);
        keys_box.set_halign(gtk4::Align::Start);
        keys_box.set_valign(gtk4::Align::Center);

        for (i, modifier) in bind.mods.iter().enumerate() {
            if i > 0 {
                let plus = Label::new(Some("+"));
                plus.add_css_class("plus");
                keys_box.append(&plus);
            }
            let (icon, is_super) = get_modifier_display(modifier);
            keys_box.append(&create_keycap(icon, is_super));
        }

        if !bind.mods.is_empty() {
            let plus = Label::new(Some("+"));
            plus.add_css_class("plus");
            keys_box.append(&plus);
        }

        let (k_icon, _) = get_modifier_display(&bind.key);
        let key_display =
            if k_icon.chars().count() == 1 && k_icon.chars().next().unwrap().is_alphabetic() {
                k_icon.to_uppercase()
            } else {
                k_icon.to_string()
            };

        keys_box.append(&create_keycap(&key_display, false));

        main_row.append(&keys_box);

        // Spacer to push info icon to the right
        let spacer = GtkBox::new(Orientation::Horizontal, 0);
        spacer.set_hexpand(true);
        main_row.append(&spacer);

        // Info icon on the right (same level as keys) - now clickable
        let popover_option = if let Some(desc) = &bind.description {
            let info_icon = Label::new(Some("🛈"));
            info_icon.add_css_class("info-icon");
            info_icon.set_halign(gtk4::Align::End);
            info_icon.set_valign(gtk4::Align::Center);

            // Create popover with description
            let popover = Popover::new();
            let desc_label = Label::new(Some(desc));
            desc_label.set_wrap(true);
            desc_label.set_max_width_chars(40);
            desc_label.set_margin_start(12);
            desc_label.set_margin_end(12);
            desc_label.set_margin_top(8);
            desc_label.set_margin_bottom(8);
            popover.set_child(Some(&desc_label));
            popover.set_parent(&info_icon);

            // Add click gesture to label
            let popover_clone = popover.clone();
            let gesture = gtk4::GestureClick::new();
            gesture.connect_released(move |_, _, _, _| {
                popover_clone.popup();
            });
            info_icon.add_controller(gesture);

            main_row.append(&info_icon);
            Some(popover)
        } else {
            None
        };

        card.append(&main_row);

        // Command label - selectable for copying
        let lbl_cmd = Label::new(Some(&bind.command));
        lbl_cmd.set_wrap(true);
        lbl_cmd.set_max_width_chars(35);
        lbl_cmd.set_xalign(0.0);
        lbl_cmd.set_selectable(true);
        lbl_cmd.add_css_class("command");
        card.append(&lbl_cmd);

        let child = FlowBoxChild::new();
        child.set_child(Some(&card));

        // Include description in search string
        let search_string = format!(
            "{} {} {} {}",
            bind.mods.join(" "),
            bind.key,
            bind.command,
            bind.description.as_deref().unwrap_or("")
        )
        .to_lowercase();
        child.set_widget_name(&search_string);

        // Store command for execution (unsafe operation)
        unsafe {
            child.set_data("command", bind.command.clone());
        }

        flow_box.insert(&child, -1);
        popovers.borrow_mut().push(popover_option);
    }

    let entry_weak = search_entry.downgrade();
    flow_box.set_filter_func(move |child| {
        if let Some(entry) = entry_weak.upgrade() {
            let query = entry.text().to_string().to_lowercase();
            if query.is_empty() {
                return true;
            }
            child.widget_name().to_string().contains(&query)
        } else {
            true
        }
    });

    let flow_box_weak = flow_box.downgrade();
    search_entry.connect_search_changed(move |_| {
        if let Some(fb) = flow_box_weak.upgrade() {
            fb.invalidate_filter();
            
            // After filtering, select first visible card if nothing is selected
            glib::idle_add_local_once(move || {
                let visible_children: Vec<FlowBoxChild> = (0..fb.observe_children().n_items())
                    .filter_map(|i| {
                        fb.observe_children()
                            .item(i)
                            .and_then(|obj| obj.downcast::<FlowBoxChild>().ok())
                    })
                    .filter(|child| child.is_visible())
                    .collect();

                if let Some(first_visible) = visible_children.first() {
                    fb.select_child(first_visible);
                }
            });
        }
    });

    // Keyboard navigation for cards
    let key_controller = gtk4::EventControllerKey::new();
    let flow_box_weak_nav = flow_box.downgrade();
    let popovers_clone = popovers.clone();
    let scrolled_weak = scrolled.downgrade();

    key_controller.connect_key_pressed(move |_, keyval, keycode, _state| {
        if let Some(flow_box) = flow_box_weak_nav.upgrade() {
            // Get all children (for popover index mapping)
            let all_children: Vec<FlowBoxChild> = (0..flow_box.observe_children().n_items())
                .filter_map(|i| {
                    flow_box
                        .observe_children()
                        .item(i)
                        .and_then(|obj| obj.downcast::<FlowBoxChild>().ok())
                })
                .collect();

            // Arrow key navigation with 2D grid logic
            if keyval == gdk::Key::Up
                || keyval == gdk::Key::Down
                || keyval == gdk::Key::Left
                || keyval == gdk::Key::Right
            {
                let selected = flow_box.selected_children();
                
                // Only get visible children for navigation
                let visible_children: Vec<FlowBoxChild> = all_children
                    .iter()
                    .filter(|child| child.is_visible())
                    .cloned()
                    .collect();

                if visible_children.is_empty() {
                    return glib::Propagation::Proceed;
                }

                // Get current index in visible children
                let current_index = if let Some(selected_child) = selected.first() {
                    visible_children.iter().position(|c| c == selected_child)
                } else {
                    None
                };

                // Build grid structure by analyzing X and Y positions of VISIBLE cards
                // Group by rows based on Y coordinate, then sort within rows by X coordinate
                let mut positions: Vec<(usize, i32, i32)> = visible_children
                    .iter()
                    .enumerate()
                    .map(|(idx, child)| {
                        let allocation = child.allocation();
                        (idx, allocation.y(), allocation.x())
                    })
                    .collect();

                // Sort by Y first, then by X
                positions.sort_by_key(|&(_, y, x)| (y, x));

                // Group into rows based on Y coordinate similarity
                let mut grid: Vec<Vec<usize>> = Vec::new();
                let mut current_row: Vec<usize> = Vec::new();
                let mut last_y: Option<i32> = None;

                for (idx, y, _x) in positions {
                    if let Some(ly) = last_y {
                        // New row if y position changed significantly
                        if (y - ly).abs() > 30 {
                            if !current_row.is_empty() {
                                grid.push(current_row.clone());
                                current_row.clear();
                            }
                        }
                    }

                    current_row.push(idx);
                    last_y = Some(y);
                }

                if !current_row.is_empty() {
                    grid.push(current_row);
                }

                if grid.is_empty() {
                    return glib::Propagation::Proceed;
                }

                // Find current position in grid
                let mut current_pos: Option<(usize, usize)> = None; // (row, col)
                if let Some(current_idx) = current_index {
                    for (row_idx, row) in grid.iter().enumerate() {
                        if let Some(col_idx) = row.iter().position(|&idx| idx == current_idx) {
                            current_pos = Some((row_idx, col_idx));
                            break;
                        }
                    }
                }

                // Calculate new position based on arrow key
                let new_index = match keyval {
                    gdk::Key::Up => {
                        if let Some((row, col)) = current_pos {
                            if row > 0 {
                                let prev_row = &grid[row - 1];
                                let new_col = col.min(prev_row.len() - 1);
                                Some(prev_row[new_col])
                            } else {
                                // Wrap to last row
                                let last_row = &grid[grid.len() - 1];
                                let new_col = col.min(last_row.len() - 1);
                                Some(last_row[new_col])
                            }
                        } else {
                            Some(0)
                        }
                    }
                    gdk::Key::Down => {
                        if let Some((row, col)) = current_pos {
                            if row < grid.len() - 1 {
                                let next_row = &grid[row + 1];
                                let new_col = col.min(next_row.len() - 1);
                                Some(next_row[new_col])
                            } else {
                                // Wrap to first row
                                let first_row = &grid[0];
                                let new_col = col.min(first_row.len() - 1);
                                Some(first_row[new_col])
                            }
                        } else {
                            Some(0)
                        }
                    }
                    gdk::Key::Left => {
                        if let Some((row, col)) = current_pos {
                            let current_row = &grid[row];
                            if col > 0 {
                                Some(current_row[col - 1])
                            } else {
                                // Wrap to end of row
                                Some(current_row[current_row.len() - 1])
                            }
                        } else {
                            Some(0)
                        }
                    }
                    gdk::Key::Right => {
                        if let Some((row, col)) = current_pos {
                            let current_row = &grid[row];
                            if col < current_row.len() - 1 {
                                Some(current_row[col + 1])
                            } else {
                                // Wrap to start of row
                                Some(current_row[0])
                            }
                        } else {
                            Some(0)
                        }
                    }
                    _ => None,
                };

                if let Some(idx) = new_index {
                    if let Some(child) = visible_children.get(idx) {
                        flow_box.select_child(child);

                        // Scroll to selected child
                        if let Some(scrolled) = scrolled_weak.upgrade() {
                            let allocation = child.allocation();
                            let vadj = scrolled.vadjustment();
                            let scroll_value = vadj.value();
                            let page_size = vadj.page_size();
                            let child_y = allocation.y() as f64;
                            let child_height = allocation.height() as f64;

                            if child_y < scroll_value {
                                vadj.set_value(child_y);
                            } else if child_y + child_height > scroll_value + page_size {
                                vadj.set_value(child_y + child_height - page_size);
                            }
                        }
                    }
                }

                return glib::Propagation::Stop;
            }

            // 'i' key to show tooltip
            if keycode == 31 {
                let selected = flow_box.selected_children();
                if let Some(selected_child) = selected.first() {
                    // Find index of selected child in ALL children (for popover access)
                    if let Some(idx) = all_children.iter().position(|c| c == selected_child) {
                        if let Some(Some(popover)) = popovers_clone.borrow().get(idx) {
                            popover.popup();
                            return glib::Propagation::Stop;
                        }
                    }
                }
            }
        }

        glib::Propagation::Proceed
    });

    flow_box.add_controller(key_controller);

    // Update visual styling when selection changes
    let flow_box_weak_select = flow_box.downgrade();
    flow_box.connect_selected_children_changed(move |_| {
        if let Some(flow_box) = flow_box_weak_select.upgrade() {
            // Remove active class from all FlowBoxChild elements
            let all_children: Vec<FlowBoxChild> = (0..flow_box.observe_children().n_items())
                .filter_map(|i| {
                    flow_box
                        .observe_children()
                        .item(i)
                        .and_then(|obj| obj.downcast::<FlowBoxChild>().ok())
                })
                .collect();

            for child in &all_children {
                child.remove_css_class("flow-child-active");
            }

            // Add active class to selected FlowBoxChild
            let selected = flow_box.selected_children();
            if let Some(selected_child) = selected.first() {
                selected_child.add_css_class("flow-child-active");
            }
        }
    });

    scrolled.set_child(Some(&flow_box));
    container.append(&scrolled);

    (container, search_entry, scrolled)
}
