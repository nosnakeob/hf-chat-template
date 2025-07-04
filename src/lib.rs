#[macro_use]
extern crate anyhow;

use anyhow::{Error, Result};
use derive_new::new;
use hf_hub::api::tokio::Api;
use minijinja::{Environment, Template};
use minijinja_contrib::pycompat;
use serde::Serialize;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::sync::LazyLock;

/// Environment存在生命周期标注，放置全局避免在ChatContext中处理生命周期问题
static TEMPLATE_ENV: LazyLock<Environment> = LazyLock::new(|| {
    let mut env = Environment::new();
    env.set_unknown_method_callback(pycompat::unknown_method_callback);
    env
});

pub async fn load_template(tokenizer_repo: &str) -> Result<Value> {
    let pth = Api::new()?
        .model(tokenizer_repo.to_string())
        .get("tokenizer_config.json")
        .await?;

    let file = File::open(pth)?;
    let mut json: Value = serde_json::from_reader(BufReader::new(file))?;

    Ok(json["chat_template"].take())
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, new, PartialEq)]
pub struct Message {
    pub role: Role,
    #[new(into)]
    pub content: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChatContext {
    pub messages: Vec<Message>,
    pub add_generation_prompt: bool,
    // qwen3特有
    pub enable_thinking: bool,

    #[serde(skip_serializing)]
    template: Template<'static, 'static>,
}

impl ChatContext {
    pub async fn new(tokenizer_repo: &str) -> Result<Self> {
        let template_str = load_template(&tokenizer_repo)
            .await?
            .as_str()
            .unwrap()
            .to_string();

        Ok(Self {
            messages: vec![],
            add_generation_prompt: true,
            enable_thinking: false,
            template: TEMPLATE_ENV.template_from_str(Box::leak(template_str.into_boxed_str()))?,
        })
    }

    /// 添加消息到对话上下文中  
    /// 发送消息角色根据上一条消息自动切换  
    /// User->Assistant->User->...
    pub fn push_msg(&mut self, content: &str) {
        let role = match self.messages.last() {
            None => Role::User,
            Some(msg) => match msg.role {
                Role::User => Role::Assistant,
                _ => Role::User,
            },
        };
        self.messages.push(Message::new(
            role,
            // 带思考过程只取回答
            content.split("</think>").last().unwrap(),
        ));
    }

    pub fn render(&self) -> Result<String> {
        if self.messages.is_empty() {
            bail!("no messages")
        }

        let ctx = serde_json::to_value(self)?;

        self.template.render(&ctx).map_err(Error::msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn t_push_msg() -> Result<()> {
        let mut template = ChatContext::new("Qwen/Qwen2.5-7B-Instruct").await?;

        template.push_msg("hello");
        template.push_msg("hi");
        template.push_msg("how are you");

        assert_eq!(
            template.messages,
            vec![
                Message::new(Role::User, "hello"),
                Message::new(Role::Assistant, "hi"),
                Message::new(Role::User, "how are you"),
            ]
        );

        Ok(())
    }

    #[tokio::test]
    async fn t_ctx2prompt() -> Result<()> {
        let mut template = ChatContext::new("Qwen/Qwen2.5-7B-Instruct").await?;

        template.push_msg("hello");
        template.push_msg("hi");
        template.push_msg("how are you");

        assert_eq!(
            template.render()?,
            "<|im_start|>system\n\
            You are Qwen, created by Alibaba Cloud. You are a helpful assistant.<|im_end|>\n\
            <|im_start|>user\n\
            hello<|im_end|>\n\
            <|im_start|>assistant\n\
            hi<|im_end|>\n\
            <|im_start|>user\n\
            how are you<|im_end|>\n\
            <|im_start|>assistant\n"
        );

        let mut template = ChatContext::new("Qwen/Qwen3-8B").await?;

        template.push_msg("hello");
        template.push_msg("hi");
        template.push_msg("how are you");

        assert_eq!(
            template.render()?,
            "<|im_start|>user\n\
            hello<|im_end|>\n\
            <|im_start|>assistant\n\
            hi<|im_end|>\n\
            <|im_start|>user\n\
            how are you<|im_end|>\n\
            <|im_start|>assistant\n\
            <think>\n\n</think>\n\n"
        );

        // 带思考过程
        template = ChatContext::new("deepseek-ai/DeepSeek-R1-Distill-Llama-8B").await?;

        template.push_msg("hello");
        template.push_msg("balababa</think>hi");
        template.push_msg("how are you");

        assert_eq!(
            template.render()?,
            "<｜User｜>hello<｜Assistant｜>hi<｜end▁of▁sentence｜><｜User｜>how are you<｜Assistant｜><think>\n"
        );

        Ok(())
    }
}
