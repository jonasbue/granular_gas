use gnuplot::{Figure, Color, BorderLocation2D};
//use gnuplot::linestyle; 
use gnuplot::*;
use crate::particle;
use crate::parameters;

//extern crate ndarray;
use ndarray::s;

pub fn plot_positions(particles: &particle::Particles)
{
    let mut fig = Figure::new();
    //let dt = 1.;
    fig.axes2d()
        .points(
            particles.pos.slice(s![0,..]), 
            particles.pos.slice(s![1,..]), 
            &[Color("black"), 
            PointSize(parameters::R * 15.),
            PointSymbol('O'),
        ])
        .set_x_range(Fix(parameters::X_MIN), Fix(parameters::X_MAX))
        .set_y_range(Fix(parameters::Y_MIN), Fix(parameters::Y_MAX));
        /*
        .points(
            &particles.pos.slice(s![0,..]) + &particles.vel.slice(s![0,..]), 
            &particles.pos.slice(s![1,..]) + &particles.vel.slice(s![1,..]), 
            &[Color("black"), 
            PointSize(parameters::R * 100. / parameters::N as f64),
            PointSymbol('O'),
        ]);
        */
        /*
        // There should be an arrows()-function in gnuplot
        .arrow(
            Axis(particles.pos[[0,..]]),
            Axis(particles.pos[[1,..]]),
            Axis(particles.pos[[0,..]] + 10. * particles.vel[[0,..]]),
            Axis(particles.pos[[1,..]] + 10. * particles.vel[[1,..]]),
            &[
                Color("black"),
                ArrowType(Filled),
                ArrowSize(1.0),
            ]);
        */

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
