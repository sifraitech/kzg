use criterion::{criterion_group, criterion_main, Criterion};
use kzg_bench::benches::recover::{bench_recover};

use arkworks::kzg_types::FsFr;
use arkworks::kzg_proofs::FFTSettings;
use arkworks::utils::PolyData;

fn bench_recover_(c: &mut Criterion) {
    bench_recover::<FsFr, FFTSettings, PolyData, PolyData>(c);
}

criterion_group!(benches, bench_recover_);
criterion_main!(benches);