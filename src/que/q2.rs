use proconio::input;

pub fn run() {
    input! {
        num: i64,
    }

    if num > 5 && num < 20 {
        println!("Ok");
    } else {
        println!("Err");
    }
}
