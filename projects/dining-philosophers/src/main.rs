use std::sync::mpsc::{Sender, Receiver};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: Sender<String>,
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        // Pick up forks...
        println!("{} is eating...", &self.name);
        thread::sleep(Duration::from_millis(10));
    }

    fn action(&self) {
        let left_fork = self.left_fork.try_lock();
        let left_fork_unused: bool = match left_fork {
            Ok(_) => true,
            Err(_) => false,
        };

        let right_fork = self.right_fork.try_lock();
        let right_fork_unused: bool = match right_fork {
            Ok(_) => true,
            Err(_) => false,
        };

        if left_fork_unused && right_fork_unused {
            self.eat()
        } else {
            self.think();
        };
    }
}

static PHILOSOPHERS: &[&str] =
    &["Socrates", "Hypatia", "Plato", "Aristotle", "Pythagoras"];

fn main() {
    // Create forks
    let mut forks: Vec<Arc<Mutex<Fork>>> = Vec::new();
    for _ in 0..PHILOSOPHERS.len() {
        forks.push(Arc::new(Mutex::new(Fork{})))
    }

    // Create philosophers
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let mut philosophers: Vec<Philosopher> = Vec::new();
    for n in 0..PHILOSOPHERS.len() {
        let philosopher = Philosopher{
            name: PHILOSOPHERS[n].to_string(),
            left_fork: Arc::clone(&forks[if n == 0 {PHILOSOPHERS.len()-1} else {n - 1}]),
            right_fork: Arc::clone(&forks[n]),
            thoughts: mpsc::Sender::clone(&tx),
        };
        philosophers.push(philosopher)
    }

    // Make each of them think and eat 100 times
    let mut handles = vec![];
    for philosopher in philosophers {
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                // println!("{}: {}", philosopher.name, i);
                philosopher.action();
            }
        });
        handles.push(handle);
    }

    // Output their thoughts
    for handle in handles {
        handle.join().unwrap();
    }
    drop(tx);

    for thought in rx {
        println!("{thought}");
    }

    println!("finish.")
}
