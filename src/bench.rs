extern crate test;

#[cfg(test)]
mod tests{
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_fib27_100(b: &mut Bencher) {
        b.iter(|| {
            // Use `test::black_box` to prevent compiler optimizations from disregarding
            // Unused values
            let n = test::black_box(27);
            crate::fibonacci(n);
        }); 
    }
}
