macro_rules! some {
    ($b: block) => {
        for _ in 0..5 {
            $b
        }
    };
}

macro_rules! vec_strs {
    ($($element: expr), *) => {
        {
            let mut v = Vec::new();
            $(
                v.push(format!("{}", $element));
            )*
            v
        }
    };
}

fn main() {
    some!({
        println!("hello world");
    });
    let strs = vec_strs!["hello world", 1.324, 1, true];
    assert_eq!(strs, &["hello world", "1.324", "1", "true"]);

    let _a: ();
}
