//! UI components for StrategyForge

use bevy::prelude::*;

/// A UI button component with visual states
#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct UiButton {
    /// Whether the button is currently pressed
    pub pressed: bool,
    /// Whether the button is currently hovered
    pub hovered: bool,
    /// Whether the button is disabled
    pub disabled: bool,
    /// Optional tooltip text
    pub tooltip: Option<String>,
}

/// A checkbox component with toggle state
#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct UiCheckbox {
    /// Whether the checkbox is checked
    pub checked: bool,
    /// Whether the checkbox is disabled
    pub disabled: bool,
    /// Optional tooltip text
    pub tooltip: Option<String>,
}

/// Component for interactive sliders
#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct UiSlider {
    /// Current value (normalized between min and max)
    pub value: f32,
    /// Minimum allowed value
    pub min: f32,
    /// Maximum allowed value
    pub max: f32,
    /// Format string for display (e.g. "{:.1}%")
    pub format: String,
    /// Optional step size for discrete values
    pub step: Option<f32>,
    /// Whether the slider is disabled
    pub disabled: bool,
}

/// A dropdown menu component
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct UiDropdown {
    /// Whether the dropdown is currently open
    pub open: bool,
    /// The currently selected option index
    pub selected: Option<usize>,
    /// The available options
    pub options: Vec<String>,
    /// Whether the dropdown is disabled
    pub disabled: bool,
}

/// A tooltip component
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct UiTooltip {
    /// The tooltip text
    pub text: String,
    /// The position offset from the parent
    pub offset: Vec2,
}

/// Component for scrollable areas
#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct Scrollable {
    /// Current scroll position (in pixels)
    pub scroll: f32,
    /// Whether the content is scrollable vertically
    pub vertical: bool,
    /// Whether the content is scrollable horizontally
    pub horizontal: bool,
}

/// A tab container component
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct TabContainer {
    /// The currently active tab index
    pub active_tab: usize,
}

/// A tab component
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Tab {
    /// The index of this tab
    pub index: usize,
    /// Whether this tab is the active tab
    pub active: bool,
}

/// A panel component for grouping UI elements
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Panel {
    /// The panel title
    pub title: Option<String>,
    /// Whether the panel is collapsible
    pub collapsible: bool,
    /// Whether the panel is currently collapsed
    pub collapsed: bool,
}

/// A progress bar component
#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct ProgressBar {
    /// Current value (0.0 to 1.0)
    pub value: f32,
    /// Background color
    pub background_color: Color,
    /// Fill color
    pub fill_color: Color,
    /// Whether to show text percentage
    pub show_text: bool,
}

/// Component for focusable UI elements
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Focusable {
    /// Current focus state of the element
    pub state: FocusState,
    /// Type of focusable element
    pub focus_type: FocusableType,
}

/// Represents the focus state of a UI element
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum FocusState {
    /// Element is not focused
    NotFocused,
    /// Element has keyboard focus
    Focused,
    /// Element is actively being interacted with
    #[default]
    Active,
}

/// Types of focusable UI elements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum FocusableType {
    /// Standard button element
    Button,
    /// Slider control
    Slider,
    /// Checkbox toggle
    Checkbox,
    /// Dropdown selector
    #[default]
    Dropdown,
}

/// Component for setting rows in configuration menus
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct SettingRow {
    /// Display label for the setting
    pub label: String,
    /// Optional help text/tooltip content
    pub help_text: Option<String>,
}

/// Component for dropdown selectors
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Dropdown {
    /// Available options in the dropdown
    pub options: Vec<String>,
    /// Currently selected option index
    pub selected_index: usize,
    /// Whether the dropdown is currently open
    pub opened: bool,
}

impl Default for Dropdown {
    fn default() -> Self {
        Self {
            options: Vec::new(),
            selected_index: 0,
            opened: false,
        }
    }
}

/// Component for tabbed interfaces
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct TabbedContainer {
    /// Names of available tabs
    pub tabs: Vec<String>,
    /// Currently active tab index
    pub active_tab: usize,
}

impl Default for TabbedContainer {
    fn default() -> Self {
        Self {
            tabs: Vec::new(),
            active_tab: 0,
        }
    }
}

/// Component for collapsible panels
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Collapsible {
    /// Panel title text
    pub title: String,
    /// Whether the panel can be collapsed
    pub collapsible: bool,
    /// Current collapsed state
    pub collapsed: bool,
}

/// Position options for tooltips
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum TooltipPosition {
    /// Position above the target element
    #[default]
    Top,
    /// Position below the target element
    Bottom,
    /// Position to the left of the target
    Left,
    /// Position to the right of the target
    Right,
}

/// Tooltip with positioning
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Tooltip {
    /// The tooltip text
    pub text: String,
    /// The position of the tooltip
    pub position: TooltipPosition,
    /// The offset of the tooltip from the parent
    pub offset: f32,
}

/// Component for scrollable areas
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct ScrollPane {
    /// Current scroll position (in pixels)
    pub scroll_position: Vec2,
    /// Maximum scroll distance (in pixels)
    pub max_scroll: Vec2,
}

impl Default for ScrollPane {
    fn default() -> Self {
        Self {
            scroll_position: Vec2::ZERO,
            max_scroll: Vec2::ZERO,
        }
    }
}

/// Component for displaying numeric values with min/max indicators
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ValueDisplay {
    /// Current value to display
    pub value: f32,
    /// Minimum reference value
    pub min: f32,
    /// Maximum reference value
    pub max: f32,
    /// Format string for display (e.g. "{:.1}%")
    pub format: String,
}

impl Default for ValueDisplay {
    fn default() -> Self {
        Self {
            value: 0.0,
            min: 0.0,
            max: 100.0,
            format: "{:.1}".into(),
        }
    }
}

/// Numeric slider component for precise value selection
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct NumericSlider {
    /// Current numeric value (between min and max)
    pub value: f32,
    /// Minimum allowed value
    pub min: f32,
    /// Maximum allowed value
    pub max: f32,
    /// Format string for display (e.g. "{:.1}%")
    pub format: String,
}
