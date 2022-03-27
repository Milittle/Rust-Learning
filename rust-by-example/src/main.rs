fn main() {
    fn f() -> &'static str {
        "hello, "
    }
    let f = || -> &'static str { "world\n" };
    println!("{}", crate::m!());
    #[macro_export]
    macro_rules! m {
        () => {
            f()
        };
    }
    println!("{}", crate::m!());
}
