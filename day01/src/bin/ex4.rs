fn collatz(mut n: u64) -> u32 {
    let mut cnt = 0;
    while n > 1 {
        n = if n.is_multiple_of(2) { n / 2 } else { 3 * n + 1 };
        cnt += 1;
    }
    cnt + 1
}

fn main() {
    let mut mx = 0;
    let mut start = 0;
    for i in 1..5_000_001 {
        let steps = collatz(i);
        if steps > mx {
            mx = steps;
            start = i;
        }
    }
    println!("Longest sequence: {mx} starting from {start}");
}
