// use ferris_says::say; 
// use std::io::{stdout, BufWriter};

fn main() {
    // let mut str1:String = "Tom".to_string();
    // let mut str2 = String::from("Jerry");
    // let str3 = hello_world(str1);
    // hello_world(str2);
    // println!("{}2",str3);
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}
// fn hello_world(mut x:String)->String {
//     x.push_str("hello world")
// }
