use criterion::{Criterion, black_box, criterion_group, criterion_main};
use hf_hub::api::sync::Api;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

const REPOS: &[&str] = &[
    "Qwen/Qwen2.5-7B-Instruct",
    "deepseek-ai/DeepSeek-R1-Distill-Llama-8B",
];

fn bench_w_bufreader(c: &mut Criterion) {
    let mut group = c.benchmark_group("with_bufreader");

    for repo in REPOS {
        group.bench_function(*repo, |b| {
            let pth = Api::new()
                .unwrap()
                .model(repo.to_string())
                .get("tokenizer_config.json")
                .unwrap();
            
            b.iter(|| {
                let file = File::open(&pth).unwrap();
                let reader = BufReader::new(file);
                let _: Value = serde_json::from_reader(black_box(reader)).unwrap();
            });
        });
    }

    group.finish();
}

fn bench_wo_bufreader(c: &mut Criterion) {
    let mut group = c.benchmark_group("without_bufreader");

    for repo in REPOS {
        group.bench_function(*repo, |b| {
            let pth = Api::new()
                .unwrap()
                .model(repo.to_string())
                .get("tokenizer_config.json")
                .unwrap();

            b.iter(|| {
                let file = File::open(&pth).unwrap();
                let _: Value = serde_json::from_reader(black_box(file)).unwrap();
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_w_bufreader, bench_wo_bufreader);
criterion_main!(benches);
