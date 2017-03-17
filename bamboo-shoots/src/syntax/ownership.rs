pub fn say_hello() {
    println!("hello, ownership!");
}

pub fn take(v: Vec<i32>) {
    println!("vector is: {:?}", v);
}

pub fn take_ref(v: &Vec<i32>) {
    println!("ref vector is: {:?}", v);
}

pub fn take_string(s: String) {
    println!("String s is: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics_test() {
        println!("++++++ move semantics test");
        let v = vec![1, 2, 3];
        let mv = v;
        // use of moved value: `v` (value used here after move) [E0382]
        // move occurs because `v` has type `std::vec::Vec<i32>`,
        // which does not implement the `Copy` trait [E0382]
        // 错误提示：使用了所有权被转移了的变量；
        // 假如是一个实现了`Copy` trait的变量会是什么结果？
        // println!("vector is: {:?}", v);
        println!("vector is: {:?}.", mv);

        let i: i32 = 100;
        let mi = i;
        // 此处并没有上诉[E0382]错误。为何？
        // 因为i32类型实现了Copy trait，此处并没有发生所有权转移。let mi = i,
        // 只是发生了一次数据拷贝，将i的数据拷贝至mi，i仍然具有所有权。
        println!("i32 value is: {}", i);
        println!("mi32 value is:{}", mi);

        // 定义了一个取得所有权的函数,并尝试在我们把变量作为参数传递给函数之
        // 后使用这个变量时,会发生什么那？
        take(mv);
        // 也会出现[E0382]错误。这种情况通常我们因用引用做为参数，而不是直接使用
        // 参数所有权。
        // use of moved value: `v` (value used here after move) [E0382]
        // move occurs because `v` has type `std::vec::Vec<i32>`,
        // which does not implement the `Copy` trait [E0382]
        // println!("move vector is: {:?}", mv);

        let v2 = vec![4, 5, 6];
        take_ref(&v2);
        // v2 还能继续被使用吗？
        // 当然可以，因为v2的所有权并没有被转移
        println!("vector 2 is: {:?}", v2);

        let v = vec!["a".to_string(), "b".to_string()];
        // 注意此处转移了v的所有权
        for s in v {
            println!("v s is {}", s);
        }
        // use of moved value: `v` (value used here after move) [E0382]
        // move occurs because `v` has type `std::vec::Vec<std::string::String>`
        // , which does not implement the `Copy` trait [E0382]
        // println!("vector is {:?}", v);

        let v2 = vec!["a".to_string(), "b".to_string()];
        for s in &v2 {
            println!("v2 s is {}.", s);
        }

        // v2 还能继续被使用吗？显然是可以的。
        println!("vector 2 is {:?}", v2);

        for s in &v2 {
            // 这里有错误吗？
            // take_string(s);
            // found type `&std::string::String` [E0308]
            // expected type `std::string::String` [E0308]
            // mismatched types (expected struct `std::string::String`,
            // found reference) [E0308]
            // 当我们以引用方式遍历vector，对其元素也同样是引用调用
            println!("v2 s2 is {}.", s);
        }
    }
}
