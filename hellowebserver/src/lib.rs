use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub enum Message {
    NewJob(Job),
    Terminate,
}

#[allow(dead_code)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Creating ThreadPool
    ///
    /// size: thread size
    ///
    /// # Panics
    ///
    /// `new` When size less than 1, will return Err str.
    pub fn new(size: usize) -> Result<ThreadPool, &'static str> {
        if size > 0 {
            let mut workers = Vec::with_capacity(4);
            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));
            for i in 0..size {
                workers.push(Worker::new(i, Arc::clone(&receiver)));
            }
            Ok(ThreadPool { workers, sender })
        } else {
            return Err("The pool size can not be less 1");
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

#[allow(dead_code)]
pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} is terminated", id);
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

pub mod handler {
    use super::*;
    pub fn handle_stream(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        let get = b"GET /hello HTTP/1.1\r\n";
        stream.read(&mut buffer).unwrap();
        let (status_line, filename) = if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK", "hello.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };
        let contents = fs::read_to_string(filename).unwrap();
        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
