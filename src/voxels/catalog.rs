use super::voxel::TextureType;
use super::voxel::Voxel;
use super::voxel::VoxelDescriptor;
use super::voxel::VoxelID;

pub struct VoxelCatalog {
    descriptors: Vec<VoxelDescriptor>,
}

impl VoxelCatalog {
    pub fn create() -> Self {
        let mut descriptors: Vec<VoxelDescriptor> = vec![];
        let dummy_voxel_descriptor = VoxelDescriptor { max_life: 0.0, texture_type: TextureType::Core };
        descriptors.resize(VoxelID::COUNT as usize, dummy_voxel_descriptor);

        descriptors[VoxelID::ShipCore as usize] = VoxelDescriptor {
            max_life: 5.0,
            texture_type: TextureType::Core,
        };
        descriptors[VoxelID::LightHull as usize] = VoxelDescriptor {
            max_life: 2.0,
            texture_type: TextureType::Hull,
        };
        Self {
            descriptors: vec![
                VoxelDescriptor {
                    max_life: 5.0,
                    texture_type: TextureType::Core,
                },
                VoxelDescriptor {
                    max_life: 2.0,
                    texture_type: TextureType::Hull,
                },
            ]
        }
    }

    pub fn get_descriptor(&self, id: VoxelID) -> &VoxelDescriptor {
        &self.descriptors[id as usize]
    }

    pub fn create_voxel(&self, id: VoxelID) -> Voxel {
        let descriptor = self.get_descriptor(id);
        assert!(descriptor.max_life > 0.0);
        Voxel {
            id: id,
            life: descriptor.max_life,
        }
    }
}
