//! Systems for handling UI interactions and updates

use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::ui::{Interaction, BackgroundColor, BorderColor};
use crate::components::*;
use crate::advanced_components::{scroll_area_system, calculate_scroll_bounds_system};
use crate::colors;

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
            Interaction::Pressed => colors::button::PRESSED.into(),
            Interaction::Hovered => colors::button::HOVERED.into(),
            _ => colors::button::NORMAL.into(),
        };

        // Update text color if this button has text children
        if let Some(children) = children {
            for &child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    for section in text.sections.iter_mut() {
                        section.style.color = match interaction {
                            Interaction::Pressed => colors::focus::TEXT,
                            Interaction::Hovered => colors::text::NORMAL,
                            _ => colors::text::NORMAL,
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
                colors::button::PRESSED.into()
            } else {
                colors::button::NORMAL.into()
            };
        }
    }
}

/// System to handle slider interactions and visual feedback
pub fn slider_interaction_system(
    mut query: Query<(&Interaction, &UiSlider, &mut Style), (Changed<Interaction>, With<UiSlider>)>,
) {
    for (interaction, slider, mut style) in &mut query {
        match interaction {
            Interaction::Pressed => {
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

/// System to handle tooltip visibility and positioning
pub fn tooltip_system(
    mut commands: Commands,
    mut tooltip_query: Query<(Entity, &Tooltip, &Parent), Without<Text>>,
    parent_query: Query<(), With<Node>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, tooltip, _) in &mut tooltip_query {
        if parent_query.get(entity).is_ok() {
            commands.entity(entity).insert(
                TextBundle::from_section(
                    &tooltip.text,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                        font_size: 16.0,
                        color: colors::WHITE,
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    padding: UiRect::all(Val::Px(8.0)),
                    ..default()
                }),
            );
        }
    }
}

/// System to handle keyboard navigation between focusable elements
pub fn focus_navigation_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut focus_query: Query<(Entity, &mut Focusable, &mut BackgroundColor, &mut BorderColor, &GlobalTransform)>,
    mut current_focus: Local<Option<Entity>>,
) {
    // Handle tab navigation
    if keyboard_input.just_pressed(KeyCode::Tab) {
        let mut focusables: Vec<_> = focus_query.iter_mut().collect();
        
        // Sort by vertical then horizontal position
        focusables.sort_by(|a, b| {
            let a_pos = a.4.translation();
            let b_pos = b.4.translation();
            a_pos.y.total_cmp(&b_pos.y).then(a_pos.x.total_cmp(&b_pos.x))
        });
        
        if let Some(current) = *current_focus {
            if let Some(pos) = focusables.iter().position(|(e, _, _, _, _)| *e == current) {
                let next_pos = (pos + 1) % focusables.len();
                *current_focus = Some(focusables[next_pos].0);
            }
        } else if !focusables.is_empty() {
            *current_focus = Some(focusables[0].0);
        }
    }
    
    // Update focus states
    for (entity, mut focusable, mut bg_color, mut border_color, _) in &mut focus_query {
        let is_focused = *current_focus == Some(entity);
        
        focusable.state = if is_focused {
            FocusState::Focused
        } else {
            FocusState::NotFocused
        };
        
        // Visual feedback
        if is_focused {
            *bg_color = colors::focus::HIGHLIGHT.into();
            *border_color = colors::focus::BORDER.into();
        }
    }
}

/// System to update visual feedback for focused elements
pub fn focus_visual_system(
    mut query: Query<(
        &Focusable,
        &mut BackgroundColor,
        &mut BorderColor,
        &mut Style,
    )>,
) {
    // TODO: Implement focus visuals
}

/// System to handle dropdown interactions
pub fn dropdown_system(
    mut commands: Commands,
    mut dropdown_query: Query<(Entity, &mut Dropdown, &Interaction, &GlobalTransform, &Node), Changed<Interaction>>,
    _text_query: Query<&mut Text>,
    asset_server: Res<AssetServer>,
) {
    for (entity, mut dropdown, interaction, transform, node) in &mut dropdown_query {
        match interaction {
            Interaction::Pressed => {
                dropdown.opened = !dropdown.opened;
                
                if dropdown.opened {
                    commands.entity(entity).with_children(|parent| {
                        for (i, option) in dropdown.options.iter().enumerate() {
                            parent.spawn((
                                ButtonBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Px(30.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                                Name::new(format!("DropdownOption_{}", i)),
                            )).with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    option,
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                                        font_size: 16.0,
                                        ..default()
                                    },
                                ));
                            });
                        }
                    });
                } else {
                    // Despawn dropdown options
                    commands.entity(entity).despawn_descendants();
                }
            }
            _ => {}
        }
    }
}

