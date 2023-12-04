use crate::maths::segment::Segm3f;
use crate::maths::vector::Vect3i;
use crate::graphic::camera::Camera;
use crate::graphic::windowing::event_handler::EventHandler;
use crate::graphic::windowing::event_handler::Key;
use crate::graphic::windowing::event_handler::MouseButton;
use crate::voxels::structure::Structure;
use crate::voxels::catalog::VoxelCatalog;
use crate::voxels::voxel::Voxel;
use crate::voxels::voxel::VoxelID;

pub struct Editor {
    pub structure: Structure,
    pub voxel_position: Option<Vect3i>,
    pub symetry_x: bool,
    pub symetry_y: bool,
    pub symetry_z: bool,

    voxel_catalog: VoxelCatalog,
}

const SAVE_FILENAME: &str = "save.vors";

impl Editor {
    pub fn new() -> Self {
        Self {
            structure: Structure::new(0, 0, 0, 0, 0, 0, Voxel{id: VoxelID::ShipCore, life: 5.0}),
            voxel_position: None,
            symetry_x: false,
            symetry_y: false,
            symetry_z: false,

            voxel_catalog: VoxelCatalog::create(),
        }
    }

    pub fn update_from_events(&mut self, camera: &Camera, event_handler: &EventHandler) {
        if event_handler.is_ctrl_pressed() && event_handler.is_key_just_pressed(Key::X) {
            self.symetry_x = !self.symetry_x;
        }
        if event_handler.is_ctrl_pressed() && event_handler.is_key_just_pressed(Key::Y) {
            self.symetry_y = !self.symetry_y;
        }
        if event_handler.is_ctrl_pressed() && event_handler.is_key_just_pressed(Key::Z) {
            self.symetry_z = !self.symetry_z;
        }

        if event_handler.is_key_just_pressed(Key::F5) {
            let str = self.structure.serialize();
            std::fs::write(SAVE_FILENAME, str).expect(&format!("Unable to save {SAVE_FILENAME}"));
            println!("Saved file '{SAVE_FILENAME}'");
        }
        if event_handler.is_key_just_pressed(Key::F9) {
            let str = std::fs::read_to_string(SAVE_FILENAME).expect(&format!("Unable to read {SAVE_FILENAME}"));
            self.structure = Structure::deserialize(&self.voxel_catalog, &str);
            println!("Loaded file '{SAVE_FILENAME}'");
        }

        if self.voxel_position.is_some() && event_handler.is_mouse_button_just_released(MouseButton::Right) {
            let voxel = self.voxel_catalog.create_voxel(VoxelID::LightHull);
            let position = self.voxel_position.unwrap();
            self.structure.add_voxel(position, voxel);
            if self.symetry_x {
                self.structure.add_voxel(Vect3i::new([-position[0], position[1], position[2]]), voxel);
            }
            if self.symetry_y {
                self.structure.add_voxel(Vect3i::new([position[0], -position[1], position[2]]), voxel);
            }
            if self.symetry_z {
                self.structure.add_voxel(Vect3i::new([position[0], position[1], -position[2]]), voxel);
            }
            if self.symetry_x && self.symetry_y {
                self.structure.add_voxel(Vect3i::new([-position[0], -position[1], position[2]]), voxel);
            }
            if self.symetry_x && self.symetry_z {
                self.structure.add_voxel(Vect3i::new([-position[0], position[1], -position[2]]), voxel);
            }
            if self.symetry_y && self.symetry_z {
                self.structure.add_voxel(Vect3i::new([position[0], -position[1], -position[2]]), voxel);
            }
            if self.symetry_x && self.symetry_y && self.symetry_z {
                self.structure.add_voxel(Vect3i::new([-position[0], -position[1], -position[2]]), voxel);
            }
        }
        if event_handler.is_mouse_button_pressed(MouseButton::Right) {
            let segment = Segm3f::new(camera.position(), camera.position() + camera.forward() * 4.0);
            self.voxel_position = self.structure.outside_voxel_coords(segment);
        } else {
            self.voxel_position = None;
        }
    }
}

