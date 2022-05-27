use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("THREE_HOURS_IN_SECONDS: {}", THREE_HOURS_IN_SECONDS);

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x is {}", x);
    }

    println!("The value of x is {}", x);

    let guess = "42".parse::<u32>().expect("Not a number!");

    println!("The value of guess is {}", guess);

    let x: f32 = 2.0;
    println!("The value of x is {}", x);

    let y: f64 = 3.0;
    println!("The value of y is {}", y);

    let sum = 5 + 10;
    println!("The value of sum is {}", sum);

    let difference = 95.5 - 4.3;
    println!("The value of difference is {}", difference);

    let product = 4 * 30;
    println!("The value of product is {}", product);

    let floored = 2 / 3;
    println!("The value of floored is {}", floored);

    let remainder = 43 % 5;
    println!("The value of remainder is {}", remainder);

    let t = true;
    println!("The value of t is {}", t);

    let f: bool = false;
    println!("The value of f is {}", f);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!(
        "The value of c is {} and z is {} and heart_eyed_cat is {}",
        c, z, heart_eyed_cat
    );

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of tup is {:?}", tup);
    println!("The value of x is {} and y is {} and z is {}", x, y, z);
    println!(
        "The value of tup.0 is {} and tup.1 is {} and tup.2 is {}",
        tup.0, tup.1, tup.2
    );

    let arr = [1, 2, 3, 4, 5];
    println!("The value of arr is {:?}", arr);

    let a: [u32; 5] = [1, 2, 3, 4, 5];
    println!("The value of a is {:?}", a);

    let a = [3; 5];
    println!("The value of a is {:?}", a);

    let a = [1, 2, 3, 4, 5];
    let a1 = a[0];
    let a2 = a[1];
    println!("The value of a1 is {} and a2 is {}", a1, a2);

    let a = [1, 2, 3, 4, 5];
    println!("Pelase enter an arrary index: ");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let index = input.trim().parse::<usize>().expect("Not a number!");

    let element = a[index];

    println!("The value of element is {}", element);

    another_function0();

    another_function1(32);

    another_function2(3, 'h');

    let x = {
        let y = 3;
        y + 1
    };
    println!("The value of x is {}", x);

    let x = another_function3(1, 3);
    println!("The value of x is {}", x);

    // Hello, world.
}

fn another_function0() {
    println!("Another function.");
}

fn another_function1(x: i32) {
    println!("Another function with x: {}", x);
}

fn another_function2(x: i32, y: char) {
    println!("Another function with x: {} and y: {}", x, y);
}

fn another_function3(x: i32, y: i32) -> i32 {
    // x + y
    x + y
}
