use bevy::input::mouse::MouseScrollUnit;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

/// Advanced scrollable area component with improved performance
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct ScrollArea {
    /// Current scroll position
    pub scroll_position: Vec2,
    /// Maximum scrollable area
    pub max_scroll: Vec2,
    /// Whether scrolling is currently enabled
    pub enabled: bool,
    /// Scroll sensitivity (pixels per wheel tick)
    pub sensitivity: f32,
}

impl ScrollArea {
    /// Creates a new scroll area with default settings
    pub fn new() -> Self {
        Self {
            scroll_position: Vec2::ZERO,
            max_scroll: Vec2::ZERO,
            enabled: true,
            sensitivity: 20.0,
        }
    }

    /// Creates a scroll area with custom sensitivity
    pub fn with_sensitivity(sensitivity: f32) -> Self {
        Self {
            sensitivity,
            ..Default::default()
        }
    }
}

/// System to handle scroll area interactions
pub fn scroll_area_system(
    mut scroll_areas: Query<(&mut ScrollArea, &Node, &GlobalTransform)>,
    mut scroll_events: EventReader<MouseWheel>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    
    for event in scroll_events.read() {
        for (mut scroll_area, node, transform) in &mut scroll_areas {
            if !scroll_area.enabled {
                continue;
            }
            
            // Check if cursor is over this scroll area
            if let Some(cursor_pos) = window.cursor_position() {
                let node_rect = node.logical_rect(transform);
                
                if node_rect.contains(cursor_pos) {
                    // Update scroll position based on wheel movement
                    let scroll_delta = match event.unit {
                        MouseScrollUnit::Line => event.y * 20.0,
                        MouseScrollUnit::Pixel => event.y,
                    };
                    
                    scroll_area.scroll_position = (scroll_area.scroll_position + scroll_delta * scroll_area.sensitivity)
                        .max(Vec2::ZERO).min(scroll_area.max_scroll);
                }
            }
        }
    }
}

/// System to calculate maximum scrollable area
pub fn calculate_scroll_bounds_system(
    mut scroll_query: Query<(&mut ScrollArea, &Node, &Children)>,
    node_query: Query<&Node>,
) {
    for (mut scroll_area, node, children) in &mut scroll_query {
        let mut total_height = 0.0;
        
        for &child in children {
            if let Ok(child_node) = node_query.get(child) {
                total_height += child_node.size().y;
            }
        }
        
        scroll_area.max_scroll = Vec2::new(
            0.0,
            (total_height - node.size().y).max(0.0)
        );
    }
}
