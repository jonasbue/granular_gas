use gnuplot::{Figure, Color};
//use gnuplot::linestyle; 
use gnuplot::*;
use crate::particle;
use crate::parameters;

//extern crate ndarray;
use ndarray::s;

pub fn plot_positions(particles: particle::Particles)
{
    let mut fig = Figure::new();
    fig.axes2d()
        .points(particles.pos.slice(s![0,..]), 
            particles.pos.slice(s![1,..]), 
            &[Color("black"), 
            PointSize(parameters::R * 10.),
            PointSymbol('O')]);

    match fig.show()
    {
        Ok(show) =>
        {
            println!("Figure rendered correctly");
            drop(show);
        }
        Err(gnu_error) => println!("Figure could not be rendered: {:?}", gnu_error),
    };
}
