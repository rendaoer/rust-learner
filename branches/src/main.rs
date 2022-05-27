fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    } else {
        println!("number was zero");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    loop {
        println!("again!");

        if number == 5 {
            break;
        }
    }

    let mut counter = 0;
    'top: loop {
        println!("couner = {}", counter);
        let mut remainder = 10;
        loop {
            println!("remainder = {}", remainder);
            if remainder == 9 {
                break;
            }
            if counter == 2 {
                break 'top;
            }
            remainder -= 1;
        }
        counter += 1;
    }
    println!("End counter = {}", counter);

    let mut counter = 0;

    let result = loop {
        println!("counter = {}", counter);
        counter += 1;

        if counter == 6{
            break counter % 4 * 15;
        }
    };

    println!("result = {}", result);

    let mut number = 3;

    while number != 0 {
        println!("number is {}", number);

        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for i in a {
        println!("the value is: {}", i);
    }
}
