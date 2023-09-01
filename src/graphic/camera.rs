use crate::maths::vector::Vect3f;
use crate::maths::matrix::Mat4f;
use super::windowing::event_handler;

pub struct Camera {
    position: Vect3f,
    angle_x: f32,
    angle_y: f32,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: Vect3f::new([-1.0, 0.0, 0.0]),
            angle_x: 0.0,
            angle_y: 0.0,
        }
    }

    pub fn view_matrix(&self) -> Mat4f {
        let target = self.position + self.forward();
        Mat4f::look_at(self.position, target, self.up())
    }

    pub fn position(&self) -> Vect3f {
        self.position
    }

    pub fn up(&self) -> Vect3f {
        Vect3f::new([0.0, 0.0, 1.0])
    }

    pub fn forward(&self) -> Vect3f {
        Vect3f::new([
            self.angle_y.cos() * self.angle_x.cos(),
            self.angle_y.cos() * self.angle_x.sin(),
            self.angle_y.sin(),
        ])
    }

    pub fn update_from_events(&mut self, event_handler: &event_handler::EventHandler) {
        const CAMERA_SPEED: f32 = 0.1;
        const CAMERA_SENSITIVITY: f32 = 0.005;
        let camera_forward = self.forward();
        let camera_right = Vect3f::cross(camera_forward, self.up()).normalize();

        if event_handler.is_key_pressed(event_handler::Key::W) { self.position += camera_forward * CAMERA_SPEED }
        if event_handler.is_key_pressed(event_handler::Key::S) { self.position -= camera_forward * CAMERA_SPEED }
        if event_handler.is_key_pressed(event_handler::Key::A) { self.position -= camera_right * CAMERA_SPEED }
        if event_handler.is_key_pressed(event_handler::Key::D) { self.position += camera_right * CAMERA_SPEED }

        let cursor_movement = event_handler.cursor_movement();
        self.angle_x -= cursor_movement.0 as f32 * CAMERA_SENSITIVITY;
        self.angle_y -= cursor_movement.1 as f32 * CAMERA_SENSITIVITY;
        self.angle_y = self.angle_y.clamp(-89.0_f32.to_radians(), 89.0_f32.to_radians());
    }
}
