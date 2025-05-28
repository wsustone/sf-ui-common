use bevy::prelude::*;

/// Enum representing different settings tabs
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States, Reflect)]
pub enum SettingsTab {
    /// Video settings tab (default)
    #[default]
    Video,
    /// Audio settings tab
    Audio,
    /// Controls settings tab
    Controls,
    /// Gameplay settings tab
    Gameplay,
    /// Interface settings tab
    Interface,
}

/// Types of sliders in the settings menu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum SliderType {
    /// Master volume control
    MasterVolume,
    /// Background music volume control
    MusicVolume,
    /// Sound effects volume control
    SfxVolume,
    /// Voice volume control
    VoiceVolume,
    /// Ambient/background volume control
    AmbientVolume,
    /// Horizontal slider orientation
    Horizontal,
    /// Vertical slider orientation
    Vertical,
}

/// Types of checkboxes in the settings menu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum CheckboxType {
    /// A toggle switch (on/off)
    Toggle,
    /// A radio button (select one from many)
    Radio,
}

/// Window mode options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum WindowMode {
    /// Fullscreen mode with exclusive display
    Fullscreen,
    /// Windowed mode with borders
    Windowed,
    /// Borderless fullscreen window
    BorderlessFullscreen,
}
