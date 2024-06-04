use crate::prelude::*;
use bevy::prelude::*;
use rantz_spatial2d::prelude::*;

pub fn camera_follow_target(
    mut camera_query: Query<
        (
            &mut Position2D,
            &CameraStyle,
            &CameraLerp,
            &mut CameraLead,
            &CameraTarget,
            &OrthographicProjection,
        ),
        (With<Camera>, With<Camera2d>),
    >,
    all_positions_query: Query<&Position2D, Without<Camera>>,
    time: Res<Time>,
) {
    for (mut camera_position, style, lp, mut ld, camera_target, ortho) in camera_query.iter_mut() {
        if camera_target.0.is_none() {
            return;
        }

        if !all_positions_query.contains(camera_target.0.unwrap()) {
            return;
        }

        match style {
            CameraStyle::DeadZone(deadzone) => {
                handle_deadzone(
                    &all_positions_query.get(camera_target.0.unwrap()).unwrap(),
                    &mut camera_position,
                    &deadzone,
                    &mut ld,
                    &lp,
                    time.delta_seconds(),
                );
            }
            CameraStyle::ScreenByScreen => {
                handle_screen_by_screen(
                    &ortho,
                    &all_positions_query.get(camera_target.0.unwrap()).unwrap(),
                    &mut camera_position,
                    &lp.factor,
                    time.delta_seconds(),
                );
            }
            CameraStyle::Exact => {
                handle_exact(
                    &mut camera_position,
                    &all_positions_query.get(camera_target.0.unwrap()).unwrap(),
                );
            }
        }
    }
}

fn handle_exact(camera_position: &mut Position2D, target_pos: &Position2D) {
    *camera_position = target_pos.clone();
}

fn handle_deadzone(
    target_pos: &Position2D,
    cam_pos: &mut Position2D,
    deadzone: &DeadZone,
    ld: &mut CameraLead,
    lp: &CameraLerp,
    dt: f32,
) {
    let mut scroll_dist = calculate_scroll_distance_for_deadzone(&deadzone, &target_pos, &cam_pos);

    // Apply lead
    if ld.last_target_position.is_none() {
        ld.last_target_position = Some(target_pos.clone());
    }
    let last_target = ld.last_target_position.unwrap();
    let dir_of_target_movement = Vec2::from(target_pos - last_target).normalize_or_zero();
    let lead_vec = dir_of_target_movement * ld.lead_amount;
    scroll_dist = scroll_dist + lead_vec;
    ld.last_target_position = Some(target_pos.clone());

    // Apply lerp
    let new_pos = Position2D::new(
        lerp(cam_pos.x + scroll_dist.x, cam_pos.x, lp.factor.x * dt),
        lerp(cam_pos.y + scroll_dist.y, cam_pos.y, lp.factor.y * dt),
    );

    *cam_pos = new_pos;
}

fn handle_screen_by_screen(
    ortho: &OrthographicProjection,
    target_pos: &Position2D,
    camera_position: &mut Position2D,
    lerp_factor: &Vec2,
    dt: f32,
) {
    let scaled_bounds = calculate_scaled_screenbounds(ortho);
    let cam_pos = camera_position.clone();
    let sw = scaled_bounds.x;
    let sh = scaled_bounds.y;
    let hw = sw / 2.;
    let hh = sh / 2.;
    let scroll_dist = distance_to_target_with_screenbounds(
        target_pos,
        cam_pos,
        distance_to_next_screenbounds(cam_pos, sw, sh),
        hw,
        sw,
        hh,
        sh,
    );

    // Apply lerp
    let new_pos = Position2D::new(
        lerp(cam_pos.x + scroll_dist.x, cam_pos.x, lerp_factor.x * dt),
        lerp(cam_pos.y + scroll_dist.y, cam_pos.y, lerp_factor.y * dt),
    );

    *camera_position = new_pos;
}

fn calculate_scaled_screenbounds(ortho: &OrthographicProjection) -> Vec2 {
    ortho.scale * ortho.area.size()
}

fn distance_to_target_with_screenbounds(
    target_pos: &Position2D,
    cam_pos: Position2D,
    initial_scroll_dist: Vec2,
    hw: f32,
    sw: f32,
    hh: f32,
    sh: f32,
) -> Vec2 {
    let mut scroll_dist = initial_scroll_dist;
    if target_pos.x < (cam_pos.x + scroll_dist.x) - hw {
        scroll_dist.x -= sw;
    }
    if target_pos.x > (cam_pos.x + scroll_dist.x) + hw {
        scroll_dist.x += sw;
    }

    if target_pos.y < (cam_pos.y + scroll_dist.y) - hh {
        scroll_dist.y -= sh;
    }
    if target_pos.y > (cam_pos.y + scroll_dist.y) + hh {
        scroll_dist.y += sh;
    }

    scroll_dist
}

fn distance_to_next_screenbounds(cam_pos: Position2D, sw: f32, sh: f32) -> Vec2 {
    let mut scroll_dist = Vec2::new(0.0, 0.0);
    let mod_x = cam_pos.x % sw;
    let mod_y = cam_pos.y % sh;
    if mod_x < 0.0 || mod_x > 0.0 {
        scroll_dist.x = -sw - mod_x;
    }

    if mod_x > 0.0 {
        scroll_dist.x = sw - mod_x;
    }

    if mod_y < 0.0 {
        scroll_dist.y = -sh - mod_y;
    }

    if mod_y > 0.0 {
        scroll_dist.y = sh - mod_y;
    }

    scroll_dist
}

fn lerp(target: f32, current: f32, factor: f32) -> f32 {
    return current + (target - current) * factor;
}

fn calculate_scroll_distance_for_deadzone(
    deadzone: &DeadZone,
    target_pos: &Position2D,
    cam_pos: &Position2D,
) -> Vec2 {
    let hw = deadzone.half_width();
    let hh = deadzone.half_height();
    let mut scroll_dist = Vec2::new(0.0, 0.0);

    if target_pos.x < cam_pos.x - hw {
        scroll_dist.x = target_pos.x - (cam_pos.x - hw);
    }
    if target_pos.x > cam_pos.x + hw {
        scroll_dist.x = target_pos.x - (cam_pos.x + hw);
    }

    if target_pos.y < cam_pos.y - hh {
        scroll_dist.y = target_pos.y - (cam_pos.y - hh);
    }
    if target_pos.y > cam_pos.y + hh {
        scroll_dist.y = target_pos.y - (cam_pos.y + hh);
    }

    scroll_dist
}
