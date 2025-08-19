use std::{thread, sync::Arc, sync::Mutex, time::Duration};

fn main() {
    // let data = Arc::new(vec![1,2,3,4,5]);
    let data = Arc::new(Mutex::new(vec![1,2,3,4,5]));
    let mut handles = vec![];

    for i in 0..=3 {

        let data_clone = Arc::clone(&data);

        let handle = thread::spawn(move || {
            let mut data = data_clone.lock().unwrap();
            data.push(i);
            // println!("Print data in thread {}: {:?}", i, data_clone);
        });

        // let handle = thread::spawn(move || {
        //     if i == 3 {
        //         let mut data = data_clone.write().unwrap();
        //         println!("Writting to data {:?} in thread {} ...", data, i);

        //         thread::sleep(Duration::sleep(1));
        //         data.push(i);
        //         println!("Done!");
        //     } else {
        //         let mut data = data_clone.read().unwrap();
        //         println!("Reading data {:?} in thread {} ...", data, i);
        //         thread::sleep(Duration::sleep(1));
        //     }
            
        // });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Print data in main: {:?}", data);
}
