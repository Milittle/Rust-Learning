use std::collections::HashMap;
use std::sync::RwLock;

use anyhow::anyhow;
use lazy_static::lazy_static;

lazy_static! {
    static ref GLOBAL_MAP: RwLock<HashMap<String, String>> = RwLock::new({
        let map = HashMap::new();
        map
    });
}

fn main() {
    for i in 0..=3 {
        insert_global_map(i.to_string(), i.to_string());
    }

    print_global_map();

    get_and_remove(0.to_string())
}

#[allow(dead_code)]
fn insert_global_map(k: String, v: String) {
    let mut gpw = GLOBAL_MAP.write().unwrap();
    gpw.insert(k, v);
}

#[allow(dead_code)]
fn print_global_map() {
    let gpr = GLOBAL_MAP.read().unwrap();
    for pair in gpr.iter() {
        println!("{:?}", pair);
    }
}

#[allow(dead_code)]
fn get_and_remove(k: String) {
    println!("execute get_and_remove");
    let v = {
        let gpr = GLOBAL_MAP.read().unwrap();
        let v = gpr.get(&*k.clone());
        match v {
            None => Err(anyhow!("get_and_remove failed")),
            Some(pair) => Ok(pair.to_string().clone()),
        }
    };

    let vstr = v.unwrap();
    println!("get value is {:?}", vstr.clone());
    let mut gpw = GLOBAL_MAP.write().unwrap();
    gpw.remove(&*vstr);
}
