mod particle;
mod collisions;
mod parameters;
mod tests;
mod plotting;
mod simulation;
mod tasks;
mod save_data;

fn main() 
{
    let arg = "sim";
    match arg
    {
        "test" => tests::test_main(),
        "sim" => tasks::tasks_main(),
        _ => panic!("That was probably a typo."),
    }
}

