#[allow(dead_code)]
use crate::particle;
use crate::parameters;
use std::collections::BinaryHeap;
use std::cmp::{Ordering, Eq};

#[derive(PartialEq)]
pub struct Collision
{
    time: f64,
    particle_1: particle::Particle,
    particle_2: particle::Particle, // Negative values imply that particle_2 is really a wall
                                    // this is interpreted in resolve_collision(), and
                                    // the value is set when the collision is enqueued.

    collision_count_1: u8,  // Collision count at the time the collision
    collision_count_2: u8,  // was detected. When it is resolved, compare
                            // with the particles' actual collision count.
}

impl Eq for Collision {}

struct CollisionQueue
{
    heap: BinaryHeap<Collision>,
}

impl CollisionQueue
{
    // Iterates through all existing particles, and
    // adds all expected collisions to CollisionQueue.
    fn search_for_collisions(&self, particles: Vec<particle::Particle>)
    {
        let horizontal_wall: Particle = Particle{ pos: Point{-2., 0.}, 0};
        let vertical_wall: Particle = Particle{0., -2., 0};
        for p in particles.iter()
        {
            let t = p.wall_collition_time(p.pos.x, p.vel.x, p.r);
            let collision = Collision 
            { 
                time: t, 
                particle_1: p, 
                particle_2: wall, 
                p.collision_count 
            };
            self.push(collision);
        }
    }
    // Adjust the velocities of the particles involved in the collision.
    fn resolve_collision(collision: &mut Collision)
    {
        // particle_2 is either a particle or a wall, depending on wether
        // the value of its position is valid (positive) or not (negative).
        // TODO: Consider changing this method. 
        // Making Particle a struct of arrays would probably allow using 
        // None or some other type for the walls in particle_2[]. 

        if collision.particle_2.pos.x < -1.
        {
            // Collide with horizontal wall
            collision.particle_1.vel.x *= parameters::XI;
            collision.particle_1.vel.y *= - parameters::XI;
        }
        if collision.particle_2.pos.y < -1.
        {
            // Collide with vertical wall
            collision.particle_1.vel.x *= - parameters::XI;
            collision.particle_1.vel.y *= parameters::XI;
        }
        else
        {
            // Make sure particles are within the valid area.
            assert!(collision.particle_1.pos.x > parameters::X_MIN && 
                    collision.particle_1.pos.x < parameters::X_MAX);
            assert!(collision.particle_1.pos.y > parameters::Y_MIN && 
                    collision.particle_1.pos.y < parameters::Y_MAX);
            assert!(collision.particle_2.pos.x > parameters::X_MIN && 
                    collision.particle_2.pos.x < parameters::X_MAX);
            assert!(collision.particle_2.pos.y > parameters::Y_MIN && 
                    collision.particle_1.pos.y < parameters::Y_MAX);

            // Collide two particles.
            println!("This has not been implemented yet.");
        }
    }
}


// PartialOrd defines how collisions are sorted
// in the heap queue: By time, lowest first.
impl PartialOrd for Collision
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> 
    {
        //Some(self.cmp(other)); // From rust documentation
        return other.time.partial_cmp(&self.time); // From stackoverflow
    }
}

// Ord sorts collisions in the heap queue.
impl Ord for Collision
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        // Only the time parameter is a reasonable sorting criterium.
        //other.time.cmp(&self.time).unwrap(); // Rust docs solution
        return self.partial_cmp(other).unwrap(); //Stackoverflow solution
    }
}
