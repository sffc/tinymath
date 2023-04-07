// Copyright 2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use tinymath::i8_mul_div_128;
use tinymath::i8_mul_div_128_reference;
use tinymath::saturating_i16_mul_div_1024;
use tinymath::saturating_i16_mul_div_1024_reference;

fn bench_i8_mul_div_128(c: &mut Criterion) {
    let mut group = c.benchmark_group("tinymath/muldiv/i8/128");

    let values = [0i8, 1, 2, 3, 4, 5, 10, 16, 20, 32, 40, 63, 64, 65, 90, 127];

    group.bench_function("impl", |b| {
        b.iter(|| {
            for a in black_box(values).iter() {
                for b in black_box(values).iter() {
                    for (p, q) in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
                        let a = a * p;
                        let b = b * q;
                        let _ = i8_mul_div_128(a, b);
                    }
                }
            }
        })
    });

    group.bench_function("ref", |b| {
        b.iter(|| {
            for a in black_box(values).iter() {
                for b in black_box(values).iter() {
                    for (p, q) in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
                        let a = a * p;
                        let b = b * q;
                        let _ = i8_mul_div_128_reference(a, b);
                    }
                }
            }
        })
    });
}

fn bench_saturating_i16_mul_div_1024(c: &mut Criterion) {
    let mut group = c.benchmark_group("tinymath/muldiv/i16/1024");

    let values = [
        0, 1, 2, 3, 10, 16, 20, 63, 64, 65, 90, 128, 511, 512, 1023, 1024, 4000,
    ];

    group.bench_function("impl", |b| {
        b.iter(|| {
            for a in black_box(values).iter() {
                for b in black_box(values).iter() {
                    for (p, q) in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
                        let a = a * p;
                        let b = b * q;
                        let _ = saturating_i16_mul_div_1024(a, b);
                    }
                }
            }
        })
    });

    group.bench_function("ref", |b| {
        b.iter(|| {
            for a in black_box(values).iter() {
                for b in black_box(values).iter() {
                    for (p, q) in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
                        let a = a * p;
                        let b = b * q;
                        let _ = saturating_i16_mul_div_1024_reference(a, b);
                    }
                }
            }
        })
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = bench_i8_mul_div_128, bench_saturating_i16_mul_div_1024
);
criterion_main!(benches);
