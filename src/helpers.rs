use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use rand::Rng;

// Codigo utilizado para gerar os arquivos de dados utilizados
pub fn generate_data(data_size: &Vec<i32>) {
    for (index, &n) in data_size.iter().enumerate() {
        let file_name: String = format!(r"src\data\dados_dez_a_{}.txt", index + 4);
        let mut file: File = File::create(&file_name).expect("Failed to create file");

        for _ in 0..n {
            let value = rand::thread_rng().gen_range(0..n);
            file.write_all(&format!("{}, ", value).as_bytes()).expect("Failed to write to file");
        }
    }
}
pub fn generate_keys(data_size: &Vec<i32>) {
    for (index, &n) in data_size.iter().enumerate() {
        let file_name: String = format!(r"src\data\chaves_dez_a_{}.txt", index + 2);
        let mut file: File = File::create(&file_name).expect("Failed to create file");

        let max_rand: i32 = ((n as f64) * 1.5) as i32;
        for _ in 0..n {
            let value = rand::thread_rng().gen_range(0..max_rand);
            file.write_all(&format!("{}, ", value).as_bytes()).expect("Failed to write to file");
        }
    }
}

pub fn generate_keys_optm(data_size: &Vec<i32>) {
    for (index, &n) in data_size.iter().enumerate() {
        let file_name: String = format!(r"src\data\chaves_dez_a_{}.txt", index + 4);
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
