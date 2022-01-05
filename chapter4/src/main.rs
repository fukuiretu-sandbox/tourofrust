struct BagOfHolding<T> {
    // item: T,
    item: Option<T>,
}

fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("正しい値ではありません"))
    }
}

fn main() {
    // let i32_bag = BagOfHolding::<i32> { item: 42 };
    // let bool_bag = BagOfHolding::<bool> { item: true };

    // let float_bag = BagOfHolding { item: 3.14 };

    // let bag_in_bag = BagOfHolding {
    //     item: BagOfHolding { item: "boom!" },
    // };

    // println!(
    //     "{}, {}, {}, {}",
    //     i32_bag.item, bool_bag.item, float_bag.item, bag_in_bag.item.item,
    // );

    // let i32_bag = BagOfHolding::<i32> { item: None };
    // if i32_bag.item.is_none() {
    //     println!("バッグには何もない!")
    // } else {
    //     println!("バッグには何かある！")
    // }

    // let i32_bag = BagOfHolding::<i32> { item: Some(40) };
    // if i32_bag.item.is_none() {
    //     println!("バッグには何もない!")
    // } else {
    //     println!("バッグには何かある！")
    // }
    let result = do_something_that_might_fail(12);
    match result {
        Ok(v) => println!("発見 {}", v),
        Err(e) => println!("Error: {}", e),
    }
}
