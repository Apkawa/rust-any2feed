/// https://doc.rust-lang.org/book/ch20-02-multithreaded.html
/// https://doc.rust-lang.org/book/ch20-03-graceful-shutdown-and-cleanup.html
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    /// Кладем воркеров, они нужны больше для аккуратной остановки
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut threads: Vec<Worker> = Vec::with_capacity(size);
        // Создаем однонаправленный FIFO очередь
        let (sender, reciver) = mpsc::channel();
        // получатель может быть только один, но его кладем в ссылку и мьютекс и затем размножим.
        let reciver = Arc::new(Mutex::new(reciver));
        for i in 0..size {
            let worker = Worker::new(
                i,
                // И множим ссылку на получателя
                Arc::clone(&reciver),
            );
            // Если закоментировать эту часть - все будет работать по причине того что
            // thread::spawn замыкает в себя ссылку на reciver
            // но когда родительский процесс умирает - умирают и дети
            // Но без этого не будет работать graceful stop (дать процессу возможность до конца отработать)
            threads.push(worker);
        }
        ThreadPool {
            workers: threads,
            sender: Some(sender),
        }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender
            .as_ref() // as_ref тут нужен чтобы достать в unwrap sender по ссылке, а не по значению (копирование)
            .unwrap()
            // Ставим в очередь задачу
            .send(job)
            .unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("ThreadPool drop");
        // Без этой части кода не будет работать остановка процессов
        // по причине того что Worker.reciver будет вечно ждать сообщений
        // При дропе sender умирает канал reciver
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, reciver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        println!("Worker {id}: started");
        // Спавним поток
        let thread = thread::spawn(move // перемещаем ссылку reciver внутрь замыкания
                || loop { // И поток зависает в бесконечном цикле
                let job = reciver.lock().unwrap().recv();
                match job {
                    Ok(job) => {
                        println!("Worker {id}: got a job; executing.");
                        job()
                    }
                    Err(_) => {
                        // Когда дропнули ссылку на sender,
                        // то умирает канал и воркер выходит из чата
                        println!("Worker {id}: disconnected; shutting down.");
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
