mod helpers;
use std::time::{ Duration, Instant };
use std::fs::File;
use std::io::prelude::*;
use helpers::*;

pub fn busca_sequencial(data: &Vec<i32>, element: i32) -> isize {
    for (index, &value) in data.iter().enumerate() {
        if value == element {
            return index as isize;
        }
    }
    return -1;
}

pub fn busca_sequencial_otimizada(data: &Vec<i32>, element: i32) -> isize {
    for (index, &value) in data.iter().enumerate() {
        if value > element {
            return -1;
        } else if value == element {
            return index as isize;
        }
    }
    return -1;
}

pub fn busca_binaria(data: &Vec<i32>, element: i32) -> isize {
    let (mut start, mut end) = (0, data.len() - 1);
    while start < end {
        let mid = start + (end - start) / 2;
        if data[mid] == element {
            return mid as isize;
        }
        if data[mid] < element {
            start = mid + 1;
        } else {
            end = mid - 1; // Adjust end index when element is greater
        }
    }
    return -1;
}

pub fn busca_binaria_recursiva(data: &Vec<i32>, element: i32, start: usize, end: usize) -> isize {
    if start >= end {
        return -1;
    }
    let mid: usize = (start + end) / 2;
    if data[mid] == element {
        return mid as isize;
    } else if data[mid] > element {
        return busca_binaria_recursiva(data, element, start, mid - 1);
    } else {
        return busca_binaria_recursiva(data, element, mid + 1, end);
    }
}

pub fn measure_execution_time<F, R>(closure: F) -> Duration where F: FnOnce() -> R {
    let start_time = Instant::now(); // Get the current time before executing the closure
    let result = closure(); // Execute the closure
    let _aux = result;
    let duration = start_time.elapsed(); // Calculate the duration of execution

    duration // Return the duration in microseconds
}

fn regular_run(_n: Vec<i32>, q: Vec<i32>) {
    for (index, &q_size) in q.iter().enumerate() {
        let data_path = format!(r"src\data\dados_dez_a_{}.txt", index + 4);
        let mut data = read_data(&data_path);

        let keys_path = format!(r"src\data\chaves_dez_a_{}.txt", index + 2);
        let keys = read_data(&keys_path);

        let (mut bsc_sum, mut bco_sum, mut bbs_sum, mut bbr_sum) = (
            Duration::from_secs(0),
            Duration::from_secs(0),
            Duration::from_secs(0),
            Duration::from_secs(0),
        );
        let res_data = format!(r"src\resultados\tempo_exec_dez_a_{:?}.csv", index + 4);
        let mut res_data = File::create(&res_data).expect("Failed to create file");

        for &key in keys.iter() {
            let bsc_benchmark = measure_execution_time(|| busca_sequencial(&data, key));
            // bsc_sum += bsc_benchmark;
            println!("BSC: {:?}", bsc_benchmark);
            res_data
                .write(format!("BSC;{:?};\n", bsc_benchmark).as_bytes())
                .expect("Failed to write to file");
        }

        let start_sort_time = Instant::now();
        data.sort();
        let end_sort_time = Instant::now();
        let sort_duration = end_sort_time.duration_since(start_sort_time);
        res_data
            .write(format!("Sort_Time;{:?};\n", sort_duration).as_bytes())
            .expect("Erro ao inserir tempo de sorting");
        // bco_sum += sort_duration;
        // bbr_sum += sort_duration;
        // bbs_sum += sort_duration;

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
                        "BCO;{:?};\nBBS;{:?};\nBBR;{:?};\n",
                        bco_benchmark,
                        bbs_benchmark,
                        bbr_benchmark
                    ).as_bytes()
                )
                .expect("Failed to write to file");
            // bco_sum += bco_benchmark;
            // bbs_sum += bbs_benchmark;
            // bbr_sum += bbr_benchmark;
        }

        // let res_avg_name = format!(r"src\resultados\media_dez_a_{:?}.txt", index + 4);
        // let mut res_avg_file = File::create(&res_avg_name).expect("Failed to create file");
        // res_avg_file
        //     .write(
        //         format!(
        //             "BSC: {:?} \tBCO: {:?}\tBBS: {:?}\tBBR: {:?}\t Sort_Time: {:?}",
        //             bsc_sum / (q_size as u32),
        //             bco_sum / (q_size as u32),
        //             bbs_sum / (q_size as u32),
        //             bbr_sum / (q_size as u32),
        //             sort_duration
        //         ).as_bytes()
        //     )
        //     .expect("Failed to write to file");
        // println!(
        //     "BSC: {:?} \tBCO: {:?}\tBBS: {:?}\tBBR: {:?}\t Sort_Time: {:?}",
        //     bsc_sum / (q_size as u32),
        //     bco_sum / (q_size as u32),
        //     bbs_sum / (q_size as u32),
        //     bbr_sum / (q_size as u32),
        //     sort_duration
        // );
    }
}

