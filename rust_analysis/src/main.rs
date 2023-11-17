use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
use std::time::Instant;
use rayon::prelude::*;
use serde_derive::Deserialize;
//use sys_info;

#[derive(Debug, Deserialize)]
struct Insurance {
    charges: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();

    let file = File::open("insurance.csv")?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
    let mut insurances: Vec<Insurance> = rdr.deserialize().collect::<Result<_, _>>()?;

    insurances.par_sort_unstable_by(|a, b| a.charges.partial_cmp(&b.charges).unwrap());

    let mean = insurances.par_iter().map(|i| i.charges).sum::<f64>() / insurances.len() as f64;
    let median = insurances[insurances.len() / 2].charges;
    let std_dev = (insurances.par_iter().map(|i| (i.charges - mean).powi(2)).sum::<f64>() / insurances.len() as f64).sqrt();

    //let cpu = 0;
    let mem = sys_info::mem_info().unwrap();
    let mem_usage_mb = (mem.total - mem.avail) / 1024;  // Convert to MB
    let execution_time = start_time.elapsed();

    println!("Mean: {}, Median: {}, Std Dev: {}", mean, median, std_dev);
    println!("Memory Usage: {}, Execution Time: {:?}", mem_usage_mb, execution_time);

    Ok(())
}