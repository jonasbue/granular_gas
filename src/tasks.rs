use ndarray::prelude::*;
use crate::simulation;
use crate::plotting;
use crate::particle;
use crate::save_data;


pub fn tasks_main()
{
    //task_1();
    task_2();
    task_3();
    //Task 4 is done, I think.
    ////task_4();
}

fn task_1() 
{
    let n: Array1<usize> = array![4000];
    let r: Array1<f64> = array![0.001];
    let m: Array1<f64> = array![0.001];

    let xi = 1.0;
    let x_max = 1.0;
    let y_max = 1.0;
    print_task_info(1, &n, &r, &m);
    println!("Packing fraction of particles: {}", 
        particle::get_packing_fraction(&n, &r, 0., 0., x_max, y_max));
    
    let (p, energy, speeds) = simulation::simulate_system(&n, &r, &m, 
        xi, x_max, y_max);

    save_data::particles_to_file(&p, "task_1_final");
    save_data::speed_to_file(&speeds, "task_1_final");
    save_data::energy_to_file(&energy, "task_1_final");

    //plotting::plot_energy_single_mass(&energy);
    //plotting::plot_stats(speeds.slice(s![0,..]), speeds.slice(s![1,..]));
    println!("");

}


fn task_2() 
{
    let n: Array1<usize> = array![2000, 2000];
    let r: Array1<f64> = array![0.001, 0.002];
    let m: Array1<f64> = array![0.001, 0.004];

    let xi = 1.0;
    let x_max = 1.0;
    let y_max = 1.0;
    print_task_info(2, &n, &r, &m);
    let (p, energy, speeds) = simulation::simulate_system(&n, &r, &m, xi, x_max, y_max);

    save_data::particles_to_file(&p, "task_2_diff_r");
    save_data::speed_to_file(&speeds, "task_2_diff_r");
    save_data::energy_to_file(&energy, "task_2_diff_r");

    //plotting::plot_energy_single_mass(&energy);
    //plotting::plot_stats(speeds.slice(s![0,..]), speeds.slice(s![1,..]));
    println!("");
}


// To count average particle collisions
// might not be necessary in order
// to implement a stopping criterium.
// The average number is known before start.
fn task_3() 
{
    let n: Array1<usize> = array![1000, 1000];
    let r: Array1<f64> = array![0.001, 0.002];
    let m: Array1<f64> = array![0.001, 0.004];
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
        //plotting::plot_energy_two_masses(&energy);
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
    let wall_amount: usize = 1800;
    let wall_radius = 0.005;

    let projectile_mass: f64 = 2.5;
    let n: Array1<usize> = array![wall_amount, 1];
    let r: Array1<f64> = array![wall_radius, 0.0];
    let m: Array1<f64> = array![0.1, projectile_mass];


    print_task_info(4, &n, &r, &m);

    // Both initiate_system and evolve_system 
    // work well, even for high packing fractions.
    // If more speed is needed, initiating the 
    // system with a grid and then propagating might be faster.
    let mut particles_init = simulation::initiate_system(&n, &r, &m, x_max, y_max);

    println!("Packing fraction of particles: {}", 
        particle::get_packing_fraction(&n, &r, 0., 0., x_max, y_max));
    particles_init.stop_all_particles();
    particles_init.set_particle_state(wall_amount, 0.5, 0.75, 0., -5.0, 0.025, projectile_mass);
    save_data::particles_to_file(&particles_init, "task_4_initial");

    y_max = 1.0;
    //let xi = 0.5;
    let v_0 = 3.0;
    let energy_cutoff_fraction = 0.10;
    let max_number_of_events = 10000;
    let number_of_scans = 4;
    let mut crater_sizes = Array2::zeros((2, number_of_scans));
    let mut vals = Array::linspace(0.01, 0.07, number_of_scans);


    for (i, val) in vals.iter().enumerate()
    {
        // Here, set the correct value (xi, v_0, m_i) to be 
        // equal to val, in order to do the parameter sweep.
        let xi = *val;
        println!("xi = {:.3}", val);
        //particles.m[wall_amount] = *m_i;
        //let m = array![0.01, *m_i];
        let mut particles = particles_init.copy();
        particles.set_particle_state(wall_amount, 0.5, 0.75, 0., -v_0, 0.1, 10.);
        let mut q = simulation::fill_queue(&particles, 0., x_max, y_max);

        //plotting::plot_positions(&particles, x_max, 1.0);
        let (energy, speeds) = simulation::evolve_system(&mut particles, &mut q, 
            max_number_of_events, 0., &m, &n, xi, x_max, y_max, energy_cutoff_fraction, true, false);

        crater_sizes[[0, i]] = xi;
        crater_sizes[[1, i]] = get_crater_size(&particles_init, &particles, 0.5);

        //if i == 0 || i == 3 || i == 6 || i == 9 || i == 12
        //{
        //    let filename = format!("{}{:.2}", "task_4_final_", val);
        //    save_data::particles_to_file(&particles, &filename);
        //    save_data::speed_to_file(&speeds, &filename);
        //    save_data::energy_to_file(&energy, &filename);
        //    //plotting::plot_positions(&particles, x_max, 1.0);
        //    //plotting::plot_energy_two_masses(&energy);
        //}
        println!("");
    }
    save_data::crater_size_to_file(&crater_sizes, "task_4_low");

}

fn get_crater_size(
    p_init: &particle::Particles, 
    p_final: &particle::Particles,
    r: f64)
    -> f64
{
    let n = p_init.get_len();
    assert_eq!(n, p_final.get_len());

    let mut d_pos = Array::zeros(n);
    // Count every particle that has 
    // moved more than r times it's diameter,
    // where r is a provided argument.
    for i in 0..n
    {
        if  (p_final.pos[[0, i]] - p_init.pos[[0, i]]).powi(2)
            + (p_final.pos[[0, i]] - p_init.pos[[0, i]]).powi(2)
            > (r*2.*p_final.r[i]).powi(2)
        {
            d_pos[i] = 1.;
        }
    }
    // Use the number of moved particles as an indicator 
    // of how many particles that were affected.
    return d_pos.scalar_sum();
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
