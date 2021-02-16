use ndarray::prelude::*;
mod particle;
mod collisions;
mod parameters;
mod tests;
mod plotting;
mod simulation;

fn main() 
{
    let n: Array1<usize> = array![100, 100];
    let r: Array1<f64> = array![0.01, 0.01];
    // Different masses cause trouble
    let m: Array1<f64> = array![1., 4.];

    let arg = "test";
    match arg
    {
        "test" => tests::test_main(),
        "simulate" => simulation::simulate_system(&n, &r, &m),
        _ => panic!("That was probably a typo."),
    }
}

