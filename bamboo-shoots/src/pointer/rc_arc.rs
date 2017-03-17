pub fn say_hello() {
    println!("hello, Rc and Arc!");
}

#[cfg(test)]
#[allow(unused_mut)]
mod tests {
    use std::rc::Rc;
    use std::sync::Arc;
    use std::{thread, time};

    fn use_and_ref(x: &i32) {
        println!("& ref x value is:{}", x);
    }
    fn use_rc_ref(x: Rc<i32>) {
        println!("rc ref x value is:{}", x);
    }
    fn use_ref_rc(x: &Rc<i32>) {
        println!("ref rc value is:{}", **x);
        let x1 = x.clone();
        use_rc_ref(x1);
    }

    #[test]
    fn rc_test() {
        println!("++++++ rc test.");
        let mut x: Rc<i32> = Rc::new(100);
        // cannot assign to immutable borrowed content
        // Rc包装起来的类型对象,是不可变的；
        // 即使我们生命为`mut`也无法更改内容
        // *x += 1;

        // rc实质是一个指针，我们可以使用`*`访问对象内容
        println!("rc x value is:{}", *x);

        // 那`Rc`和普通不可变引用有什么区别那？
        let x1 = x.clone();
        let x2 = x1.clone();
        let x3 = x.clone();
        println!("rc x1 value is:{}", *x1);
        println!("rc x2 value is:{}", *x2);
        println!("rc x3 value is:{}", *x3);

        let y = 100;
        let y1 = &y;
        let y2 = y1;
        let y3 = &y;
        println!("y1 value is:{}", *y1);
        println!("y2 value is:{}", *y2);
        println!("y3 value is:{}", *y3);

        let s = "abcde".to_string();
        let s1 = &s;
        let s2 = s1;
        let s3 = &s;

        println!("s1 value is:{}", *s1);
        println!("s2 value is:{}", *s2);
        println!("s3 value is:{}", *s3);
        // 从上述例子中看，似乎并没啥区别。
        // 我们在看个例子
        let x4 = x.clone();
        let y4 = &y;
        use_rc_ref(x4);
        use_and_ref(y4);

        // use of moved value: `x4` (value used here after move) [E0382]
        // move occurs because `x4` has type `std::rc::Rc<i32>`,
        // which does not implement the `Copy` trait [E0382]
        // `x4`无法在被使用，因为发生了所有权转移
        // 注意一点`Rc`虽本质是一个指针，但是它还是一个没有实现`Copy`
        // 的类型，rust所有权系统对其当然也适用。
        // println!("x4 value is:{}", *x4);
        println!("y4 value is:{}", *y4);

        // 可以通过引用来访问
        use_ref_rc(&x1);
        println!("rc x1 value is:{}", *x1);

        // 有办法修改Rc包裹内容的值吗？
        *Rc::make_mut(&mut x) += 1;
        // x值是多少？x1,x2的值又是多少？
        println!("mut rc x value is:{}", *x); // 101
        println!("mut rc x1 value is:{}", *x1); // 100
        println!("mut rc x2 value is:{}", *x2); // 100

        // 拆解一下`*Rc::make_mut(&mut x) += 1;`
        {
            let xm: &mut i32 = Rc::make_mut(&mut x);
            *xm = *xm + 1;
        }
        println!("mut rc x value is:{}", *x);
    }

    #[test]
    fn arc_test() {
        println!("++++++ arc test!");
        let thread_number = Arc::new(3);
        for _ in 0..5 {
            let child_number = thread_number.clone();
            // move 闭包所有权转移
            thread::spawn(move || {
                println!("thread child number: {}", child_number);
            });
        }

        // 引用是否可以跨线程使用; 不可以。生命周期
        // let thread_string = "thread string".to_string();
        // for _ in 0..5 {
        // let ref_string = &thread_string;
        // borrowed value only lives until here
        // thread::spawn(move || {
        //     println!("thread child string: {}", ref_string);
        // });
        // }

        let thread_string: &'static str = "thread string";
        for _ in 0..5 {
            thread::spawn(move || {
                println!("thread child string: {}", thread_string);
            });
        }

        // 尝试修改ARC内容？
        let thread_number = Arc::new(0);
        for i in 0..5 {
            let mut child_number = thread_number.clone();
            // move 闭包所有权转移
            thread::spawn(move || {
                if i == 3 {
                    // copy 出新内容，内存地址不同
                    *Arc::make_mut(&mut child_number) += 1;
                }
                println!("thread child number: {}, address: {:?}",
                         child_number,
                         &*child_number as *const i32);
                thread::sleep(time::Duration::from_millis(100));
            });
        }

        println!("thread number: {}", thread_number);
        thread::sleep(time::Duration::from_secs(3));
        println!("thread number: {}, address: {:?}",
                 thread_number,
                 &*thread_number as *const i32);
    }
}
