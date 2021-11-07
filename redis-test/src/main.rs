extern crate redis;
use redis::Commands;

fn fetch_an_integer() -> redis::RedisResult<isize> {
    // connect to redis
    let client = redis::Client::open(("2400:8902::f03c:92ff:fe51:5871", 6379))?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    let _ : () = con.set("my_key", 42)?;
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    con.get("my_key")
}

//use std::fs::File;

fn main() {
    //let f = File::open("hello.txt").expect("Failed to open the hello.txt");
    //println!("the file is: {:?}", f);
    let s = fetch_an_integer();
    let s = match s {
        Ok(value) => value,
        Err(error) => {
            panic!("the error is: {:?}", error);
        },
    };
    println!("the value is: {}", s);
}
