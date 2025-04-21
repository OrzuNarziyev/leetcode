pub struct Solution;

// impl Solution {
//     pub fn count_and_say(n: i32) -> String {
//         let mut str = String::from("1");
//         if n == 1 {
//             return str;
//         }
//         for i in 1..n {
//             // let mut chars = str.chars();
//             for c in str.split("") {}
//         }
//
//         str
//     }
// }

impl Solution {
    pub fn count_and_say(n: i32) -> String {
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
}

fn main() {
    let result = Solution::c_and_say(5);
    println!("last::{}", result)
}