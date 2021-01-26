#[allow(dead_code)]
extern crate ndarray;
//use ndarray::prelude::*;

use crate::particle;
use crate::parameters;
use std::collections::BinaryHeap;
use std::cmp::{Ordering, Eq};

#[derive(PartialEq)]
pub struct Collision
{
    time: f64,
    particle_1: i8, 
    particle_2: i8, // Negative values imply that particle_2 is really a wall
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
    fn search_for_collisions(&self, particles: particle::Particles)
    {
        /*
        for p in 0..parameters::N
        {
            let collision = Collision 
            { 
                time: t, 
                particle_1: 2,
                particle_2: wall, 
                p.collision_count 
            };
            self.push(collision);
        }
        */
    }

    // Transform the velocities of the particles involved in the collision.
    fn resolve_collision(collision: &mut Collision)
    {
        // particle_2 is either a particle or a wall.
        // a positive index means particle, a negative means wall

        if collision.particle_2 == -1
        {
            // Collide with horizontal wall
            /*
            collision.particle_1.vel.x *= parameters::XI;
            collision.particle_1.vel.y *= - parameters::XI;
            */
        }
        if collision.particle_2 == -2
        {
            // Collide with vertical wall
            /*
            collision.particle_1.vel.x *= - parameters::XI;
            collision.particle_1.vel.y *= parameters::XI;
            */
        }
        else
        {
            // Make sure particles are within the valid area.
            /*
            assert!(collision.particle_1.pos.x > parameters::X_MIN && 
                    collision.particle_1.pos.x < parameters::X_MAX);
            assert!(collision.particle_1.pos.y > parameters::Y_MIN && 
                    collision.particle_1.pos.y < parameters::Y_MAX);
            assert!(collision.particle_2.pos.x > parameters::X_MIN && 
                    collision.particle_2.pos.x < parameters::X_MAX);
            assert!(collision.particle_2.pos.y > parameters::Y_MIN && 
                    collision.particle_1.pos.y < parameters::Y_MAX);
            */

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
