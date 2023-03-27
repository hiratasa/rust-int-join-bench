use bencher::{benchmark_group, benchmark_main, Bencher};
use itertools::Itertools;
use once_cell::sync::Lazy;
use rand::Rng;
use std::fmt::Write;

const N: usize = 100000;

static VALUES: Lazy<Vec<usize>> = Lazy::new(|| {
    let mut rng = rand::thread_rng();

    (0..N).map(|_| rng.gen_range(0..1000000)).collect()
});

fn to_string_and_join(bench: &mut Bencher) {
    let values = &*VALUES;
    bench.iter(|| -> String {
        values
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    })
}

fn fold_by_to_string_without_capacity(bench: &mut Bencher) {
    let values = &*VALUES;
    bench.iter(|| -> String {
        values
            .iter()
            .map(|s| s.to_string())
            .fold(String::new(), |mut acc, cur| {
                acc.push_str(&cur);
                acc.push_str(" ");
                acc
            })
    })
}

fn fold_by_to_string_with_capacity(bench: &mut Bencher) {
    let values = &*VALUES;
    bench.iter(|| -> String {
        values
            .iter()
            .map(|s| s.to_string())
            // 数値一つが最大6桁 + separator1文字で7文字
            .fold(String::with_capacity(7 * values.len()), |mut acc, cur| {
                acc.push_str(&cur);
                acc.push_str(" ");
                acc
            })
    })
}

fn itertools_join(bench: &mut Bencher) {
    let values = &*VALUES;
    bench.iter(|| -> String { values.iter().join(" ") })
}

fn fold_by_write_without_capacity(bench: &mut Bencher) {
    let values = &*VALUES;
    bench.iter(|| -> String {
        values.iter().fold(String::new(), |mut acc, cur| {
            write!(&mut acc, "{} ", cur).unwrap();
            acc
        })
    })
}

fn fold_by_write_with_capacity(bench: &mut Bencher) {
    let values = &*VALUES;
    bench.iter(|| -> String {
        values
            .iter()
            // 数値一つが最大6桁 + separator1文字で7文字
            .fold(String::with_capacity(7 * values.len()), |mut acc, cur| {
                write!(&mut acc, "{} ", cur).unwrap();
                acc
            })
    })
}

benchmark_group!(
    benches,
    to_string_and_join,
    fold_by_to_string_without_capacity,
    fold_by_to_string_with_capacity,
    itertools_join,
    fold_by_write_without_capacity,
    fold_by_write_with_capacity
);
benchmark_main!(benches);
