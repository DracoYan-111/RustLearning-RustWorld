use rand::Rng;
/*use std::io;use std::cmp::Ordering;
use std::os::linux::raw::stat;*/

fn main() {
    /* fn test() -> i32 {
         let a = [2, 2, 6, 4, 5];
         let mut count: i32 = 0;
         for i in 0..5 {
             count = count + a[i];
         }
         count
     }


     println!("游戏开始");
     loop {
         println!("请猜一个数字");
         /*let mut fee = String::new();
         io::stdin().read_line(&mut fee).expect("接收失败");
         let fee: u32 = match fee.trim().parse()
         {
             Ok(num) => num,
             Err(_) => continue,
         };*/

         let ser: i32 = rand::thread_rng().gen_range(1, 101);

         if test() < ser {
             println!("太大了");
         }
         if test() == ser {
             println!("答对了");
             break;
         }
         if test() > ser {
             println!("太小了");
         }
         println!("------{}", ser);
     }
     println!("//////{}", test());

     */

    struct User {
        name: String,
        age: u32,
        sex: String,
        grades: [i32;3],
    }

    let vvv:i32 = rand::thread_rng().gen_range(0, 3);
    let strings: [&str; 3] = ["简单","中等","复杂"];
    let user_test = User {
        name: String::from("RUST"),
        age: rand::thread_rng().gen_range(1, 101),

        sex: String::from(strings[vvv]),
        grades:[rand::thread_rng().gen_range(1, 101),rand::thread_rng().gen_range(1, 101),rand::thread_rng().gen_range(1, 101)],
    };
    println!("user name == {} ", user_test.name);
    println!("user age == {} ", user_test.age);
    println!("user sex == {} ", user_test.sex);

    for i  in 0..3 {
        println!("user grades == {} ",user_test.grades[i]);
    }
}
