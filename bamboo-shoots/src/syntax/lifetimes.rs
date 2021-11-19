pub fn say_hello() {
    println!("hello, lifetimes!");
}

pub fn merge_str<'a, 'b>(line: &'a str, prefix: &'b str) -> &'a str {
    println!("{}", format!("{}.{}", line, prefix));

    "hello lifetimes"
}

pub fn static_ref2<T>(t: &'static T) -> &'static T {
    t
}

pub fn static_ref<T: 'static>(t: T) -> T
where
    T: 'static,
{
    t
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Foo<'a> {
        x: &'a str,
    }

    #[test]
    fn lifetimes_test() {
        println!("++++++ lifetimes test.");
        let line = "/home/bill.huang/handsome";

        let r;
        {
            let prefix = "jpg";
            r = merge_str(line, prefix);
            println!("{}", r);
        }

        println!("can use r? r:{}", r);

        let prefix2 = "jpg";
        let r2;
        {
            let line2 = "/home/bill.huang/handsome2";
            r2 = merge_str(line2, prefix2);
            println!("{}", r2);
        }
        // r2 为何能够继续被使用？
        // 这就要涉及到一个特殊的作用域static，line2的完整定义为：
        // let line2: &'static str = "..."
        // static 生命周期跨越整个程序，所以这里自然能够使用。
        println!("can use r? r2:{}", r2);

        // let r3;
        // {
        //     let line3 = String::from("/home/bill.huang/handsome3");
        //     r3 = merge_str(&line3, prefix2);
        //     println!("r3 is:{}", r3);
        // }
        // `line3` does not live long enough
        // (`line3` dropped here while still borrowed)

        let foo;
        {
            let x = "abcde";
            let f = Foo { x: x };
            foo = f.x;
        }

        println!("can use foo? foo:{:?}", foo);
    }

    #[test]
    fn static_test() {
        use rand;

        fn drop_static<T: 'static>(t: T) {
            std::mem::drop(t);
        }

        fn drop_static2<T>(t: &'static T) {
            std::mem::drop(t);
        }

        // 将独占引用降级为共享引用
        fn some_function<T>(some_arg: &mut T) -> &T {
            unimplemented!()
        }

        struct Struct;

        impl Struct {
            // 将独占的 self 引用降级为共享的 self 引用
            fn some_method(&mut self) -> &Self {
                unimplemented!()
            }

            // 将独占的 self 引用降级为共享的 T 引用
            fn other_method<T>(&mut self) -> &T {
                unimplemented!()
            }
        }

        // 但是 "impl trait" 可以作为函数的返回值类型
        fn return_identity() -> impl Fn(&i32) -> &i32 {
            |x| x
        }
        let identity = return_identity();

        let mut strings: Vec<String> = Vec::new();
        for _ in 0..10 {
            if rand::random() {
                // 所有字符串都是随机生成的
                // 并且在运行时动态分配
                let string = rand::random::<u64>().to_string();
                strings.push(string);
            }
        }

        // 这些字符串是所有权类型，所以他们满足 'static 生命周期约束
        for string in strings {
            // 这些字符串是可变的
            //string.push_str("a mutation");
            // 这些字符串都可以被 drop
            drop_static(string); // 编译通过
        }

        // 这些字符串在程序结束之前就已经全部失效了
        // 但是你无法在持有他们的引用了
        println!("i am the end of the program");

        // for s in &strings {
        //     println!("string: {:?}", s)
        // }
    }
}
