#![macro_use]
//trait - Model Access Controller
pub trait Mac {
    fn table_name(&self) -> &str;
    fn columns_default(&self) -> &[&str];
}
//e.g. `def_mac!(TodoMac, "todo", &["id", "title", "description"]);`
macro_rules! def_mac {
    ($s:ident, $table:tt, $cols:expr) => {
        pub struct $s {}
        impl $s {
            const TABLE: &'static str = $table;
            const COLUMS_DEFAULT: &'static [&'static str] = $cols;
            fn new() -> $s {
                $s {}
            }
        }

        impl crate::mac::Mac for $s {
            fn table_name(&self) -> &str {
                Self::TABLE
            }
            fn columns_default(&self) -> &[&str] {
                Self::COLUMS_DEFAULT
            }
        }
    };
}
