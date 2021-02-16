use ndarray::prelude::*;
use crate::simulation;
use crate::plotting;

pub fn task_1() 
{
    let n: Array1<usize> = array![100];
    let r: Array1<f64> = array![0.01];
    let m: Array1<f64> = array![1.];

    let (energy, speeds) = simulation::simulate_system(&n, &r, &m);

    plotting::plot_energy(&energy);
    plotting::plot_stats(speeds.slice(s![0,..]), speeds.slice(s![1,..]));

}


pub fn task_2() 
{
    let n: Array1<usize> = array![100, 100];
    let r: Array1<f64> = array![0.01, 0.01];
    let m: Array1<f64> = array![1., 4.];

    let (energy, speeds) = simulation::simulate_system(&n, &r, &m);

    plotting::plot_energy(&energy);
    plotting::plot_stats(speeds.slice(s![0,..]), speeds.slice(s![1,..]));
}


//TODO: Count average particle collisions
//to implement a stopping criterium.
pub fn task_3() 
{
    let n: Array1<usize> = array![100, 100];
    let r: Array1<f64> = array![0.01, 0.01];
    let m: Array1<f64> = array![1., 4.];

    //TODO: Set xi to be a variable.
    let (energy, speeds) = simulation::simulate_system(&n, &r, &m);

    plotting::plot_energy(&energy);
    plotting::plot_stats(speeds.slice(s![0,..]), speeds.slice(s![1,..]));
}

