mod helpers;
mod algorithms;
use std::any::{type_name_of_val, Any};
use std::{ env, fs };
use std::time::Instant;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use helpers::*;
use algorithms::*;

fn exec_algms(_n: Vec<i32>, q: Vec<i32>, optimized: bool) {
    for (index, &_q_size) in q.iter().enumerate() {
        let data_path = format!(r"src\data\data_ten_to_{}.txt", index + 4);
        let mut data = read_data(&data_path);

        let (keys_path, res_data): (String, String);
        if optimized {
            keys_path = format!(r"src\data\keys_ten_to_{}.txt", index + 4);
            res_data = format!(r"src\results\optm_exec_10_to_{:?}.csv", index + 4);
        } else {
            keys_path = format!(r"src\data\keys_ten_to_{}.txt", index + 2);
            res_data = format!(r"src\results\exec_10_to_{:?}.csv", index + 4);
        }

        let keys = read_data(&keys_path);
        let mut res_data = File::create(&res_data).expect(
            "Failed to create file, perhaps run it using generate-data"
        );

        for &key in keys.iter() {
            let bsc_benchmark = measure_execution_time(|| busca_sequencial(&data, key));
            println!("BSC: {:?}", bsc_benchmark);
            res_data
                .write(format!("BSC;{:.10};\n", bsc_benchmark.as_secs_f64() * 1000.0).as_bytes())
                .expect("Failed to write to file");
        }

        let start_sort_time = Instant::now();
        data.sort();
        let end_sort_time = Instant::now();
        let sort_duration = end_sort_time.duration_since(start_sort_time);
        res_data
            .write(format!("Sort_Time;{:.10};\n", sort_duration.as_secs_f64() * 1000.0).as_bytes())
            .expect("Error inserting sorting time");

        for &key in keys.iter() {
            let bco_benchmark = measure_execution_time(|| busca_sequencial_otimizada(&data, key));
            println!("BCO: {:?}", bco_benchmark);
            let bbs_benchmark = measure_execution_time(|| busca_binaria(&data, key));
            println!("BBS: {:?}", bbs_benchmark);
            let bbr_benchmark = measure_execution_time(||
                busca_binaria_recursiva(&data, key, 0, data.len() - 1)
            );
            println!("BBR: {:?}", bbr_benchmark);
            res_data
                .write(
                    format!(
                        "BCO;{:.10};\nBBS;{:.10};\nBBR;{:.10};\n",
                        bco_benchmark.as_secs_f64() * 1000.0,
                        bbs_benchmark.as_secs_f64() * 1000.0,
                        bbr_benchmark.as_secs_f64() * 1000.0
                    ).as_bytes()
                )
                .expect("Failed to write to file");
        }
    }
}

fn main() {
    let n = vec![10000, 100000, 1000000, 10000000];
    let q;
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 && args[1] == "generate-data" {
        println!("Generating data...");
        generate_data(&n);
    }

    if cfg!(debug_assertions) {
        println!("Running unoptimized compilation");
        q = vec![100, 1000, 10000, 100000];
        generate_keys(&q, 2);
        exec_algms(n, q, false);
    } else {
        println!("Running optimized compilation");
        q = vec![10000, 100000, 1000000, 10000000];
        generate_keys(&q, 4);
        exec_algms(n, q, true);
    }

    process_result();
}
