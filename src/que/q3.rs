use proconio::input;

pub fn run() {
    input! {
        n: i64,
    }
    
    if n % 2 == 0 {
        println!("Even");
    } else if n % 2 == 1 {
        println!("Odd");
    }
}
