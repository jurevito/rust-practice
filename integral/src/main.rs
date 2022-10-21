
fn quad<F>(f: F, a: f64, b: f64, n_intervals: i32) -> f64 
where F: Fn(f64) -> f64
{   
    let eps = (b - a) / n_intervals as f64;
    let mut sum: f64 = 0.0;

    for i in 1..n_intervals-1 {
        let x: f64 = a + eps * i as f64;
        sum += f(x);
    }

    return (b - a) / n_intervals as f64 * (sum + (f(a) + f(b)) / 2.0);
}

fn main() {
    let n_intervals = 100_000_000;

    let f = |x: f64| -> f64 { (x*x).sin() };
    let result = quad(f, 0.0, 100.0, n_intervals);
    println!("result: {:.5}", result);
}
