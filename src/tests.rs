use crate::particle;
use crate::parameters;
use crate::plotting;

pub fn test_main()
{
    test_generate_particles();
}

fn test_generate_particles()
{
    let p = particle::generate_particles(
        parameters::N, 
        parameters::X_MIN, 
        parameters::X_MAX, 
        parameters::Y_MIN, 
        parameters::Y_MAX);

    //println!("p.pos: {} \np.vel: {}", p.pos, p.vel);
    plotting::plot_positions(p);
}

