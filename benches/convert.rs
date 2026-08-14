use convert_case::{split, Boundary, Case, Casing};
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_split(c: &mut Criterion) {
    let mut group = c.benchmark_group("split");
    group.measurement_time(core::time::Duration::from_secs(2));
    group.warm_up_time(core::time::Duration::from_secs(1));

    // --- Simple delimiter-based (single boundary) ---
    let snake_short = "hello_world";
    group.bench_with_input("snake_short", &snake_short, |b, input| {
        b.iter(|| split(input, &[Boundary::Underscore]))
    });

    let kebab_short = "hello-world";
    group.bench_with_input("kebab_short", &kebab_short, |b, input| {
        b.iter(|| split(input, &[Boundary::Hyphen]))
    });

    let space_short = "hello world";
    group.bench_with_input("space_short", &space_short, |b, input| {
        b.iter(|| split(input, &[Boundary::Space]))
    });

    // --- Case transitions ---
    let lower_upper = "lowerUpperUpper";
    group.bench_with_input("lower_upper", &lower_upper, |b, input| {
        b.iter(|| split(input, &[Boundary::LowerUpper]))
    });

    let pascal = "GetTotalLength";
    group.bench_with_input("pascal", &pascal, |b, input| {
        b.iter(|| split(input, &[Boundary::LowerUpper]))
    });

    let upper_lower = "ABc";
    group.bench_with_input("upper_lower", &upper_lower, |b, input| {
        b.iter(|| split(input, &[Boundary::UpperLower]))
    });

    // --- Digits ---
    let lower_digit = "abc123";
    group.bench_with_input("lower_digit", &lower_digit, |b, input| {
        b.iter(|| split(input, &[Boundary::LowerDigit]))
    });

    let upper_digit = "ABC123";
    group.bench_with_input("upper_digit", &upper_digit, |b, input| {
        b.iter(|| split(input, &[Boundary::UpperDigit]))
    });

    let digit_lower = "123abc";
    group.bench_with_input("digit_lower", &digit_lower, |b, input| {
        b.iter(|| split(input, &[Boundary::DigitLower]))
    });

    let digit_upper = "123ABC";
    group.bench_with_input("digit_upper", &digit_upper, |b, input| {
        b.iter(|| split(input, &[Boundary::DigitUpper]))
    });

    // --- Acronyms ---
    let acronym = "XMLRequest";
    group.bench_with_input("acronym", &acronym, |b, input| {
        b.iter(|| split(input, &[Boundary::Acronym]))
    });

    // --- Camel-case boundary set (used by Pascal/Camel) ---
    const CAMEL_BOUNDARIES: [Boundary; 6] = [
        Boundary::LowerUpper,
        Boundary::Acronym,
        Boundary::LowerDigit,
        Boundary::UpperDigit,
        Boundary::DigitLower,
        Boundary::DigitUpper,
    ];
    let camel_set = "getTotalLength3D";
    group.bench_with_input("camel_set", &camel_set, |b, input| {
        b.iter(|| split(input, &CAMEL_BOUNDARIES))
    });

    // --- ALL 9 default boundaries — the hot path ---
    const ALL_DEFAULTS: [Boundary; 9] = Boundary::defaults();
    let defaults_mixed = "super_mario-64 game XMLHttpRequest";
    group.bench_with_input("defaults_mixed", &defaults_mixed, |b, input| {
        b.iter(|| split(input, &ALL_DEFAULTS))
    });

    let defaults_realistic = "i'veSeen_the_toughest_around";
    group.bench_with_input("defaults_realistic", &defaults_realistic, |b, input| {
        b.iter(|| split(input, &ALL_DEFAULTS))
    });

    // --- Long strings to stress O(n*b) ---
    let long_snake: String = "abcdef_ghijkl_mnopqr_stuvwx_yz012_34567_89AB_CDEF_GHIJ_".repeat(5);
    let long_snake = long_snake.trim_end_matches('_').to_string();
    group.bench_with_input("defaults_long_snake", &long_snake, |b, input| {
        b.iter(|| split(input, &ALL_DEFAULTS))
    });

    let long_camel: String = {
        let mut s = String::with_capacity(200);
        for i in 0..20 {
            if i % 2 == 0 {
                s.push_str("getTotal");
            } else {
                s.push_str("Length3D");
            }
        }
        s
    };
    group.bench_with_input("defaults_long_camel", &long_camel, |b, input| {
        b.iter(|| split(input, &ALL_DEFAULTS))
    });

    // --- Many boundaries but simple delimiter-only string ---
    let many_boundaries_simple = "hello_world_more_words";
    group.bench_with_input(
        "many_boundaries_simple",
        &many_boundaries_simple,
        |b, input| b.iter(|| split(input, &ALL_DEFAULTS)),
    );

    // --- Worst case: no boundary matches at any position ---
    let no_boundary_matches = "aaaaaaabbbbbbbcccccccdddddd";
    group.bench_with_input("no_boundary_matches", &no_boundary_matches, |b, input| {
        b.iter(|| split(input, &ALL_DEFAULTS))
    });

    // --- Unicode ---
    let unicode_cyrillic = "ПЕРСПЕКТИВА24";
    group.bench_with_input("unicode_cyrillic", &unicode_cyrillic, |b, input| {
        b.iter(|| split(input, &ALL_DEFAULTS))
    });

    // --- Edge case: empty string ---
    let empty = "";
    group.bench_with_input("empty_string", &empty, |b, input| {
        b.iter(|| split(input, &ALL_DEFAULTS))
    });

    group.finish();
}

/// Benchmark the full from/to conversion pipeline (existing benchmark, preserved).
fn bench_conversion(c: &mut Criterion) {
    let mut group = c.benchmark_group("conversion");

    group.bench_function("from_to_all", |b| {
        b.iter(|| {
            let words = vec![
                "iGetUp",
                "and-nothing-gets-me-down",
                "YOUGotItTough",
                "i'veSeen_the_toughest_around",
            ];
            for word in &words {
                for to_case in Case::all_cases() {
                    for from_case in Case::all_cases() {
                        word.from_case(*from_case).to_case(*to_case);
                    }
                    word.to_case(*to_case);
                }
            }
        })
    });

    group.finish();
}

criterion_group!(benches, bench_split, bench_conversion);
criterion_main!(benches);
