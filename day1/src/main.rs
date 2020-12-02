use std::fs;
use std::io::BufReader;
use std::io::BufRead;
use std::env;

fn main() {
    let input_file = fs::File::open(env::args().nth(1).unwrap()).unwrap();
    let in_vec = BufReader::new(input_file)
                 .lines()
                 .flatten()
                 .map(|x| x.parse::<i32>())
                 .flatten()
                 .collect::<Vec<_>>();

    match find_ans(in_vec) {
        Ok(ans) => println!("Found the ans: {}", ans),
        Err(e) => eprintln!("Error: {}", e)
    }
}

fn find_ans(input: Vec<i32>) -> Result<i32, &'static str> {
    for i in 0..input.len() {
        for j in i..input.len() {
            for k in j..input.len() {
                if input[i] + input[j] + input[k] == 2020 {
                    return Ok(input[i] * input[j] * input[k]);
                }
            }
        }
    }
    Err("Could nof find numbers that add up to 2020")
}
