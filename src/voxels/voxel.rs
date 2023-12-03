#[derive(Clone, Copy)]
pub enum TextureType {
    Hull = 0,
    Core = 1,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum VoxelID {
    ShipCore = 0,
    LightHull = 1,
    COUNT = 2,
}

impl From<i32> for VoxelID {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::ShipCore,
            1 => Self::LightHull,
            _ => todo!(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Voxel {
    pub id: VoxelID,
    pub life: f32,
}

#[derive(Clone)]
pub struct VoxelDescriptor {
    pub max_life: f32,
    pub texture_type: TextureType,
}
