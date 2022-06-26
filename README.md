# slug
A small library for generating [slugs][wikipedia] from unicode strings.

Documentation: https://docs.rs/slug

[wikipedia]: https://en.wikipedia.org/wiki/Semantic_URL#Slug

## Usage
```rust
extern crate slug;
use slug::slugify;

let slug = slugify("Hello world");
```

## Benchmarks

You should run in nighly because the crate uses `criterion` + implicitly `test::benchmark`
```shell
# For example
cargo +nightly bench -- bench_slug
# cargo +nightly bench -- bench_slug_normal
# cargo +nightly bench -- bench_unicode
# ...
```
