use crate::maths::matrix::Mat4f;
use crate::maths::segment::Segm3f;
use crate::maths::vector::Vect3i;
use super::structure::Structure;
use super::voxel::Voxel;

pub struct Body {
    repere: Mat4f,
    structure: Structure,
}

impl Body {
    pub fn new(structure: Structure) -> Self {
        Self {
            repere: Mat4f::identity(),
            structure: structure,
        }
    }

    pub fn repere(&self) -> &Mat4f {
        return &self.repere;
    }

    pub fn structure(&self) -> &Structure {
        return &self.structure;
    }

    pub fn structure_mut(&mut self) -> &mut Structure {
        return &mut self.structure;
    }

    pub fn apply_transformation(&mut self, transformation: Mat4f) {
        self.repere = self.repere.clone() * transformation;
    }

    pub fn for_first_voxel_in_segment<F: FnMut(&mut Option<Voxel>, &Vect3i)>(&mut self, segment: Segm3f, f: F) -> bool {
        let segment_in_repere = segment.transform(&self.repere.inverse());
        self.structure.for_first_voxel_in_segment(segment_in_repere, f)
    }
}
