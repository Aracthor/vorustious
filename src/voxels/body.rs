use crate::maths::matrix::Mat4f;
use crate::maths::segment::Segm3f;
use crate::maths::vector::Vect3f;
use crate::maths::vector::Vect3i;
use super::structure::Structure;
use super::voxel::Voxel;

pub struct Body {
    repere: Mat4f,
    structure: Structure,
    movement: Vect3f,
}

impl Body {
    pub fn new(structure: Structure, repere: Mat4f) -> Self {
        Self {
            repere: repere,
            structure: structure,
            movement: Vect3f::zero(),
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

    pub fn for_first_voxel_in_segment<F: FnMut(&mut Option<Voxel>, &Vect3i)>(&mut self, segment: Segm3f, f: F) -> bool {
        let segment_in_repere = segment.transform(&self.repere.inverse());
        self.structure.for_first_voxel_in_segment(segment_in_repere, f)
    }

    pub fn movement(&self) -> Vect3f {
        self.movement
    }

    pub fn add_to_movement(&mut self, movement: Vect3f) {
        self.movement += movement;
    }

    pub fn apply_movement(&mut self, elapsed_time: f32) {
        self.repere = self.repere.clone() * Mat4f::translation(self.movement * elapsed_time);
    }
}
