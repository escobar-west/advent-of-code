use std::fs;
use std::io::BufReader;
use std::io::BufRead;
use std::env;

fn main() {
    let input_file = fs::File::open(env::args().nth(1).unwrap()).unwrap();
    let ans = BufReader::new(input_file)
                 .lines()
                 .flatten()
                 .filter(|x| find_ans2(x))
                 .count();

    println!("{}", ans);   
}

fn find_ans1(line: &String) -> bool {
    let mut line_iter = line.split_whitespace();
    let mut range = line_iter.next().unwrap().split("-").map(|x| x.parse::<usize>());
    let min = range.next().unwrap().unwrap();
    let max = range.next().unwrap().unwrap();

    let letter = line_iter.next().unwrap().chars().next().unwrap();

    let n_chars = line_iter.next().unwrap().chars().filter(|x| *x == letter).count();

    n_chars >= min && n_chars <= max
}

fn find_ans2(line: &String) -> bool {
    let mut line_iter = line.split_whitespace();
    let mut range = line_iter.next().unwrap().split("-").map(|x| x.parse::<usize>());
    let p1 = range.next().unwrap().unwrap();
    let p2 = range.next().unwrap().unwrap();

    let letter = line_iter.next().unwrap().chars().next();
    let mut pword_iter = line_iter.next().unwrap().chars();

    let c1 = pword_iter.clone().nth(p1-1);
    let c2 = pword_iter.nth(p2-1);

    (c1 == letter) ^ (c2 == letter)
}
