use bencher::{benchmark_group, benchmark_main, Bencher};
use itertools::Itertools;
use once_cell::sync::Lazy;
use rand::Rng;
use std::io;
use std::io::Write;

const N: usize = 100000;

static VALUES: Lazy<Vec<usize>> = Lazy::new(|| {
    let mut rng = rand::thread_rng();

    (0..N).map(|_| rng.gen_range(0..1000000)).collect()
});

fn itertools_join(bench: &mut Bencher) {
    let values = &*VALUES;
    bench.iter(|| {
        eprintln!("{}", values.iter().join(" "));
    })
}

fn itertools_format(bench: &mut Bencher) {
    let values = &*VALUES;
    bench.iter(|| {
        eprintln!("{}", values.iter().format(" "));
    })
}

fn foreach_print(bench: &mut Bencher) {
    let values = &*VALUES;
    bench.iter(|| {
        values.iter().for_each(|x| {
            eprint!("{} ", x);
        });
        eprintln!();
    })
}

fn itertools_join_with_bufwriter(bench: &mut Bencher) {
    let values = &*VALUES;
    bench.iter(|| {
        let mut stderr = io::BufWriter::new(io::stderr().lock());
        writeln!(stderr, "{}", values.iter().join(" ")).unwrap();
    })
}

fn itertools_format_with_bufwriter(bench: &mut Bencher) {
    let values = &*VALUES;
    bench.iter(|| {
        let mut stderr = io::BufWriter::new(io::stderr().lock());
        writeln!(stderr, "{}", values.iter().format(" ")).unwrap();
    })
}

fn foreach_print_with_bufwriter(bench: &mut Bencher) {
    let values = &*VALUES;
    bench.iter(|| {
        let mut stderr = io::BufWriter::new(io::stderr().lock());
        values.iter().for_each(|x| {
            write!(stderr, "{} ", x).unwrap();
        });
        writeln!(stderr).unwrap();
    })
}

benchmark_group!(
    benches,
    itertools_join,
    itertools_format,
    foreach_print,
    itertools_join_with_bufwriter,
    itertools_format_with_bufwriter,
    foreach_print_with_bufwriter,
);
benchmark_main!(benches);
