pub struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return "1".to_string();
        }
        let mut result = String::from("1");
        for _ in 1..n {
            let mut v: Vec<Vec<char>> = vec![];
            let mut chars = result.chars();
            if let Some(mut last) = chars.next() {
                let mut g = vec![last];
                for c in chars {
                    if c == last {
                        g.push(c);
                    } else {
                        v.push(g);
                        g = vec![c];
                        last = c
                    }
                }
                v.push(g)
            };
            result = "".to_string();
            for (i, v) in v.iter().enumerate() {
                result.push_str(&format!("{}{}", v.len(), v.iter().next().unwrap()));
            }
        }
        return result;
    }
}

pub fn count_and_say_two(n: i32) -> String {
    use std::fmt::Write;
    let mut result = "1".to_string();
    for _ in 1..n {
        result = result
            .as_bytes()
            .chunk_by(|a, b| a == b)
            .fold(String::new(), |mut s, c| {
                write!(&mut s, "{}{}", c.len(), c[0] as char).unwrap();
                s
            });
    }
    result
}

pub fn count_and_say_three(n: i32) -> String {
    if n == 1 {
        return "1".to_string();
    }
    let mut result = String::from("1");
    for _ in 1..n {
        let mut next_result: String = String::new();
        let mut chars = result.chars();
        if let Some(mut last) = chars.next() {
            let mut count: u8 = 1;
            for c in chars {
                if c == last {
                    count += 1;
                } else {
                    next_result.push_str(&format!("{}{}", count.to_string(), last.to_string()));
                    count = 1;
                    last = c
                }
                println!("{}", last)
            }
            next_result.push_str(&format!("{}{}", count.to_string(), last.to_string()));
        };
        result = next_result;
    }
    return result;
}
}

fn main() {
    let result = Solution::count_and_say_three(5);
    println!("last::{}", result)
}