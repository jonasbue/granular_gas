extern crate ndarray;
extern crate csv;
extern crate ndarray_csv;

use ndarray::prelude::*;
use ndarray::{stack_new_axis};
use ndarray_rand::rand::{Rng, thread_rng};
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

use crate::parameters;
use crate::simulation;


pub struct Particles
{
    pub pos: Array2<f64>,
    pub vel: Array2<f64>,
    pub r: Array1<f64>,
    pub m: Array1<f64>,
    pub collision_count: Array1<u32> // Number of times each 
                                    // particle has collided
}


impl Particles
{
    pub fn get_len(&self) -> usize
    {
        self.r.len()
    }

    pub fn get_collision_count(&self, index: i32) -> u32
    {
        assert!(index >= -2);
        match index
        {
            -1 | -2 => 0,
            _ => self.collision_count[index as usize],
        }
    }

    pub fn get_avg_collision_count(&self) -> f64
    {
        let mut cc: f64 = 0.;
        for i in 0..self.get_len()
        {
            cc += self.get_collision_count(i as i32) as f64;
        }
        cc
    }

    pub fn get_kinetic_energy(&self, i: usize) -> f64
    {
        0.5 * self.m[i] * self.get_speed(i).powi(2)
    }
        

    pub fn get_tot_kinetic_energy(&self) -> f64
    {
        let mut energy: f64 = 0.;
        for i in 0..self.get_len()
        {
            energy += self.get_kinetic_energy(i);
        }
        energy
    }

    pub fn get_kinetic_energy_for_mass(&self, mass: f64) -> f64
    {
        let mut energy: f64 = 0.;
        for i in 0..self.get_len()
        {
            if (self.get_mass(i) - mass).abs() <= 1e-8
            {
                energy += self.get_kinetic_energy(i);
            }
        }
        energy
    }


    pub fn get_mass(&self, i: usize) -> f64
    {
        self.m[i]
    }


    pub fn get_speed(&self, i: usize) -> f64
    {
        return (self.vel[[0, i]].powi(2) 
            + self.vel[[1, i]].powi(2)).sqrt();
    }


    pub fn stop_all_particles(&mut self)
    {
        self.vel = Array2::zeros((2, self.get_len()))
    }


    pub fn set_particle_state(&mut self, i: usize,
        x: f64, y: f64, vx: f64, vy: f64, r: f64, m: f64)
    {
        println!("WARNING: A particle was edited manually.");
        self.pos[[0, i]] = x;
        self.pos[[1, i]] = y;
        self.vel[[0, i]] = vx;
        self.vel[[1, i]] = vy;
    
        self.r[i] = r;
        self.m[i] = m;
        self.collision_count[i] = 0;
    }


