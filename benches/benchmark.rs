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

const UUID_EQ: &'static str = "05c9a89b-1ee1-4f38-9837-92a20f0e6d30";
const UUID_NEQ: &'static str = "22770c29-0a84-4fb3-bdd8-f8f246cf8686";
fn eq_single_match_string(c: &mut Criterion) {
    c.bench_function("eq_single_match_string", |b| {
        let s = UUID_EQ.to_string();
        b.iter(|| optimizing_serde_slash_rust_match_statement::single_match_string(&s));
    });
}

fn eq_single_match_bytes(c: &mut Criterion) {
    c.bench_function("eq_single_match_bytes", |b| {
        let s = UUID_EQ.to_string();
        b.iter(|| optimizing_serde_slash_rust_match_statement::single_match_bytes(&s));
    });
}

fn neq_single_match_string(c: &mut Criterion) {
    c.bench_function("neq_single_match_string", |b| {
        let s = UUID_NEQ.to_string();
        b.iter(|| optimizing_serde_slash_rust_match_statement::single_match_string(&s));
    });
}

fn neq_single_match_bytes(c: &mut Criterion) {
    c.bench_function("neq_single_match_bytes", |b| {
        let s = UUID_NEQ.to_string();
        b.iter(|| optimizing_serde_slash_rust_match_statement::single_match_bytes(&s));
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
                uuid_v4_cmp_bytes_1000,
                eq_single_match_string,
                eq_single_match_bytes,
                neq_single_match_string,
                neq_single_match_bytes
);
criterion_main!(benches);
