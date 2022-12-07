use advent_of_code_2022::solutions::day_05::{part_1, part_2};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_samples(c: &mut Criterion) {
    let mut group = c.benchmark_group("sample-size-example");
    group.sample_size(10);
    let src = std::fs::read_to_string("inputs/05_xxl.txt").unwrap();

    group.bench_function("Part 1", |bencher| bencher.iter(|| part_1(black_box(&src))));
    // group.bench_function("Part 2", |bencher| bencher.iter(|| part_2(black_box(&src))));
}

criterion_group!(benches, bench_samples);
criterion_main!(benches);
