use crate::particle;
use crate::parameters;
use crate::plotting;
use crate::collisions;

use std::collections::BinaryHeap;

pub fn test_main()
{
    //test_generate_particles();
    test_resolve_collition();
}

fn test_generate_particles()
{
    let p = particle::generate_particles(
        parameters::N, 
        parameters::X_MIN, 
        parameters::X_MAX, 
        parameters::Y_MIN, 
        parameters::Y_MAX);

    //println!("p.pos: {} \np.vel: {}", p.pos, p.vel);
    plotting::plot_positions(&p);
}

fn test_fill_queue() -> collisions::CollisionQueue
{
    let p = particle::generate_particles(
        1,
        parameters::X_MIN, 
        parameters::X_MAX, 
        parameters::Y_MIN, 
        parameters::Y_MAX);

    let mut collisions = collisions::CollisionQueue::new();
    plotting::plot_positions(&p);
    collisions.fill_collision_queue(&p);
    let c = collisions.get_next();
    println!("Time until next collition: {:?}", c.get_time());
    return collisions;
}

fn test_resolve_collition()
{
    let mut p = particle::generate_particles(
        1,
        parameters::X_MIN, 
        parameters::X_MAX, 
        parameters::Y_MIN, 
        parameters::Y_MAX);

    let mut collisions = collisions::CollisionQueue::new();
    collisions.fill_collision_queue(&p);
    for i in 0..5
    {
        //plotting::plot_positions(&p);
        print_particle_stats(&p);
        print_collision_stats(&collisions);

        let c = collisions.pop_next();
        let dt = c.get_time();
        p.propagate(dt);
        /*
        //plotting::plot_positions(&p);
        //plotting::plot_positions(&p);

        collisions.resolve_next_collision(&mut p);
        */
        //println!("Time until next next collition: {:?}", c_next.get_time());
    }
}

pub fn print_collision_stats(q: &collisions::CollisionQueue)
{
    for i in 0..q.get_len()
    {
        let c = q.get_next();
        println!("Collision {:.2}:\tt={:.2}, Particle {} colliding with particle {}", 
            i, c.get_time(), c.get_particle_1(), c.get_particle_2());
    }
}

fn print_particle_stats(p: &particle::Particles)
{
    for i in 0..p.r.len()
    {
        println!("Particle {}", i);
        println!("Position: x = {:.2}, y = {:.2}", p.pos[[0,i]], p.pos[[1,i]]);
        println!("Velocity: x = {:.2}, y = {:.2}\n", p.vel[[0,i]], p.vel[[1,i]]);
    }
}

