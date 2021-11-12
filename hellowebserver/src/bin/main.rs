use hellowebserver::handler;
use hellowebserver::ThreadPool;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handler::handle_stream(stream);
        })
    }
}
