# hf-chat-template

基于 Rust 实现的轻量级、高效的大语言模型（LLM）聊天模板库。参考了 [Hugging Face text-generation-inference](https://github.com/huggingface/text-generation-inference/blob/main/router/src/infer/mod.rs) 的优秀设计理念，旨在提供一个简洁、易用的 API，帮助开发者方便地获取和应用 Hugging Face 模型库中预定义的聊天模板。无论您是构建聊天机器人、智能助手还是其他基于 LLM 的应用，`hf-chat-template` 都能让您轻松处理不同模型的特定对话格式，从而专注于核心业务逻辑的开发。

## ✨ 特性

* **轻量高效**：基于 Rust 构建，性能卓越，资源占用低。
* **易于集成**：简洁的 API 设计，方便快速集成到现有项目中。
* **Hugging Face 兼容**：支持直接从 Hugging Face Hub 加载模型所需的 `tokenizer_config.json` 中的聊天模板。
* **自动化模板处理**：自动解析和应用聊天模板，简化多模型适配的复杂度。
* **异步支持**：基于 `tokio` 实现异步操作，提升应用性能。

## 🚀 安装

1.  确保您的开发环境中已安装 Rust 工具链。
2.  在您的 `Cargo.toml` 文件中添加以下依赖：

```toml
[dependencies]
hf-chat-template = { version = "0.3", git = "https://github.com/nosnakeob/hf-chat-template.git" }
```

## 💡 使用示例

```rust
use hf_chat_template::{ChatContext, Role};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // 从 Hugging Face Hub 加载指定模型的聊天模板
    // 例如：Qwen/Qwen2.5-7B-Instruct
    let mut chat_context = ChatContext::new("Qwen/Qwen2.5-7B-Instruct").await?;

    // 添加对话消息
    chat_context.push_msg("你好！"); // 用户消息
    chat_context.push_msg("你好！有什么可以帮助你的吗？"); // 助手消息
    chat_context.push_msg("我想了解一下 Rust 语言。"); // 用户消息

    // 渲染成模型所需的输入字符串
    let prompt = chat_context.render()?;
    println!("{}", prompt);

    // 预期输出 (具体格式取决于模型模板):
    // <|im_start|>system
    // You are Qwen, created by Alibaba Cloud. You are a helpful assistant.<|im_end|>
    // <|im_start|>user
    // 你好！<|im_end|>
    // <|im_start|>assistant
    // 你好！有什么可以帮助你的吗？<|im_end|>
    // <|im_start|>user
    // 我想了解一下 Rust 语言。<|im_end|>
    // <|im_start|>assistant

    Ok(())
}
```

## 🤝 贡献

欢迎各种形式的贡献！如果您有任何建议、发现 Bug 或希望添加新功能，请随时提交 Issue 或 Pull Request。

## 📄 许可证

该项目采用 [MIT 许可证](LICENSE) 。