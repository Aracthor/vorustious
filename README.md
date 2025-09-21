# Vorustious

A Rust voxel-based spacefight game with homemade rendering engine, maths library and collision detection.

![Vorustious -- shooting](/screenshots/shooting.png)

## Features

Currently, player pilot a ship able to collide with other bodies.<br/>
The ship can shoot projectiles damaging first intersecting voxel on other bodies.<br/>
Voxels taking damages are eventually destroyed.<br/>
If a destroyed voxel was the last one connecting to part of one body, the body is effectively cut in two separate bodies.

## Testing

Run unit tests covering maths functions, physics code, voxel coordinate system and gameplay code (projectile system) with:
```
cargo test
```

Run profiler for collision detection with:
```
cargo run -r -- profile
```

## Running the game

Test a scene with a ship and an inert other body with:
```
cargo run -r
```

Rotate the ship up and down or right or left with mouse movement, accelerate with **W**, accelerate backward with **S**, roll with **Q** or **E**, shoot with left mouse button.<br/>
You should be able to "push" the body by colliding it, to destroy its voxels by shooting them, to cut it into several bodies by destroying a voxel among those linking its "wings" to the center cockpit.

![Vorustious -- body cut in two](/screenshots/cut_in_two.png)

## Running the editor

Start the body editor with:

```
cargo run -r -- editor
```

Move the free-fly camera with wasd, rotate it with mouse movement.<br/>
Add a voxel by pressing right mouse button aiming a voxel face close to the camera. Iterates through different voxel types to add using the mouse wheel while pressing the right mouse button.<br/>
Delete a voxel by pressing left mouse button aiming a voxel close to the camera.<br/>
Enable or disable "symmetry planes" by pressing Ctrl+X, Ctrl+Y or Ctrl+Z. When a symmetry plane is enabled, each voxel added on one side of the plane will be reflected on the other side of the plane.<br/>
Several symmetry plane can be enabled together, creating up to 8 voxels on each added voxel when they are all enabled.<br/>
Save currently edited body with **F5**. Load last save with **F9**.

![Vorustious -- editor](/screenshots/editor.png)

## Dependencies
* [GLFW](https://crates.io/crates/glfw) for window creation and event handling.
* [OpenGL ES 3.0 (gl crate)](https://crates.io/crates/gl) for rendering.
