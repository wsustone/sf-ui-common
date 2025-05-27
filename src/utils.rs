//! Utility functions and helpers for UI components

use bevy::{
    prelude::*,
    ui::{AlignItems, JustifyContent, Style, UiRect, Val},
};

/// Creates a centered container with the given content
pub fn centered_container(style: Style) -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..style
        },
        ..default()
    }
}

/// Creates a vertical stack container
pub fn v_stack(style: Style) -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: bevy::ui::FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
            ..style
        },
        ..default()
    }
}

/// Creates a horizontal stack container
pub fn h_stack(style: Style) -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: bevy::ui::FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
            ..style
        },
        ..default()
    }
}

/// Creates a simple text bundle with the given style
pub fn text_bundle(
    text: impl Into<String>,
    asset_server: &AssetServer,
    font_size: f32,
    color: Color,
) -> TextBundle {
    TextBundle::from_section(
        text,
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Medium.ttf"),
            font_size,
            color,
        },
    )
    .with_style(Style {
        margin: UiRect::all(Val::Px(5.0)),
        ..default()
    })
}

/// Creates a button with the given text and style
pub fn button_bundle(
    text: impl Into<String>,
    asset_server: &AssetServer,
    style: Style,
) -> (ButtonBundle, TextBundle) {
    let button = ButtonBundle {
        style: Style {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(10.0)),
            margin: UiRect::all(Val::Px(5.0)),
            ..style
        },
        background_color: Color::srgb(0.15, 0.15, 0.15).into(),
        ..default()
    };

    let text = TextBundle::from_section(
        text,
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 24.0,
            color: Color::WHITE,
        },
    );

    (button, text)
}

/// Creates a slider with the given range and value
pub fn slider_bundle(
    min: f32,
    max: f32,
    value: f32,
    style: Style,
) -> (NodeBundle, NodeBundle, NodeBundle) {
    let track = NodeBundle {
        style: Style {
            width: Val::Px(200.0),
            height: Val::Px(8.0),
            margin: UiRect::horizontal(Val::Px(10.0)),
            ..style
        },
        background_color: Color::srgb(0.3, 0.3, 0.3).into(),
        ..default()
    };

    let fill_width = ((value - min) / (max - min)).clamp(0.0, 1.0) * 200.0;
    let fill = NodeBundle {
        style: Style {
            width: Val::Px(fill_width),
            height: Val::Percent(100.0),
            ..default()
        },
        background_color: Color::srgb(0.2, 0.6, 1.0).into(),
        ..default()
    };

    let handle = NodeBundle {
        style: Style {
            width: Val::Px(16.0),
            height: Val::Px(24.0),
            position_type: bevy::ui::PositionType::Absolute,
            left: Val::Px(fill_width - 8.0),
            ..default()
        },
        background_color: Color::WHITE.into(),
        ..default()
    };

    (track, fill, handle)
}

/// Creates a checkbox with the given state
pub fn checkbox_bundle(
    checked: bool,
    asset_server: &AssetServer,
) -> (NodeBundle, TextBundle) {
    let checkbox = NodeBundle {
        style: Style {
            width: Val::Px(24.0),
            height: Val::Px(24.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(5.0)),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        background_color: Color::srgb(0.1, 0.1, 0.1).into(),
        border_color: Color::WHITE.into(),
        ..default()
    };

    let check = TextBundle::from_section(
        if checked { "X" } else { "" },
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 20.0,
            color: Color::WHITE,
        },
    );

    (checkbox, check)
}

/// Creates a tooltip component
pub fn tooltip_bundle(
    text: impl Into<String>,
    asset_server: &AssetServer,
    position: Vec2,
) -> (NodeBundle, TextBundle) {
    let tooltip = NodeBundle {
        style: Style {
            position_type: bevy::ui::PositionType::Absolute,
            left: Val::Px(position.x),
            top: Val::Px(position.y),
            padding: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        background_color: Color::srgba(0.1, 0.1, 0.1, 0.9).into(),
        ..default()
    };

    let text = TextBundle::from_section(
        text,
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Medium.ttf"),
            font_size: 16.0,
            color: Color::WHITE,
        },
    );

    (tooltip, text)
}
