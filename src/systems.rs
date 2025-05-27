//! Systems for handling UI interactions and updates

use bevy::prelude::*;
use bevy::ui::{Interaction, BackgroundColor};
use crate::components::*;

/// System to handle button interactions and visual feedback
pub fn button_interaction_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut UiButton,
            &mut BackgroundColor,
            Option<&Children>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut button, mut bg_color, children) in &mut interaction_query {
        button.hovered = matches!(interaction, Interaction::Hovered);
        button.pressed = matches!(interaction, Interaction::Pressed);

        // Update background color based on state
        *bg_color = match interaction {
            Interaction::Pressed => Color::srgb(0.3, 0.3, 0.4).into(),
            Interaction::Hovered => Color::srgb(0.25, 0.25, 0.35).into(),
            _ => Color::srgb(0.2, 0.2, 0.3).into(),
        };

        // Update text color if this button has text children
        if let Some(children) = children {
            for &child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    for section in text.sections.iter_mut() {
                        section.style.color = match interaction {
                            Interaction::Pressed => Color::srgb(0.9, 0.9, 1.0),
                            Interaction::Hovered => Color::srgb(0.95, 0.95, 1.0),
                            _ => Color::WHITE,
                        };
                    }
                }
            }
        }
    }
}

/// System to handle checkbox interactions
pub fn checkbox_interaction_system(
    mut query: Query<(&Interaction, &mut UiCheckbox, &mut BackgroundColor), Changed<Interaction>>,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut checkbox, mut bg_color) in &mut query {
        if *interaction == Interaction::Pressed && !checkbox.disabled {
            checkbox.checked = !checkbox.checked;
            // Update checkbox background
            *bg_color = if checkbox.checked {
                Color::srgb(0.25, 0.5, 0.75).into()
            } else {
                Color::srgb(0.15, 0.15, 0.15).into()
            };
        }
    }
}

/// System to handle slider interactions
pub fn slider_interaction_system(
    mut query: Query<(
        &Interaction,
        &mut UiSlider,
        &mut Style,
        &Node,
        &GlobalTransform,
        &mut BackgroundColor,
    )>,
    windows: Query<&Window>,
) {
    for (interaction, mut slider, mut style, node, transform, mut bg_color) in &mut query {
        if slider.disabled {
            continue;
        }

        // Update appearance based on interaction state
        *bg_color = match interaction {
            Interaction::Pressed => Color::srgb(0.3, 0.3, 0.4).into(),
            Interaction::Hovered => Color::srgb(0.25, 0.25, 0.35).into(),
            _ => Color::srgb(0.2, 0.2, 0.3).into(),
        };

        // Handle dragging
        if *interaction == Interaction::Pressed {
            if let Ok(window) = windows.get_single() {
                if let Some(cursor_pos) = window.cursor_position() {
                    // Convert cursor position to local space
                    let node_pos = transform.translation();
                    let node_rect = node.logical_rect(transform);
                    
                    // Calculate relative position (0.0 to 1.0)
                    let rel_x = ((cursor_pos.x - node_rect.min.x) / node_rect.width())
                        .clamp(0.0, 1.0);
                    
                    // Update slider value based on position
                    let range = slider.max - slider.min;
                    let mut new_value = slider.min + (range * rel_x);
                    
                    // Apply step if specified
                    if let Some(step) = slider.step {
                        new_value = (new_value / step).round() * step;
                    }
                    
                    slider.value = new_value.clamp(slider.min, slider.max);
                    
                    // Update fill width
                    if let Val::Percent(_) = style.width {
                        style.width = Val::Percent((rel_x * 100.0).clamp(0.0, 100.0));
                    }
                }
            }
        }
    }
}

/// System to update progress bars
fn update_progress_bars(
    mut query: Query<(&ProgressBar, &mut Style, &Children), Changed<ProgressBar>>,
    mut text_query: Query<&mut Text>,
) {
    for (progress_bar, mut style, children) in &mut query {
        // Update width based on progress
        if let Val::Percent(_) = style.width {
            style.width = Val::Percent((progress_bar.value * 100.0).clamp(0.0, 100.0));
        }
        
        // Update text if enabled
        if progress_bar.show_text {
            if let Ok(mut text) = text_query.get_mut(children[0]) {
                text.sections[0].value = format!("{:.0}%", progress_bar.value * 100.0);
            }
        }
    }
}

/// System to handle tooltips
pub fn tooltip_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    interaction_query: Query<(&Interaction, &GlobalTransform, &Node, &UiTooltip), With<Button>>,
    mut tooltip_query: Query<Entity, With<Tooltip>>,
    windows: Query<&Window>,
) {
    // Remove existing tooltips
    for entity in &mut tooltip_query {
        commands.entity(entity).despawn_recursive();
    }

    // Show tooltip for hovered elements
    for (_interaction, _transform, _node, tooltip) in &interaction_query {
        if let Ok(window) = windows.get_single() {
            if let Some(cursor_pos) = window.cursor_position() {
                commands.spawn((
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            left: Val::Px(cursor_pos.x + tooltip.offset.x),
                            top: Val::Px(cursor_pos.y + tooltip.offset.y),
                            padding: UiRect::all(Val::Px(8.0)),
                            ..default()
                        },
                        background_color: Color::srgba(0.1, 0.1, 0.1, 0.9).into(),
                        ..default()
                    },
                    TextBundle::from_section(
                        &tooltip.text,
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                            font_size: 16.0,
                            color: Color::WHITE,
                        },
                    ),
                    Tooltip,
                ));
            }
        }
    }
}

/// Marker component for tooltip entities
#[derive(Component, Reflect)]
pub struct Tooltip;

pub fn update(app: &mut App) {
    app.add_systems(
        Update,
        (
            button_interaction_system,
            checkbox_interaction_system,
            slider_interaction_system,
            update_progress_bars,
            tooltip_system
        )
    );
}
