#[derive(Clone, Copy)]
pub enum TextureType {
    Hull,
    Core,
}

impl Into<i32> for TextureType {
    fn into(self) -> i32 {
        match self {
            Self::Hull => 0,
            Self::Core => 1,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Voxel {
    pub life: f32,
    pub max_life: f32,
    pub texture_type: TextureType,
}
