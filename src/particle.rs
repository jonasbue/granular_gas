extern crate ndarray;
use ndarray::prelude::*;

use ndarray_rand::rand::{Rng, thread_rng};
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

use crate::parameters;

pub struct Particles
{
    pub pos: Array2<f64>,
    pub vel: Array2<f64>,
    pub r: Array1<f64>,
    pub m: Array1<f64>,
    pub collision_count: Array1<u8> // Number of times each 
                                    // particle has collided
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

    pub fn time_until_next_collisions(&self, i: usize, j: i8) 
        -> (f64, i8)
    {
        assert_eq!(self.r[i], parameters::R);
        assert!(j >= -2, "Undefined index for particle 2 encountered.");
        match j 
        {
            -1 =>
                (wall_collition_time(
                    self.pos[[1, i]], self.vel[[1, i]], self.r[i]), -1),

            -2 =>
                (wall_collition_time(
                    self.pos[[0, i]], self.vel[[0, i]], self.r[i]), -2),

            _ =>
                (particle_collision_time(
                    &self.pos, &self.vel, &self.r, i, j as usize), j), 
        }
    } 

    // Propagates all particles in list for a time dt
    pub fn propagate(&mut self, dt: f64)
    {
        for i in 0..self.get_len()
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

    // Checks if particle i is overlapping 
    // with any of the other particles
    pub fn is_overlapping(&self, i: usize) -> bool
    {
        for j in 0..self.get_len()
        {
            if j != i
            {
                let dx = self.pos[[0, i]] - self.pos[[0, j]];
                let dy = self.pos[[1, i]] - self.pos[[1, j]];

                if dx.powi(2) + dy.powi(2)
                    < (self.r[[i]] + self.r[[j]]).powi(2)
                {
                    return true;
                }
            }
            else
            {
                println!("It was avoided");
            }
        }
        return false;
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
    // Note: If this is only barely true, the 
    // initialization will take a very long time.
    assert!((x_max - x_min) * (_y_max - _y_min) 
        > 2.*std::f64::consts::PI*r*r*n as f64, 
        "{} particles of radius {} will not fit within the box", n, r);

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
        while !particles.is_within_box(i) || particles.is_overlapping(i)
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

    // Invalid positions (outside box or overlap between particles) can
    // give negative times. If a particle hits a corner, this might happen.
    assert!(delta_t > -1e-5, "Non-positive time computed: delta_t = {}", delta_t);

    return delta_t;
}


// Returns time until particle i will collide with particle j.
fn particle_collision_time(
    pos: &Array2::<f64>, 
    vel: &Array2::<f64>,
    r: &Array1::<f64>,
    i: usize,
    j: usize)
    -> f64
{
    let (_r_2, d, dvdx, _dx_2, dv_2, _dx) 
        = calculate_impact_stats(&pos, &vel, &r, i, j);

    if dvdx < 0. && d > 0.
    {
        -((dvdx) + d.sqrt())/dv_2
    }
    else
    {
        f64::INFINITY
    }
}

pub fn calculate_impact_stats(
    pos: &Array2::<f64>, vel: &Array2::<f64>, 
    r: &Array1::<f64>, i: usize, j: usize) 
    -> (f64, f64, f64, f64, f64, Array1::<f64>)
{
    let xi: f64 = pos[[0, i]];
    let yi: f64 = pos[[1, i]];
    let xj: f64 = pos[[0, j]];
    let yj: f64 = pos[[1, j]];

    let vxi: f64 = vel[[0, i]];
    let vyi: f64 = vel[[1, i]];
    let vxj: f64 = vel[[0, j]];
    let vyj: f64 = vel[[1, j]];

    let r_ij_squared: f64 = (r[j] + r[i]).powi(2);

    let dx: Array1::<f64> = arr1(&[xj - xi, yj - yi]);
    let dv: Array1::<f64> = arr1(&[vxj - vxi, vyj - vyi]);
    let d: f64 = dv.dot(&dx).powi(2) - dv.dot(&dv) * (dx.dot(&dx) - r_ij_squared);

    return (r_ij_squared, d, dv.dot(&dx), dx.dot(&dx), dv.dot(&dv), dx);
}
