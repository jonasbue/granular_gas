use ndarray::prelude::*;
use crate::simulation;
use crate::plotting;
use crate::particle;
use crate::parameters;
use crate::save_data;


pub fn tasks_main()
{
    //task_1();
    //task_2();
    //task_3();
    task_4();
}

fn task_1() 
{
    let n: Array1<usize> = array![100];
    let r: Array1<f64> = array![0.01];
    let m: Array1<f64> = array![1.];

    let xi = 1.0;
    let x_max = 1.0;
    let y_max = 1.0;
    print_task_info(1, &n, &r, &m);
    let (p, energy, speeds) = simulation::simulate_system(&n, &r, &m, xi, x_max, y_max);

    save_data::particles_to_file(&p, "task_1");
    save_data::speed_to_file(&speeds, "task_1");
    save_data::energy_to_file(&energy, "task_1");

    plotting::plot_energy_single_mass(&energy);
    //plotting::plot_stats(speeds.slice(s![0,..]), speeds.slice(s![1,..]));
    println!("");

}


fn task_2() 
{
    let n: Array1<usize> = array![100, 100];
    let r: Array1<f64> = array![0.01, 0.01];
    let m: Array1<f64> = array![1., 4.];

    let xi = 1.0;
    let x_max = 1.0;
    let y_max = 1.0;
    print_task_info(2, &n, &r, &m);
    let (p, energy, speeds) = simulation::simulate_system(&n, &r, &m, xi, x_max, y_max);

    save_data::particles_to_file(&p, "task_2");
    save_data::speed_to_file(&speeds, "task_2");
    save_data::energy_to_file(&energy, "task_2");

    plotting::plot_energy_single_mass(&energy);
    //plotting::plot_stats(speeds.slice(s![0,..]), speeds.slice(s![1,..]));
    println!("");
}


// To count average particle collisions
// might not be necessary in order
// to implement a stopping criterium.
// The average number is known before start.
fn task_3() 
{
    let n: Array1<usize> = array![100, 100];
    let r: Array1<f64> = array![0.01, 0.01];
    let m: Array1<f64> = array![1., 4.];
    let x_max = 1.0;
    let y_max = 1.0;
    print_task_info(3, &n, &r, &m);

    for xi in [1.0, 0.9, 0.8].iter()
    {
        println!("Restitution coefficient Xi = {}", xi);
        let (p, energy, speeds) 
            = simulation::simulate_system(&n, &r, &m, *xi, x_max, y_max);

        save_data::particles_to_file(&p, &format!("{}{}", "task_3_", xi));
        save_data::speed_to_file(&speeds, &format!("{}{}", &"task_3_", xi));
        save_data::energy_to_file(&energy, &format!("{}{}", &"task_3_", xi));
        plotting::plot_energy_two_masses(&energy);
        //plotting::plot_stats(speeds.slice(s![0,..]), speeds.slice(s![1,..]));
        println!("");
    }
}

// Task 4 is different from the previous three.
// There will be a need to change the other functions.
fn task_4()
{   
    let x_max = 1.0;
    let mut y_max = 0.5;

    // A particle with radius 0 will never collide with other particles.
    let wall_amount: usize = 450;
    let wall_radius = 0.01;

    let projectile_mass: f64 = 10.;
    let n: Array1<usize> = array![wall_amount, 1];
    let r: Array1<f64> = array![wall_radius, 0.0];
    let m: Array1<f64> = array![0.01, projectile_mass];


    print_task_info(4, &n, &r, &m);

    // Both initiate_system and evolve_system 
    // work well, even for high packing fractions.
    // If more speed is needed, initiating the 
    // system with a grid and then propagating might be faster.
    let mut particles_init = simulation::initiate_system(&n, &r, &m, x_max, y_max);

    println!("Packing fraction of particles: {}", 
        particle::get_packing_fraction(&n, &r, 0., 0., x_max, y_max));
    particles_init.stop_all_particles();
    particles_init.set_particle_state(wall_amount, 0.5, 0.75, 0., -1.0, 0.1, 10.);
    save_data::particles_to_file(&particles_init, "task_4_initial");

    y_max = 1.0;
    //let xi = 0.5;
    let v_0 = 3.0;
    let energy_cutoff_fraction = 0.10;
    let max_number_of_events = 500;

    for val in [-0.5, 0.5, -1.0].iter()
    {
        let xi = *val;

        let mut particles = particles_init.copy();
        particles.set_particle_state(wall_amount, 0.5, 0.75, 0., -v_0, 0.1, 10.);

        //particles.m[wall_amount] = *m_i;
        //let m = array![0.01, *m_i];

        let mut q = simulation::fill_queue(&particles, 0., x_max, y_max);

        //plotting::plot_positions(&particles, x_max, 1.0);

        let (energy, speeds) = simulation::evolve_system(&mut particles, &mut q, 
            max_number_of_events, 0., &m, &n, xi, x_max, y_max, energy_cutoff_fraction, true, false);

        let filename = format!("{}{}", "task_4_final_", val);

        save_data::particles_to_file(&particles, &filename);
        save_data::speed_to_file(&speeds, &filename);
        save_data::energy_to_file(&energy, &filename);

        plotting::plot_positions(&particles, x_max, 1.0);
        plotting::plot_energy_two_masses(&energy);
        println!("");
    }

}


fn print_task_info(task: usize, n: &Array1<usize>, r: &Array1<f64>, m: &Array1<f64>)
{
    println!("\nRunning task {}.", task);
    println!("Particles in system:");
    println!("-------------------------------");
    println!("Amount\tRadius\tMass");
    println!("-------------------------------");
    for i in 0..n.len()
    {
        println!("{}\t{}\t{}", n[i], r[i], m[i]);
    }
    println!("-------------------------------");
}
