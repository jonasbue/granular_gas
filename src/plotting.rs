use gnuplot::{Figure, Color};
//use gnuplot::linestyle; 
use gnuplot::*;
use crate::particle;
use crate::parameters;

//extern crate ndarray;
use ndarray::s;
use ndarray::prelude::*;

pub fn plot_positions(particles: &particle::Particles, x_max: f64, y_max: f64)
{
    let mut fig = Figure::new();
    //let mut dv = particles.pos;
    //dv = dv.slice_mut(s![0,..]) + particles.vel.slice_mut(s![0,..]);
    //dv = dv.slice_mut(s![1,..]) + particles.vel.slice_mut(s![1,..]);


    //let dt = 1.;
    fig.axes2d()
        .points(
            particles.pos.slice(s![0,..]), 
            particles.pos.slice(s![1,..]), 
            &[Color("black"), 
            PointSize(1.),// 140 is a scaling factor 
            PointSymbol('O'),               // set by hand to give the
                                            // particles the right size.
        ])
        .set_x_range(Fix(parameters::X_MIN), Fix(x_max))
        .set_y_range(Fix(parameters::Y_MIN), Fix(y_max));
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

pub fn plot_stats(t: ArrayView<f64, Ix1>, y: ArrayView<f64, Ix1>)
{
    let mut fig = Figure::new();
    fig.axes2d()
        .points(
            t,
            y,
            &[Color("black"),
            PointSize(3.),
            ])
        .set_x_range(Fix(0.), Fix(2.))
        //.set_y_range(Fix(0.), Fix(60.))
        ;

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

pub fn plot_energy(e: &Array2<f64>)
{
    let mut fig = Figure::new();
    fig.axes2d()
        .lines(
            e.slice(s![0,..]),
            e.slice(s![1,..]),
            &[Color("black"),
            LineWidth(3.),
            ])
        .lines(
            e.slice(s![0,..]),
            e.slice(s![2,..]),
            &[Color("red"),
            LineWidth(3.),
            ])
         .lines(
            e.slice(s![0,..]),
            e.slice(s![3,..]),
            &[Color("blue"),
            LineWidth(3.),
            ])       
        //.set_x_range(Fix(0.), Fix(2.))
        //.set_y_range(Fix(0.), Fix(60.))
        ;

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
 
