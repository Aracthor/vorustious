use super::body::Body;
use super::projectile::Projectile;
use crate::maths::segment::Segm3f;
use crate::voxels::voxel::Voxel;

struct BodyList {
    pub inert_bodies: Vec<Body>,
    pub player_body: Option<Body>,
}

impl BodyList {
    pub fn new() -> Self {
        Self {
            inert_bodies: vec![],
            player_body: None,
        }
    }

    pub fn bodies(&self) -> Vec<&Body> {
        let mut bodies: Vec<&Body> = self.inert_bodies.iter().collect();
        if self.player_body.is_some() {
            bodies.push(self.player_body.as_ref().unwrap());
        }
        bodies
    }

    pub fn bodies_mut(&mut self) -> Vec<&mut Body> {
        let mut bodies: Vec<&mut Body> = self.inert_bodies.iter_mut().collect();
        if self.player_body.is_some() {
            bodies.push(self.player_body.as_mut().unwrap());
        }
        bodies
    }

}

pub struct Battle {
    body_list: BodyList,
    projectiles: Vec<Projectile>,
}

impl Battle {
    pub fn new() -> Self {
        Self {
            body_list: BodyList::new(),
            projectiles: vec![],
        }
    }

    pub fn add_inert_body(&mut self, body: Body) {
        self.body_list.inert_bodies.push(body);
    }

    pub fn set_player_body(&mut self, body: Body) {
        assert!(self.body_list.player_body.is_none());
        self.body_list.player_body = Some(body);
    }

    #[cfg(test)]
    pub fn add_projectile(&mut self, projectile: Projectile) {
        self.projectiles.push(projectile);
    }

    pub fn add_projectiles(&mut self, projectiles: Vec<Projectile>) {
        self.projectiles.extend(projectiles);
    }

    pub fn player_body(&self) -> Option<&Body> {
        self.body_list.player_body.as_ref()
    }

    pub fn player_body_mut(&mut self) -> Option<&mut Body> {
        self.body_list.player_body.as_mut()
    }

    pub fn bodies(&self) -> Vec<&Body> {
        self.body_list.bodies()
    }

    pub fn projectiles(&self) -> &Vec<Projectile> {
        &self.projectiles
    }

    pub fn update(&mut self, elapsed_time: f32) {
        for body in self.body_list.bodies_mut() {
            body.apply_velocity(elapsed_time);
        }

        self.projectiles.retain_mut(|projectile| {
            let segment_start = projectile.position();
            projectile.moove(elapsed_time);
            let segment_end = projectile.position();
            let mut hit = false;
            if !projectile.is_out_of_max_range() {
                let segment = Segm3f::new(segment_start, segment_end);
                for body in self.body_list.bodies_mut() {
                    hit |= body.for_first_voxel_in_segment(segment, |voxel: &mut Option<Voxel>, _coords| {
                        voxel.as_mut().unwrap().life -= projectile.damage();
                    });
                }
            }
            !hit && !projectile.is_out_of_max_range()
        });

        let mut new_bodies = vec![];
        for body in self.body_list.bodies_mut() {
            new_bodies.extend(body.update_dead_voxels());
        }
        self.body_list.inert_bodies.extend(new_bodies);

        self.body_list.inert_bodies.retain(|body| !body.structure().is_empty());
    }
}
