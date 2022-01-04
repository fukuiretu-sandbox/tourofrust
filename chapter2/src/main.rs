fn example_match() {
    let x = 10;

    match x {
        0 => {
            println!("found zero");
        }
        1 | 2 => {
            println!("found 1 or 2");
        }
        3..=9 => {
            println!("found a number 3 to 9 inclusively");
        }
        matched_num @ 10..=100 => {
            println!("found {} number between 10 to 100!", matched_num);
        }
        _ => {
            println!("found something else!");
        }
    }
}

fn example_loop() {
    let mut x = 0;

    let v = loop {
        x += 1;
        if x == 13 {
            break "13を発見";
        }
    };

    println!("loopの戻り値: {}", v)
}

fn example_block() -> i32 {
    let x = 42;
    let v = if x < 42 { -1 } else { 1 };
    println!("ifより: {}", v);

    let food = "ハンバーガー";
    let result = match food {
        "ホットドック" => "ホットドックです",
        _ => "ホットドックではありません",
    };
    println!("食品の識別:{}", result);

    let v = {
        let a = 1;
        let b = 2;
        a + b
    };
    v + 4
}

fn main() {
    // match
    example_match();
    // loop
    example_loop();
    // block
    println!("block result: {}", example_block().to_string());
}
