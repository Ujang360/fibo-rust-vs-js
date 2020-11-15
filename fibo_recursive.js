var fibo_n = process.argv.slice(2)[0];

function fibo_recursive(n) {
    if (n < 2) {
        return n;
    }

    return fibo_recursive(n - 1) + fibo_recursive(n - 2);
}

console.log(fibo_recursive(fibo_n));
