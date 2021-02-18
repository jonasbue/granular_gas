extern crate ndarray;
use ndarray::prelude::*;

use crate::particle;
use crate::parameters;
use crate::simulation;
use crate::plotting;

pub fn test_main()
{
    assert_correct_impact_stats();
    test_one_particle();
    test_two_particles();
    test_collision_angle();
    test_some_particles();
    test_many_particles();
}


fn test_one_particle()
{
    let mut p = particle::Particles
    {
        pos: arr2(&[[0.2], [0.3]]),
        vel: arr2(&[[1.], [1.]]),
        r: Array1::from_elem(1, 0.01),
        m: Array1::from_elem(1, 0.01),
        collision_count: Array1::zeros(1),
    };
    let xi = 1.0;
    let x_max = 1.0;
    let y_max = 1.0;
    let mut q = simulation::fill_queue(&p, 0., x_max, y_max);
    
    println!("Running simulation with one particle.");
    println!("Behaving correctly, it should collide with \
    all four walls before returning to it's starting point \
    (x, y) = ({}, {})", 0.5, 0.5);
    simulation::evolve_system(&mut p, &mut q, 5, 0., &array![], xi, x_max, y_max, true);

}

fn test_two_particles()
{
    let mut p = particle::Particles
    {
        pos: arr2(&[[0.3, 0.7], [0.5, 0.5]]),
        vel: arr2(&[[1., -1.], [0., 0.]]),
        r: Array1::from_elem(2, 0.01),
        m: Array1::from_elem(2, 0.01),
        collision_count: Array1::zeros(2),
    };
    let xi = 1.0;
    let x_max = 1.0;
    let y_max = 1.0;
    let mut q = simulation::fill_queue(&p, 0., x_max, y_max);
    
    println!("Running simulation with two particles.");
    println!("Behaving correctly, thay should collide with \
    each other, then the walls, repeatedly.");
    simulation::evolve_system(&mut p, &mut q, 5, 0., &array![], xi, x_max, y_max, true);
}


fn test_collision_angle()
{
    let mut p = particle::Particles
    {
        pos: arr2(&[[0.3, 0.7], [0.6, 0.5]]),
        vel: arr2(&[[1., 0.], [0., 0.]]),
        r: arr1(&[0.001, 0.1]),
        m: arr1(&[1., 1e6]),
        collision_count: Array1::zeros(2),       
    };
    let xi = 1.0;
    let x_max = 1.0;
    let y_max = 1.0;
    let mut q = simulation::fill_queue(&p, 0., x_max, y_max);
    
    println!("Running simulation with two \
    particles of vastly different size.");
    println!("Behaving correctly, they should collide with \
    each other, then the smaller particle will change direction.");
    simulation::evolve_system(&mut p, &mut q, 5, 0., &array![1., 1e6], xi, x_max, y_max, true);
}

fn test_some_particles()
{
    let x_max = 1.5;
    let y_max = 0.4;
    let mut p = particle::generate_particles(
        &array![10],
        parameters::X_MIN,
        x_max,
        parameters::Y_MIN,
        y_max,
        &array![0.01],
        &array![1.]);
    let xi = 1.0;
    let mut q = simulation::fill_queue(&p, 0., x_max, y_max);
    println!("Running simulation with a small number of particles.");
    println!("Velocity and collision data will be printed.");
    let (energy, _speeds) = simulation::evolve_system(
        &mut p,&mut q, 5, 0., &array![0.01], xi, x_max, y_max, true);

    plotting::plot_energy(&energy);
    //plotting::plot_stats(speeds.slice(s![0,..]), speeds.slice(s![1,..]));
}



// Generates particles and plots the total kinetic energy of the system.
fn test_many_particles()
{
    let x_max = 1.5;
    let y_max = 0.4;

    let mut p = particle::generate_particles(
        &array![100],
        parameters::X_MIN,
        x_max,
        parameters::Y_MIN,
        y_max,
        &array![0.01],
        &array![1.]);    
    let xi = 1.0;
    let mut q = simulation::fill_queue(&p, 0., x_max, y_max);
    println!("Running simulation with many particles.");
    println!("Energy should remain constant.");
    let (energy, speeds) = simulation::evolve_system(
        &mut p ,&mut q, 500, 0., &array![0.01], xi, x_max, y_max, false);
    plotting::plot_energy(&energy);
    plotting::plot_stats(speeds.slice(s![0,..]), speeds.slice(s![1,..]));
}


// Asserts that the collision function works properly
// on one special case.
fn assert_correct_impact_stats()
{
    let pos = arr2(&[[0.,1.], [1.,1.]]);
    let vel = arr2(&[[-1.,0.], [0.,0.]]);
    let r = Array1::ones(2);

    let (r_2, d, dvdx, dx_2, dv_2, dx) 
        = particle::calculate_impact_stats(
            &pos, &vel, &r, 0 as usize, 1 as usize);
    assert_eq!(r_2, 4.);
    assert_eq!(dx_2, 1., "dx squared = {}", dx_2);
    assert_eq!(dv_2, 1., "dv squared = {}", dv_2);
    assert_eq!(dx, arr1(&[1., 0.]));
    assert_eq!(dvdx, 1.);
    assert_eq!(d, 1. - 1.*(1. - 4.), "d = {}, and 16. was expected", d);
}

