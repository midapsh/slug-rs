extern crate criterion;
extern crate slug;

use criterion::{criterion_group, criterion_main, Criterion, black_box};
use slug::slugify;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bench_slug", |b| {
        b.iter(|| black_box(slugify(black_box("My test Slug!!"))));
        b.iter(|| black_box(slugify(black_box("Test Slug2!!"))));
        b.iter(|| black_box(slugify(black_box("Æúűűűű--cool?"))));
        b.iter(|| black_box(slugify(black_box("long long long long      long"))));
    });
    c.bench_function("bench_slug_normal", |b| {
        b.iter(|| black_box(slugify(black_box("My test Slug!!"))));
        b.iter(|| black_box(slugify(black_box("Some other.. slug"))));
        b.iter(|| black_box(slugify(black_box("CAPSLOCK IS AN AUTOPILOT FOR COOL"))));
    });
    c.bench_function("bench_unicode", |b| {
        b.iter(|| black_box(slugify(black_box("Æúűűűű--cool?"))));
        b.iter(|| black_box(slugify(black_box("മലയാലമ്げんまい茶??"))));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
