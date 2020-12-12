use std::env;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let input_path = env::args().nth(1).unwrap();
    let mut ans = 1;
    for (right, down) in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter() {
        let input_file = fs::File::open(&input_path).unwrap();
        ans *= BufReader::new(input_file)
            .lines()
            .flatten()
            .step_by(*down)
            .enumerate()
            .filter(|(x, y)| did_hit_tree(x, y, right))
            .count();
    }
    println!("{}", ans);
}

fn did_hit_tree(x: &usize, y: &String, slope: &usize) -> bool {
    let pos = (slope * x) % y.len() as usize;
    y.chars().nth(pos) == Some('#')
}
