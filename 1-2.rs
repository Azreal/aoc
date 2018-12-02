use std::io::{self, Read};
use std::collections::HashSet;


fn solve(nums: Vec<i64>) -> i64 {
    let mut freq_history :HashSet<i64> = HashSet::new();
    let mut freq = 0;
    loop {
        for num in &nums {
            freq_history.insert(freq);
            freq += num;
            if freq_history.contains(&freq) {
                return freq;
            }
        }
    }
}

fn to_num(word :&str) -> i64 {
    word.parse().expect("Not a number")
}

fn main() { 
    let mut lines = String::new();
    
    io::stdin().read_to_string(&mut lines)
        .expect("Read failed");

    let nums = lines.lines().map(to_num).collect();

    let result = solve(nums);
    println!("{}", result);
}


#[test]
fn test() {
    assert_eq!(solve(vec![1, -1]), 0);
    assert_eq!(solve(vec![3, 3, 4, -2, -4]), 10);
    assert_eq!(solve(vec![-6, 3, 8, 5, -6]), 5);
    assert_eq!(solve(vec![7, 7, -2, -7, -4]), 14);
}
