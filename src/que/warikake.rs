use proconio::input;

pub fn run() {
    input! {
        mut num1: i64,
        mut num2: i64,
    }

    let total = num1 / num2 * num2;

    println!("{}", total);

    
}

