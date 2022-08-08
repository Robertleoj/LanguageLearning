use std::thread;
use std::time::Duration;


fn test1(){
    let mut handles = Vec::new();
    for i in 1..10 {
        // let i = 0;
        handles.push(thread::spawn(move || {
            for j in 1..10 {
                println!("thread {} iter {}", i, j);
                // println!("thread iter {}", j);
                thread::sleep(Duration::from_millis(1));
            }
        }));
    }
    for j in 1..10 {
        println!("main iter {}", j);
        // println!("thread iter {}", j);
        thread::sleep(Duration::from_millis(1));
    }

    handles.into_iter().for_each(|h|{h.join().unwrap()});
    println!("Joined all threads");
}


use std::sync::mpsc;

fn test2(){

    let (tx, rx) = mpsc::channel::<String>();

    for k in 1..10 {
        let tc = tx.clone();

        thread::spawn(move || {
            let mut s = String::new();

            for i in 1..10 {
                s.push_str(format!(" {i}").as_str());
                tc.send(s.clone()).unwrap();
                println!("thread {k}: {s}");
                thread::sleep(Duration::from_millis(1));
            }
        });
    }

    drop(tx);

    for r in rx {
        println!("Got: {}", r);
    }
}


use std::sync::{Mutex, Arc};

fn test3() {
    let m = Arc::new(Mutex::new(0));

    let mut handles = Vec::new();


    for i in 1..10 {
        let m2 = Arc::clone(&m);
        handles.push(thread::spawn(move || {
            println!("Thread {i} incrementing");
            let mut v = m2.lock().unwrap();
            *v += 1;
            thread::sleep(Duration::from_millis(10));
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
    
    let v = m.lock().unwrap();

    println!("m = {:?}", v);

}

fn main() {

    // test1();
    test3();

}
