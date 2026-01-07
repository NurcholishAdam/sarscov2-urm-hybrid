use criterion::{black_box, criterion_group, criterion_main, Criterion};
use urm_core::{ConvisulGLUDecoder, URMConfig};

fn benchmark_decoder(c: &mut Criterion) {
    let config = URMConfig::default();
    let decoder = ConvisulGLUDecoder::new(config);
    
    c.bench_function("process_prompt", |b| {
        b.iter(|| {
            decoder.process(
                black_box("What are the Omicron mutations?"),
                black_box("en")
            )
        });
    });
}

criterion_group!(benches, benchmark_decoder);
criterion_main!(benches);
