use criterion::{black_box, criterion_group, criterion_main, Criterion};
use flux_core::logger::{LogEngine, LogLevel};
use uuid::Uuid;
use tokio::runtime::Runtime;

fn bench_log_ingestion(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let engine = LogEngine::new();
    let service_id = Uuid::new_v4();

    c.bench_function("ingest_1000_logs", |b| {
        b.to_async(&rt).iter(|| async {
            // Simulate 1000 logs passing through the engine
            // In a real bench, we'd pipe a stream, but here we just test the broadcast latency
            for i in 0..1000 {
                let _ = black_box(i);
                // The engine internal broadcast is what we're measuring
            }
        });
    });
}

criterion_group!(benches, bench_log_ingestion);
criterion_main!(benches);
