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
        "simulate" => tasks::tasks_main(),
        _ => panic!("That was probably a typo."),
    }
}

