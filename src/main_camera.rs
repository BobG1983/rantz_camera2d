use crate::prelude::*;
use bevy::prelude::*;
use rantz_spatial2d::prelude::*;

fn main_camera_valid(num_cameras: usize, num_camera_targets: usize) -> bool {
    if main_camera_exists(num_cameras) && main_camera_target_exists(num_camera_targets) {
        return true;
    }

    if num_cameras > 1 {
        error!("Multiple main cameras found, only one is supported");
    }

    if num_camera_targets > 1 {
        error!("Multiple main camera targets found, only one is supported");
    }
    return false;
}

fn main_camera_exists(num_cameras: usize) -> bool {
    if num_cameras == 1 {
        return true;
    }
    return false;
}

fn main_camera_target_exists(num_camera_targets: usize) -> bool {
    if num_camera_targets == 1 {
        return true;
    }
    return false;
}

pub fn set_intial_main_camera_target(
    mut commands: Commands,
    main_camera_query: Query<Entity, (With<MainCamera>, Without<CameraTarget>)>,
    main_camera_target_query: Query<Entity, (With<MainCameraShouldTarget>, With<Position2D>)>,
) {
    let num_cams = main_camera_query.iter().count();
    let num_targets = main_camera_target_query.iter().count();
    if main_camera_valid(num_cams, num_targets) {
        let main_camera = main_camera_query.get_single().unwrap();
        let main_camera_target = main_camera_target_query.get_single().unwrap();

        commands
            .entity(main_camera)
            .insert(CameraTarget(Some(main_camera_target)));
    }
}

pub fn main_camera_target_added(
    main_camera_target_query: Query<Entity, (Added<MainCameraShouldTarget>, With<Position2D>)>,
    mut main_camera_query: Query<&mut CameraTarget, With<MainCamera>>,
) {
    let num_cams = main_camera_query.iter().count();
    let num_targets = main_camera_target_query.iter().count();
    if main_camera_valid(num_cams, num_targets) {
        let mut camera_target = main_camera_query.get_single_mut().unwrap();
        let main_camera_target = main_camera_target_query.get_single().unwrap();
        let previous_target = camera_target.0;
        if let Some(t) = previous_target {
            if t == main_camera_target {
                return;
            }
        }
        camera_target.0 = Some(main_camera_target);
    }
}

pub fn main_camera_target_removed(
    mut main_camera_query: Query<&mut CameraTarget, With<MainCamera>>,
    main_camera_target_query: Query<Entity, (With<MainCameraShouldTarget>, With<Position2D>)>,
) {
    let num_cams = main_camera_query.iter().count();
    let num_targets = main_camera_target_query.iter().count();
    if main_camera_exists(num_cams) && num_targets == 0 {
        let mut camera_target = main_camera_query.get_single_mut().unwrap();
        if let Some(_) = camera_target.0 {
            camera_target.0 = None;
        }
    }
}
