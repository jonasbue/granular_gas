use ndarray::prelude::*;
use crate::simulation;
use crate::plotting;


pub fn tasks_main()
{
    //task_1();
    //task_2();
    task_3();
    //task_4();
}

fn task_1() 
{
    let n: Array1<usize> = array![100];
    let r: Array1<f64> = array![0.01];
    let m: Array1<f64> = array![1.];

    let xi = 1.0;
    print_task_info(1, &n, &r, &m);
    let (energy, speeds) = simulation::simulate_system(&n, &r, &m, xi);

    plotting::plot_energy(&energy);
    plotting::plot_stats(speeds.slice(s![0,..]), speeds.slice(s![1,..]));
    println!("");

}


fn task_2() 
{
    let n: Array1<usize> = array![100, 100];
    let r: Array1<f64> = array![0.01, 0.01];
    let m: Array1<f64> = array![1., 4.];

    let xi = 1.0;
    print_task_info(2, &n, &r, &m);
    let (energy, speeds) = simulation::simulate_system(&n, &r, &m, xi);

    plotting::plot_energy(&energy);
    plotting::plot_stats(speeds.slice(s![0,..]), speeds.slice(s![1,..]));
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
    print_task_info(3, &n, &r, &m);

    for xi in [1.0, 0.9, 0.8].iter()
    {
        println!("Restitution coefficient Xi = {}", xi);
        let (energy, speeds) = simulation::simulate_system(&n, &r, &m, *xi);

        plotting::plot_energy(&energy);
        plotting::plot_stats(speeds.slice(s![0,..]), speeds.slice(s![1,..]));
        println!("");
    }
}

// Task 4 is different from the previous three.
// There will be a need to change the other functions.
fn task_4()
{
    unimplemented();
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
        println!("{}\t\t{}\t\t{}", n[i], m[i], r[i]);
    }
    println!("-------------------------------");
}