/// System to handle tab switching
pub fn tab_system(
    mut commands: Commands,
    mut tab_query: Query<(&mut TabbedContainer, &Children)>, 
    mut button_query: Query<(&Interaction, &Parent), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, parent) in &mut button_query {
        if let Interaction::Pressed = interaction {
            if let Ok((mut tab_container, children)) = tab_query.get_mut(parent.get()) {
                if let Some(index) = children.iter().position(|&child| child == parent.get()) {
                    tab_container.active_tab = index;
                    
                    // Update tab visibility
                    for (i, &child) in children.iter().enumerate() {
                        commands.entity(child).insert(Visibility::Visible);
                        if i != index {
                            commands.entity(child).insert(Visibility::Hidden);
                        }
                    }
                }
            }
        }
    }
}

/// System to handle scroll pane interactions
pub fn scroll_pane_system(
    mut panes: Query<(&mut ScrollPane, &Node, &GlobalTransform)>, 
    mut scroll_events: EventReader<MouseWheel>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    
    for event in scroll_events.read() {
        for (mut pane, node, transform) in &mut panes {
            // Check if cursor is over this pane
            if let Some(cursor_pos) = window.cursor_position() {
                let node_rect = node.logical_rect(transform);
                
                if node_rect.contains(cursor_pos) {
                    // Update scroll position based on wheel movement
                    let scroll_delta = match event.unit {
                        MouseScrollUnit::Line => event.y * 20.0,
                        MouseScrollUnit::Pixel => event.y,
                    };
                    
                    pane.scroll_position.y = (pane.scroll_position.y + scroll_delta)
                        .max(0.0).min(pane.max_scroll.y);
                }
            }
        }
    }
}

/// System to handle setting row hover/select
pub fn setting_row_system(
    mut row_query: Query<(
        &SettingRow,
        &mut BackgroundColor,
        &Interaction,
        &Focusable
    ), Changed<Interaction>>,
    mut tooltip_query: Query<&mut Tooltip>,
) {
    for (setting_row, mut bg_color, interaction, focusable) in &mut row_query {
        match (interaction, focusable.state) {
            (Interaction::Pressed, _) => {
                *bg_color = Color::srgb(0.2, 0.2, 0.4).into();
            }
            (Interaction::Hovered, FocusState::NotFocused) => {
                *bg_color = Color::srgb(0.3, 0.3, 0.5).into();
                
                // Show tooltip if available
                if let Some(help_text) = &setting_row.help_text {
                    if let Ok(mut tooltip) = tooltip_query.get_single_mut() {
                        tooltip.text = help_text.clone();
                    }
                }
            }
            (_, FocusState::Focused) => {
                *bg_color = Color::srgb(0.4, 0.4, 0.6).into();
            }
            _ => {
                *bg_color = Color::srgb(0.15, 0.15, 0.3).into();
            }
        }
    }
}

/// Registers all UI systems and components with the Bevy app
/// 
/// # Arguments
/// * `app` - The Bevy App to register systems with
pub fn update(app: &mut App) {
    app.register_type::<Tooltip>()
        .register_type::<UiSlider>()
        .register_type::<Dropdown>()
        .register_type::<ScrollPane>()
        .add_systems(
            Update,
            (
                tooltip_system,
                slider_interaction_system,
                dropdown_system,
                scroll_pane_system,
                setting_row_system,
                tab_system,
                focus_navigation_system,
            ),
        );
}
