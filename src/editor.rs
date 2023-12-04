use crate::maths::vector::Vect3i;

pub struct Editor {
    pub voxel_position: Option<Vect3i>,
    pub symetry_x: bool,
    pub symetry_y: bool,
    pub symetry_z: bool,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            voxel_position: None,
            symetry_x: false,
            symetry_y: false,
            symetry_z: false,
        }
    }
}