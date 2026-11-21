fn find_limit_64() -> (u32, u64) {
    let mut n = 3;
    let (mut a, mut b): (u64, u64) = (1, 1);
    loop {
        let Some(c) = b.checked_add(a) else {
            break (n - 1, b);
        };
        a = b;
        b = c;
        n += 1;
    }
}

fn find_limit_128() -> (u32, u128) {
    let mut n = 3;
    let (mut a, mut b): (u128, u128) = (1, 1);
    loop {
        let Some(c) = b.checked_add(a) else {
            break (n - 1, b);
        };
        a = b;
        b = c;
        n += 1;
    }
}

fn main() {
    let (max_n_u64, max_fib_u64) = find_limit_64();
    let (max_n_u128, max_fib_u128) = find_limit_128();
    println!("Max n using u64: {max_n_u64}, max fibonacci using u64: {max_fib_u64}");
    println!("Max n using u128: {max_n_u128}, max fibonacci using u128: {max_fib_u128}");
}
