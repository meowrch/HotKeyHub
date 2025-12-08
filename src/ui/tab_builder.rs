use crate::models::Keybind;
use crate::ui::widgets::{create_keycap, get_modifier_display};
use gtk4::prelude::*;
use gtk4::{
    Box as GtkBox, FlowBox, FlowBoxChild, Label, Orientation, ScrolledWindow, SearchEntry,
    SelectionMode,
};

/// Creates a tab page with keybinds table and search
pub fn create_tab_page(binds: Vec<Keybind>) -> (GtkBox, SearchEntry, ScrolledWindow) {
    let container = GtkBox::new(Orientation::Vertical, 0);
    container.add_css_class("tab-container");

    let search_entry = SearchEntry::builder()
        .placeholder_text("Search keybinds, commands...")
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
        .selection_mode(SelectionMode::None)
        .min_children_per_line(1)
        .max_children_per_line(20)
        .row_spacing(10)
        .column_spacing(10)
        .build();

    for bind in binds {
        let card = GtkBox::new(Orientation::Vertical, 6);
        card.add_css_class("bind-card");

        let keys_box = GtkBox::new(Orientation::Horizontal, 0);
        keys_box.set_halign(gtk4::Align::Start);

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

        card.append(&keys_box);

        let lbl_cmd = Label::new(Some(&bind.command));
        lbl_cmd.set_wrap(true);
        lbl_cmd.set_max_width_chars(35);
        lbl_cmd.set_xalign(0.0);
        lbl_cmd.add_css_class("command");
        card.append(&lbl_cmd);

        let child = FlowBoxChild::new();
        child.set_child(Some(&card));

        let search_string =
            format!("{} {} {}", bind.mods.join(" "), bind.key, bind.command).to_lowercase();
        child.set_widget_name(&search_string);

        flow_box.insert(&child, -1);
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
        }
    });

    scrolled.set_child(Some(&flow_box));
    container.append(&scrolled);

    (container, search_entry, scrolled)
}
