# Rantz Camera2D

A camera system for the [Bevy](https://bevyengine.org/) game engine using [rantz_spatial2d](https://www.github.com/BobG1983/rantz_spatial2d). Heavily inspired by the Love2D plugin [STALKER-X](https://github.com/a327ex/STALKER-X).

## Features

The primary feature of this plugin is the `CameraBundle2D` and `CameraStyle`. 

### `CameraStyle`

Three Camera Styles are defined:
* Exact
* Deadzone
* Screen By Screen

#### Exact

A camera with this style will follow the its `CameraTarget` exactly. `CameraLerp` and `CameraLead` have no effect.

#### Deadzone

A camera with this style will be stationary unless the `CameraTarget` leaves the defined `Deadzone`. 

Once the target has left the deadzone the camera will lerp towards the target based on the value of `CameraLerp` (values of 2.0 - 10.0 work best). 

If `CameraLead` has a value greater than 1.0 then the camera will instead attempt to target a number of world units ahead of the targets motion equal to the provided X and Y values.

`Deadzone` is defined in world units and is not effected by the projection scale. In other words, if you zoom in the camera, the value of `Deadzone` will not change.

#### Screen by Screen

A camera with this style will be stationary unless the `CameraTarget` leaves the visible screen area. 

Once the target has left the current screen the camera will lerp towards the center of the next screen based on the value of `CameraLerp` (values of 2.0 - 10.0 work best). 

`CameraLead` has no effect.

The screen size is based on the current visible screen size, and thus takes into account the various options in bevy's `OrthographicProjection` including both `Scale` and `ScalingMode`.

## Usage

Add the crate to your `Cargo.toml`, `CameraPlugin2D` to your app, and add `CameraBundle2D` to your camera.

**NOTE:** This plugin relies heavily on [rantz_spatial2d](https://www.github.com/BobG1983/rantz_spatial2d). If you are not using [rantz_spatial2d](https://www.github.com/BobG1983/rantz_spatial2d) for your games spatial system, this camera system will not function.

