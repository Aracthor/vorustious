use super::maths::segment::Segm3f;
use super::maths::vector::Vect3f;
use super::structure::Structure;

#[test]
fn structure_segment_intersection() {
    let erase_voxels = |voxel: &mut bool| {
        *voxel = false;
    };

    {
        let segment_start = Vect3f::new([-10.0, 0.0, 0.0]);
        let segment_end = Vect3f::new([10.0, 0.0, 0.0]);
        let segment = Segm3f::new(segment_start, segment_end);
        let mut structure = Structure::new(-2, 4, -1, 1, -1, 0);
        structure.for_voxels_in_segment(segment, erase_voxels);

        let expected_structure = { let mut structure = Structure::new(-2, 4, -1, 1, -1, 0);
            structure.set_voxel(-2, 0, 0, false);
            structure.set_voxel(-1, 0, 0, false);
            structure.set_voxel(0, 0, 0, false);
            structure.set_voxel(1, 0, 0, false);
            structure.set_voxel(2, 0, 0, false);
            structure.set_voxel(3, 0, 0, false);
            structure.set_voxel(4, 0, 0, false);
            structure
        };
        assert!(structure == expected_structure);
    }

    {
        let segment_1_start = Vect3f::new([-10.0, 0.6, 0.0]);
        let segment_1_end = Vect3f::new([10.0, 0.6, 0.0]);
        let segment_1 = Segm3f::new(segment_1_start, segment_1_end);
        let segment_2_start = Vect3f::new([-10.0, 1.4, 0.0]);
        let segment_2_end = Vect3f::new([10.0, 1.4, 0.0]);
        let segment_2 = Segm3f::new(segment_2_start, segment_2_end);
        let mut structure_1 = Structure::new(-2, 4, -1, 1, -1, 0);
        structure_1.for_voxels_in_segment(segment_1, erase_voxels);
        let mut structure_2 = Structure::new(-2, 4, -1, 1, -1, 0);
        structure_2.for_voxels_in_segment(segment_2, erase_voxels);

        let expected_structure = { let mut structure = Structure::new(-2, 4, -1, 1, -1, 0);
            structure.set_voxel(-2, 1, 0, false);
            structure.set_voxel(-1, 1, 0, false);
            structure.set_voxel(0, 1, 0, false);
            structure.set_voxel(1, 1, 0, false);
            structure.set_voxel(2, 1, 0, false);
            structure.set_voxel(3, 1, 0, false);
            structure.set_voxel(4, 1, 0, false);
            structure
        };
        assert!(structure_1 == expected_structure);
        assert!(structure_2 == expected_structure);
    }

    {
        let segment_1_start = Vect3f::new([-10.0, -0.6, 0.0]);
        let segment_1_end = Vect3f::new([10.0, -0.6, 0.0]);
        let segment_1 = Segm3f::new(segment_1_start, segment_1_end);
        let segment_2_start = Vect3f::new([-10.0, -1.4, 0.0]);
        let segment_2_end = Vect3f::new([10.0, -1.4, 0.0]);
        let segment_2 = Segm3f::new(segment_2_start, segment_2_end);
        let mut structure_1 = Structure::new(-2, 4, -1, 1, -1, 0);
        structure_1.for_voxels_in_segment(segment_1, erase_voxels);
        let mut structure_2 = Structure::new(-2, 4, -1, 1, -1, 0);
        structure_2.for_voxels_in_segment(segment_2, erase_voxels);

        let expected_structure = { let mut structure = Structure::new(-2, 4, -1, 1, -1, 0);
            structure.set_voxel(-2, -1, 0, false);
            structure.set_voxel(-1, -1, 0, false);
            structure.set_voxel(0, -1, 0, false);
            structure.set_voxel(1, -1, 0, false);
            structure.set_voxel(2, -1, 0, false);
            structure.set_voxel(3, -1, 0, false);
            structure.set_voxel(4, -1, 0, false);
            structure
        };
        assert!(structure_1 == expected_structure);
        assert!(structure_2 == expected_structure);
    }
}

#[test]
fn structure_segment_first_intersection() {
    let erase_voxels = |voxel: &mut bool| {
        *voxel = false;
    };

    {
        let segment_start = Vect3f::new([-10.0, 0.0, 0.0]);
        let segment_end = Vect3f::new([10.0, 0.0, 0.0]);
        let segment = Segm3f::new(segment_start, segment_end);
        let mut structure = Structure::new(-2, 4, -1, 1, -1, 0);
        structure.for_first_voxel_in_segment(segment, erase_voxels);

        let expected_structure = { let mut structure = Structure::new(-2, 4, -1, 1, -1, 0);
            structure.set_voxel(-2, 0, 0, false);
            structure
        };
        assert!(structure == expected_structure);
    }
}

#[test]
fn structure_segment_intersection_end() {
    let erase_voxels = |voxel: &mut bool| {
        *voxel = false;
    };

    {
        let segment_start = Vect3f::new([-1.0, 0.0, 0.0]);
        let segment_end = Vect3f::new([1.0, -1.0, 0.0]);
        let segment = Segm3f::new(segment_start, segment_end);
        let mut structure = Structure::new(-2, 4, -1, 1, -1, 0);
        structure.for_voxels_in_segment(segment, erase_voxels);

        let expected_structure = { let mut structure = Structure::new(-2, 4, -1, 1, -1, 0);
            structure.set_voxel(-1, 0, 0, false);
            structure.set_voxel(0, 0, 0, false);
            structure.set_voxel(0, -1, 0, false);
            structure.set_voxel(1, -1, 0, false);
            structure
        };
        assert!(structure == expected_structure);
    }
}
