use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx,rx) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("Hello"),
            String::from("from"),
            String::from("first"),
            String::from("thread"),
            ];
            
            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });
        
        thread::spawn(move || {
            let vals = vec![
                String::from("I"),
                String::from("am"),
                String::from("second"),
                String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}