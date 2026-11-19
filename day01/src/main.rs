const MAX: u32 = 100;

fn main() {
    let x = 5;
    let mut y = 10;
    y += x;

    let x = x * 2;
    let spaces = "   ";
    let spaces = spaces.len();

    let a: i32 = 7;
    let b: i64 = 3;
    let c = a as i64 + b;
    let d = i64::from(a) + b;

    let parity = if c % 2 == 0 { "even" } else { "odd" };
    let z = {
        let t = 3;
        t * t
    };

    let mut n = 0;
    let first_sq_over_50 = loop {
        n += 1;
        if n * n > 50 {
            break n * n;
        }
    };

    let tup: (i32, f64, char) = (1, 2.0, '🦀');
    let (p, q, r) = tup;
    let arr = [0u8; 4];

    println!("{y} {x} {spaces} {c} {d} {parity} {z} {first_sq_over_50} {MAX}");
    println!("{tup:?} {p} {q} {r} {arr:?}");
    println!("{}", add(2, 3));
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
