use tokio::task;

use std::rc::Rc;

fn use_rc(rc: Rc<()>) {
    // Do stuff w/ rc
}

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        // Force the `Rc` to stay in a scope with no `.await`
        /*
        {
            let rc = Rc::new(());
            use_rc(rc.clone());
        }

        task::yield_now().await;
       */

        let rc = Rc::new(());

        task::yield_now().await;

        use_rc(rc.clone());
    }).await.unwrap();
}

async fn reap() {
    tokio::spawn(async move {
        loop {
            use_rc(Rc::new(()));
        }
    })
}