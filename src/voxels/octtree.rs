use crate::maths::boxes::Box3i;
use crate::maths::vector::Vect3i;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Empty,
    Full,
    Partitioned,
}

#[derive(Clone)]
struct Octcell {
    oct: Box3i,
    state: CellState,
    children: Option<Box<[Octcell; 8]>>
}

impl Octcell {
    pub fn new(oct: Box3i, state: CellState) -> Self {
        Self {
            oct: oct,
            state: state,
            children: None,
        }
    }

    fn coord_index(&self, coord: Vect3i) -> usize {
        let center = self.oct.center();
        let mut index = 0;
        if coord[0] > center[0] {
            index += 1;
        }
        if coord[1] > center[1] {
            index += 2;
        }
        if coord[2] > center[2] {
            index += 4;
        }
        index
    }

    fn size(&self) -> i32 {
        self.oct.max()[0] - self.oct.min()[0]
    }

    fn get_child(&self, coord: Vect3i) -> &Octcell {
        assert!(self.oct.extent()[0] > 1);
        assert!(self.state == CellState::Partitioned);
        let index = self.coord_index(coord);
        let children = self.children.as_ref().unwrap();
        &children[index]
    }

    fn get_child_mut(&mut self, coord: Vect3i) -> &mut Octcell {
        assert!(self.size() > 1);
        assert!(self.state == CellState::Partitioned);
        let index = self.coord_index(coord);
        let children = self.children.as_mut().unwrap();
        &mut children[index]
    }

    pub fn has_voxel(&self, coord: Vect3i) -> bool {
        match self.state {
            CellState::Empty => false,
            CellState::Full => true,
            CellState::Partitioned => {
                self.get_child(coord).has_voxel(coord)
            }
        }
    }

    fn partition(&mut self) {
        assert!(self.size() / 2 * 2 == self.size());
        let subdivisions = self.oct.subdivide();
        let state = self.state;
        self.children = Some(Box::new(std::array::from_fn(|i| Octcell::new(subdivisions[i].clone(), state))));
        self.state = CellState::Partitioned;
    }

    fn check_for_unpartition(&mut self, state: CellState) {
        assert!(self.state == CellState::Partitioned);
        let children = self.children.as_ref().unwrap();
        for child in &**children {
            if child.state != state {
                return;
            }
        }
        self.children = None;
        self.state = state;
    }

    pub fn add_voxel(&mut self, coord: Vect3i) {
        if self.size() == 1 {
            self.state = CellState::Full;
        } else {
            match self.state {
                CellState::Empty => {
                    self.partition();
                    self.get_child_mut(coord).add_voxel(coord);
                },
                CellState::Full => (),
                CellState::Partitioned => {
                    self.get_child_mut(coord).add_voxel(coord);
                    self.check_for_unpartition(CellState::Full);
                },
            }
        }
    }

    pub fn remove_voxel(&mut self, coord: Vect3i) {
        if self.size() == 1 {
            self.state = CellState::Empty;
        } else {
            match self.state {
                CellState::Empty => (),
                CellState::Full => {
                    self.partition();
                    self.get_child_mut(coord).remove_voxel(coord);
                },
                CellState::Partitioned => {
                    self.get_child_mut(coord).remove_voxel(coord);
                    self.check_for_unpartition(CellState::Empty);
                },
            }
        }
    }

    // For debug purpose only !
    pub fn walk<F: FnMut(Box3i, CellState)>(&self, f: &mut F) {
        f(self.oct.clone(), self.state);
        if self.state == CellState::Partitioned {
            let children = self.children.as_ref().unwrap();
            for child in &**children {
                child.walk(f);
            }
        }
    }
}

pub struct Octtree {
    root: Octcell
}

#[allow(dead_code)]
impl Octtree {
    pub fn new(size: i32) -> Self {
        Self {
            root: Octcell::new(Box3i::from_min_max(Vect3i::new([-size, -size, -size]), Vect3i::new([size, size, size])), CellState::Empty)
        }
    }

    pub fn has_voxel(&self, coord: Vect3i) -> bool {
        self.root.has_voxel(coord)
    }

    // For debug purpose only !
    pub fn walk<F: FnMut(Box3i, CellState)>(&self, f: &mut F) {
        self.root.walk(f)
    }

    pub fn add_voxel(&mut self, coord: Vect3i) {
        self.root.add_voxel(coord);
    }

    pub fn remove_voxel(&mut self, coord: Vect3i) {
        self.root.remove_voxel(coord);
    }
}
