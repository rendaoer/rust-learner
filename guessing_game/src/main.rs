use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数字！");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("请输入一个数字！");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("读行失败!");

        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("输入的不是一个数字！");
                continue;
            }
        };

        println!("你输入的数字是：{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("你输入的数字小了！"),
            Ordering::Greater => println!("你输入的数字大了！"),
            Ordering::Equal => {
                println!("你输入的数字正确！");
                break;
            }
        }
    }

    println!("游戏结束！");
}
