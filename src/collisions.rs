//extern crate ndarray;
//use ndarray::prelude::*;

use crate::particle;
use crate::simulation::status_bar;

use std::collections::BinaryHeap;
use std::cmp::{Ordering, Eq};

#[derive(PartialEq)]
pub struct Collision
{
    time: f64,
    particle_1_index: i32, 
    particle_2_index: i32,   // Negative values imply that particle_2 is really a wall
                            // this is interpreted in transform_velocity(), and
                            // the value is set when the collision is enqueued.

    collision_count_1: u32,  // Collision count at the time the collision
    collision_count_2: u32,  // was detected. When it is resolved, compare
                            // with the particles' actual collision count.
}

impl Collision
{
    pub fn get_time(&self) -> f64
    {
        self.time
    }

    pub fn get_particle_1(&self) -> i32
    {
        self.particle_1_index
    }

    pub fn get_particle_2(&self) -> i32
    {
        self.particle_2_index
    }

    pub fn get_collision_count(&self, i: i32) -> u32
    {
        match i
        {
            1 => self.collision_count_1,
            2 => self.collision_count_2,
            _ => panic!("Only 1 and 2 are valid indices\
of particles in a collision"),
        }
    }
    
    pub fn is_valid(&self, p: &particle::Particles) -> bool
    {
        let p_1 = self.get_particle_1();
        let p_2 = self.get_particle_2();
        let cc_1 = self.get_collision_count(1);
        let cc_2 = self.get_collision_count(2);

        assert!(p_1 >= 0);
        assert!(p_2 >= -2);
        assert!(p_2 != p_1);

        return p.get_collision_count(p_1) == cc_1 
            && p.get_collision_count(p_2) == cc_2;
    }



    // Transform the velocities of the particles involved in the collision.
    pub fn transform_velocity(&self, p: &mut particle::Particles, xi: f64)
    {
        // particle_2 is either a particle or a wall.
        // a positive index means particle, a negative means wall

        // p_2 can be negative, and must be i32
        let p_1 = self.get_particle_1() as usize;
        let p_2 = self.get_particle_2();

        // Collide with horizontal wall
        if p_2 == -1
        {
            //println!("Horizontal wall transform complete");
            p.vel[[0, p_1]] *= xi;
            p.vel[[1, p_1]] *= - xi;
        }
        // Collide with vertical wall
        else if p_2 == -2
        {
            //println!("Vertical wall transform complete");
            p.vel[[0, p_1]] *= - xi;
            p.vel[[1, p_1]] *= xi;
        }
        // Collide with a particle.
        else
        {
            let (r_2, _d, dvdx, _dx_2, _dv_2, dx) 
                = particle::calculate_impact_stats(
                    &p.pos, &p.vel, &p.r, p_1, p_2 as usize);

            let mu_1 = p.m[p_2 as usize] 
                / (p.m[p_1] + p.m[p_2 as usize]);
            let mu_2 = p.m[p_1] 
                / (p.m[p_1] + p.m[p_2 as usize]);

            let c_1 = ((1. + xi) * mu_1 * dvdx) / r_2;
            let c_2 = ((1. + xi) * mu_2 * dvdx) / r_2;

            p.vel[[0, p_1]] += c_1*dx[0];
            p.vel[[1, p_1]] += c_1*dx[1];

            p.vel[[0, p_2 as usize]] -= c_2*dx[0];
            p.vel[[1, p_2 as usize]] -= c_2*dx[1];

            // p_2 must be positive for this code to execute
            // casting to usize is therefore safe.
            p.increment_collision_count(p_2 as usize);
        }
        p.increment_collision_count(p_1);
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

    /*
    pub fn get_len(&self) -> usize
    {
        self.heap.len()
    }
    */

    /*
    pub fn get_next(&self) -> &Collision
    {
        let c = self.heap.peek();
        assert!(c.is_some());
        return c.unwrap();
    }
    */

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

    // Returns an unmutable reference to the unsorted heap queue
    pub fn get_heap(&self) -> &BinaryHeap<Collision>
    {
        &self.heap
    }

    // Iterates through all existing particles, and
    // adds all expected collisions to CollisionQueue.
    // This will create double entries.
    pub fn fill_collision_queue(&mut self, particles: &particle::Particles, t_0: f64, x_max: f64, y_max: f64)
    {
        for i in 0..particles.get_len()
        {
            status_bar(i, particles.get_len());
            self.add_new_collisions(particles, i, t_0, x_max, y_max);
        }
    }

    // This will not create double entries, because
    // particle 1 and 2 cannot crash twice in a row.
    pub fn resolve_next_collision(
        &mut self, 
        c: &Collision, 
        mut particles: &mut particle::Particles, 
        t: f64, 
        xi: f64, 
        x_max: f64, 
        y_max: f64)
    {
        //let c = self.pop_next();
        c.transform_velocity(&mut particles, xi);
        let p_1 = c.particle_1_index;
        let p_2 = c.particle_2_index;

        assert!(p_1 >= 0);
        self.add_new_collisions(particles, p_1 as usize, t, x_max, y_max);
        
        if p_2 >= 0 && p_2 != p_1
        {
            self.add_new_collisions(particles, p_2 as usize, t, x_max, y_max);
        }
    }


    pub fn add_new_collisions(
        &mut self, particles: &particle::Particles, i: usize, t: f64, x_max: f64, y_max: f64)
    {
        for j in 0..particles.get_len() + 2
        {
            let c = find_new_collision(particles, i, j as i32 - 2, t, x_max, y_max);
            if c.get_time().is_finite()
            {
                self.push_collision(c);
            }
        }
    }
}


pub fn find_new_collision(
    particles: &particle::Particles, i: usize, j: i32, t: f64, x_max: f64, y_max: f64) 
    -> Collision
{
    let cc_1 = particles.get_collision_count(i as i32);
    let cc_2 = particles.get_collision_count(j);

    let (dt, n) = particles.time_until_next_collisions(i, j, x_max, y_max);
    return make_collision(t + dt, i as usize, n, cc_1, cc_2);
}


pub fn make_collision(t: f64, p_1: usize, p_2: i32, cc_1: u32, cc_2: u32) 
    -> Collision
{
    assert!(p_2 >= -2);
    Collision 
    { 
        time: t, 
        particle_1_index: p_1 as i32,
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
        // other.time.cmp(&self.time).unwrap(); // Rust docs solution
        return self.partial_cmp(other).unwrap(); //Stackoverflow solution
    }
}
