fn main() {
    let n: u64 = 42;
    println!("Iterative fibonacci ({}): {}", n, fib_iter(n));
    println!("Recursive fibonacci ({}): {}", n, fib_recurs(n));
}

fn fib_recurs(n: u64) -> u64 {
    if n == 1 || n == 2 {
        return 1;
    }

    return fib_recurs(n-1) + fib_recurs(n-2);
} 


fn fib_iter(n: u64) -> u64 {
    let mut a: u64 = 1;
    let mut b: u64 = 1;

    for _ in 2..n {
        let c = a + b;
        a = b;
        b = c;
    }

    return b;
}