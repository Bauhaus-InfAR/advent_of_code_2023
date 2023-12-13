use std::{fs, str::Split};


const FILE_PATH: &str = "./input/input.txt";

fn main() {
    let contents = fs::read_to_string(FILE_PATH)
        .expect("File should load");
    let blocs: Split<'_, &str> = contents.split("\r\n\r\n");
    let rows1 = find_mirror_position(&blocs, 0); 
    let rows2 = find_mirror_position(&blocs, 1);
    let transpose = blocs.map(|b| {
        let n_chars = b.clone().lines().next().unwrap().chars().count();
        let mut out: Vec<String> = vec![String::new(); n_chars];
        b.lines().for_each(|l| {
            l.chars().enumerate().for_each(|(i, c)| {
                out[i].push(c);
            })
        });
        out.join("\n")
    }).collect::<Vec<_>>()
        .join("---");
    
    let cols1 = find_mirror_position(&transpose.split("---"), 0);
    let cols2 = find_mirror_position(&transpose.split("---"), 1);
    
    let rows1 = rows1.iter().sum::<usize>() * 100;
    let cols1 = cols1.iter().sum::<usize>();
    let task1 = rows1+cols1;
    println!("Task 1: {}", task1);
    
    let rows2 = rows2.iter().sum::<usize>() * 100;
    let cols2 = cols2.iter().sum::<usize>();
    let task2 = rows2+cols2;
    println!("Task 2: {}", task2);
}

fn find_mirror_position(x: &Split<'_, &str>, tolerance: u8) -> Vec<usize> {
    let ns = x.clone().map(|c| {
        let bloc = c.lines().collect::<Vec<_>>();
        let bloc_length = bloc.len();
        let mut n = 0;
        for i in 0..=bloc_length - 2 {
            let size = i + 1;
            let remainder = bloc_length - size;
            let kernel_start = if remainder >= size { 0 } else { size - remainder };
            let kernel = &bloc[kernel_start..size];
            let mirror = &bloc[size..size + size - kernel_start];
            let mirror_length = mirror.len() - 1;
            let n_mismatches = kernel
                .iter()
                .enumerate()
                .flat_map(|(i, e)| {
                    e.chars().enumerate().map(|(j, c)|
                        if c == mirror[mirror_length - i].chars().nth(j).unwrap() {0} else {1}
                    ).collect::<Vec<u8>>()
                }).sum::<u8>();
            if n_mismatches == tolerance {
                n = size;
                break;
            }
        }
        n
    });
    ns.collect::<Vec<_>>()
}