pub fn say_hello() {
    println!("hello, unsizedtype!");
}

#[cfg(test)]
#[allow(dead_code)]
#[allow(unused_variables)]
mod tests {
    fn sized_param(ids: [i32; 3], names: [&str; 3]) {}
    fn sized_param2(ids: Vec<i32>, names: Vec<&str>) {}

    // unsized
    // all local variables must have a statically known size [E0277]
    // `[&str]` does not have a constant size known at compile-time [E0277]
    // the trait bound `[&str]: std::marker::Sized` is not satisfied
    // (the trait `std::marker::Sized` is not implemented for `[&str]`) [E0277]
    // fn unsized_param(ids: [i32], names: [&str]) {}
    // 不定长类型不能作为函数参数。

    #[test]
    fn unsized_type_test() {
        println!("++++++ unsized type test");
        // sized
        let mut ids: [i32; 3] = [0; 3];
        ids[0] = 100;
        ids[1] = 101;
        ids[2] = 102;

        let x = &ids[0];

        println!("{}", x);

        let _ids = ids;
        println!("{:?}", _ids);

        // sized
        let mut names: [&str; 3] = [""; 3];
        names[0] = "a";
        names[1] = "b";
        names[2] = "c";

        let n = &names[0];
        println!("{}", n);
        // unsized
        // all local variables must have a statically known size [E0277]
        // `[i32]` does not have a constant size known at compile-time [E0277]
        // the trait bound `[i32]: std::marker::Sized` is not satisfied
        // (the trait `std::marker::Sized` is not implemented for `[i32]`) [E0277]
        // let ids: [i32];
        // 变量不能绑定不定长类型。
        // 变量可以绑定不定长类型引用。
        let ids: &[i32] = &[100, 101, 102];

        // 来看看`str`吧
        // all local variables must have a statically known size [E0277]
        // `str` does not have a constant size known at compile-time [E0277]
        // the trait bound `str: std::marker::Sized` is not satisfied
        // (the trait `std::marker::Sized` is not implemented for `str`) [E0277]
        // str 是不定长的哦，在Rust常用类型里不定长类型应该就这一个吧？
        // 如果发现有新的不定长类型我们在研究吧
        // 所以通常我们使用的都是`str`的引用`&str`
        // let name: str;
    }

    use std::mem::size_of;
    struct MyVec<'a> {
        a: Vec<i32>,
        b: Vec<f64>,
        c: &'a [i32],
        d: [i32],
    }

    #[test]
    fn size_of_test() {
        println!("Vec<i32> size: {}", size_of::<Vec<i32>>());
        println!("Vec<f64> size: {}", size_of::<Vec<f64>>());
        println!("sized slice i32 size: {}", size_of::<&[i32; 3]>());
        println!("unsized slice i32 size: {}", size_of::<&[i32]>());
        println!("MyVec size: {}", size_of::<MyVec>());
        println!("MyVec ref size: {}", size_of::<&MyVec>());

        //dbg!("dyn size: {}", size_of::<&(dyn ToString + Clone)>()); compile error
        dbg!("dyn size: {}", size_of::<&(dyn Send + Sync)>());
    }
}
