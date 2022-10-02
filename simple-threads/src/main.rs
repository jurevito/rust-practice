use std::vec;
use std::thread;
use std::time::Duration;
use rand::Rng;
use time;

fn main() {
    /* 
    let size = 100_000;
    let mut vals: Vec<i32> = vec![0; size];

    for i in 0..vals.len() {
        vals[i] = rand::thread_rng().gen_range(0..100);
    }


    let sum: i32 = vals.iter().sum();

    //let num = rand::thread_rng().gen_range(0..101);
    println!("Random number: {}", vals[0]);
    println!("Sum: {}", sum);
    */

    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Number {} from thread 1.", i);
            thread::sleep(Duration::from_millis(5));
        }
    });

    for i in 1..5 {
        println!("Number {} from thread 2.", i);
        thread::sleep(Duration::from_millis(5));
    }

    handle.join().unwrap();
}
