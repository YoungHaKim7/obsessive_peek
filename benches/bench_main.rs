use criterion::criterion_main;

mod benchmarks;

criterion_main!(benchmarks::std_peekmore::benches);
