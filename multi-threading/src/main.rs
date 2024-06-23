use std::thread;

fn main() {
    let mut handles = vec![];
    for _ in 1..10 {
        let h = thread::spawn(|| {
            println!("Hi from thread id {:?}", thread::current().id());
        });
        handles.push(h);
    }

    for i in 10..20 {
        let builder = thread::Builder::new().name(format!("mythread{}", i));
        let h = builder
            .spawn(|| {
                println!("Hi from thread id {:?}", thread::current().name().unwrap());
            })
            .unwrap();
        handles.push(h);
    }
    for h in handles {
        h.join().unwrap();
    }
}
