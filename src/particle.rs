#[allow(dead_code)]
extern crate ndarray;
use ndarray::prelude::*;
use ndarray::{Array, stack};
use ndarray_rand::{rand_distr::Uniform, RandomExt};

use crate::parameters;
use crate::collisions;

/*
#[derive(PartialEq)]
pub struct Point
{
    pub x: f64,
    pub y: f64,
}
*/

//impl Eq for Point {}

//#[derive(PartialEq, Clone)]
pub struct Particles
{
    pub pos: Array2<f64>,
    pub vel: Array2<f64>,
    pub r: Array1<f64>,
    pub m: Array1<f64>,
    pub collision_count: Array1<u8> // Number of times each particle has collided
}

//impl Eq for Particle {}
//impl Clone for Particles {}

impl Particles
{
    pub fn get_collision_count(&self, index: i8) -> u8
    {
        assert!(index >= -2);
        match index
        {
            -1 | -2 => 0,
            _ => self.collision_count[index as usize],
        }
    }

    // Returns time until particle number i collides with
    // a horizontal and a vertical wall, respetively.
    pub fn time_until_wall_collisions(&self, i: usize) -> (f64, f64)
    {
        (wall_collition_time(self.vel[[0, i]], self.pos[[0, i]], self.r[i]),
        wall_collition_time(self.vel[[1, i]], self.pos[[1, i]], self.r[i]))
    } 

    pub fn propagate(&mut self, dt: f64)
    {
        for i in 0..self.r.len()
        {
            //assert!(self.is_within_box(i));
            self.pos[[0,i]] += self.vel[[0,i]] * dt;
            self.pos[[1,i]] += self.vel[[1,i]] * dt;
        }
    }

    pub fn is_within_box(&self, i: usize) -> bool
    {
        self.pos[[0, i]] > parameters::X_MIN && 
        self.pos[[1, i]] > parameters::Y_MIN && 
        self.pos[[0, i]] < parameters::X_MAX &&
        self.pos[[1, i]] < parameters::Y_MAX
    }

    pub fn increment_collision_count(&mut self, i: usize)
    {
        self.collision_count[i] += 1;
    }
}


pub fn generate_particles(n: usize, x_min: f64, x_max: f64, _y_min: f64, _y_max: f64) -> Particles
{
    let mut particles = Particles { 
        pos: Array2::random((2, n), Uniform::new(x_min, x_max)),
        vel: Array2::random((2, n), Uniform::new(0., 2.*std::f64::consts::PI)),
        r: Array1::ones(n),
        m: Array1::ones(n),
        collision_count: Array1::zeros(n),
    };

    let v_0 = parameters::V_0;
    particles.vel.slice_mut(s![0,..]).mapv_inplace(|a| v_0*a.cos());
    particles.vel.slice_mut(s![1,..]).mapv_inplace(|a| v_0*a.sin());

    return particles;
}


fn wall_collition_time(pos: f64, v: f64, radius: f64) -> f64
{
    /* Returns time until particle collides with a wall */
    let mut delta_t = 0.;

    if v > 0. { delta_t = (1. - radius - pos) / v; }
    else if v < 0. { delta_t = (radius - pos) / v; }
    else if v == 0. { delta_t = f64::INFINITY; } // This case is redundant, right?
    return -delta_t;
    // The minus sign is ad hoc
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
