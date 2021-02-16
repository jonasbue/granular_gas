extern crate ndarray;
use ndarray::prelude::*;

use crate::particle;
use crate::parameters;
use crate::plotting;
use crate::collisions;


pub fn task_1(
    mut p: &mut particle::Particles, 
    q: &mut collisions::CollisionQueue, 
    number_of_events: usize,
    t_0: f64,
    test: bool)
    -> Array2<f64>
{
    let mut t = t_0;
    let mut i: usize = 0;

    // system_data contains some data about the system.
    // index 0: time of collisions
    // index 1: kinetic energy at these times
    let mut system_data: Array2<f64> = Array2::zeros((2, number_of_events));

    // speeds ccontains the speed of each particle
    // before and after simulation.
    let mut speeds: Array2<f64> = Array2::zeros((2, p.get_len()));

    for j in 0..p.get_len()
    {
        speeds[[0, j]] = get_speed(j);
    }

    while i < number_of_events
    {

        let c = q.pop_next();

        if c.is_valid(p)
        {
            println!("Event number: {}", i);

            if test
            {
                print_particle_stats(&p);
                print_collision_stats(&q);
                plotting::plot_positions(&p);
            }

            system_data[[0, i]] = t;
            system_data[[1, i]] = p.get_kinetic_energy();
            // t is time of previous collision,
            // dt is time between previous and next collision.
            let dt = c.get_time() - t;
            p.propagate(dt);
            t += dt;
            i += 1;

            q.resolve_next_collision(&c, &mut p, t);
        }
    }

    for j in 0..p.get_len()
    {
        speeds[[1, j]] = get_speed(j);
    }

    return system_data;
}


// Test functions for a lot of stuff:
fn initiate_system(n: usize) -> particle::Particles
{
    let p = particle::generate_particles(
        n,
        parameters::X_MIN, 
        parameters::X_MAX, 
        parameters::Y_MIN, 
        parameters::Y_MAX,
        parameters::R,
        parameters::M);
    p
}

