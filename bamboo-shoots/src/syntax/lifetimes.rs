pub fn say_hello() {
    println!("hello, lifetimes!");
}

pub fn merge_str<'a, 'b>(line: &'a str, prefix: &'b str) -> &'a str {
    println!("{}", format!("{}.{}", line, prefix));

    let result = "hello lifetimes";
    result
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
}
