use std::time::Instant;

fn quad<F>(f: F, a: f64, b: f64, n_intervals: i32) -> f64 
where F: Fn(f64) -> f64
{   
    let eps = (b - a) / n_intervals as f64;
    let sum: f64 = (1..n_intervals-1).map(|i| f(a+eps*i as f64)).sum();

    return (b - a) / n_intervals as f64 * (sum + (f(a) + f(b)) / 2.0);
}

fn main() {

    let n_intervals = 1000_000_000;
    let f = |x: f64| (x*x).sin();

    let start_time = Instant::now();
    let result = quad(f, 0.0, 100.0, n_intervals);
    let elapsed = start_time.elapsed();
    
    println!("Result: {:.12}, Time: {} ms", result, elapsed.as_millis());
}
