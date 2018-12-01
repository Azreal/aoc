use std::io::{self, Read};


fn solve(lines: String) -> i64 {
    let mut sum = 0;
    for line in lines.lines() {
        let x: i64 = line.parse().expect("Not a number");
        sum += x;
    }

    sum
}


fn main() { 
    let mut lines = String::new();
    
    io::stdin().read_to_string(&mut lines)
        .expect("Read failed");

    let result = solve(lines);
    println!("{}", result);
}


#[test]
fn test() {
    let x = "+1\n+1\n+1".split(',').collect();
    assert!(solve(x) == 3);
}
