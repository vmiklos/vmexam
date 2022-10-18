fn main() {
    let mut sum: f64 = 0.0;
    let mut count: f64 = 0.0;
    let mut iter = std::env::args();
    iter.next().unwrap();
    for arg in iter {
        sum += arg.parse::<f64>().unwrap();
        count += 1.0;
    }
    println!("{:.3}", sum / count);
}
