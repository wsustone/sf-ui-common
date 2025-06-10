//! Wrapper for bevy_egui's ScrollArea for use in menu UIs.

use bevy_egui::egui;

/// Wraps egui's ScrollArea for consistent use in menu UIs.
/// Example usage in an egui context:
///
/// ```rust
/// menu_scroll_area(|ui| {
///     ui.label("Lots of content...");
/// });
/// ```
pub fn menu_scroll_area<R>(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui) -> R) -> R {
    egui::ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, add_contents)
        .inner
}

/// Wrapper for an egui button with a consistent menu style.
/// Returns true if the button was clicked.
///
/// Example:
/// ```rust
/// if menu_button(ui, "Click me") {
///     // handle click
/// }
/// ```
pub fn menu_button(ui: &mut egui::Ui, text: &str) -> bool {
    ui.add(egui::Button::new(text).wrap(true)).clicked()
}

/// Wrapper for an egui label with menu styling.
///
/// Example:
/// ```rust
/// menu_label(ui, "This is a label");
/// ```
pub fn menu_label(ui: &mut egui::Ui, text: &str) {
    ui.label(text);
}

/// Wrapper for a single-line text edit field.
/// Returns true if the value was changed.
///
/// Example:
/// ```rust
/// let mut value = String::new();
/// if menu_text_edit_singleline(ui, &mut value) {
///     // value changed
/// }
/// ```
pub fn menu_text_edit_singleline(ui: &mut egui::Ui, value: &mut String) -> bool {
    ui.text_edit_singleline(value).changed()
}

/// Wrapper for a menu checkbox.
/// Returns true if the checkbox was toggled.
///
/// Example:
/// ```rust
/// let mut checked = false;
/// if menu_checkbox(ui, &mut checked, "Enable feature") {
///     // toggled
/// }
/// ```
pub fn menu_checkbox(ui: &mut egui::Ui, checked: &mut bool, label: &str) -> bool {
    ui.checkbox(checked, label).changed()
}

/// Wrapper for a simple egui table (Grid).
///
/// Example:
/// ```rust
/// menu_table(ui, &["Header 1", "Header 2"], |ui| {
///     ui.label("Row 1, Col 1");
///     ui.label("Row 1, Col 2");
///     ui.end_row();
///     ui.label("Row 2, Col 1");
///     ui.label("Row 2, Col 2");
///     ui.end_row();
/// });
/// ```
pub fn menu_table<R>(ui: &mut egui::Ui, headers: &[&str], add_rows: impl FnOnce(&mut egui::Ui) -> R) -> R {
    egui::Grid::new("menu_table")
        .striped(true)
        .show(ui, |ui| {
            for header in headers {
                ui.heading(*header);
            }
            ui.end_row();
            add_rows(ui)
        })
        .inner
}

