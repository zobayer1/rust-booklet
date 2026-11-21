fn f_to_c(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

fn main() {
    println!("{:>6} {:>8}", "°F", "°C");
    println!("{:>6} {:>8}", "----", "------");
    for f in (-40..=220).step_by(20) {
        println!("{f:>6} {:>8.2}", f_to_c(f64::from(f)));
    }
}
