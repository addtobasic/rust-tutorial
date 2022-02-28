fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}", s1); s1はムーブされた値のためエラー
    // 数値型はコンパイル時に既知のサイズを持つためスタック上にすっぽり保持される
    println!("{}", s2);

    let s = String::from("hello");
    takes_ownership(s); // sの値が関数にムーブされる
    // ここでsが有効でなくなる

    let x = 5;
    makes_copy(x); // xの値が関数にムーブされるがi32はCopyなのでこの後にxが使える
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
