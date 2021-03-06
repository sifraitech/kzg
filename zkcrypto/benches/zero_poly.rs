use criterion::{criterion_group, criterion_main, Criterion};
use kzg_bench::benches::zero_poly::bench_zero_poly;
use zkcrypto::zkfr::blsScalar;
use zkcrypto::poly::ZPoly;
use zkcrypto::fftsettings::ZkFFTSettings;

fn bench_zero_poly_(c: &mut Criterion) {
    bench_zero_poly::<blsScalar, ZkFFTSettings, ZPoly>(c);
}

criterion_group!(benches, bench_zero_poly_);
criterion_main!(benches);