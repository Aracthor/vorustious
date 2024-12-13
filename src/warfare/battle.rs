use super::projectile::Projectile;
use super::ship::Ship;
use crate::physics::body::Body;
use crate::maths::segment::Segm3f;
use crate::voxels::voxel::Voxel;

struct BodyList {
    pub inert_bodies: Vec<Body>,
    pub player_ship: Option<Ship>,
}

impl BodyList {
    pub fn new() -> Self {
        Self {
            inert_bodies: vec![],
            player_ship: None,
        }
    }

    pub fn bodies(&self) -> Vec<&Body> {
        let mut bodies: Vec<&Body> = self.inert_bodies.iter().collect();
        if self.player_ship.is_some() {
            bodies.push(self.player_ship.as_ref().unwrap().body());
        }
        bodies
    }

    pub fn bodies_mut(&mut self) -> Vec<&mut Body> {
        let mut bodies: Vec<&mut Body> = self.inert_bodies.iter_mut().collect();
        if self.player_ship.is_some() {
            bodies.push(self.player_ship.as_mut().unwrap().body_mut());
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

    pub fn set_player_ship(&mut self, ship: Ship) {
        assert!(self.body_list.player_ship.is_none());
        self.body_list.player_ship = Some(ship);
    }

    #[cfg(test)]
    pub fn add_projectile(&mut self, projectile: Projectile) {
        self.projectiles.push(projectile);
    }

    pub fn add_projectiles(&mut self, projectiles: Vec<Projectile>) {
        self.projectiles.extend(projectiles);
    }

    pub fn player_ship(&self) -> Option<&Ship> {
        self.body_list.player_ship.as_ref()
    }

    pub fn player_ship_mut(&mut self) -> Option<&mut Ship> {
        self.body_list.player_ship.as_mut()
    }

    pub fn bodies(&self) -> Vec<&Body> {
        self.body_list.bodies()
    }

    pub fn projectiles(&self) -> &Vec<Projectile> {
        &self.projectiles
    }

    pub fn update(&mut self, elapsed_time: f32) {
        for body in self.body_list.bodies_mut() {
            body.apply_velocity_and_rotation(elapsed_time);
        }

        self.projectiles.retain_mut(|projectile| {
            let segment_start = projectile.position();
            projectile.moove(elapsed_time);
            let segment_end = projectile.position();
            let mut hit = false;
            if !projectile.is_out_of_max_range() {
                let segment = Segm3f::new(segment_start, segment_end);
                for body in self.body_list.bodies_mut() {
                    hit |= body.for_first_voxel_in_segment(segment, |voxel: &mut Voxel, _coords| {
                        voxel.life -= projectile.damage();
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
