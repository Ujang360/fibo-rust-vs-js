use cached::proc_macro::cached;

pub fn fibonacci_recursive(n: u128) -> u128 {
    if n < 2 {
        return n;
    }

    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}

#[cached]
pub fn fibonacci_recursive_cached(n: u128) -> u128 {
    if n < 2 {
        return n;
    }

    fibonacci_recursive_cached(n - 1) + fibonacci_recursive_cached(n - 2)
}

pub fn fibonacci_iterative(n: u128) -> u128 {
    let mut a;
    let mut b = 0;
    let mut c = 1;
    let mut i = 1;

    while i < n {
        a = b;
        b = c;
        c = a + b;
        i += 1;
    }

    c
}
