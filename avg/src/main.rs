fn main() {
    let mut sum: i64 = 0;
    let mut count: i64 = 0;
    let mut iter = std::env::args();
    iter.next().unwrap();
    for arg in iter {
        sum += arg.parse::<i64>().unwrap();
        count += 1;
    }
    println!("{}", sum / count);
}
