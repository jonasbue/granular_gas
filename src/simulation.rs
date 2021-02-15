extern crate ndarray;
use ndarray::prelude::*;

use crate::particle;
use crate::parameters;
use crate::plotting;
use crate::collisions;


pub fn simulate_system()
{
    let t_0 = parameters::T_0;
    let mut p = initiate_system(parameters::N);
    let mut q = fill_queue(&p, t_0);
    let n = parameters::NUMBER_OF_COLLISIONS;


    println!("Running simulation.");
    let system_data = evolve_system(&mut p, &mut q, n, t_0, false);

}

pub fn evolve_system(
    mut p: &mut particle::Particles, 
    q: &mut collisions::CollisionQueue, 
    number_of_events: usize,
    t_0: f64,
    test: bool)
    -> Array2<f64>
{
    let mut t = t_0;
    let mut i: usize = 0;

    // system_data contains all valuable data of system.
    // index 0: time of collisions
    // index 1: kinetic energy at these times
    let mut system_data: Array2<f64> = Array2::zeros((2, number_of_events));

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
    return system_data;
}

/*
fn append_system_stats() -> Array6<f64>
{

}

fn write_simulation_to_file(stats: Array6<f64>)
{

}
*/

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

    //plotting::plot_positions(&p);
    //print_particle_stats(&p);
    p
}

pub fn fill_queue(p: &particle::Particles, t_0: f64) 
    -> collisions::CollisionQueue
{
    let mut q = collisions::CollisionQueue::new();

    q.fill_collision_queue(&p, t_0);
    println!("Queue filled successfully.");
    //print_collision_stats(&q);

    return q;
}


pub fn print_collision_stats(q: &collisions::CollisionQueue)
{
    let mut i = 0;
    println!("Collision queue\n---------------------------------");
    for c in q.get_heap()
    {
        println!("Collision {}:\tt={:.2}, Particle {} colliding with particle {}", 
            i, c.get_time(), c.get_particle_1(), c.get_particle_2());
            i += 1;
    }
    println!("------------------------------");
}

fn print_particle_stats(p: &particle::Particles)
{
    for i in 0..p.get_len()
    {
        println!("Particle {}", i);
        println!("Position: x = {:.2}, y = {:.2}", p.pos[[0,i]], p.pos[[1,i]]);
        println!("Velocity: x = {:.2}, y = {:.2}\n", p.vel[[0,i]], p.vel[[1,i]]);
    }
}