    pub fn time_until_next_collisions(&self, i: usize, j: i32, x_max: f64, y_max: f64) 
        -> (f64, i32)
    {
        assert!(j >= -2, "Undefined index for particle 2 encountered.");
        match j 
        {
            -1 =>
                (wall_collition_time(
                    self.pos[[1, i]], self.vel[[1, i]], self.r[i], y_max), -1),

            -2 =>
                (wall_collition_time(
                    self.pos[[0, i]], self.vel[[0, i]], self.r[i], x_max), -2),

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
    pub fn is_within_box(
        &self, 
        i: usize, 
        x_min: f64, 
        x_max: f64, 
        y_min: f64, 
        y_max: f64) 
    -> bool
    {
        self.pos[[0, i]] > x_min + self.r[i] && 
        self.pos[[1, i]] > y_min + self.r[i] && 
        self.pos[[0, i]] < x_max - self.r[i] &&
        self.pos[[1, i]] < y_max - self.r[i]
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
pub fn generate_particles(
    n_arr: &Array1<usize>, 
    x_min: f64, 
    x_max: f64, 
    y_min: f64, 
    y_max: f64, 
    r_arr: &Array1<f64>, 
    m_arr: &Array1<f64>) 
    -> Particles
{
    // Check that the particles can fit within the box
    // This is a naîve assertion:
    // Note: If this is only barely true, the 
    // initialization will take a very long time.
    
    let pf = get_packing_fraction(n_arr, r_arr, x_min, y_min, x_max, y_max);
    assert!(pf < 1., "The particles have a packing fraction of {}, \
    and they will not fit in the system.", pf);

    let n = n_arr.sum();
    let positions: Array2<f64> = stack_new_axis![Axis(0), 
        Array1::random(n, Uniform::new(x_min, x_max)),
        Array1::random(n, Uniform::new(y_min, y_max))];

    let angles = Array1::random(n, Uniform::new(0., 2.*std::f64::consts::PI));
    let mut velocities = stack_new_axis![Axis(0), angles, angles];

    let v_0 = parameters::V_0;
    velocities.slice_mut(s![0,..]).mapv_inplace(|a| v_0*a.cos());
    velocities.slice_mut(s![1,..]).mapv_inplace(|a| v_0*a.sin());

    let mut radii = Array1::zeros(n);
    let mut masses = Array1::zeros(n);

    // Fill radii and masses with values.
    for i in 0..n_arr.len()
    {
        for j in 0..n_arr[i]
        {
            if i != 0
            {
                radii[n_arr[i-1] + j] = r_arr[i];
                masses[n_arr[i-1] + j] = m_arr[i];
            }
            else
            {
                radii[j] = r_arr[i];
                masses[j] = m_arr[i];
            }
               
        }
    }

    let mut particles = Particles { 
        pos: positions,
        vel: velocities,
        r: radii,
        m: masses,
        collision_count: Array1::zeros(n),
    };

    replace_overlapping_particles(&mut particles, x_min, x_max, y_min, y_max);
    return particles;
}


fn replace_overlapping_particles(particles: &mut Particles, x_min: f64, x_max: f64, y_min: f64, y_max: f64)
{
    let mut rng = thread_rng();
    let mut replaces: i32 = 0;
    println!("Replacing overlapping particles.");
    for i in 0..particles.get_len()
    {
        simulation::status_bar(i, particles.get_len());
        while !particles.is_within_box(i, x_min, x_max, y_min, y_max) || particles.is_overlapping(i)
        {
            particles.pos[[0,i]] = rng.sample(Uniform::new(x_min, x_max));
            particles.pos[[1,i]] = rng.sample(Uniform::new(y_min, y_max));

            replaces += 1;
            /*
            if replaces >= 10 * particles.get_len() as i32
            {
                panic!("Replacing particles took too long.\
                Are you sure there is enough space?");
            }
            */
        }
    }
    print!(" Done.\n");
    println!("Number of times a particle was replaced: {}", replaces);
}


fn wall_collition_time(pos: f64, v: f64, radius: f64, length: f64) -> f64
{
    // Returns time until particle collides with a wall 
    // length is the length of the box, given that
    // two borders are x and y axis, and length > 0.
    // That is, the box is in the first quadrant.
    assert!(length > 0.);

    let mut delta_t = 0.;

    if v > 0. { delta_t = (length - radius - pos) / v; }
    else if v < 0. { delta_t = (radius - pos) / v; }
    else if v == 0. { delta_t = f64::INFINITY; } // This case is redundant

    // Invalid positions (outside box or overlap between particles) can
    // give negative times. If a particle hits a corner, this might happen.
    // However, collisions happening simultaneously can get a time slightly
    // below zero(?), due to numerical inaccuracy.
    assert!(delta_t > -1e6, "Non-positive time computed: delta_t = {}", delta_t);

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


pub fn get_packing_fraction(
    n_arr: &Array1<usize>, r_arr: &Array1<f64>, 
    x_min: f64, 
    x_max: f64, 
    y_min: f64, 
    y_max: f64) -> f64
{
    let mut area: f64 = 0.;
    let box_area: f64 = (x_max - x_min) * (y_max - y_min);
    for i in 0..n_arr.len()
    {
        area += n_arr[i] as f64 * r_arr[i].powi(2);
    }
    area *= 2.*std::f64::consts::PI;
    return area/box_area;
}




