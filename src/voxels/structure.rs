use crate::maths::boxes::Box3i;
use crate::maths::segment::Segm3f;
use crate::maths::vector::Vect3i;
use super::catalog::VoxelCatalog;
use super::voxel::Voxel;

pub struct Structure {
    voxel_box: Box3i,
    data: Vec<Option<Voxel>>,
}

impl Structure {
    pub fn new(min_x: i32, max_x: i32, min_y: i32, max_y: i32, min_z: i32, max_z: i32, voxel: Voxel) -> Self {
        let extent_x = max_x - min_x + 1;
        let extent_y = max_y - min_y + 1;
        let extent_z = max_z - min_z + 1;
        let vec_size: usize = (extent_x * extent_y * extent_z).try_into().unwrap();
        Self {
            voxel_box: Box3i::from_min_max(Vect3i::new([min_x, min_y, min_z]), Vect3i::new([max_x, max_y, max_z])),
            data: vec![Some(voxel); vec_size],
        }
    }

    pub fn empty(min_x: i32, max_x: i32, min_y: i32, max_y: i32, min_z: i32, max_z: i32) -> Self {
        let extent_x = max_x - min_x + 1;
        let extent_y = max_y - min_y + 1;
        let extent_z = max_z - min_z + 1;
        let vec_size: usize = (extent_x * extent_y * extent_z).try_into().unwrap();
        Self {
            voxel_box: Box3i::from_min_max(Vect3i::new([min_x, min_y, min_z]), Vect3i::new([max_x, max_y, max_z])),
            data: vec![None; vec_size],
        }
    }

    const SEPARATOR_EXTENT: &str = ";";
    const SEPARATOR_X: &str = " ";
    const SEPARATOR_Y: &str = "|";
    const SEPARATOR_Z: &str = "\n";

    #[allow(dead_code)]
    pub fn serialize(&self) -> String {
        let mut result =
        self.voxel_box.min()[0].to_string() + Self::SEPARATOR_EXTENT +
        &self.voxel_box.max()[0].to_string() + Self::SEPARATOR_EXTENT +
        &self.voxel_box.min()[1].to_string() + Self::SEPARATOR_EXTENT +
        &self.voxel_box.max()[1].to_string() + Self::SEPARATOR_EXTENT +
        &self.voxel_box.min()[2].to_string() + Self::SEPARATOR_EXTENT +
        &self.voxel_box.max()[2].to_string();
        for z in self.voxel_box.min()[2]..self.voxel_box.max()[2] + 1 {
            result += Self::SEPARATOR_Z;
            for y in self.voxel_box.min()[1]..self.voxel_box.max()[1] + 1 {
                if y > self.voxel_box.min()[1] {
                    result += Self::SEPARATOR_Y;
                }
                for x in self.voxel_box.min()[0]..self.voxel_box.max()[0] + 1 {
                    if x > self.voxel_box.min()[0] {
                        result += Self::SEPARATOR_X;
                    }
                    let voxel = self.data[self.voxel_index(Vect3i::new([x, y, z]))];
                    if voxel.is_some() {
                        result += &(voxel.unwrap().id as i32).to_string();
                    }
                }
            }
        }
        result
    }

    #[allow(dead_code)]
    pub fn deserialize(catalog: &VoxelCatalog, str: &str) -> Self {
        let mut lines = str.split(Self::SEPARATOR_Z);
        let first_line = lines.next();
        let mut extent_lines = first_line.unwrap().split(Self::SEPARATOR_EXTENT);
        let box_min_x: i32 = extent_lines.next().unwrap().parse().unwrap();
        let box_max_x: i32 = extent_lines.next().unwrap().parse().unwrap();
        let box_min_y: i32 = extent_lines.next().unwrap().parse().unwrap();
        let box_max_y: i32 = extent_lines.next().unwrap().parse().unwrap();
        let box_min_z: i32 = extent_lines.next().unwrap().parse().unwrap();
        let box_max_z: i32 = extent_lines.next().unwrap().parse().unwrap();
        assert!(extent_lines.next().is_none());
        let mut result = Self::empty(box_min_x, box_max_x, box_min_y, box_max_y, box_min_z, box_max_z);
        for z in box_min_z..box_max_z + 1 {
            let mut next_column = lines.next().unwrap().split(Self::SEPARATOR_Y);
            for y in box_min_y..box_max_y + 1 {
                let mut next_line = next_column.next().unwrap().split(Self::SEPARATOR_X);
                for x in box_min_x..box_max_x + 1 {
                    let voxel_str = next_line.next().unwrap();
                    if !voxel_str.is_empty() {
                        let voxel_id: i32 = voxel_str.parse().unwrap();
                        result.add_voxel(Vect3i::new([x, y, z]), catalog.create_voxel(voxel_id.try_into().unwrap()));
                    }
                }
            }
        }
        result
    }

