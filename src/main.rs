mod particle;
mod collisions;
mod parameters;
mod tests;
mod plotting;
mod simulation;
mod tasks;

fn main() 
{
    let arg = "simulate";
    match arg
    {
        "test" => tests::test_main(),
        "simulate" => tasks::task_1(),
        _ => panic!("That was probably a typo."),
    }
}

