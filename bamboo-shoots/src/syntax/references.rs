pub fn say_hello() {
    println!("hello, References!")
}

#[cfg(test)]
#[allow(unused_variables)]
mod tests {
    use super::*;

    #[test]
    fn borrowing_test() {
        println!("++++++ borrowing test");
        let mut x = 100;
        {
            let y = &mut x;
            println!("y is: {}", y);

            // binary assignment operation `+=` cannot be applied
            // to type `&mut {integer}` (cannot use `+=` on type
            // `&mut {integer}`) [E0368]
            // y 只是x的一个可变引用，因此不能直接用以运算符操作。
            // y += 1;
            // 所有需要使用`*`来访问引用的内容
            *y += 1;
            println!("y value is: {}", y);

            // 在大括号内调用x，会出现什么情况那？ 为什么那？
            // cannot borrow `x` as immutable because it is also
            // borrowed as mutable (immutable borrow occurs here) [E0502]
            // 出现错误[E0502]，此处已经有一个可变引用，不能在使用`x`这个可变引用。
            // 具体原因就需要了解可变引用的使用规则了，下述内容会做说明。
            // println!("x value is: {}", x);
        } // 为何这里需要括号那？ 删除会发生什么？

        // 删除括号后，引用操作似乎都正常，但是在println x时出现错误了。
        // 错误仍然是[E0502]，其实产生错误的原因也是一样的。
        // 涉及到可变引用规则、上下文环境及作用域。
        let y = &mut x;
        *y += 1;
        // println!("x value is:{}", x);
    }

    #[test]
    fn borrowing_rule_test() {
        println!("++++++ borrowing rule test");
        // 关于引用的两条规则：
        // 1个或多个不可变引用(&T)；
        // 唯一1个可变引用(&mut T)。
        // 注：当保护可变引用时，不可变引用也是不运行使用的哦。
        let mut x = 100;
        {
            let y = &x;
            let z = &x;
            // 可以使用多个不可变引用
            println!("x is:{}", x);
            println!("y is:{}", y);
            println!("z is:{}", z);
            println!("y is:{}", *y);
            let m = *y;
            println!("m is:{}", m);
        }
        {
            let mut my = &mut x;
            *my += 1;
            println!("my is:{}", *my);

            // cannot borrow `x` as immutable because it is also
            // borrowed as mutable (immutable borrow occurs here) [E0502]
            // 只能存在一个可变引用哦。
            // let x = &x;
        }

        let mut v = vec!["a".to_string(), "b".to_string()];
        for mut s in &mut v {
            s.push_str(" new");
        }
        // 这里为何可以再次被引用？ 这就涉及到作用域的知识点了。
        let v1 = &v;
        println!("vector is:{:?}", v1);

        // 一点扩展
        let n = 100;
        let s = "abcd".to_string();
        let n_ref = &n;
        let s_ref = &s;
        // Ok 没有问题
        let n1 = *n_ref;
        // 错误，cannot move out of borrowed content (cannot move out of
        // borrowed content) [E0507]
        // let s1 = *s_ref;

        // 为何会有上述现象？我们在看一个例子。
        let mut mn = 100;
        let mut mn_ref = &mut mn;
        *mn_ref += 1;
        let mut mn1 = *mn_ref;
        mn1 += 1;
        println!("mn value is:{}", mn_ref);
        println!("mn1 value is:{}", mn1);

        // mn, mn1的值都会是多少那？
        // mn value is:101
        // mn1 value is:102
        // 应该明白了产生上述现象的原因了吧～
        // 被借用或引用的内容是不允许转移所有权的，而`n`是Integer类型，实现了Copy trait
        // 所以不会转移所有权，`let n1 = *n_ref`只不过是Copy了一个新的内容赋值给了n1
    }

    #[test]
    fn scopes_test() {}
}
