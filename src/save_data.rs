use ndarray::prelude::*;

use std::vec::Vec;
use std::error::Error;
use std::fs::File;
use std::io::{Write};
use std::path::Path;
use std::env;

use crate::particle;

pub fn particles_to_file(p: &particle::Particles, filename: &str) 
-> Result<(), Box<dyn Error>>
{
    let mut data: Vec<Vec<f64>> = vec![
        p.pos.row(0).to_vec(),
        p.pos.row(1).to_vec(),
        p.vel.row(0).to_vec(),
        p.vel.row(1).to_vec(),
        p.r.to_vec(),
        p.m.to_vec(),
        p.collision_count.map(|a| *a as f64).to_vec()];

    let wd = env::current_dir().unwrap().display().to_string();
    let path_name = wd + &"/../data/".to_owned() + &filename.to_owned() + "_particles.csv";
    let path = Path::new(&path_name);
    let mut f = File::create(&path).expect("Could not open file.");

    write!(f, "x\ty\tv_x\tv_y\tradius\tmass\tcount\n")?;
    for i in 0..p.get_len()
    {
        for j in 0..7
        {
            write!(f, "{:?}\t", data[j][i])?;
        }
        write!(f, "\n");
    }
    println!("Data saved succesfully to file:\n{}", path_name);
    Ok(())
}

pub fn speed_to_file(data: &Array2<f64>, filename: &str)
-> Result<(), Box<dyn Error>>
{
    let wd = env::current_dir().unwrap().display().to_string();
    let path_name = wd + &"/../data/".to_owned() + &filename.to_owned() + "_speeds.csv";
    let path = Path::new(&path_name);
    let mut f = File::create(&path).expect("Could not open file.");

    for i in 0..data.nrows()
    {
        write!(f, "v_{:?}\t", i)?; 
    }
    write!(f, "\n")?;

    for i in 0..data.ncols()
    {
        for j in 0..data.nrows()
        {
            write!(f, "{:?}\t", data[[j, i]])?;
        }
        write!(f, "\n");
    }
    println!("Data saved succesfully to file:\n{}", path_name);
    Ok(())
}

pub fn energy_to_file(data: &Array2<f64>, filename: &str)
-> Result<(), Box<dyn Error>>
{
    let wd = env::current_dir().unwrap().display().to_string();
    let path_name = wd + &"/../data/".to_owned() + &filename.to_owned() + "_energy.csv";
    let path = Path::new(&path_name);
    let mut f = File::create(&path).expect("Could not open file.");

    write!(f, "time\te_tot")?;
    for i in 0..data.nrows()-1
    {
        write!(f, "\te_{:?}", i)?; 
    }
    write!(f, "\n")?;

    for i in 0..data.ncols()
    {
        for j in 0..data.nrows()
        {
            write!(f, "{:?}\t", data[[j, i]])?;
        }
        write!(f, "\n");
    }
    println!("Data saved succesfully to file:\n{}", path_name);
    Ok(())
}
/*
pub fn file_to_particles(filename: &str, n: usize) -> Result<Particles, Box<dyn Error>>
{
    let f_pos = File::open(filename.to_owned() + "_pos")?;
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(f);
    let pos_read: Array2<f64> = reader.deserialize_array2((2, n))?;

    let f_vel = File::open(filename.to_owned() + "_vel")?;
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(f);
    let vel_read: Array2<f64> = reader.deserialize_array2((2, n))?;

    /*
    let f_r = File::open(filename.to_owned() + "_r")?;
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(f);
    let r_read: Array1<f64> = reader.deserialize_array1(n)?;

    let f_m = File::open(filename.to_owned() + "_m")?;
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(f);
    let m_read: Array1<f64> = reader.deserialize_array1(n)?;

    let f_cc = File::open(filename.to_owned() + "_cc")?;
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(f);
    let cc_read: Array1<u16> = reader.deserialize_array1(n)?;
    */

    Ok(Particles{ 
        pos: pos_read, 
        vel: vel_read, 
        r: r_read, 
        m: m_read, 
        collision_count: cc_read })
}
*/

// TODO: Make a file that saves energy and speeds.
