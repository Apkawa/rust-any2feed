/// https://doc.rust-lang.org/book/ch20-02-multithreaded.html
/// https://doc.rust-lang.org/book/ch20-03-graceful-shutdown-and-cleanup.html
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    /// Кладем воркеров, они нужны больше для аккуратной остановки
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
    receiver: Option<Arc<Mutex<mpsc::Receiver<Job>>>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        log::trace!("ThreadPool::new(size={:?})", size);
        // Создаем однонаправленный FIFO очередь
        let (sender, reciver) = mpsc::channel();
        // получатель может быть только один, но его кладем в ссылку и мьютекс и затем размножим.
        let reciver = Arc::new(Mutex::new(reciver));
        let mut pool = ThreadPool {
            workers: Vec::with_capacity(size),
            sender: Some(sender),
            receiver: Some(reciver),
        };
        pool.add_workers(size);
        pool
    }
    fn add_workers(&mut self, num: usize) {
        for i in 0..num {
            let worker = Worker::new(
                i,
                // И множим ссылку на получателя
                Arc::clone(self.receiver.as_ref().unwrap()),
            );
            self.workers.push(worker);
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
        log::trace!("ThreadPool drop");
        // Без этой части кода не будет работать остановка процессов
        // по причине того что Worker.reciver будет вечно ждать сообщений
        // При дропе sender умирает канал reciver
        drop(self.sender.take());
        for worker in &mut self.workers {
            log::trace!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
        drop(self.receiver.take());
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, reciver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        log::debug!("Worker {:?}: started", id);
        // Спавним поток
        let thread = thread::spawn(move // перемещаем ссылку reciver внутрь замыкания
            || loop { // И поток зависает в бесконечном цикле
            let job = reciver.lock().unwrap().recv();
            match job {
                Ok(job) => {
                    log::debug!("Worker {:?}: got a job; executing.", id);
                    job()
                }
                Err(_) => {
                    // Когда дропнули ссылку на sender,
                    // то умирает канал и воркер выходит из чата
                    log::debug!("Worker {:?}: disconnected; shutting down.", id);
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
