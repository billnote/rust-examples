use futures::executor::{block_on, ThreadPoolBuilder};
use futures::task::SpawnExt;
use futures::future::TryFutureExt;

use std::thread;
use std::time::Duration;

// async function
async fn say_hello() {
    println!("Hello, rust async world");
}

struct Song;

impl Song {
    fn new() -> Self {
        println!("new song.");
        Song
    }

    fn song(&self) {
        println!("I'm sing");
    }
}

async fn learn_song() -> Song {
    println!("learn song thread id: {:?}", thread::current().id());
    println!("start learn....");
    thread::sleep(Duration::from_millis(1000));
    println!("finish learn.");

    Song::new()
}

async fn sing_song(song: Song) {
    println!("sing song thread id: {:?}", thread::current().id());
    song.song();
}

async fn dance() {
    println!("dancing thread id: {:?}", thread::current().id());
    println!("I'm dancing");
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.
    futures::join!(f1, f2);
}

fn main() {
    // async function
    let future = say_hello();
    println!("Hello, rust world");
    block_on(future);

    /*
    // async closures
    let closure = async || {
        println!("Hello, async closure!");
    };
    println!("Hello, main.");
    let future = closure();
    println!("Hello, main again.");
    block_on(future);
    */

    // async block
    let future = async {
        println!("Hello, async block");
    };
    println!("Hello, block main.");
    block_on(future);

    let mut pool = ThreadPoolBuilder::new()
        .pool_size(5)
        //.after_start(| x|{println!("{} exec end.", x);})
        .create()
        .expect("Failed to create threadpool");

    block_on(async_main());

    let sing = pool.spawn_with_handle(learn_and_sing()).unwrap();
    let dance = pool.spawn_with_handle(dance()).unwrap();

    block_on(async {
        println!("runner thread id: {:?}", thread::current().id());
        (sing.await, dance.await);
    });

    println!("main thread id: {:?}", thread::current().id());


    let future = async {
        println!("Hello, async block");
    };

    let and = async { Ok::<i32, i32>(1) };
    let and = and.and_then(|x| async move { Ok::<i32, i32>(x + 3) });

    let f_a = futures::future::join(future, and);
    //assert_eq!(f_a.await, Ok(4));
    block_on(f_a);
}
