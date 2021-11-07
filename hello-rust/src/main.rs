use std::fs;

#[derive(Debug)]
#[allow(dead_code)]
enum Genders {
    UnSpecified = 0,
    Female = 1,
    Male = 2,
}

#[derive(Debug, Copy, Clone)]
struct UserId(u64);

#[derive(Debug, Copy, Clone)]
struct TopicId(u64);

#[derive(Debug)]
struct User {
    id: UserId,
    name: String,
    genders: Genders,
}

#[derive(Debug)]
struct Topic {
    id: TopicId,
    name: String,
    owner: UserId,
}

// 定义聊天室中可能发生的事件
#[derive(Debug)]
#[allow(dead_code)]
enum Event { 
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String)),
}

fn main() {
    //test_url();
    let is_pi = pi();
    let is_unit1 = not_pi();
    let is_unit2 = {
        not_pi();
    };
    println!("is_pi: {:?}, is_unit1: {:?}, is_unit2: {:?}", is_pi, is_unit1, is_unit2);

    let alice = User { id: UserId(1), name: "Alice".into(), genders: Genders::Female };
    let bob = User { id: UserId(2), name: "Bob".into(), genders: Genders::Male };
    let topic = Topic { id: TopicId(1), name: "rust".into(), owner: UserId(1) };
    let event1 = Event::Join((alice.id, topic.id));
    let event2 = Event::Join((bob.id, topic.id));
    let event3 = Event::Message((alice.id, topic.id, "Hello world!".into()));
    println!("event1: {:?}, event2: {:?}, event3: {:?}", event1, event2, event3);
}

#[allow(dead_code)]
fn test_url() {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";
    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);
    fs::write(output, md.as_bytes()).unwrap();
    println!("Converted markdown has been saved in {}.", output);
}

fn pi() -> f64 {
    3.1415926
}

fn not_pi() -> f64 {
    3.1415926
}