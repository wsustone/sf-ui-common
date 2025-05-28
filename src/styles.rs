//! Common UI styles and themes for StrategyForge

use bevy::prelude::*;
use bevy::ui::{Style, UiRect, Val};

/// Common UI styles
pub mod common {
    use super::*;

    /// Default button style
    pub fn button() -> ButtonBundle {
        ButtonBundle {
            style: Style {
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(5.0)),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            background_color: Color::srgb(0.15, 0.15, 0.15).into(),
            ..default()
        }
    }

    /// Default text style for buttons
    pub fn button_text(asset_server: &AssetServer) -> TextStyle {
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 24.0,
            color: Color::WHITE,
        }
    }

    /// Default panel style
    pub fn panel() -> NodeBundle {
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::srgba(0.1, 0.1, 0.1, 0.9).into(),
            ..default()
        }
    }

    /// Style for subsection titles
    pub fn subsection_title_style(asset_server: &Res<AssetServer>) -> TextStyle {
        TextStyle {
            font: asset_server.load("fonts/FiraSans-SemiBold.ttf"),
            font_size: 18.0,
            color: Color::WHITE,
        }
    }

    /// Style for regular text content
    pub fn regular_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Regular.ttf"),
            font_size: 14.0,
            color: Color::WHITE,
        }
    }
}

/// Menu-specific styles
pub mod menu {
    use super::*;

    /// Menu button style
    pub fn menu_button() -> ButtonBundle {
        let mut button = common::button();
        button.background_color = Color::srgb(0.2, 0.2, 0.4).into();
        button.style.width = Val::Px(250.0);
        button.style.height = Val::Px(65.0);
        button
    }

    /// Menu title text style
    pub fn title_text(asset_server: &AssetServer) -> TextStyle {
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 48.0,
            color: Color::WHITE,
        }
    }

    /// Menu container style
    pub fn menu_container() -> NodeBundle {
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                ..default()
            },
            ..default()
        }
    }
}

/// Settings-specific styles
pub mod settings {
    use super::*;

    /// Settings panel style
    pub fn settings_panel() -> NodeBundle {
        NodeBundle {
            style: Style {
                width: Val::Percent(80.0),
                height: Val::Percent(80.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                row_gap: Val::Px(20.0),
                ..default()
            },
            background_color: Color::srgba(0.1, 0.1, 0.1, 0.9).into(),
            ..default()
        }
    }

    /// Settings section style
    pub fn settings_section() -> NodeBundle {
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                row_gap: Val::Px(10.0),
                ..default()
            },
            background_color: Color::srgba(0.2, 0.2, 0.2, 0.5).into(),
            ..default()
        }
    }

    /// Settings row style
    pub fn settings_row() -> NodeBundle {
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(10.0)),
                ..default()
            },
            ..default()
        }
    }
}

/// In-game HUD styles
pub mod hud {
    use super::*;

    /// Main HUD container
    pub fn hud_container() -> NodeBundle {
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(80.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            background_color: Color::srgba(0.1, 0.1, 0.1, 0.7).into(),
            ..default()
        }
    }

    /// Resource display style
    pub fn resource_display() -> NodeBundle {
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                margin: UiRect::right(Val::Px(20.0)),
                ..default()
            },
            ..default()
        }
    }

    /// Resource icon style
    pub fn resource_icon() -> NodeBundle {
        NodeBundle {
            style: Style {
                width: Val::Px(24.0),
                height: Val::Px(24.0),
                margin: UiRect::right(Val::Px(5.0)),
                ..default()
            },
            ..default()
        }
    }
}
