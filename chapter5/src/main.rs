struct Foo {
    x: i32,
}

fn main() {
    // 参照による所有権の借用
    let foo = Foo { x: 42 };
    let f = &foo;
    println!("{}", f.x);
    // f はここでドロップ
    // foo はここでドロップ
}
