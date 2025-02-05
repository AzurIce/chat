# Chat

> A simple command line tool enables you to interact with LLMs.
>
> 98% code is written by AI.

## Install

```bash
cargo install --git https://github.com/YXHXianYu/chat.git --locked
```

## Usage

```bash
chat 帮我写一段求解斐波那契数列的代码，rust语言
chat Write a code to solve the Fibonacci sequence, rust language
```

## Config

```bash
chat config get
chat config get model
chat config set model deepseek-ai/DeepSeek-V3
chat config get max_history
chat config set max_history 10
```

## History

```bash
chat history get
chat history clear
```