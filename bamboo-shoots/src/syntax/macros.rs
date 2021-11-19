#[cfg(test)]
mod tests {
    macro_rules! foo {
        ($l:tt) => {
            bar!($l);
        };
    }

    macro_rules! bar {
        (3) => {
            println!("match!");
        };
        (5) => {
            println!("default!");
        };
    }

    #[test]
    fn macro_test1() {
        foo!(3);
        foo!(5);
    }

    macro_rules! ss {
        ("async") => {
            pub fn test_async(self) {
                println!("async");
            }
        };
        ("sync") => {
            pub fn test_sync(self) {
                println!("sync");
            }
        };
    }

    macro_rules! call_on_self {
        ($self:ident, $F:ident) => {
            $self.$F()
        };
    }

    struct MacroStruct;

    impl MacroStruct {
        pub fn new() -> Self {
            MacroStruct {}
        }

        ss!("async");

        pub fn hello(self) {
            println!("ok");
        }
    }

    #[test]
    fn macro_test2() {
        MacroStruct::new().test_async();
    }
}
