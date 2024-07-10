 pub fn run() {
    let mut num = 1;
    loop {
        if num % 15 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{}", num)
        }

        if num == 30 {
             break;
        }
        num += 1;
    }
}


