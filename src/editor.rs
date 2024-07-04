use crate::maths::segment::Segm3f;
use crate::maths::vector::Vect3i;
use crate::maths::matrix::Mat4f;
use crate::graphic::camera::Camera;
use crate::graphic::renderer::Renderer;
use crate::graphic::windowing::event_handler::EventHandler;
use crate::graphic::windowing::event_handler::Key;
use crate::graphic::windowing::event_handler::MouseButton;
use crate::graphic::windowing::window::Window;
use crate::voxels::structure::Structure;
use crate::voxels::catalog::VoxelCatalog;
use crate::voxels::voxel::Voxel;
use crate::voxels::voxel::VoxelID;
use crate::warfare::body::Body;

pub struct Editor {
    pub structure: Structure,
    pub voxel_position: Option<Vect3i>,
    pub voxel_id: VoxelID,
    pub symetry_x: bool,
    pub symetry_y: bool,
    pub symetry_z: bool,

    voxel_catalog: VoxelCatalog,
}

const SAVE_FILENAME: &str = "save.vors";

impl Editor {
    fn new() -> Self {
        Self {
            structure: Structure::new(0, 0, 0, 0, 0, 0, Voxel{id: VoxelID::ShipCore, life: 5.0}),
            voxel_position: None,
            voxel_id: VoxelID::LightHull,
            symetry_x: false,
            symetry_y: false,
            symetry_z: false,

            voxel_catalog: VoxelCatalog::create(),
        }
    }

    fn update_from_events(&mut self, camera: &Camera, event_handler: &EventHandler) {
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
            self.structure.recalculate_box();
            let str = self.structure.serialize();
            std::fs::write(SAVE_FILENAME, str).expect(&format!("Unable to save {SAVE_FILENAME}"));
            println!("Saved file '{SAVE_FILENAME}'");
        }
        if event_handler.is_key_just_pressed(Key::F9) {
            let str = std::fs::read_to_string(SAVE_FILENAME).expect(&format!("Unable to read {SAVE_FILENAME}"));
            self.structure = Structure::deserialize(&self.voxel_catalog, &str);
            println!("Loaded file '{SAVE_FILENAME}'");
        }
        if event_handler.scroll_status() < 0.0 {
            self.voxel_id = (if self.voxel_id as i32 == 0 { VoxelID::COUNT as i32 - 1 } else { self.voxel_id as i32 - 1}).into();
        }
        else if event_handler.scroll_status() > 0.0 {
            self.voxel_id = (if self.voxel_id as i32 == VoxelID::COUNT as i32 - 1 { 0 } else { self.voxel_id as i32 + 1}).into();
        }

        if event_handler.is_mouse_button_just_released(MouseButton::Left) {
            let segment = Segm3f::new(camera.position(), camera.position() + camera.forward() * 4.0);
            let mut aimed_coords = None;
            self.structure.for_first_voxel_in_segment(segment, |_voxel, coords| {
                aimed_coords = Some(*coords);
            });
            if aimed_coords.is_some() {
                let position = aimed_coords.unwrap();
                self.structure.remove_voxel(position);
                if self.symetry_x {
                    self.structure.remove_voxel_ifp(Vect3i::new([-position[0], position[1], position[2]]));
                }
                if self.symetry_y {
                    self.structure.remove_voxel_ifp(Vect3i::new([position[0], -position[1], position[2]]));
                }
                if self.symetry_z {
                    self.structure.remove_voxel_ifp(Vect3i::new([position[0], position[1], -position[2]]));
                }
                if self.symetry_x && self.symetry_y {
                    self.structure.remove_voxel_ifp(Vect3i::new([-position[0], -position[1], position[2]]));
                }
                if self.symetry_x && self.symetry_z {
                    self.structure.remove_voxel_ifp(Vect3i::new([-position[0], position[1], -position[2]]));
                }
                if self.symetry_y && self.symetry_z {
                    self.structure.remove_voxel_ifp(Vect3i::new([position[0], -position[1], -position[2]]));
                }
                if self.symetry_x && self.symetry_y && self.symetry_z {
                    self.structure.remove_voxel_ifp(Vect3i::new([-position[0], -position[1], -position[2]]));
                }
            }
        }

        if self.voxel_position.is_some() && event_handler.is_mouse_button_just_released(MouseButton::Right) {
            let voxel = self.voxel_catalog.create_voxel(self.voxel_id);
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

pub fn run_editor() {
    const WINDOW_WIDTH:u32 = 800;
    const WINDOW_HEIGHT:u32 = 600;

    let mut window = Window::create_window(WINDOW_WIDTH, WINDOW_HEIGHT, "Vorustious - Editor");
    let mut renderer = Renderer::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32, 80.0_f32.to_radians(), 0.1, 1000.0);

    let mut camera = Camera::new();
    let mut editor = Editor::new();

    while !window.should_close() {
        camera.update_from_events(&window.event_handler());
        editor.update_from_events(&camera, &window.event_handler());

        window.clear();

        let bodies = vec![Body::new(editor.structure.clone(), Mat4f::identity())];
        renderer.render_frame(camera.view_matrix(), bodies.iter().collect(), &vec![], Some(&editor));

        window.update();
    }
}