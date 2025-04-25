# hf-chat-template

这是一个基于 Rust 实现的轻量级大语言模型聊天模板库。该库参考了 [Hugging Face text-generation-inference](https://github.com/huggingface/text-generation-inference/blob/main/router/src/infer/mod.rs) 的设计，提供了简洁易用的 API 来获取和使用 Hugging Face 模型库中的预定义聊天模板。通过本库，开发者可以轻松构建符合各种大语言模型特定对话格式的应用。

## 安装

1. 确保已安装Rust工具链
2. 在Cargo.toml中添加依赖：

```toml
[dependencies]
hf-chat-template = { version = "0.3", git = "https://github.com/nosnakeob/hf-chat-template.git" }
```