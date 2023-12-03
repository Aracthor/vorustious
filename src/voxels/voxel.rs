#[derive(Clone, Copy)]
pub enum TextureType {
    Hull = 0,
    Core = 1,
}

#[derive(Clone, Copy)]
pub struct Voxel {
    pub life: f32,
    pub max_life: f32,
    pub texture_type: TextureType,
}
