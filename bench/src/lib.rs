pub fn bench<T: Fn()>(iterations: usize, test: T) {
    let now = chrono::Utc::now();

    for _i in 0..iterations {
        test();
    }

    let end = chrono::Utc::now();

    let duration = (end - now).num_milliseconds();

    let duration_per_run = duration as f64 / iterations as f64;

    println!(
        "Iterations: {iterations}, total took: {duration}ms, per iteration: {duration_per_run}ms"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fest() {
        bench(100, || {});
    }
}
