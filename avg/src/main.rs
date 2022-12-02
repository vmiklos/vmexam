fn main() {
    let count = (std::env::args().count() - 1) as i64;
    let mut iter = std::env::args();
    iter.next().unwrap();
    let sum: i64 = iter.map(|arg| arg.parse::<i64>().unwrap()).sum();
    println!("{}", sum / count);
}
