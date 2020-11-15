use demo_fibonacci::{fibonacci_iterative, fibonacci_recursive, fibonacci_recursive_cached};
use xshell::cmd;

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
    let result_fr_js;
    let result_fi_js;

    println!("---");
    time_it!("[Node] Recursive Time", {
        let fn_str = FN.to_string();
        let str_result = cmd!("node fibo_recursive.js {fn_str}").read().unwrap();
        result_fr_js = u128::from_str_radix(&str_result, 10).unwrap();
    });
    time_it!("[Rust] Recursive Time", {
        result_fr = fibonacci_recursive(FN);
    });
    time_it!("[Rust] Recursive Cached Time", {
        result_frc = fibonacci_recursive_cached(FN);
    });

    println!("---");
    time_it!("[Node] Iterative Time", {
        let fn_str = FN.to_string();
        let str_result = cmd!("node fibo_iterative.js {fn_str}").read().unwrap();
        result_fi_js = u128::from_str_radix(&str_result, 10).unwrap();
    });
    time_it!("[Rust] Iterative Time", {
        result_fi = fibonacci_iterative(FN);
    });

    assert!(
        (result_fi == result_fr)
            && (result_frc == result_fr)
            && (result_fr_js == result_fr)
            && (result_fi_js == result_fr)
    );

    println!("---");
    println!("Fibo({}): {}", FN, result_fi);
}
