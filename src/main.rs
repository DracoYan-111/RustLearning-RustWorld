use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let str = rand::thread_rng().gen_range(1, 101);
    println!("猜数游戏开始");
    println!("答案是：{}", str);
    loop {
        println!("请输入你的数字");
        let mut fee = String::new();
        io::stdin().read_line(&mut fee).expect("接收失败");
        let fee: u32 = match fee.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };
        match fee.cmp(&str) {
            Ordering::Less => {
                println!("太小");
            }
            Ordering::Equal => {
                println!("答对了");
                break;
            }
            Ordering::Greater => {
                println!("太大");
            }
        }
        println!("你的输入是:{}", fee);
    }
}
