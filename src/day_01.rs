use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run() {
    println!("Day 01");

    let file = File::open("data/01.txt").unwrap();
    let buffer = BufReader::new(file);
    let mut values = Vec::new();

    for line in buffer.lines() {
        values.push(line.unwrap().parse::<usize>().unwrap());
    }

    'one: for i in 0..values.len() {
        for j in 0..values.len() {
            if i == j {
                continue;
            }

            let sum = values[i] + values[j];
            if sum == 2020 {
                let result = values[i] * values[j];
                println!("Part One");
                println!("{} - {} & {} - {}", i, values[i], j, values[j]);
                println!("Result: {}", result);
                println!();
                break 'one;
            }
        }
    }

    'two: for i in 0..values.len() {
        for j in 0..values.len() {
            for k in 0..values.len() {
                if i == j || i == k || j == k {
                    continue;
                }

                let sum = values[i] + values[k] + values[j];
                if sum == 2020 {
                    let result = values[i] * values[j] * values[k];
                    println!("Part Two");
                    println!("{} - {}", i, values[i]);
                    println!("{} - {}", j, values[j]);
                    println!("{} - {}", k, values[k]);
                    println!("Result: {}", result);
                    break 'two;
                }
            }
        }
    }
}
