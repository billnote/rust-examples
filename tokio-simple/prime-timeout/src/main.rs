extern crate futures;
extern crate futures_cpupool;

use futures::Future;
use futures_cpupool::CpuPool;

const BIG_PRIME: u64 = 15485867;

fn main() {
    // Synchronous
    if is_prime(BIG_PRIME) {
        println!("Prime");
    } else {
        println!("Not prime");
    }

    // Asynchronous
    let pool = CpuPool::new_num_cpus();

    let prime_future = pool.spawn_fn(|| {
        let prime = is_prime(BIG_PRIME);

        let res: Result<bool, ()> = Ok(prime);
        res
    });
    println!("Created the future");
    // do somethings here
    if prime_future.wait().unwrap() {
        println!("Prime");
    } else {
        println!("Not prime");
    }
}

fn is_prime(num: u64) -> bool {
    // 有没有更好的办法判断是否是素数那？
    // 当然是有的
    for i in 2..num {
        if num % i == 0 {
            return false;
        }
    }
    true
}
