#[no_mangle]
pub extern "C" fn hello() {
    println!("hello, world!");
}

#[no_mangle]
pub extern "C" fn fib(n: usize) {
    let mut v: Vec<i32> = vec![0; n + 1];
    v[0] = 1;
    v[1] = 1;
    for i in 2..n + 1 {
        v[i] = (v[i - 1] + v[i - 2]) % 10_000;
    }
    println!("{}", v[n]);
}

#[no_mangle]
pub extern "C" fn fib_rec(n: usize) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        let ret = fib_rec(n - 1) + fib_rec(n - 2);
        ret
    }
}

// 竹内関数
#[no_mangle]
pub extern "C" fn tarai(x: i32, y: i32, z: i32) -> i32 {
    if x <= y {
        y
    } else {
        tarai(tarai(x - 1, y, z), tarai(y - 1, z, x), tarai(z - 1, x, y))
    }
}

#[no_mangle]
pub extern "C" fn leibniz(n: usize) {
    let mut sum: f64 = 0.0;
    let mut signum = 1;
    let mut denom = 1.0;
    for _ in 0..n {
        sum += signum as f64 / denom;
        signum *= -1;
        denom += 2.0;
    }
    println!("{}", sum);
}
