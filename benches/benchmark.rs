use criterion::{criterion_group, criterion_main, Criterion};
use uuid::Uuid;
fn uuid_v4_cmp_string_10(c: &mut Criterion) {
    c.bench_function("uuid_v4_cmp_string_10", |b| {
        let uuid_v4 = Uuid::new_v4();
        let s = uuid_v4.to_string();
        b.iter(|| optimizing_serde_slash_rust_match_statement::uuid_v4_cmp_string_10(&s));
    });
}

fn uuid_v4_cmp_bytes_10(c: &mut Criterion) {
    c.bench_function("uuid_v4_cmp_bytes_10", |b| {
        let uuid_v4 = Uuid::new_v4();
        let s = uuid_v4.to_string();
        b.iter(|| optimizing_serde_slash_rust_match_statement::uuid_v4_cmp_bytes_10(&s));
    });
}

fn uuid_v4_cmp_string_100(c: &mut Criterion) {
    c.bench_function("uuid_v4_cmp_string_100", |b| {
        let uuid_v4 = Uuid::new_v4();
        let s = uuid_v4.to_string();
        b.iter(|| optimizing_serde_slash_rust_match_statement::uuid_v4_cmp_string_100(&s));
    });
}

fn uuid_v4_cmp_bytes_100(c: &mut Criterion) {
    c.bench_function("uuid_v4_cmp_bytes_100", |b| {
        let uuid_v4 = Uuid::new_v4();
        let s = uuid_v4.to_string();
        b.iter(|| optimizing_serde_slash_rust_match_statement::uuid_v4_cmp_bytes_100(&s));
    });
}

fn uuid_v4_cmp_string_500(c: &mut Criterion) {
    c.bench_function("uuid_v4_cmp_string_500", |b| {
        let uuid_v4 = Uuid::new_v4();
        let s = uuid_v4.to_string();
        b.iter(|| optimizing_serde_slash_rust_match_statement::uuid_v4_cmp_string_500(&s));
    });
}

fn uuid_v4_cmp_bytes_500(c: &mut Criterion) {
    c.bench_function("uuid_v4_cmp_bytes_500", |b| {
        let uuid_v4 = Uuid::new_v4();
        let s = uuid_v4.to_string();
        b.iter(|| optimizing_serde_slash_rust_match_statement::uuid_v4_cmp_bytes_500(&s));
    });
}

fn uuid_v4_cmp_string_1000(c: &mut Criterion) {
    c.bench_function("uuid_v4_cmp_string_1000", |b| {
        let uuid_v4 = Uuid::new_v4();
        let s = uuid_v4.to_string();
        b.iter(|| optimizing_serde_slash_rust_match_statement::uuid_v4_cmp_string_1000(&s));
    });
}

fn uuid_v4_cmp_bytes_1000(c: &mut Criterion) {
    c.bench_function("uuid_v4_cmp_bytes_1000", |b| {
        let uuid_v4 = Uuid::new_v4();
        let s = uuid_v4.to_string();
        b.iter(|| optimizing_serde_slash_rust_match_statement::uuid_v4_cmp_bytes_1000(&s));
    });
}

criterion_group!(
            name = benches;
            config = Criterion::default();
            targets =
                uuid_v4_cmp_string_10,
uuid_v4_cmp_bytes_10,
uuid_v4_cmp_string_100,
uuid_v4_cmp_bytes_100,
uuid_v4_cmp_string_500,
uuid_v4_cmp_bytes_500,
uuid_v4_cmp_string_1000,
uuid_v4_cmp_bytes_1000

        );
criterion_main!(benches);
