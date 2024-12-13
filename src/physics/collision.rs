use super::body::Body;

pub fn apply_collision_if_any(body_a: &mut Body, body_b: &mut Body, restitution: f32) {
    let intersections = Body::intersection(body_a, body_b);
    if !intersections.is_empty() {
        let momentum_a = body_a.momentum();
        let momentum_b = body_b.momentum();
        let mass_a = body_a.structure().mass();
        let mass_b = body_b.structure().mass();
        let velocity_a = body_a.velocity();
        let velocity_b = body_b.velocity();
        let total_mass = mass_a + mass_b;

        let new_velocity_a = (momentum_a + momentum_b + (velocity_b - velocity_a) * mass_b * restitution) / total_mass;
        let new_velocity_b = (momentum_a + momentum_b + (velocity_a - velocity_b) * mass_a * restitution) / total_mass;

        body_a.set_velocity(new_velocity_a);
        body_b.set_velocity(new_velocity_b);
    }
}