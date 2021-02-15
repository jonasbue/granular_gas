mod particle;
mod collisions;
mod parameters;
mod tests;
mod plotting;
mod simulation;

fn main() 
{
    let arg = "simulate";
    match arg
    {
        "test" => tests::test_main(),
        "simulate" => simulation::simulate_system(),
        _ => panic!(),
    }
}

