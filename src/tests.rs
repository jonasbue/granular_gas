use crate::particle;
use crate::parameters;
use crate::plotting;
use crate::collisions;

pub fn test_main()
{
    test_simulation();
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

fn test_fill_queue(p: &particle::Particles) -> collisions::CollisionQueue
{
    let mut collisions = collisions::CollisionQueue::new();

    collisions.fill_collision_queue(&p);
    println!("Queue filled successfully.");
    print_collision_stats(&collisions);

    // An assertion that no duplicates exist could be nice
    return collisions;
}

fn test_resolve_collision(mut p: &mut particle::Particles, q: &mut collisions::CollisionQueue, number_of_events: usize)
{
    for i in 0..number_of_events
    {
        println!("Event number: {}", i);
        print_particle_stats(&p);
        print_collision_stats(&q);

        let c = q.pop_next();

        // Check here if collision is valid
        let p_1 = c.get_particle_1();
        let p_2 = c.get_particle_2();
        let cc_1 = c.get_collision_count(1);
        let cc_2 = c.get_collision_count(2);

        if p.get_collision_count(p_1) == cc_1 
        && p.get_collision_count(p_2) == cc_2
        {
            plotting::plot_positions(&p);

            let dt = c.get_time();
            println!("Propagating for a time {}", dt);
            p.propagate(dt);

            q.resolve_next_collision(&c, &mut p);
        }
    }
}

fn test_simulation()
{
    let mut p = test_generate_particles(parameters::N);
    let mut q = test_fill_queue(&p);
    let n = parameters::NUMBER_OF_COLLISIONS;

    println!("Running simulation.");
    test_resolve_collision(&mut p, &mut q, n);
}

pub fn print_collision_stats(q: &collisions::CollisionQueue)
{
    let mut i = 0;
    println!("Collision queue\n---------------------------------");
    for c in q.get_heap()
    {
        println!("Collision {}:\tt={:.3}, Particle {} colliding with particle {}", 
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

