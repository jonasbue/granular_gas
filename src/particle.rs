#[allow(dead_code)]
extern crate ndarray;
use ndarray::prelude::*;
use ndarray_rand::rand::{Rng, thread_rng};
use ndarray_rand::{rand_distr::Uniform, RandomExt};

use crate::parameters;

pub struct Particles
{
    pub pos: Array2<f64>,
    pub vel: Array2<f64>,
    pub r: Array1<f64>,
    pub m: Array1<f64>,
    pub collision_count: Array1<u8> // Number of times each particle has collided
}


impl Particles
{
    pub fn get_len(&self) -> usize
    {
        self.r.len()
    }

    pub fn get_collision_count(&self, index: i8) -> u8
    {
        assert!(index >= -2);
        match index
        {
            -1 | -2 => 0,
            _ => self.collision_count[index as usize],
        }
    }

    pub fn time_until_next_collisions(&self, i: usize, wall: &str) -> (f64, i8)
    {
        assert_eq!(self.r[i], parameters::R);
        match wall
        {
            "horizontal" =>
            (wall_collition_time(
                self.pos[[1, i]], self.vel[[1, i]], self.r[i]), -1),
            "vertical" =>
            (wall_collition_time(
                self.pos[[0, i]], self.vel[[0, i]], self.r[i]), -2),
            "particle" =>
            (particle_collision_time(), 100),
            _ => panic!("That object is not defined. Did you type correctly?")
        }
    } 

    // Propagates all particles in list for a time dt
    pub fn propagate(&mut self, dt: f64)
    {
        for i in 0..self.r.len()
        {
            //assert!(self.is_within_box(i));
            self.pos[[0,i]] += self.vel[[0,i]] * dt;
            self.pos[[1,i]] += self.vel[[1,i]] * dt;
        }
    }

    // Returns true if all particles are located within the box
    pub fn is_within_box(&self, i: usize) -> bool
    {
        self.pos[[0, i]] > parameters::X_MIN + parameters::R && 
        self.pos[[1, i]] > parameters::Y_MIN + parameters::R && 
        self.pos[[0, i]] < parameters::X_MAX - parameters::R &&
        self.pos[[1, i]] < parameters::Y_MAX - parameters::R
    }

    // Increments collision count of particle i
    pub fn increment_collision_count(&mut self, i: usize)
    {
        self.collision_count[i] += 1;
    }
}


// Fill a box with borders at x_min and x_max with particles
pub fn generate_particles(n: usize, x_min: f64, x_max: f64, _y_min: f64, _y_max: f64, r: f64, m: f64) -> Particles
{
    // Check that the particles can fit within the box
    // This is a naîve assertion:
    // If this is only barely true, the 
    // initialization will take a very long time.
    assert!((x_max - x_min) * (_y_max - _y_min) 
        > 2.*std::f64::consts::PI*r*n as f64, 
        "{} particles of this size will not fit within the box", n);

    // Y_min and y_max are not currently used, 
    // but could be implemented for a rectangular box.
    if _y_max != x_max
    {
        unimplemented!("Rectangular box size is not implemented.");
    }

    let mut particles = Particles { 
        pos: Array2::random((2, n), Uniform::new(x_min, x_max)),
        vel: Array2::random((2, n), Uniform::new(0., 2.*std::f64::consts::PI)),
        r: Array1::from_elem(n, r),
        m: Array1::from_elem(n, m),
        collision_count: Array1::zeros(n),
    };

    let v_0 = parameters::V_0;
    particles.vel.slice_mut(s![0,..]).mapv_inplace(|a| v_0*a.cos());
    particles.vel.slice_mut(s![1,..]).mapv_inplace(|a| v_0*a.sin());
    // if particles spawn at an unphysical location,
    // negative collision times are generated.
    // TODO: Check if particles overlap with other
    // particles or walls.

    let mut rng = thread_rng();
    for i in 0..n
    {
        while !particles.is_within_box(i)
        {
            particles.pos[[0,i]] = rng.sample(Uniform::new(x_min, x_max));
            particles.pos[[1,i]] = rng.sample(Uniform::new(x_min, x_max));
        }
    }

    return particles;
}


fn wall_collition_time(pos: f64, v: f64, radius: f64) -> f64
{
    /* Returns time until particle collides with a wall */
    let mut delta_t = 0.;

    if v > 0. { delta_t = (1. - radius - pos) / v; }
    else if v < 0. { delta_t = (radius - pos) / v; }
    else if v == 0. { delta_t = f64::INFINITY; } // This case is redundant

    // Invalid positions (outside box or overlap between particles)
    // give negative times. If a particle hits a corner, this might happen.
    assert!(delta_t > -1e-5, "Non-positive time computed: delta_t = {}", delta_t);
    return delta_t;
}


fn particle_collision_time() -> f64
{
    return f64::INFINITY;
    // I mean, atoms are - like - REALLY small.
}

    // Sets the velocity of a particle
    /*
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
    */
