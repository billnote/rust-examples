pub fn say_hello() {
    println!("hello, thread!");
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::{thread, time};

    #[test]
    fn thread_test() {
        println!("++++++ thread test!");
        // 线程创建方式：
        // 1. spawn
        let new_spawn_thread = thread::spawn(move || {
            println!("I am a new thread.");
        });
        // 等待新线程执行完毕
        new_spawn_thread.join().unwrap();

        // 2. builder
        let new_builder_thread = thread::Builder::new().name("builder thread".to_string()).stack_size(4 * 1024 * 1024).spawn(move || {
            println!("I am builder thread");
        });
        new_builder_thread.unwrap().join().unwrap();
        // 通过builder创建线程可以指定名称和栈大小
    }

    #[test]
    fn channel_test() {
        println!("++++++ channel test");
        // 通道类型：同步通道，异步通道
        // 1. 同步通道，一个同步发送者，一个接收者
        let (stx, rx): (mpsc::SyncSender<String>, mpsc::Receiver<String>) = mpsc::sync_channel(0);
        let stx_thread = thread::spawn(move || {
            println!("1. start send");
            stx.send("hello sync channel".to_string()).unwrap();
            println!("2. end send");
        });
        thread::sleep(time::Duration::from_secs(1));
        println!("3. receive: {}", rx.recv().unwrap());
        // 等待子线程结束
        stx_thread.join().unwrap();
        // result:
        // 1. start send
        // 3. receive: hello sync channel
        // 2. end send

        // 2. 异步通道，一个异步发送者，一个接收者
        let (tx, rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
        let tx_thread = thread::spawn(move || {
            println!("1. start send");
            tx.send("hello channel".to_string()).unwrap();
            println!("2. end send");
        });
        thread::sleep(time::Duration::from_secs(1));
        println!("3. receive: {}", rx.recv().unwrap());
        // 等待子线程结束
        tx_thread.join().unwrap();
        // result:
        // 1. start send
        // 2. end send
        // 3. receive: hello channel

        // 异步通道和同步通道有何不同那？
        // 从例子中我们可以看到以下两点不同：
        // 1. 同步通道生成方法需要指定一个i32值，该值表示可缓存内容数量
        // 2. 输出结果步骤不同：同步为1->3->2，异步为1->2->3
        // 为何有上述不同那？这是同步与异步通道的最大区别
        // 即：
        // 1. 同步通道需要指定缓存内容数量，而异步通道在内存大小允许的情况下
        // 可无限缓存内容。
        // 2. 当同步通道缓存被用完时，发送者会被堵塞。
        // 所以使用同步通道时，要注意对消息及时接收，不然可能导致进程阻塞。

        // 在看几个问题：
        // 1.通道能保证消息的顺序吗?是否先发送的消息,先接收?
        // 2.通道的发送者和接收者支持N:1,1:N,N:N模式吗?
        // 3.通道能发送任何数据吗?
        // 4.发送后的数据,在线程中继续使用没有问题吗?

        // 看例子吧：
        let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();
        for i in 0..5 {
            let thread_tx = tx.clone();
            thread::spawn(move || {
                thread_tx.send(i).unwrap();
                println!("send {}.", i);
            });
        }

        thread::sleep(time::Duration::from_secs(1));
        for _ in 0..5 {
            println!("receive: {}", rx.recv().unwrap());
        }
        // result:
        // send 0.
        // send 2.
        // send 3.
        // send 1.
        // send 4.
        // receive: 0
        // receive: 2
        // receive: 3
        // receive: 1
        // receive: 4
        // 1. 消息是有序的
        // 2. 发送者可以通过`clone`方式，实现多个发送者，但是接受者只能有一个。N:1
        // 3. 通道只能发送实现了`send` trait 的数据。
        // 4. 发送数据后，在发送线程中可以继续使用。

        // 几种接收方式：
        // recv, try_recv, recv_timeout
        // 一个例子：发送者还未发送请求，接收者就接收消息
        let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();
        // println!("receive: {}", rx.recv().unwrap());
        // thread::spawn(move || {
        //     tx.send(1).unwrap();
        // });
        // `recv`方式接收，线程阻塞一直等待消息

        // println!("receive: {}", rx.try_recv().unwrap());
        // thread::spawn(move || {
        //     tx.send(1).unwrap();
        // });
        // `try_recv` panicked, 因为读取到空值
        // thread 'parallel::threads::tests::channel_test' panicked at
        // 'called `Result::unwrap()` on an `Err` value: Empty'

        // println!("receive: {}",
        //          rx.recv_timeout(time::Duration::from_secs(1)).unwrap());
        // thread::spawn(move || {
        //     tx.send(1).unwrap();
        // });
        // `recv_timeout` panicked, 超时
        // thread 'parallel::threads::tests::channel_test' panicked at
        // 'called `Result::unwrap()` on an `Err` value: Timeout'

        // 所以在实际使用中，结合实际需求选择合适的接收方法。处理好各种异常情况。

        // 一个常见应用场景：一个线程不断发送消息，一个线程接收消耗消息
        let send_thread = thread::Builder::new()
            .name("send thread".to_string())
            .stack_size(4 * 1024 * 1024)
            .spawn(move || {
                for i in 0..10 {
                    tx.send(i).unwrap();
                    thread::sleep(time::Duration::from_millis(100));
                }
            })
            .unwrap();

        let recv_thread = thread::Builder::new()
            .name("send thread".to_string())
            .stack_size(4 * 1024 * 1024)
            .spawn(move || {
                let mut i = 0;
                while i < 10 {
                    match rx.try_recv() {
                        Ok(m) => {
                            println!("receive: {}", m);
                            i += 1;
                        }
                        Err(_) => println!("no message."),
                    }
                    thread::sleep(time::Duration::from_millis(50));
                }
            })
            .unwrap();

        send_thread.join().unwrap();
        recv_thread.join().unwrap();
    }
}
