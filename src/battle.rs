use super::projectile::Projectile;
use crate::maths::segment::Segm3f;
use crate::voxels::body::Body;
use crate::voxels::voxel::Voxel;

pub struct Battle {
    bodies: Vec<Body>,
    projectiles: Vec<Projectile>,
}

impl Battle {
    pub fn new() -> Self {
        Self {
            bodies: vec![],
            projectiles: vec![],
        }
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }

    pub fn add_projectile(&mut self, projectile: Projectile) {
        self.projectiles.push(projectile);
    }

    pub fn bodies(&self) -> &Vec<Body> {
        &self.bodies
    }

    pub fn projectiles(&self) -> &Vec<Projectile> {
        &self.projectiles
    }

    pub fn update(&mut self) {
        self.projectiles.retain_mut(|projectile| {
            let segment_start = projectile.position();
            projectile.moove(1.0);
            let segment_end = projectile.position();
            let mut hit = false;
            if !projectile.is_out_of_max_range() {
                let segment = Segm3f::new(segment_start, segment_end);
                for body in &mut self.bodies {
                    hit = body.for_first_voxel_in_segment(segment, |voxel: &mut Option<Voxel>| {
                        voxel.as_mut().unwrap().life -= projectile.damage();
                    });
                }
            }
            !hit && !projectile.is_out_of_max_range()
        });

        for body in &mut self.bodies {
            body.structure_mut().for_each_voxel_mut(|_coords, voxel: &mut Option<Voxel>| {
                if voxel.unwrap().life <= 0.0 {
                    *voxel = None;
                }
            });
        }
    }
}
