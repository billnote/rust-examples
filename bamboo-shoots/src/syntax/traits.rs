trait Super {
    fn super_method(&mut self) {
        println!("in default super");
    }
}

trait Sub: Super {
    fn sub_method(&mut self);
}

struct CallSuperFromSub;

impl Super for CallSuperFromSub {
    fn super_method(&mut self) {
        println!("in super");
    }
}

impl Sub for CallSuperFromSub {
    fn sub_method(&mut self) {
        println!("in sub");
        self.super_method();
    }
}

fn copy_move() {
    // copy example
    let mut src = Some(1);
    let mut dest = src; // copy happend here
    if let Some(v) = src.as_mut() {
        *v = 2;
    }
    println!("{:?}", src);
    if let Some(v) = dest.as_mut() {
        *v = 3;
    }
    println!("{:?}", dest);

    // move example
    let mut arr_src = vec![1, 2, 3];
    println!("&src: {:p}", &arr_src);
    println!("&&src: {:p}", &&arr_src);
    let arr_dest = arr_src; // move happend here
    println!("&dest: {:p}", &arr_dest);

    // println!("{:?}", arr_src); // compile error: borrow of moved value
    arr_src = vec![3, 2, 1];
    println!("new &src: {:p}", &arr_src);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_move_test() {
        copy_move();
    }
}