    #[cfg(test)]
    pub fn set_voxel(&mut self, x: i32, y: i32, z: i32, voxel: Option<Voxel>) {
        let index = self.voxel_index(Vect3i::new([x, y, z]));
        self.data[index] = voxel;
    }

    pub fn add_voxel(&mut self, coords: Vect3i, voxel: Voxel) {
        if !self.voxel_box.contains(coords) {
            let mut new_box = self.voxel_box.clone();
            new_box.add(coords);
            self.resize(new_box);
        }
        let index = self.voxel_index(coords);
        self.data[index] = Some(voxel);
    }

    pub fn for_each_voxel<F: FnMut(i32, i32, i32, &Voxel)>(&self, mut f: F) {
        for z in self.voxel_box.min()[2]..self.voxel_box.max()[2] + 1 {
            for y in self.voxel_box.min()[1]..self.voxel_box.max()[1] + 1 {
                for x in self.voxel_box.min()[0]..self.voxel_box.max()[0] + 1 {
                    if self.has_voxel(x, y, z) {
                        let index = self.voxel_index(Vect3i::new([x, y, z]));
                        let voxel = &mut self.data[index].unwrap();
                        f(x, y, z, voxel);
                    }
                }
            }
        }
    }

    pub fn for_each_voxel_mut<F: Fn(i32, i32, i32, &mut Option<Voxel>)>(&mut self, f: F) {
        for z in self.voxel_box.min()[2]..self.voxel_box.max()[2] + 1 {
            for y in self.voxel_box.min()[1]..self.voxel_box.max()[1] + 1 {
                for x in self.voxel_box.min()[0]..self.voxel_box.max()[0] + 1 {
                    if self.has_voxel(x, y, z) {
                        let index = self.voxel_index(Vect3i::new([x, y, z]));
                        let voxel = &mut self.data[index];
                        f(x, y, z, voxel);
                    }
                }
            }
        }
    }

    // TODO this function should not mut self, but it calls apply_on_voxels and... Read its comment.
    pub fn outside_voxel_coords(&mut self, segment: Segm3f) -> Option<Vect3i> {
        let mut result: Option<Vect3i> = None;
        self.apply_on_voxels(segment, |voxel: &mut Option<Voxel>, coords: &Vect3i, face: &Vect3i| {
            if voxel.is_some() && *face != Vect3i::zero() {
                result = Some(*coords + *face);
            }
            voxel.is_some()
        });
        result
    }

    #[cfg(test)]
    pub fn get_face(&mut self, segment: Segm3f) -> Vect3i {
        let mut result = Vect3i::zero();
        self.apply_on_voxels(segment, |voxel: &mut Option<Voxel>, _coords, face: &Vect3i| {
            let has_voxel = voxel.is_some();
            if has_voxel {
                result = *face;
            }
            has_voxel
        });
        result
    }

    pub fn for_first_voxel_in_segment<F: FnMut(&mut Option<Voxel>)>(&mut self, segment: Segm3f, mut f: F) -> bool {
        self.apply_on_voxels(segment, |voxel: &mut Option<Voxel>, _coords, _face| {
            let has_voxel = voxel.is_some();
            if has_voxel {
                f(voxel);
            }
            has_voxel
        })
    }

    #[allow(dead_code)]
    pub fn for_voxels_in_segment<F: Fn(&mut Option<Voxel>)>(&mut self, segment: Segm3f, f: F) -> bool {
        self.apply_on_voxels(segment, |voxel: &mut Option<Voxel>, _coords, _face| {
            f(voxel);
            false
        })
    }

