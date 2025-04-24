use criterion::{Criterion, black_box, criterion_group, criterion_main};
use hf_hub::api::sync::Api;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

const REPOS: &[&str] = &[
    "Qwen/Qwen2.5-7B-Instruct",
    "deepseek-ai/DeepSeek-R1-Distill-Llama-8B",
];

fn bench_take(c: &mut Criterion) {
    let mut group = c.benchmark_group("take");

    for &repo in REPOS {
        let pth = Api::new()
            .unwrap()
            .model(repo.to_string())
            .get("tokenizer_config.json")
            .unwrap();
        let file = File::open(pth).unwrap();
        let mut json: Value = serde_json::from_reader(BufReader::new(file)).unwrap();

        group.bench_function(repo, |b| b.iter(|| black_box(json["chat_template"].take())));
    }

    group.finish();
}

fn bench_clone(c: &mut Criterion) {
    let mut group = c.benchmark_group("clone");

    for &repo in REPOS {
        let pth = Api::new()
            .unwrap()
            .model(repo.to_string())
            .get("tokenizer_config.json")
            .unwrap();
        let file = File::open(&pth).unwrap();
        let json: Value = serde_json::from_reader(BufReader::new(file)).unwrap();

        group.bench_function(repo, |b| {
            b.iter(|| black_box(json["chat_template"].clone()))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_take, bench_clone);
criterion_main!(benches);
