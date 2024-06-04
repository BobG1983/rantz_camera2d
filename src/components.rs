use bevy::prelude::*;
use rantz_spatial2d::prelude::*;

#[derive(Component, Default, Clone, Copy, PartialEq, Debug, Reflect)]
pub struct MainCamera;

#[derive(Component, Default, Clone, Copy, PartialEq, Debug, Reflect)]
pub struct MainCameraShouldTarget;

#[derive(Component, Default, Clone, Copy, PartialEq, Eq, Debug, Reflect)]
pub struct CameraTarget(pub Option<Entity>);

#[derive(Component, Default, Clone, Copy, PartialEq, Debug, Reflect)]
pub enum CameraStyle {
    DeadZone(DeadZone),
    ScreenByScreen,
    #[default]
    Exact,
}

#[derive(Default, Clone, Copy, PartialEq, Debug, Reflect)]
pub struct DeadZone {
    pub width: f32,
    pub height: f32,
}

impl DeadZone {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
    pub fn half_width(&self) -> f32 {
        self.width / 2.
    }
    pub fn half_height(&self) -> f32 {
        self.height / 2.
    }
}

#[derive(Component, Clone, Copy, PartialEq, Debug, Reflect)]
pub struct CameraLerp {
    pub factor: Vec2,
}

impl CameraLerp {
    pub fn new(factor: f32) -> Self {
        Self {
            factor: Vec2::new(factor, factor),
        }
    }
}

impl Default for CameraLerp {
    fn default() -> Self {
        Self { factor: Vec2::ONE }
    }
}

#[derive(Component, Default, Clone, Copy, PartialEq, Debug, Reflect)]
pub struct CameraLead {
    pub lead_amount: Vec2,
    pub last_target_position: Option<Position2D>,
}

impl CameraLead {
    pub fn new(lead_amount: Vec2) -> Self {
        Self {
            lead_amount,
            last_target_position: None,
        }
    }
}

#[derive(Bundle, Default)]
pub struct CameraBundle2D {
    pub spatial: SpatialBundle2DRaw,
    pub camera_bundle: Camera2dBundle,
    pub style: CameraStyle,
    pub lerp: CameraLerp,
    pub lead: CameraLead,
    pub target: CameraTarget,
}
