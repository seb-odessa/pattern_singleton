mod singleton {
    use std::sync::{Arc, Mutex, Once, ONCE_INIT};
    use std::mem;

    pub trait Interface {
        fn add(&mut self, value: u32);
        fn exec(&mut self);
        fn get(&self) -> u32;
    }

    #[derive(Clone,Debug)]
    struct Singleton {
        data: Arc<Mutex<u32>>,
    }
    impl Singleton {
        fn new() -> Self {
            Singleton { data: Arc::new(Mutex::new(0)) }
        }
    }
    impl Interface for Singleton {
        fn add(&mut self, value: u32) {
            *self.data.lock().unwrap() += value;
        }
        fn exec(&mut self) {
            self.add(1);
        }
        fn get(&self) -> u32 {
            *self.data.lock().unwrap()
        }
    }

    pub fn instance() -> Box<Interface> {
        static mut SINGLETON: *const Singleton = 0 as *const Singleton;
        static ONCE: Once = ONCE_INIT;
        unsafe {
            ONCE.call_once(|| {
                SINGLETON = mem::transmute(Box::new(Singleton::new()));
            });
            Box::new((*SINGLETON).clone())
        }
    }
}

use std::thread;
use std::time::Duration;
use self::singleton::instance;

fn main() {
    println!("START");
    let _: Vec<_> = (0..50)
                        .map(|i| {
                            thread::spawn(move || {
                                instance().add(i);
                            })
                        })
                        .collect();

    thread::sleep(Duration::from_millis(1000));
    println!("data: {}", instance().get());
    println!("END");
}
