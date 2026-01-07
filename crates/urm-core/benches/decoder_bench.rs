use criterion::{black_box, criterion_group, criterion_main, Criterion};
use urm_core::{ConvisulGLUDecoder, URMConfig};

fn benchmark_decoder_process(c: &mut Criterion) {
    let config = URMConfig::default();
    let decoder = ConvisulGLUDecoder::new(config);
    
    c.bench_function("decoder_process", |b| {
        b.iter(|| {
            decoder.process(
                black_box("What are the Omicron mutations?"),
                black_box("en")
            )
        });
    });
}

fn benchmark_multilingual_processing(c: &mut Criterion) {
    let config = URMConfig::default();
    let decoder = ConvisulGLUDecoder::new(config);
    
    c.bench_function("multilingual_process", |b| {
        b.iter(|| {
            for lang in &["en", "zh", "es"] {
                decoder.process(
                    black_box("Test query"),
                    black_box(lang)
                ).ok();
            }
        });
    });
}

criterion_group!(benches, benchmark_decoder_process, benchmark_multilingual_processing);
criterion_main!(benches);
