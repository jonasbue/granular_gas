extern crate ndarray;
use ndarray::prelude::*;

use crate::particle;
use crate::parameters;
use crate::plotting;
use crate::collisions;


pub fn simulate_system(
    n_arr: &Array1<usize>, 
    r_arr: &Array1<f64>, 
    m_arr: &Array1<f64>, 
    xi: f64, 
    x_max: f64, 
    y_max: f64)
-> (particle::Particles, Array2<f64>, Array2<f64>)
{
    let t_0 = parameters::T_0;
    let mut p = initiate_system(n_arr, r_arr, m_arr, x_max, y_max);
    let mut q = fill_queue(&p, t_0, x_max, y_max);
    let n = parameters::NUMBER_OF_COLLISIONS;

    println!("Running simulation.");
    let (energy, speeds) 
        = evolve_system(&mut p, &mut q, n, t_0, &m_arr, xi, x_max, y_max, false);

    return (p, energy, speeds);
}


pub fn evolve_system(
    mut p: &mut particle::Particles, 
    q: &mut collisions::CollisionQueue, 
    number_of_events: usize,
    t_0: f64,
    m_arr: &Array1<f64>,
    xi: f64,
    x_max: f64,
    y_max: f64,
    test: bool)
    -> (Array2<f64>, Array2<f64>)
{
    let mut t = t_0;
    let mut i: usize = 0;

    // system_data contains some data about the system.
    // index 0: time of collisions
    // index 1: kinetic energy at these times
    let mut system_data: Array2<f64> = Array::zeros((4, number_of_events));

    // speeds ccontains the speed of each particle
    // before and after simulation.
    let mut speeds: Array2<f64> = Array2::zeros((2+m_arr.len(), p.get_len()));

    for j in 0..p.get_len()
    {
        speeds[[0, j]] = p.get_speed(j);
    }

    println!("Evolving system.");
    while i < number_of_events
    {
        if !test
        {
            status_bar(i, number_of_events);
        }

        let c = q.pop_next();
        if c.is_valid(p)
        {
            if test
            {
                print_particle_stats(&p);
                print_collision_stats(&q);
                plotting::plot_positions(&p, x_max, y_max);
            }

            system_data[[0, i]] = t;
            system_data[[1, i]] = p.get_tot_kinetic_energy();
            for j in 0..m_arr.len()
            {
                system_data[[j + 2, i]] = p.get_kinetic_energy_for_mass(m_arr[j]);
            }

            // t is time of previous collision,
            // dt is time between previous and next collision.
            let dt = c.get_time() - t;
            t += dt;
            i += 1;

            p.propagate(dt);
            q.resolve_next_collision(&c, &mut p, t, xi, x_max, y_max);
        }
    }
    print!(" Done.\n");

    for j in 0..p.get_len()
    {
        speeds[[1, j]] = p.get_speed(j);
    }

    return (system_data, speeds);
}


pub fn initiate_system(n: &Array1<usize>, r: &Array1<f64>, m: &Array1<f64>, x_max: f64, y_max: f64) 
-> particle::Particles
{
    let p = particle::generate_particles(
        n,
        parameters::X_MIN,
        x_max,
        parameters::Y_MIN,
        y_max,
        r,
        m);
    p
}


pub fn fill_queue(p: &particle::Particles, t_0: f64, x_max: f64, y_max: f64) 
    -> collisions::CollisionQueue
{
    let mut q = collisions::CollisionQueue::new();
    q.fill_collision_queue(&p, t_0, x_max, y_max);

    println!("Queue filled successfully.");
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


pub fn print_particle_stats(p: &particle::Particles)
{
    for i in 0..p.get_len()
    {
        println!("Particle {}", i);
        println!("Position: x = {:.2}, y = {:.2}", p.pos[[0,i]], p.pos[[1,i]]);
        println!("Velocity: x = {:.2}, y = {:.2}\n", p.vel[[0,i]], p.vel[[1,i]]);
    }
}

pub fn status_bar(i: usize, max: usize)
{
    let length: usize = 50;
    let prog = "=".repeat(length*i/max);
    let spaces = " ".repeat(length-prog.len()-1);
    print!("\rProgress: [{}>{}]", prog, spaces);
}

