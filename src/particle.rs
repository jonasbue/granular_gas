#[allow(dead_code)]
use crate::parameters;

#[derive(PartialEq)]
pub struct Point
{
    pub x: f64,
    pub y: f64,
}

impl Eq for Point {}

#[derive(PartialEq)]
pub struct Particle
{
    pub pos: Point, 
    pub vel: Point,
    pub r: f64,
    pub m: f64,
    pub collision_count: u8, // Number of collisions the particle has been in
}

impl Eq for Particle {}

impl Particle
{
    // Returns time until the a particle 
    // next will collide with a wall
    fn time_until_collision(&self) -> f64
    {
        let t_vertical_wall = wall_collition_time(
            self.vel.x, self.pos.x, self.r);
        let t_horizontal_wall = wall_collition_time(
            self.vel.y, self.pos.y, self.r);
        return t_vertical_wall.min(t_horizontal_wall); 
    }

    // Sets the velocity of a particle
    pub fn set_velocity(&mut self, v_x: f64, v_y: f64)
    {
        // Infinity can't be good. Let's avoid that.
        // If this by any means should panic, it probably 
        // means that some division statement is wrong.
        assert_ne!(v_x, f64::INFINITY);
        assert_ne!(v_y, f64::INFINITY);

        self.vel.x = v_x;
        self.vel.y = v_y;
    }

    pub fn get_wall(direction: str) -> Particle
    {
        let mut pos = if direction == "horizontal"
            {
                Point {-2, 0}
            }
            else if str == "vertical"
            {
                Point { 0, -2 }
            }

        }
        return Particle
        {
            pos: pos,
            vel: Point{ 0, 0 },
            r: 0.,
            m: 0.,
            collision_count: 0,
        }
    }
}

// All particles should be stored in an ndarray.
// This ndarray could be a field in a struct,
// so that various statistics could be calculated
// in implementations on that struct.

// An alternative approach to this,
// is to store the data of all particles in a set of arrays:
// One for position, one for velocity, etc.
// This allows for more concurrent updates of each particle's
// state, which should be more effective.
struct ParticleVec 
{
    all_particles: Vec<Particle>,
}

impl ParticleVec 
{
    /*
    fn initialize_system()
    {
        self.all_particles = Vec::new()
        generate_particles

    }
    */

    fn generate_particles(&mut self)
    {
        let r = 0.1;
        let m = 1.;
        for _i in 0..parameters::N
        {
            let x: f64 = 0.5;
            let y: f64 = 0.5;
            let pos = Point { x, y };

            let v_x: f64 = 1.;
            let v_y: f64 = 1.;
            let vel = Point { x: v_x, y: v_y };
            
            let particle_i = Particle { pos, vel, r, m, collision_count: 0 };

            self.all_particles.push(particle_i);
        }
    }
}


fn wall_collition_time(pos: f64, v: f64, radius: f64) -> f64
{
    /* Returns time until particle collides with a wall */
    let mut delta_t = 0.;

    if v > 0. { delta_t = (1. - radius - pos) / v; }
    else if v < 0. { delta_t = (radius - pos) / v; }
    else if v == 0. { delta_t = f64::INFINITY; } // This case is redundant, right?
    return delta_t;
}


fn particle_collision_time() -> f64
{
    return f64::INFINITY;
    // I mean, atoms are - like - REALLY small.
}
