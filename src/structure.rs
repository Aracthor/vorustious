use crate::maths::boxes::Box;
use crate::maths::segment::Segm3f;
use crate::maths::vector::Vect;

pub struct Structure {
    voxel_box: Box<3, i32>,
    data: Vec<bool>,
}

fn sign(n: f32) -> i32 {
    if n > 0.0 {
        return 1;
    }
    if n < 0.0 {
        return -1;
    }
    0
}

fn less_than(n: i32, step: i32, max: i32) -> bool {
    if step > 0 {
        n <= max
    } else {
        n >= max
    }
}

impl Structure {
    pub fn new(min_x: i32, max_x: i32, min_y: i32, max_y: i32, min_z: i32, max_z: i32) -> Self {
        let extent_x = max_x - min_x + 1;
        let extent_y = max_y - min_y + 1;
        let extent_z = max_z - min_z + 1;
        let vec_size: usize = (extent_x * extent_y * extent_z).try_into().unwrap();
        Self {
            voxel_box: Box::<3, i32>::from_min_max(Vect::<3, i32>::new([min_x, min_y, min_z]), Vect::<3, i32>::new([max_x, max_y, max_z])),
            data: vec![true; vec_size],
        }
    }

    fn voxel_index(&self, x: i32, y: i32, z: i32) -> usize {
        assert!(self.voxel_box.contains(Vect::<3, i32>::new([x, y, z])));
        let extent = self.voxel_box.extent() + Vect::<3, i32>::new([1, 1, 1]);
        let x_in_data = x - self.voxel_box.min()[0];
        let y_in_data = y - self.voxel_box.min()[1];
        let z_in_data = z - self.voxel_box.min()[2];
        (z_in_data * (extent[0] * extent[1]) + y_in_data * extent[0] + x_in_data).try_into().unwrap()
    }

    pub fn for_each_voxel<F: Fn(i32, i32, i32)>(&self, f: F) {
        for z in self.voxel_box.min()[2]..self.voxel_box.max()[2] + 1 {
            for y in self.voxel_box.min()[1]..self.voxel_box.max()[1] + 1 {
                for x in self.voxel_box.min()[0]..self.voxel_box.max()[0] + 1 {
                    if self.has_voxel(x, y, z) {
                        f(x, y, z);
                    }
                }
            }
        }
    }

    pub fn for_voxels_in_segment<F: Fn(&mut bool)>(&mut self, segment: Segm3f, f: F) {
        let dir = segment.direction();

        let mut new_segment = segment;
        new_segment.start += crate::maths::vector::Vect3f::new([0.5, 0.5, 0.5]);
        new_segment.end += crate::maths::vector::Vect3f::new([0.5, 0.5, 0.5]);

        let mut x = new_segment.start[0] as i32;
        let mut y = new_segment.start[1] as i32;
        let mut z = new_segment.start[2] as i32;

        let step_x = sign(dir[0]);
        let step_y = sign(dir[1]);
        let step_z = sign(dir[2]);

        let next_pixel_boundary_x = x + if step_x < 0 {0} else {1};
        let next_pixel_boundary_y = y + if step_y < 0 {0} else {1};
        let next_pixel_boundary_z = z + if step_z < 0 {0} else {1};

        let mut max_x = if dir[0] != 0.0 { (next_pixel_boundary_x as f32 - new_segment.start[0]) / dir[0] } else { f32::MAX };
        let mut max_y = if dir[1] != 0.0 { (next_pixel_boundary_y as f32 - new_segment.start[1]) / dir[1] } else { f32::MAX };
        let mut max_z = if dir[2] != 0.0 { (next_pixel_boundary_z as f32 - new_segment.start[2]) / dir[2] } else { f32::MAX };

        let delta_x = step_x as f32 / dir[0];
        let delta_y = step_y as f32 / dir[1];
        let delta_z = step_z as f32 / dir[2];

        while
            less_than(x, step_x, new_segment.end[0] as i32) &&
            less_than(y, step_y, new_segment.end[1] as i32) &&
            less_than(z, step_z, new_segment.end[2] as i32)
        {
            if self.voxel_box.contains(Vect::<3, i32>::new([x, y, z])) {
                let index = self.voxel_index(x, y, z);
                f(&mut self.data[index]);
            }
            if max_x < max_y {
                if max_x < max_z {
                    x += step_x;
                    max_x += delta_x;
                } else {
                    z += step_z;
                    max_z += delta_z;
                }
            } else {
                if max_y < max_z {
                    y += step_y;
                    max_y += delta_y;
                } else {
                    z += step_z;
                    max_z += delta_z;
                }
            }
        }
    }

    fn has_voxel(&self, x: i32, y: i32, z: i32) -> bool {
        assert!(self.voxel_box.contains(Vect::<3, i32>::new([x, y, z])));
        let index = self.voxel_index(x, y, z);
        self.data[index]
    }
}
