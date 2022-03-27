fn main() {
    fn f() -> &str {
        "hello\n"
    }
    let f = || -> &str {
        "world\n"
    };
    println!("{}", crate::m!())

    #[macro_export]
    macro_rules! m {
        () => {
            f()
        };
    }
    println!("{}", crate::m!())
}