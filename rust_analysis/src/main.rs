use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
use std::process::Command;
use std::time::Instant;

pub struct Statistics {
    pub mean: f64,
    pub median: f64,
    pub std: f64,
    pub size: usize,
}

pub fn compute_statistics(data: &Vec<f64>) -> Statistics {
    let size = data.len();
    let sum: f64 = data.iter().sum();
    let mean = sum / size as f64;

    let mut sorted_data = data.clone();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let median = if size % 2 == 1 {
        sorted_data[size / 2]
    } else {
        (sorted_data[size / 2 - 1] + sorted_data[size / 2]) / 2.0
    };

    let variance: f64 = data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / size as f64;
    let std = variance.sqrt();

    Statistics {
        mean,
        median,
        std,
        size,
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    // 1. Read the cars.csv file
    let file = File::open("insurance.csv")?;

    // Create the CSV reader with the specified delimiter
    let mut rdr = ReaderBuilder::new()
        .delimiter(b',') // Set the delimiter to ;
        .has_headers(true)
        .from_reader(file);

    // Find the index of first
    let headers = rdr.headers()?;
    let charges_index = headers
        .iter()
        .position(|h| h == "charges")
        .ok_or("charges column not found")?;

    // 2. Extract the "charges" column from the CSV data
    let mut charges1: Vec<f64> = Vec::new();
    for result in rdr.records() {
        let record = result?;
        if let Some(charges_str) = record.get(charges_index) {
            if let Ok(charges) = charges_str.parse::<f64>() {
                charges1.push(charges);
            }
        }
    }

    // 3. Compute the statistics
    let stats = compute_statistics(&charges1);
    println!("Mean: {}", stats.mean);
    println!("Median: {}", stats.median);
    println!("Standard Deviation: {}", stats.std);
    println!("Size: {}", stats.size);
    let end_time = Instant::now();

    // Calculate the elapsed time and resource usage
    let elapsed_time = end_time.duration_since(start_time);
    println!("Total execution time: {:?}", elapsed_time); // Print the elapsed time
                                                          // Memory usage
    let mem_info = sys_info::mem_info().unwrap();
    println!(
        "Memory Usage: {}%",
        (mem_info.total - mem_info.avail) as f32 / mem_info.total as f32 * 100.0
    );
    // CPU calculation
    let output = Command::new("ps")
        .arg("-o")
        .arg("%cpu")
        .arg("-p")
        .arg(format!("{}", std::process::id()))
        .output()
        .expect("Failed to execute ps command");

    // Convert the output to a string
    let usage = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = usage.split('\n').collect();

    // Parse the CPU usage from the output
    if lines.len() >= 2 {
        let usage_str = lines[1].trim();
        let usage_float: Result<f32, _> = usage_str.parse();
        match usage_float {
            Ok(usage) => println!("CPU Usage: {:.2}%", usage),
            Err(_) => println!("Failed to parse CPU usage"),
        }
    } else {
        println!("Failed to get CPU usage");
    }
    Ok(())
}