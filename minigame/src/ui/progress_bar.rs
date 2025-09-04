use std::marker::PhantomData;

use bevy::{prelude::*, sprite::Material2dPlugin};

use crate::{core::entities::MeshHandles, ui::progress_bar_material::ProgressBarMaterial};

pub const DEFAULT_BACKGROUND_COLOR: Color = Color::srgba(0., 0., 0., 0.75);
pub const DEFAULT_BORDER_COLOR: Color = Color::srgba(0.02, 0.02, 0.02, 0.95);
pub const DEFAULT_HIGH_COLOR: Color = Color::srgba(0., 1., 0., 0.95);
pub const DEFAULT_MODERATE_COLOR: Color = Color::srgba(1., 1., 0., 0.95);
pub const DEFAULT_LOW_COLOR: Color = Color::srgba(1., 0., 0., 0.95);

pub const DEFAULT_WIDTH: f32 = 1.2;
pub const DEFAULT_RELATIVE_HEIGHT: f32 = 0.1666;

/// Component to configure a bar
#[derive(Component, Debug, Clone, Reflect)]
pub struct BarSettings<T: Percentage + Component + TypePath> {
    /// Configure the width of the bar
    pub width: f32,
    /// Configures the offset of the bar relative to the entity its attached to.
    /// For horizontal bars, this is an offset along the y-axis, for vertical bars along the x-axis.
    pub offset: Vec2,
    pub height: BarHeight,
    pub border: BarBorder,
    pub orientation: BarOrientation,
    #[reflect(ignore)]
    pub phantom_data: PhantomData<T>,
    /// Configures the threshold for the value stage.
    /// low_color (threshold.x) moderate_color (threshold.y) high_color
    pub threshold: Vec2,
}

impl<T: Percentage + Component + TypePath> BarSettings<T> {
    fn absolute_height(&self) -> f32 {
        match self.height {
            BarHeight::Relative(pct) => pct * self.width,
            BarHeight::Static(height) => height,
        }
    }

    pub fn normalized_height(&self) -> f32 {
        match self.orientation {
            BarOrientation::Horizontal => self.absolute_height(),
            BarOrientation::Vertical => self.width,
        }
    }

    pub fn normalized_width(&self) -> f32 {
        match self.orientation {
            BarOrientation::Horizontal => self.width,
            BarOrientation::Vertical => self.absolute_height(),
        }
    }

    // fn offset_axis(&self) -> Vec2 {
    //     match self.orientation {
    //         BarOrientation::Horizontal => Vec2::Y,
    //         BarOrientation::Vertical => Vec2::X,
    //     }
    // }

    // pub fn normalized_offset(&self) -> Vec2 {
    //     self.offset * self.offset_axis()
    // }
}

impl<T: Percentage + Component + TypePath> Default for BarSettings<T> {
    fn default() -> Self {
        Self {
            width: DEFAULT_WIDTH,
            offset: Vec2::ZERO,
            height: default(),
            border: default(),
            orientation: default(),
            threshold: Vec2::ZERO,
            phantom_data: default(),
        }
    }
}

/// Describes the border of a bar. Defaults to no border
#[derive(Debug, Clone, Reflect)]
pub struct BarBorder {
    pub width: f32,
    pub color: Color,
}

impl BarBorder {
    pub fn new(width: f32) -> Self {
        Self {
            width,
            color: DEFAULT_BORDER_COLOR,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl Default for BarBorder {
    fn default() -> Self {
        Self {
            width: 0.,
            color: DEFAULT_BORDER_COLOR,
        }
    }
}

/// Describes the height of the bar
#[derive(Debug, Clone, Reflect)]
pub enum BarHeight {
    /// Bar height relative to its width
    Relative(f32),
    /// Static bar width
    Static(f32),
}

impl Default for BarHeight {
    fn default() -> Self {
        Self::Relative(DEFAULT_RELATIVE_HEIGHT)
    }
}

/// Describes the orientation a bar
/// ```
#[derive(Reflect, Debug, Clone, PartialEq, Eq, Default)]
pub enum BarOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Trait implemented by the component to be tracked by the health bar
pub trait Percentage {
    /// Value between 0 and 1
    fn value(&self) -> f32;
}

/// ForegroundColor enum. The foreground color can either be static or a tri-color spectrum
/// The tri-color spectrum defines three colors: high, moderate, and low.
/// The high color is applied when the tracked component's value is more than or equal to 80%,
/// moderate when it's between 40% and 80%, and low when it is less than 40%.
#[derive(Debug, Clone, Reflect)]
pub enum ForegroundColor {
    Static(Color),
    TriSpectrum {
        high: Color,
        moderate: Color,
        low: Color,
    },
}

/// Resource to customize the appearance of bars per tracked component type.
#[derive(Resource, Debug, Clone, Reflect)]
pub struct PBarColorScheme<T: Percentage + Component + TypePath> {
    pub foreground_color: ForegroundColor,
    pub background_color: Color,
    #[reflect(ignore)]
    phantom_data: PhantomData<T>,
}

impl<T: Percentage + Component + TypePath> PBarColorScheme<T> {
    /// Returns a default initialized ColorScheme for the given component type
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy_health_bar3d::prelude::ColorScheme;
    /// let color_scheme = ColorScheme::<Health>::new();
    /// ```
    pub fn new() -> Self {
        Self {
            phantom_data: PhantomData,
            ..default()
        }
    }

    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    /// Sets the foreground color to either a static value or a tri-color spectrum
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy::prelude::Color;
    /// use bevy_health_bar3d::prelude::{ColorScheme, ForegroundColor};
    /// let mana_scheme = ColorScheme::<Mana>::new().foreground_color(ForegroundColor::Static(BLUE.into()));
    /// let health_scheme = ColorScheme::<Health>::new().foreground_color(ForegroundColor::TriSpectrum {
    ///     high: GREEN.into(),
    ///     moderate: YELLOW.into(),
    ///     low: RED.into(),
    /// });
    /// ```
    pub fn foreground_color(mut self, color: ForegroundColor) -> Self {
        self.foreground_color = color;
        self
    }
}

impl<T: Percentage + Component + TypePath> Default for PBarColorScheme<T> {
    fn default() -> Self {
        Self {
            foreground_color: ForegroundColor::TriSpectrum {
                high: DEFAULT_HIGH_COLOR,
                moderate: DEFAULT_MODERATE_COLOR,
                low: DEFAULT_LOW_COLOR,
            },
            background_color: DEFAULT_BACKGROUND_COLOR,
            phantom_data: PhantomData,
        }
    }
}

pub struct ProgressBarPlugin<T: Percentage + Component + TypePath> {
    phantom: PhantomData<T>,
}

impl<T: Percentage + Component + TypePath> Default for ProgressBarPlugin<T> {
    fn default() -> Self {
        Self {
            phantom: Default::default(),
        }
    }
}

impl<T: Percentage + Component + TypePath> Plugin for ProgressBarPlugin<T> {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<Material2dPlugin<ProgressBarMaterial>>() {
            app.add_plugins(Material2dPlugin::<ProgressBarMaterial>::default());
        }

        app.init_resource::<MeshHandles>();
    }
}