fn optimized_run(q: Vec<i32>) {
    for (index, &q_size) in q.iter().enumerate() {
        let data_path = format!(r"src\data\dados_dez_a_{}.txt", index + 4);
        let mut data = read_data(&data_path);

        let keys_path = format!(r"src\data\chaves_dez_a_{}.txt", index + 4);
        let keys = read_data(&keys_path);

        let (mut bsc_sum, mut bco_sum, mut bbs_sum, mut bbr_sum) = (
            Duration::from_secs(0),
            Duration::from_secs(0),
            Duration::from_secs(0),
            Duration::from_secs(0),
        );
        let res_data = format!(r"src\resultados\optm_tempo_exec_dez_a_{:?}.csv", index + 4);
        let mut res_data = File::create(&res_data).expect("Failed to create file");

        for &key in keys.iter() {
            let bsc_benchmark = measure_execution_time(|| busca_sequencial(&data, key));
            bsc_sum += bsc_benchmark;
            println!("BSC: {:?}", bsc_benchmark);
            res_data
                .write(format!("BSC;{:?};\n", bsc_benchmark).as_bytes())
                .expect("Failed to write to file");
        }

        let start_sort_time = Instant::now();
        data.sort();
        let end_sort_time = Instant::now();
        let sort_duration = end_sort_time.duration_since(start_sort_time);
        res_data
            .write(format!("Sort_Time;{:?};\n", sort_duration).as_bytes())
            .expect("Erro ao inserir tempo de sorting");
        bco_sum += sort_duration;
        bbr_sum += sort_duration;
        bbs_sum += sort_duration;

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
                        "BCO;{:?};\nBBS;{:?};\nBBR;{:?};\n",
                        bco_benchmark,
                        bbs_benchmark,
                        bbr_benchmark
                    ).as_bytes()
                )
                .expect("Failed to write to file");
            bco_sum += bco_benchmark;
            bbs_sum += bbs_benchmark;
            bbr_sum += bbr_benchmark;
        }

        let res_avg_name = format!(r"src\resultados\optm_media_dez_a_{:?}.txt", index + 4);
        let mut res_avg_file = File::create(&res_avg_name).expect("Failed to create file");
        res_avg_file
            .write(
                format!(
                    "BSC: {:?} \tBCO: {:?}\tBBS: {:?}\tBBR: {:?}\t Sort_Time: {:?}",
                    bsc_sum / (q_size as u32),
                    bco_sum / (q_size as u32),
                    bbs_sum / (q_size as u32),
                    bbr_sum / (q_size as u32),
                    sort_duration
                ).as_bytes()
            )
            .expect("Failed to write to file");
        println!(
            "BSC: {:?} \tBCO: {:?}\tBBS: {:?}\tBBR: {:?}\t Sort_Time: {:?}",
            bsc_sum / (q_size as u32),
            bco_sum / (q_size as u32),
            bbs_sum / (q_size as u32),
            bbr_sum / (q_size as u32),
            sort_duration
        );
    }
}

fn main() {
    let n = vec![10000, 100000, 1000000, 10000000];
    let q = vec![100, 1000, 10000, 100000];
    let q_optimzed = vec![10000, 100000, 1000000, 10000000];
    // generate_data(&n);
    // generate_keys(&q);
    generate_keys_optm(&q_optimzed);

    // regular_run(n, q);
    optimized_run(n);
}
