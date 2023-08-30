use crate::maths::vector::Vect3f;
use crate::maths::matrix::Mat4f;

pub struct Camera {
    pub position: Vect3f,
    pub angle_x: f32,
    pub angle_y: f32,
}

impl Camera {
    pub fn view_matrix(&self) -> Mat4f {
        let target = self.position + self.forward();
        Mat4f::look_at(self.position, target, self.up())
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
}
