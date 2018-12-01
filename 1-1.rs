use std::io::{self, Read};


fn solve(lines: String) {
    let mut sum = 0;
    for line in lines.lines() {
        let x: i64 = line.parse().expect("Not a number");
        sum += x;
    }

    println!("{}", sum);
}

fn main() { 
    let mut lines = String::new();
    
    io::stdin().read_to_string(&mut lines)
        .expect("Read failed");

    solve(lines);
}

