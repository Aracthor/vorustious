use crate::maths::segment::Segm3f;

pub struct Structure {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    min_z: i32,
    max_z: i32,
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
            min_x: min_x,
            max_x: max_x,
            min_y: min_y,
            max_y: max_y,
            min_z: min_z,
            max_z: max_z,
            data: vec![true; vec_size],
        }
    }

    fn voxel_index(&self, x: i32, y: i32, z: i32) -> usize {
        assert!(x >= self.min_x && x <= self.max_x);
        assert!(y >= self.min_y && y <= self.max_y);
        assert!(z >= self.min_z && z <= self.max_z);
        let extent_x = self.max_x - self.min_x + 1;
        let extent_y = self.max_y - self.min_y + 1;
        let z_in_data = z - self.min_z;
        let y_in_data = y - self.min_y;
        let x_in_data = x - self.min_x;
        (z_in_data * (extent_x * extent_y) + y_in_data * extent_x + x_in_data).try_into().unwrap()
    }

    pub fn for_each_voxel<F: Fn(i32, i32, i32)>(&self, f: F) {
        for z in self.min_z..self.max_z + 1 {
            for y in self.min_y..self.max_y + 1 {
                for x in self.min_x..self.max_x + 1 {
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
            if x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y && z >= self.min_z && z <= self.max_z {
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
        assert!(x >= self.min_x && x <= self.max_x);
        assert!(y >= self.min_y && y <= self.max_y);
        assert!(z >= self.min_z && z <= self.max_z);
        let index = self.voxel_index(x, y, z);
        self.data[index]
    }
}
