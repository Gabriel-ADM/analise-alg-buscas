use std::io::prelude::*;
use std::fs::*;
use std::io::BufReader;
use rand::Rng;
use std::time::{ Duration, Instant };

// Codigo utilizado para gerar os arquivos de dados utilizados
pub fn generate_data(data_size: &Vec<i32>) {
    for (index, &n) in data_size.iter().enumerate() {
        let file_name: String = format!(r"src\data\data_ten_to_{}.txt", index + 4);
        let mut file: File = File::create(&file_name).expect("Failed to create file");

        for _ in 0..n {
            let value = rand::thread_rng().gen_range(0..n);
            file.write_all(&format!("{}, ", value).as_bytes()).expect("Failed to write to file");
        }
    }
}
pub fn generate_keys(data_size: &Vec<i32>, initial_size: usize) {
    for (index, &n) in data_size.iter().enumerate() {
        let file_name: String = format!(r"src\data\keys_ten_to_{}.txt", index + initial_size);
        let mut file: File = File::create(&file_name).expect("Failed to create file");

        let max_rand: i32 = ((n as f64) * 1.5) as i32;
        for _ in 0..n {
            let value = rand::thread_rng().gen_range(0..max_rand);
            file.write_all(&format!("{}, ", value).as_bytes()).expect("Failed to write to file");
        }
    }
}

// Codigo utilizado para ler e armazenar dados gerados
pub fn read_data(file_path: &str) -> Vec<i32> {
    let file = File::open(file_path).expect("Failed to get file");
    let buf_reader = BufReader::new(file);
    let mut data: Vec<i32> = Vec::new();

    for line in buf_reader.lines() {
        let line = line.expect("Error getting line");
        data = line
            .split(", ")
            .filter_map(|num_str| num_str.parse::<i32>().ok())
            .collect();
    }

    data
}

pub fn measure_execution_time<F, R>(closure: F) -> Duration where F: FnOnce() -> R {
    let start_time = Instant::now();
    let result: R = closure();
    let _aux: R = result;
    let duration: Duration = start_time.elapsed();

    duration
}

pub fn process_result() {
    let result_files = read_dir(r"src\results").unwrap();
    for file in result_files {
        let file_path = file.unwrap().path();
        let file_name = file_path.display().to_string();

        if let Ok(mut file) = File::open(&file_path) {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                println!("Name: {}", file_name);
                // println!("Content: \n{}", contents);
                let mut values: Vec<f32> = Vec::new();

                let file = File::open(file_path).expect("Failed to get file");
                let buf_reader = BufReader::new(file);
                let mut data: Vec<(String, f32)> = Vec::new();

                for line in buf_reader.lines() {
                    let line = line.expect("Error getting line");
                    if let Some((alg_str, exec_str)) = line.split_once(";") {
                        let (alg, exec_time): (String, f32) = (
                            alg_str.to_string(),
                            exec_str
                                .replace(";", "")
                                .parse::<f32>()
                                .unwrap_or_else(|_| panic!("Failed to parse the number")),
                        );
                        data.push((alg, exec_time));
                        // println!("{:?}", data);
                    } else {
                        println!("Error trying to process csv line");
                    };
                }
            } else {
                eprintln!("Failed to read file: {}", file_name);
            }
        } else {
            eprintln!("Failed to open file: {}", file_name);
        }
    }
}
