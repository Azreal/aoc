use std::io::{self, Read};
use std::collections::HashMap;


fn solve(lines: &str) -> i64 {
    let mut two_count = 0;
    let mut three_count = 0;

    for line in lines.lines() {
        let mut char_count :HashMap<char, i64> = HashMap::new();
        for c in line.chars() {
            let count = char_count.entry(c).or_insert(0);
            *count += 1;
        }

        let mut has_two = false;
        let mut has_three = false;
        for (_, v) in &char_count {
            match v {
                2 => { has_two = true },
                3 => { has_three = true },
                _ => ()
            }
        }

        if has_two {
            two_count += 1;
        }

        if has_three {
            three_count += 1;
        }
    }
    
    two_count * three_count
}


fn main() { 
    let mut lines = String::new();
    
    io::stdin().read_to_string(&mut lines)
        .expect("Read failed");

    let result = solve(&lines);
    println!("{}", result);
}


#[test]
fn test() {
    let x = 
"abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab";

    assert_eq!(solve(x), 4 * 3);
}
