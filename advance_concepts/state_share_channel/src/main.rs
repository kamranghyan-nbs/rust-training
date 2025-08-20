use std::{sync::mpsc, thread::{self, sleep}, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel::<String>();
    let tx2 = tx.clone();

    // let thread = thread::spawn(move || {
    //     sleep(Duration::from_secs(3));
    //     tx.send("Message 1 from thread 1".to_string()).unwrap();
    //     tx.send("Message 2 from thread 1".to_string()).unwrap();
    // });

    // let thread2 = thread::spawn(move || {
    //     sleep(Duration::from_secs(3));
    //     tx2.send("Message 1 from thread 2".to_string()).unwrap();
    //     tx2.send("Message 2 from thread 2".to_string()).unwrap();
    // });

    let thread = thread::spawn(move || {
       for i in 1..=5 {
        tx.send(format!("Message {i} from thread.")).unwrap();
       }

       println!("Finished sending, thread terminating.");
    });

    // Blocking receive
    // let msg = rx.recv().unwrap();
    // println!("Received message: {}", msg);

    // Non-blocking receive
    // for i in 0..10_000 {
    //     let possible_message = rx.try_recv();
    //     match possible_message {
    //         Ok(msg) => println!("Received message in iteration {i}: {}", msg),
    //         Err(_) => (),
    //     }
    // }

    // Timeset 
    // let possible_message = rx.recv_timeout(Duration::from_secs(5));
    // match possible_message {
    //     Ok(msg) => println!("Received message before timeout: {}", msg),
    //     Err(_) => println!("No message timeout"),
    // }

    // Receiver itrator
    // for msg  in rx {
    //     println!("Received message: {}", msg);
    // }
    // println!("Receive iteration terminated.");

    // Async unbounded 
    sleep(Duration::from_secs(3));
    for m in rx.iter() {
        println!("Got msg in loop: {}", m);
    }
    println!("Rx loop termindated");

    thread.join().unwrap();
    // thread2.join().unwrap();
}
