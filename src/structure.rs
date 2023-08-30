pub struct Structure {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    min_z: i32,
    max_z: i32,
    data: Vec<bool>,
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

    fn has_voxel(&self, x: i32, y: i32, z: i32) -> bool {
        assert!(x >= self.min_x && x <= self.max_x);
        assert!(y >= self.min_y && y <= self.max_y);
        assert!(z >= self.min_z && z <= self.max_z);
        let extent_x = self.max_x - self.min_x + 1;
        let extent_y = self.max_y - self.min_y + 1;
        let z_in_data = z - self.min_z;
        let y_in_data = y - self.min_y;
        let x_in_data = x - self.min_x;
        let index: usize = (z_in_data * (extent_x * extent_y) + y_in_data * extent_x + x_in_data).try_into().unwrap();
        self.data[index]
    }
}
