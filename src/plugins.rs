use crate::prelude::*;
use bevy::prelude::*;
use rantz_spatial2d::prelude::*;

pub struct CameraPlugin2D;

impl Plugin for CameraPlugin2D {
    fn build(&self, app: &mut App) {
        self.register_types(app);
        self.add_systems(app);
    }
}

impl CameraPlugin2D {
    fn register_types(&self, app: &mut App) {
        app.register_type::<MainCamera>()
            .register_type::<MainCameraShouldTarget>()
            .register_type::<CameraStyle>()
            .register_type::<DeadZone>()
            .register_type::<CameraLerp>()
            .register_type::<CameraLead>()
            .register_type::<CameraTarget>();
    }

    fn add_systems(&self, app: &mut App) {
        app.add_systems(
            PostStartup,
            set_intial_main_camera_target.before(SpatialSystems2D::Propogate),
        )
        .add_systems(
            PostUpdate,
            (
                camera_follow_target,
                main_camera_target_added,
                main_camera_target_removed,
            )
                .before(SpatialSystems2D::Propogate),
        );
    }
}
