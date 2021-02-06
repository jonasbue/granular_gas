#[allow(dead_code)]
extern crate ndarray;

use crate::particle;
use crate::parameters;

use std::collections::BinaryHeap;
use std::cmp::{Ordering, Eq};

#[derive(PartialEq)]
pub struct Collision
{
    time: f64,
    particle_1_index: i8, 
    particle_2_index: i8, // Negative values imply that particle_2 is really a wall
                    // this is interpreted in transform_velocity(), and
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

    pub fn get_collision_count(&self, i: i8) -> u8
    {
        match i
        {
            1 => self.collision_count_1,
            2 => self.collision_count_2,
            _ => panic!("Only 1 and 2 are valid indices \
                            of particles in a collision"),
        }
    }

    // Transform the velocities of the particles involved in the collision.
    pub fn transform_velocity(&self, particles: &mut particle::Particles)
    {
        // particle_2 is either a particle or a wall.
        // a positive index means particle, a negative means wall

        // Particle_1 must have a nonnegative index.
        assert!(self.get_particle_1() >= 0);
        assert!(self.get_particle_2() >= -2);

        let p_1 = self.get_particle_1() as usize;
        let p_2 = self.get_particle_2() as i8;
        println!("P_1 = {}, P_2 = {}", p_1, p_2);

        if self.collision_count_1 
            == particles.get_collision_count(p_1 as i8)
        {
            // Collide with horizontal wall
            if p_2 == -1
            {
                println!("Horizontal wall transform complete");
                particles.vel[[0, p_1]] *= parameters::XI;
                particles.vel[[1, p_1]] *= - parameters::XI;
            }
            // Collide with vertical wall
            else if p_2 == -2
            {
                println!("Vertical wall transform complete");
                particles.vel[[0, p_1]] *= - parameters::XI;
                particles.vel[[1, p_1]] *= parameters::XI;
            }
            // Collide with a particle.
            else
            {
                assert!(p_2 != p_1 as i8);
                if particles.get_collision_count(p_1 as i8) 
                    == particles.get_collision_count(p_2)
                {
                    particles.increment_collision_count(p_2 as usize);
                    unimplemented!();
                }
            }
            particles.increment_collision_count(p_1);
        }
        else
        {
            println!("A collision was discarded because \
                particle 11 had index {} where {} was expected", 
                p_1, self.collision_count_1);
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

    /*
    pub fn get_len(&self) -> usize
    {
        self.heap.len()
    }
    */

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

    // Returns an unmutable reference to the unsorted heap queue
    pub fn get_heap(&self) -> &BinaryHeap<Collision>
    {
        &self.heap
    }

    // Iterates through all existing particles, and
    // adds all expected collisions to CollisionQueue.
    pub fn fill_collision_queue(&mut self, particles: &particle::Particles)
    {
        for i in 0..particles.get_len()
        {
            self.add_new_collisions(particles, i);
        }
    }

    pub fn resolve_next_collision(
        &mut self, c: &Collision, mut particles: &mut particle::Particles)
    {
        //let c = self.pop_next();
        c.transform_velocity(&mut particles);
        let p_1 = c.particle_1_index;

        assert!(p_1 >= 0);
        self.add_new_collisions(particles, p_1 as usize);
    }


    pub fn add_new_collisions(
        &mut self, particles: &particle::Particles, i: usize)
    {
        for obj in ["vertical", "horizontal", "particle"].iter()
        {
            let c = find_new_collision(particles, i, obj);
            self.push_collision(c);
        }
    }
}


pub fn find_new_collision(particles: &particle::Particles, i: usize, other: &str) 
    -> Collision
{
    let collision_count_1 = particles.get_collision_count(i as i8);

    let (t, n) = particles.time_until_next_collisions(i, other);
    return make_collision(t, i as usize, n, collision_count_1, 0);
}


pub fn make_collision(t: f64, p_1: usize, p_2: i8, cc_1: u8, cc_2: u8) 
    -> Collision
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
        // other.time.cmp(&self.time).unwrap(); // Rust docs solution
        return self.partial_cmp(other).unwrap(); //Stackoverflow solution
    }
}
