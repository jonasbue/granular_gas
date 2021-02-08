extern crate ndarray;
use ndarray::prelude::*;

use crate::particle;
use crate::parameters;
use crate::plotting;
use crate::collisions;

pub fn test_main()
{
    //test_calculate_impact_stats();
    test_simulation();
    //test_collide_two_particles();
}

fn test_simulation()
{
    let t_0 = parameters::T_0;
    let mut p = test_generate_particles(parameters::N);
    let mut q = test_fill_queue(&p, t_0);
    let n = parameters::NUMBER_OF_COLLISIONS;

    println!("Running simulation.");
    test_resolve_collision(&mut p, &mut q, n, t_0);
}

fn test_resolve_collision(
    mut p: &mut particle::Particles, 
    q: &mut collisions::CollisionQueue, 
    number_of_events: usize,
    t_0: f64)
{
    let mut t = t_0;
    let mut i: usize = 0;
    while i < number_of_events
    {

        let c = q.pop_next();

        if c.is_valid(p)
        {
            println!("Event number: {}", i);
            //print_particle_stats(&p);
            //print_collision_stats(&q);
            plotting::plot_positions(&p);

            let dt = c.get_time() - t;
            println!("Propagating for a time {}", dt);
            p.propagate(dt);
            t += dt;
            i += 1;

            q.resolve_next_collision(&c, &mut p, t);
        }
        else
        {
            println!("A collision was discarded because \
            particle 1 had index {} where {} was expected, or
            particle 2 had index {} where {} was expected", 
            c.get_collision_count(1), p.get_collision_count(c.get_particle_1()),
            c.get_collision_count(2), p.get_collision_count(c.get_particle_2()));
        }
        println!("-------------------------------------------");
    }
}

// Test functions for a lot of stuff:
fn test_generate_particles(n: usize) -> particle::Particles
{
    let p = particle::generate_particles(
        n,
        parameters::X_MIN, 
        parameters::X_MAX, 
        parameters::Y_MIN, 
        parameters::Y_MAX,
        parameters::R,
        parameters::M);

    assert_eq!(p.get_len(), n);
    for i in 0..n
    {
        assert!(p.is_within_box(i));
    }
    //plotting::plot_positions(&p);
    p
}

fn test_fill_queue(p: &particle::Particles, t_0: f64) 
    -> collisions::CollisionQueue
{
    let mut collisions = collisions::CollisionQueue::new();

    collisions.fill_collision_queue(&p, t_0);
    println!("Queue filled successfully.");
    print_collision_stats(&collisions);

    // An assertion that no duplicates exist could be nice
    return collisions;
}


fn test_collide_two_particles()
{
    let mut p = particle::Particles
    {
        pos: arr2(&[[0.3, 0.7], [0.5, 0.5]]),
        vel: arr2(&[[1., -1.], [0., 0.]]),
        r: Array1::from_elem(2, parameters::R),
        m: Array1::from_elem(2, parameters::M),
        collision_count: Array1::zeros(2),
    };

    let mut q = test_fill_queue(&p, 0.);
    
    println!("Running simulation with two particles.");
    test_resolve_collision(&mut p, &mut q, 9, 0.);
}
fn test_calculate_impact_stats()
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

