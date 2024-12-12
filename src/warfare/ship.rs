use super::projectile::Projectile;
use super::weapon::Weapon;
use crate::physics::body::Body;
use crate::maths::matrix::Mat4f;
use crate::maths::vector::Vect3f;

pub struct Ship {
    body: Body,
    weapons: Vec<(Vect3f, Weapon)>,
}

impl Ship {
    pub fn new(body: Body) -> Self {
        Self {
            body: body,
            weapons: vec![],
        }
    }

    pub fn body(&self) -> &Body {
        &self.body
    }

    pub fn body_mut(&mut self) -> &mut Body {
        &mut self.body
    }

    pub fn repere(&self) -> &Mat4f {
        self.body.repere()
    }

    pub fn add_weapon(&mut self, position: Vect3f, weapon: Weapon) {
        self.weapons.push((position, weapon));
    }

    pub fn shoot(&mut self) -> Vec<Projectile> {
        let mut result = vec![];
        let direction = self.body.repere().forward();
        for weapon in &mut self.weapons {
            let position = self.body.repere().clone() * weapon.0;
            let projectile = weapon.1.shoot(position, direction);
            if projectile.is_some() {
                result.push(projectile.unwrap());
            }
        }
        result
    }

}
