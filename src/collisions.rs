#[allow(dead_code)]
extern crate ndarray;
//use ndarray::prelude::*;

use crate::particle;
use crate::parameters;
use crate::tests;
use std::collections::BinaryHeap;
use std::cmp::{Ordering, Eq};

#[derive(PartialEq)]
pub struct Collision
{
    time: f64,
    particle_1_index: i8, 
    particle_2_index: i8, // Negative values imply that particle_2 is really a wall
                    // this is interpreted in resolve_collision(), and
                    // the value is set when the collision is enqueued.

    collision_count_1: u8,  // Collision count at the time the collision
    collision_count_2: u8,  // was detected. When it is resolved, compare
                            // with the particles' actual collision count.
}

impl Collision
{
    pub fn get_time(&self) -> f64
    {
        self.time
    }

    pub fn get_particle_1(&self) -> i8
    {
        self.particle_1_index
    }

    pub fn get_particle_2(&self) -> i8
    {
        self.particle_2_index
    }

    // Transform the velocities of the particles involved in the collision.
    pub fn resolve_collision(&self, particles: &mut particle::Particles)
    {
        // particle_2 is either a particle or a wall.
        // a positive index means particle, a negative means wall

        // Particle_1 must have a nonnegative index.
        assert!(self.particle_1_index >= 0);
        let p_1 = self.particle_1_index as usize;

        if self.collision_count_1 != particles.get_collision_count(p_1 as i8)
        {
            // Collide with horizontal wall
            if self.particle_2_index == -1
            {
                particles.vel[[0, p_1]] *= - parameters::XI;
                particles.vel[[1, p_1]] *= parameters::XI;
            }

            // Collide with vertical wall
            else if self.particle_2_index == -2
            {
                particles.vel[[0, p_1]] *= parameters::XI;
                particles.vel[[1, p_1]] *= - parameters::XI;
            }
            else
            {
                // Collide two particles.
                assert!(self.particle_2_index >= 0);
                let p_2 = self.particle_2_index as usize;

                assert!(p_2 != p_1);
                if particles.get_collision_count(p_1 as i8) == particles.get_collision_count(p_2 as i8)
                {
                    particles.increment_collision_count(p_2);
                    unimplemented!();
                }
            }
            particles.increment_collision_count(p_1);
        }
    }
}

impl Eq for Collision {}

pub struct CollisionQueue
{
    heap: BinaryHeap<Collision>,
}

impl CollisionQueue
{
    pub fn new() -> CollisionQueue
    {
        CollisionQueue { heap: BinaryHeap::new() }
    }

    pub fn get_len(&self) -> usize
    {
        self.heap.len()
    }

    pub fn get_next(&self) -> &Collision
    {
        let c = self.heap.peek();
        assert!(c.is_some());
        return c.unwrap();
    }

    pub fn pop_next(&mut self) -> Collision
    {
        let c = self.heap.pop();
        assert!(c.is_some());
        return c.unwrap();
    }

    pub fn push_collision(&mut self, c: Collision)
    {
        self.heap.push(c);
    }


    // Iterates through all existing particles, and
    // adds all expected collisions to CollisionQueue.
    pub fn fill_collision_queue(&mut self, particles: &particle::Particles)
    {
        for i in 0..particles.r.len()
        {
            let (c_horizontal, c_vertical, c_particle) = find_new_collisions(particles, i as i8);

            self.push_collision(c_horizontal);
            self.push_collision(c_vertical);
            //self.push_collision(c_particle);

            // Iterate over all other particles, searching for a collition:
            //for n in i..particles.r.len()
            //let collision_count_2 = particles.get_collision_count(n);
            //let c = make_collision(t, i, n, collision_count_1, collision_count_2);
        }
    }

    pub fn resolve_next_collision(&mut self, particles: &mut particle::Particles)
    {
        let c = self.get_next();
        c.resolve_collision(particles);

        let (c_new_h, c_new_v, c_new_p) = find_new_collisions(particles, c.particle_1_index);
        self.push_collision(c_new_h);
        self.push_collision(c_new_v);
        self.push_collision(c_new_p);
    }
}

pub fn find_new_collisions(particles: &particle::Particles, i: i8) -> (Collision, Collision, Collision)
{
    assert!(i >= 0);
    let collision_count_1 = particles.get_collision_count(i);

    let (t_h, t_v) = particles.time_until_wall_collisions(i as usize);
    let c_horizontal = make_collision(t_h, i as usize, -1, collision_count_1, 0);
    let c_vertical = make_collision(t_v, i as usize, -2, collision_count_1, 0);
    let c_particle = make_collision(t_v+1000., i as usize, -2, collision_count_1, 0);

    // Iterate over all other particles, searching for a collition:
    //for n in i..particles.r.len()
    //let collision_count_2 = particles.get_collision_count(n);
    //let c = make_collision(t, i, n, collision_count_1, collision_count_2);
    //self.push_collision(c);
    return (c_horizontal, c_vertical, c_particle);
}

pub fn make_collision(t: f64, p_1: usize, p_2: i8, cc_1: u8, cc_2: u8) -> Collision
{
    assert!(p_2 >= -2);
    Collision 
    { 
        time: t, 
        particle_1_index: p_1 as i8,
        particle_2_index: p_2, 
        collision_count_1: cc_1,
        collision_count_2: cc_2,
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
