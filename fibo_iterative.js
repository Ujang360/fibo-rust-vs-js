var fibo_n = process.argv.slice(2)[0];

function fibo_iterative(n) {
    var a;
    var b = 0;
    var c = 1;
    var i = 1;

    while (i < n) {
        a = b;
        b = c;
        c = a + b;
        i += 1;
    }

    return c;
}

console.log(fibo_iterative(fibo_n));
