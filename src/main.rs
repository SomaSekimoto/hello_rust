fn main() {
    for n in 0..10 {
        println!("{n}: Hello, Rust!");
        // NOTE: 文字列中で変数を展開する記法は、古いバージョンのRustだと動かない場合があります
        // その場合は以下に書き換えてください。
        // println!("{}: Hello, Rust!", n);
    }
}
