use crate::maths::vector::Vect3i;
use crate::voxels::structure::Structure;
use crate::voxels::voxel::Voxel;
use crate::voxels::voxel::VoxelID;

pub struct Editor {
    pub structure: Structure,
    pub voxel_position: Option<Vect3i>,
    pub symetry_x: bool,
    pub symetry_y: bool,
    pub symetry_z: bool,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            structure: Structure::new(0, 0, 0, 0, 0, 0, Voxel{id: VoxelID::ShipCore, life: 5.0}),
            voxel_position: None,
            symetry_x: false,
            symetry_y: false,
            symetry_z: false,
        }
    }
}
