pub fn run() -> String {
    let base: i64 = 10;
    let limit = base.pow(9);

    let mut res: f64 = 0.0;
    let mut numerator: f64 = 0.0;
    let mut denominator: f64 = 1.0;

    for _i in 0..limit {
        res += numerator / denominator;
        numerator += 1.0;
        denominator += 1.0;
    }
    return res.to_string();
}
