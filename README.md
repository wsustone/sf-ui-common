# sf-ui-common
Shared UI components for StrategyForge UI Common

[![Crates.io](https://img.shields.io/crates/v/sf-ui-common)](https://crates.io/crates/sf-ui-common)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A comprehensive UI toolkit for StrategyForge, providing reusable components, styles, and utilities for building consistent and accessible user interfaces.

## Features

- **Reusable UI Components**: Buttons, checkboxes, sliders, dropdowns, tooltips, and more
- **Predefined Styles**: Consistent theming and styling for all UI elements
- **Accessibility**: Built with accessibility in mind, supporting keyboard navigation and screen readers
- **Responsive Design**: Components that adapt to different screen sizes and aspect ratios
- **Easy Integration**: Simple API for adding UI elements to your game

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sf-ui-common = { path = "../sf-ui-common" }  # Use the appropriate path or version
```

## Usage

### Basic Setup

```rust
use bevy::prelude::*;
use sf_ui_common::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiCommonPlugin)
        .add_systems(Startup, setup_ui)
        .run();
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Create a simple button
    commands.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..default()
        },
        UiButton::default(),
        TextBundle::from_section(
            "Click Me!",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 24.0,
                color: Color::WHITE,
            },
        ),
    ));
}
```

### Available Components

- **UiButton**: Interactive button with hover/press states
- **UiCheckbox**: Toggleable checkbox
- **UiSlider**: Adjustable slider for numeric input
- **UiDropdown**: Dropdown menu for selecting options
- **UiTooltip**: Contextual help text that appears on hover
- **Scrollable**: Container with scrollable content
- **TabContainer** and **Tab**: Tabbed interface components
- **Panel**: Grouping container with optional title
- **ProgressBar**: Visual indicator of progress

### Styling

The `styles` module provides predefined styles for common UI elements:

```rust
use sf_ui_common::styles::menu;

// Create a menu button
let button_bundle = menu::menu_button();
```

### Utilities

The `utils` module provides helper functions for common UI patterns:

```rust
use sf_ui_common::utils::{button_bundle, text_bundle};

// Create a styled button with text
let (button, text) = button_bundle(
    "Click me!",
    &asset_server,
    Style::default(),
);
```

## Contributing

Contributions are welcome! Please follow the [StrategyForge Contribution Guidelines](https://github.com/wsustone/StrategyForge/CONTRIBUTING.md).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
