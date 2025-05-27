//! # StrategyForge UI Common
//!
//! Shared UI components and systems for StrategyForge.
//!
//! This crate provides reusable UI components, styles, and systems that are used
//! throughout the StrategyForge game interface.

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

use bevy::prelude::*;

// Re-export commonly used types
pub mod advanced_components;
pub mod components;
pub mod systems;

// Re-export commonly used items
pub use advanced_components::*;
pub use components::*;
pub use systems::*;

/// Standard color definitions for UI elements
pub mod colors {
    use bevy::prelude::Color;
    
    /// Pure white color
    pub const WHITE: Color = Color::srgb(1.0, 1.0, 1.0);
    
    /// Pure black color
    pub const BLACK: Color = Color::srgb(0.0, 0.0, 0.0);
    
    /// Fully transparent color
    pub const TRANSPARENT: Color = Color::srgba(0.0, 0.0, 0.0, 0.0);

    /// Button colors
    pub mod button {
        use bevy::prelude::Color;
        
        /// Default button color
        pub const NORMAL: Color = Color::srgb(0.15, 0.15, 0.15);
        
        /// Button color when hovered
        pub const HOVERED: Color = Color::srgb(0.25, 0.25, 0.35);
        
        /// Button color when pressed
        pub const PRESSED: Color = Color::srgb(0.35, 0.35, 0.45);
        
        /// Button color when disabled
        pub const DISABLED: Color = Color::srgb(0.3, 0.3, 0.3);
    }

    /// Focus-related colors
    pub mod focus {
        use bevy::prelude::Color;
        
        /// Focus highlight color (semi-transparent)
        pub const HIGHLIGHT: Color = Color::srgba(0.2, 0.4, 0.8, 0.3);
        
        /// Focus border color
        pub const BORDER: Color = Color::srgb(0.3, 0.6, 1.0);
        
        /// Focus text color
        pub const TEXT: Color = Color::srgb(0.9, 0.9, 1.0);
    }

    /// Text colors
    pub mod text {
        use bevy::prelude::Color;
        
        /// Default text color
        pub const NORMAL: Color = Color::srgb(0.9, 0.9, 0.9);
        
        /// Disabled text color
        pub const DISABLED: Color = Color::srgb(0.5, 0.5, 0.5);
    }

    /// Slider component colors
    pub mod slider {
        use bevy::prelude::Color;
        
        /// Slider background color
        pub const BACKGROUND: Color = Color::srgb(0.2, 0.2, 0.2);
        
        /// Slider foreground (fill) color
        pub const FOREGROUND: Color = Color::srgb(0.3, 0.6, 1.0);
        
        /// Slider handle color
        pub const HANDLE: Color = Color::srgb(0.8, 0.8, 0.8);
        
        /// Default slider height in pixels
        pub const HEIGHT: f32 = 8.0;
        
        /// Default slider handle size in pixels
        pub const HANDLE_SIZE: f32 = 16.0;
    }
}

/// Common UI component bundles
pub mod bundles {
    use bevy::prelude::*;
    use crate::colors;

    /// Creates a standard button bundle with the given text
    pub fn button_bundle(
        text: &str,
        asset_server: &Res<AssetServer>,
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
            background_color: colors::button::NORMAL.into(),
            ..default()
        };

        let text = TextBundle::from_section(
            text,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 24.0,
                color: colors::text::NORMAL,
            },
        );

        (button, text)
    }

    /// Creates a checkbox bundle with the given state
    pub fn checkbox_bundle(
        checked: bool,
        asset_server: &Res<AssetServer>,
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
            background_color: colors::button::NORMAL.into(),
            border_color: BorderColor(colors::text::NORMAL),
            ..default()
        };

        let check = TextBundle::from_section(
            if checked { "X" } else { "" },
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: colors::text::NORMAL,
            },
        );

        (checkbox, check)
    }
}

/// Plugin for common UI components
pub struct UiCommonPlugin;

impl Plugin for UiCommonPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register our custom components
            .register_type::<components::UiButton>()
            .register_type::<components::UiCheckbox>()
            .register_type::<components::UiSlider>()
            
            // Add systems
            .add_systems(Update, (
                systems::button_interaction_system,
                systems::checkbox_interaction_system,
                systems::slider_interaction_system,
            ));
    }
}

/// System to handle button interactions
pub fn button_interaction_system(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), Changed<Interaction>>,
) {
    for (interaction, mut bg_color) in &mut interaction_query {
        *bg_color = match interaction {
            Interaction::Pressed => colors::button::PRESSED.into(),
            Interaction::Hovered => colors::button::HOVERED.into(),
            Interaction::None => colors::button::NORMAL.into(),
        };
    }
}

/// System to handle checkbox interactions
pub fn checkbox_interaction_system(
    mut query: Query<(&Interaction, &mut UiCheckbox, &Children), (Changed<Interaction>, With<UiCheckbox>)>,
    mut _text_query: Query<&mut Text>,
) {
    for (interaction, mut checkbox, children) in &mut query {
        if *interaction == Interaction::Pressed {
            checkbox.checked = !checkbox.checked;
            if let Ok(mut text) = _text_query.get_mut(children[0]) {
                text.sections[0].value = if checkbox.checked { "☑" } else { "☐" }.to_string();
            }
        }
    }
}

/// System to handle slider interactions
pub fn slider_interaction_system(
    mut query: Query<(&Interaction, &mut UiSlider, &mut Style), (Changed<Interaction>, With<UiSlider>)>,
) {
    for (interaction, mut slider, mut style) in &mut query {
        match interaction {
            Interaction::Pressed => {
                slider.value = slider.value.clamp(0.0, 1.0);
                style.width = Val::Px(slider.value * 100.0);
            }
            Interaction::Hovered => {
                style.width = Val::Px(slider.value * 100.0);
            }
            Interaction::None => {
                style.width = Val::Px(slider.value * 100.0);
            }
        }
    }
}