    // Using "A Fast Voxel Traversal Algorithm for Ray Tracing" by John Amanatides and Andrew Woo, 1987
    // http://www.cse.yorku.ca/~amana/research/grid.pdf
    // Adapted to work in negative space by dogfuntom
    // https://gist.github.com/dogfuntom/cc881c8fc86ad43d55d8
    //
    // This function should exists in two versions : &mut self with Fn closure, and &self with FnMut closure.
    // But it seems impossible to do such a thing without making a complete duplicate of this already long function...
    // So let's just put a version that is &mut self AND use a FnMut for now.
    fn apply_on_voxels<F: FnMut(&mut Option<Voxel>, &Vect3i, &Vect3i) -> bool>(&mut self, segment: Segm3f, mut f: F) -> bool {
        fn sign(n: f32) -> i32 {
            if n > 0.0 {
                return 1;
            }
            if n < 0.0 {
                return -1;
            }
            0
        }

        fn intbound(s: f32, ds: f32) -> f32 {
            if ds == 0.0 {
                return f32::MAX;
            }

            (if ds > 0.0 { s.ceil() - s } else { s - s.floor()}) / ds.abs()
        }

        fn less_than(n: i32, step: i32, max: i32) -> bool {
            if step == 0 {
                return true
            }
            if step > 0 {
                n <= max
            } else {
                n >= max
            }
        }

        let mut new_segment = segment.clone();

        new_segment.start += crate::maths::vector::Vect3f::new([0.5, 0.5, 0.5]);
        new_segment.end += crate::maths::vector::Vect3f::new([0.5, 0.5, 0.5]);

        let dir = new_segment.direction();

        let mut x = new_segment.start[0].floor() as i32;
        let mut y = new_segment.start[1].floor() as i32;
        let mut z = new_segment.start[2].floor() as i32;
        let mut face = Vect3i::zero();

        let end_x = new_segment.end[0].floor() as i32;
        let end_y = new_segment.end[1].floor() as i32;
        let end_z = new_segment.end[2].floor() as i32;

        let step_x = sign(dir[0]);
        let step_y = sign(dir[1]);
        let step_z = sign(dir[2]);

        let mut max_x = intbound(new_segment.start[0], dir[0]);
        let mut max_y = intbound(new_segment.start[1], dir[1]);
        let mut max_z = intbound(new_segment.start[2], dir[2]);

        let delta_x = step_x as f32 / dir[0];
        let delta_y = step_y as f32 / dir[1];
        let delta_z = step_z as f32 / dir[2];

        let mut hit = false;
        while
            less_than(x, step_x, end_x) &&
            less_than(y, step_y, end_y) &&
            less_than(z, step_z, end_z)
        {
            let coords = Vect3i::new([x, y, z]);
            if self.voxel_box.contains(coords) {
                let index = self.voxel_index(coords);
                hit |= self.data[index].is_some();
                if f(&mut self.data[index], &coords, &face) {
                    break;
                }
            }
            if max_x < max_y {
                if max_x < max_z {
                    x += step_x;
                    max_x += delta_x;
                    face = Vect3i::new([-step_x, 0, 0]);
                } else {
                    z += step_z;
                    max_z += delta_z;
                    face = Vect3i::new([0, 0, -step_z]);
                }
            } else {
                if max_y < max_z {
                    y += step_y;
                    max_y += delta_y;
                    face = Vect3i::new([0, -step_y, 0]);
                } else {
                    z += step_z;
                    max_z += delta_z;
                    face = Vect3i::new([0, 0, -step_z]);
                }
            }
        }
        hit
    }

    fn resize(&mut self, new_box: Box3i) {
        let vec_size = (new_box.extent()[0] + 1) * (new_box.extent()[1] + 1) * (new_box.extent()[2] + 1);
        let mut new_data: Vec<Option<Voxel>> = vec![None; vec_size.try_into().unwrap()];
        for z in self.voxel_box.min()[2]..self.voxel_box.max()[2] + 1 {
            for y in self.voxel_box.min()[1]..self.voxel_box.max()[1] + 1 {
                for x in self.voxel_box.min()[0]..self.voxel_box.max()[0] + 1 {
                    let coords = Vect3i::new([x, y, z]);
                    let index = Self::voxel_index_for_box(&self.voxel_box, coords);
                    let new_index = Self::voxel_index_for_box(&new_box, coords);
                    new_data[new_index] = self.data[index];
                }
            }
        }
        self.voxel_box = new_box;
        self.data = new_data;
    }

    fn voxel_index(&self, coords: Vect3i) -> usize {
        Self::voxel_index_for_box(&self.voxel_box, coords)
    }

    fn voxel_index_for_box(voxel_box: &Box3i, coords: Vect3i) -> usize {
        assert!(voxel_box.contains(coords));
        let extent = voxel_box.extent() + Vect3i::new([1, 1, 1]);
        let coords_in_data = coords - voxel_box.min();
        (coords_in_data[2] * (extent[0] * extent[1]) + coords_in_data[1] * extent[0] + coords_in_data[0]).try_into().unwrap()
    }

    fn has_voxel(&self, x: i32, y: i32, z: i32) -> bool {
        let coords = Vect3i::new([x, y, z]);
        assert!(self.voxel_box.contains(coords));
        let index = self.voxel_index(coords);
        self.data[index].is_some()
    }
}

impl Eq for Structure {}
impl PartialEq for Structure {
    fn eq(&self, other: &Self) -> bool {
        if self.voxel_box != other.voxel_box {
            return false;
        }
        for z in self.voxel_box.min()[2]..self.voxel_box.max()[2] + 1 {
            for y in self.voxel_box.min()[1]..self.voxel_box.max()[1] + 1 {
                for x in self.voxel_box.min()[0]..self.voxel_box.max()[0] + 1 {
                    if self.has_voxel(x, y, z) != other.has_voxel(x, y, z) {
                        return false;
                    }
                }
            }
        }
        true
    }
}
