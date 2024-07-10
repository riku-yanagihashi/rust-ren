mod fizzbuzz;
mod que;
use fizzbuzz::fizzbuzz_loop;
use que::warikake;
use que::q2;
use que::q3;

use proconio::input;


fn main() {
    messe();
    input! {i: String,}

    input(i);

}

fn input(i: String) {
    // fizzbuzzのloopを使用したプログラムを実行
    if i == "loop" {
        println!("{}を実行します", i);
        fizzbuzz_loop::run();
    }
    // 割って掛けるプログラムを実行
    if i == "q1" {
        println!("{}を実行します", i);
        warikake::run();
    }
    // 5以上20未満ならokのプログラム
    if i == "q2" {
        q2::run();
    }
    // EvenOdd処理
    if i == "q3" {
        q3::run();
    }
}

fn messe() {
    print!("何しますか？:");
    // 標準出力をフラッシュして、プロンプトがすぐに表示されるように
    std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
}
