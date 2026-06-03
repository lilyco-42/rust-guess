use rand::Rng;
use std::cmp;
use std::io;
fn main() {
    let 随机数: u32 = rand::thread_rng().gen_range(0..=100);
    println!("{}", 随机数);
    loop {
        println!("请输入你的猜测:");
        let mut 用户输入 = String::new();
        io::stdin().read_line(&mut 用户输入).expect("IO 错误");
        let 用户输入: u32 = match 用户输入.trim().parse() {
            Ok(用户输入) => 用户输入,
            Err(_) => {
                println!("异常，可能输入的不是数字或超出u32范围");
                continue;
            }
        };

        match 用户输入.cmp(&随机数) {
            cmp::Ordering::Less => {
                println!("小了")
            }
            cmp::Ordering::Equal => {
                println!("你答对了");
                break;
            }
            cmp::Ordering::Greater => {
                println!("大了")
            }
        }
    }
}
