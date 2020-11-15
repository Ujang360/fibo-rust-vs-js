use demo_fibonacci::{fibonacci_iterative, fibonacci_recursive, fibonacci_recursive_cached};

const FN: u128 = 44;

macro_rules! time_it {
    ($context:literal, $($tt:tt)+) => {
        let timer = std::time::Instant::now();
        $(
            $tt
        )+
        println!("{}: {:?}", $context, timer.elapsed());
    }
}

fn main() {
    let result_fr;
    let result_fi;
    let result_frc;

    time_it!("Recursive Time", {
        result_fr = fibonacci_recursive(FN);
    });
    time_it!("Iterative Time", {
        result_fi = fibonacci_iterative(FN);
    });
    time_it!("Recursive Cached Time", {
        result_frc = fibonacci_recursive_cached(FN);
    });

    assert!((result_fi == result_fr) && (result_frc == result_fr));
    println!("Fibo({}): {}", FN, result_fi);
}
