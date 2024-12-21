use crate::maths::matrix::Mat4f;
use crate::maths::vector::Vect2f;
use crate::editor::Editor;
use crate::physics::body::Body;
use crate::warfare::projectile::Projectile;
use super::projectile_renderer::ProjectileRenderer;
use super::body_renderer::BodyRenderer;
use super::editor_renderer::EditorRenderer;
use super::interface_renderer::InterfaceRenderer;
use super::frame_limiter::FrameLimiter;
use super::reticle_renderer::ReticleRenderer;
use super::text_drawer::TextDrawer;

pub struct BattleRenderer {
    resolution: Vect2f,
    projection_matrix: Mat4f,
    ui_projection_matrix: Mat4f,
    frame_limiter: FrameLimiter,
    text_drawer: TextDrawer,
    body_renderer: BodyRenderer,
    projectile_renderer: ProjectileRenderer,
    reticle_renderer: ReticleRenderer,
}

impl BattleRenderer {
    pub fn new(window_width: f32, window_height: f32, fov: f32, z_near: f32, z_far: f32) -> Self {
        let projection_matrix = {
            let aspect = window_width / window_height;
            Mat4f::perspective(fov, aspect, z_near, z_far)
        };
        let ui_projection_matrix = Mat4f::orthographic(0.0, window_width, 0.0, window_height);

        Self {
            resolution: Vect2f::new([window_width, window_height]),
            projection_matrix: projection_matrix,
            ui_projection_matrix: ui_projection_matrix,
            frame_limiter: FrameLimiter::new(60.0),
            text_drawer: TextDrawer::create(),
            body_renderer: BodyRenderer::new(),
            projectile_renderer: ProjectileRenderer::new(),
            reticle_renderer: ReticleRenderer::new(),
        }
    }

    pub fn toggle_debug_bodies(&mut self) {
        self.body_renderer.toggle_debug_bodies();
    }

    pub fn toggle_debug_boxes(&mut self) {
        self.body_renderer.toggle_debug_boxes();
    }

    pub fn toggle_gizmo(&mut self) {
        self.body_renderer.toggle_gizmo();
    }

    pub fn toggle_octtree(&mut self) {
        self.body_renderer.toggle_octtree();
    }

    pub fn render_frame(&mut self, view_matrix: Mat4f, bodies: Vec<&Body>, projectiles: &Vec<Projectile>, player_repere: &Mat4f) {
        let projection_view_matrix = self.projection_matrix.clone() * view_matrix;

        self.body_renderer.render(&projection_view_matrix, bodies);
        self.projectile_renderer.render(&projection_view_matrix, projectiles);
        self.reticle_renderer.render(&projection_view_matrix, player_repere);

        let frame_time_info = self.frame_limiter.frame_time();
        let min_time_ms = frame_time_info.min * 1000.0;
        let max_time_ms = frame_time_info.max * 1000.0;
        let average_time_ms = frame_time_info.average * 1000.0;
        let text = format!("Frame time: {average_time_ms:.2} ms ({min_time_ms:.2}..{max_time_ms:.2})");
        let size = Vect2f::new([12.0, 12.0]);
        let position = Vect2f::new([10.0, self.resolution[1] - size[1] - 10.0]);
        self.text_drawer.add_text_to_draw(text.as_str(), position, size);
        self.text_drawer.draw(&self.ui_projection_matrix);

        self.frame_limiter.limit();
    }
}

pub struct EditorMainRenderer {
    projection_matrix: Mat4f,
    ui_projection_matrix: Mat4f,
    frame_limiter: FrameLimiter,
    body_renderer: BodyRenderer,
    editor_renderer: EditorRenderer,
    interface_renderer: InterfaceRenderer,
}

impl EditorMainRenderer {
    pub fn new(window_width: f32, window_height: f32, fov: f32, z_near: f32, z_far: f32) -> Self {
        let projection_matrix = {
            let aspect = window_width / window_height;
            Mat4f::perspective(fov, aspect, z_near, z_far)
        };
        let ui_projection_matrix = Mat4f::orthographic(0.0, window_width, 0.0, window_height);

        Self {
            projection_matrix: projection_matrix,
            ui_projection_matrix: ui_projection_matrix,
            frame_limiter: FrameLimiter::new(60.0),
            body_renderer: BodyRenderer::new(),
            editor_renderer: EditorRenderer::new(),
            interface_renderer: InterfaceRenderer::new(window_width, window_height),
        }
    }


    pub fn render_frame(&mut self, view_matrix: Mat4f, body: &Body, editor: &Editor) {
        let projection_view_matrix = self.projection_matrix.clone() * view_matrix;

        self.body_renderer.render(&projection_view_matrix, vec![body]);
        self.editor_renderer.render(&projection_view_matrix, editor);

        self.interface_renderer.draw(&self.ui_projection_matrix);

        self.frame_limiter.limit();
    }

}
