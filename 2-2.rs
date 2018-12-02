use std::io::{self, Read};
//use std::collections::HashMap;

fn solve(lines: &str) -> String {

    let lines :Vec<_> = lines.lines().collect();

    for (i1, l1) in lines.iter().enumerate() {
        for l2 in lines.iter().skip(i1 + 1) {
            if l1.chars().count() != l2.chars().count() {
                continue
            }

            let common :String = l1.chars().zip(l2.chars())
                .filter_map(|(x, y)| if x == y { 
                    return Some(x);
                }
                else {
                    return None;
                })
                .collect();


            if l1.chars().count() - 1 == common.chars().count() {
                return common;
            }
        }
    }

    "error".to_string()
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
"abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz";

    assert_eq!(solve(x), "fgij");
}
