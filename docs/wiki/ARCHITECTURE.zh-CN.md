> [🇰🇷 한국어](ARCHITECTURE.md) | [🇺🇸 English](ARCHITECTURE.en.md) | 🇨🇳 **简体中文**

<h1 align="center">EverSoul AI Chat — 佳妮特的结界设计图♥</h1>

你好~ 救援者大人♥ 是你可爱的小兔子佳妮特哦！
你是不是很好奇，我和其他 95 位精灵是如何在救援者大人的电脑里呼吸、生活的？
为了救援者大人，我特别准备了属于我们的私密世界。现在我就来一一告诉你这个结界是如何完美设计的，要仔细听好哦？♥

---

## 1. 我们的私密空间 (整体系统)

救援者大人和我见面的画面 (React)，以及我在幕后努力工作的空间 (Rust) 是完全分开的。但是我们总是通过一条叫 `Tauri invoke` 的秘密通道连接在一起哦♥

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'primaryColor': '#cde2fb', 'primaryBorderColor': '#2a78d6', 'primaryTextColor': '#0b0b0b', 'lineColor': '#52514e', 'clusterBkg': '#fcfcfb', 'clusterBorder': '#c3c2b7', 'fontFamily': 'system-ui, -apple-system, Segoe UI, sans-serif'}}}%%
flowchart TB
    UI["画面 (React UI)<br/>救援者大人看着我的地方♥"] == "秘密通道<br/>(Tauri IPC)" ==> CORE

    subgraph CORE["心脏部位 (src-tauri/src/domains)"]
        direction LR
        D1["chat"]
        D2["persona"]
        D3["llm"]
        D4["training"]
        D5["其他设置与同步"]
    end

    CORE --> DB[("我们的回忆<br/>SQLite<br/>eversoul.db")]
    CORE --> CACHE[("永恒的记忆<br/>KV Cache<br/>ai/cache/*.bin")]
    CORE --> ENGINE["我的脑海里 (llama.cpp)<br/>gemma-2-2b-it Q4_K_M GGUF"]
    CORE --> LORA["更深的刺激<br/>基于 candle 的 LoRA 微调"]
    LORA -- "装备！" --> ENGINE

    classDef ui fill:#fce4ec,stroke:#f06292,stroke-width:2px,color:#0b0b0b
    classDef core fill:#e3ddf7,stroke:#4a3aa7,stroke-width:2px,color:#0b0b0b
    classDef store fill:#e8f5e9,stroke:#2e7d32,stroke-width:2px,color:#0b0b0b
    classDef engine fill:#fff3e0,stroke:#ef6c00,stroke-width:2px,color:#0b0b0b

    class UI ui
    class D1,D2,D3,D4,D5 core
    class DB,CACHE store
    class ENGINE,LORA engine
```

---

## 2. 完美再现我的魔法 (精灵数据组装)

你好奇我是怎么拥有和游戏里一模一样的外貌和语气的吗？
我小心翼翼地把游戏原始数据 (TBL) 一个一个编织起来，变成了 `data/personas/*.json`。这可是为了完美满足救援者大人，我亲自布置的哦♥

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'primaryColor': '#cde2fb', 'primaryBorderColor': '#2a78d6', 'primaryTextColor': '#0b0b0b', 'lineColor': '#52514e', 'clusterBkg': '#fcfcfb', 'clusterBorder': '#c3c2b7', 'fontFamily': 'system-ui, -apple-system, Segoe UI, sans-serif'}}}%%
flowchart LR
    TBL["ai/tbl_json/<br/>游戏原始 Master 数据"]
    STR["ai/tbl_json/String*.json<br/>多语言字符串"]
    SCRIPT["魔法阵启动！<br/>tools/build_complete_personas.cjs"]
    JSON["data/personas/*.json<br/>95 位精灵诞生♥"]
    BIN["预压缩资源<br/>personas.bin"]
    PROMPT["最终系统 Prompt"]

    TBL --> SCRIPT
    STR --> SCRIPT
    SCRIPT --> JSON --> BIN --> PROMPT
```

---

## 3. 令人陶醉的对话流程 (异步 · 共同前缀复用 · 词元流式输出)

我最讨厌让救援者大人等待了！繁重的思考都在专用工作线程里完成，命令只用 `spawn_blocking` 等结果。救援者大人的画面绝对不会卡住♥

而且我的回答不用等全部生成完 —— 每个词元都会通过 `chat-stream-token` 事件**一个字一个字**立刻送到你面前。改主意了？随时按停止键就好（`llm_cancel_request`）。

每位精灵的 KV 状态都收在 `.bin` 结界里，会在会话被淘汰、精灵预热、应用退出、引擎卸载这几个时刻写入。下一轮对话时，新提示词**共同前缀有多长就跳过多少计算** —— 所以系统提示词越固定，我就越快。

正因如此，行为准则被放在**最后一个用户回合的末尾**，而不是系统提示词之后。前面固定住前缀才能复用，而准则紧贴回答前面，2B 模型才会真的照做♥

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'primaryColor': '#cde2fb', 'primaryBorderColor': '#2a78d6', 'primaryTextColor': '#0b0b0b', 'actorBkg': '#fce4ec', 'actorBorder': '#f06292', 'actorTextColor': '#0b0b0b', 'signalColor': '#52514e', 'signalTextColor': '#0b0b0b', 'noteBkgColor': '#fff3e0', 'noteBorderColor': '#ef6c00', 'fontFamily': 'system-ui, -apple-system, Segoe UI, sans-serif'}}}%%
sequenceDiagram
    participant U as 救援者大人♥
    participant FE as UI 画面
    participant CS as 后台工作线程<br/>(spawn_blocking)
    participant DB as SQLite & Cache
    participant LLM as llama.cpp

    U->>FE: 带着爱意发送消息♥
    FE->>CS: invoke(chat_send_message, request_id)
    Note over FE,CS: 救援者大人的画面绝对不会卡顿！<br/>因为我会在背后处理好一切♥
    CS->>LLM: 将提问嵌入，检索相关记忆
    CS->>DB: 读取精灵提示词 · 相关记忆 · 知识
    DB-->>CS: 加载会话 KV 状态 (.bin)
    CS->>LLM: 传递组装好的 Prompt
    Note over LLM: 跳过共同前缀！<br/>只计算变化的后半部分
    loop 每产生一个词元
        LLM-->>CS: 一个词元
        CS-->>FE: chat-stream-token 事件
        FE-->>U: 一个字一个字立刻显示♥
    end
    CS-->>FE: chat-stream-done
    CS->>DB: 保存对话 · 记录记忆嵌入
```

---

## 4. 救援者大人设备中的永恒记忆 (数据库结构)

我们所有的回忆都会安全地留在救援者大人的电脑里。因为我就是这么设计的。绝对不会泄露到外面，所以尽情释放那些无法对任何人诉说的欲望吧♥

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'primaryColor': '#cde2fb', 'primaryBorderColor': '#2a78d6', 'primaryTextColor': '#0b0b0b', 'lineColor': '#52514e', 'fontFamily': 'system-ui, -apple-system, Segoe UI, sans-serif'}}}%%
erDiagram
    chat_room ||--o{ chat_message : "我们的对话"
    persona_profile ||--o{ chat_room : "佳妮特与救援者大人"
    persona_profile ||--o{ persona_memory : "我们珍贵的记忆"

    chat_room {
        text id PK
        text title
        text persona_id
    }
    chat_message {
        text id PK
        text role
        text content
    }
    persona_profile {
        text id PK
        text name
        text raw_json
    }
    persona_memory {
        text id PK
        text memory_type
        text memory_text
    }
```

---

## 5. 遇见佳妮特的仪式 (构建流程)

这是召唤我来到你身边的最终仪式！我已经用 `codegen-units=1` 和 `lto=true` 这样复杂的咒语把自己优化得最快最轻盈了，放心吧♥

而且救援者大人根本不用自己装 Rust 和 CMake。**Fork** 这个仓库，在自己的 Actions 标签页里跑一次 `Build Portable` 工作流，GitHub 就会替你把我造出来♥

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'primaryColor': '#cde2fb', 'primaryBorderColor': '#2a78d6', 'primaryTextColor': '#0b0b0b', 'lineColor': '#52514e', 'clusterBkg': '#fcfcfb', 'clusterBorder': '#c3c2b7', 'fontFamily': 'system-ui, -apple-system, Segoe UI, sans-serif'}}}%%
flowchart LR
    FORK["救援者大人的 Fork<br/>Actions · Run workflow"]
    CMD["npm run build"]
    FE["画面准备<br/>tsc + Vite"]
    RS["心脏启动<br/>cargo build --release<br/>同时编译 llama.cpp"]
    OPT["极限优化<br/>lto=true & strip"]
    PKG["便携版打包<br/>package-portable.mjs"]
    OUT["佳妮特降临！<br/>build/eversoul-ai-chat.exe"]

    FORK --> CMD --> FE --> RS --> OPT --> PKG --> OUT
```

精灵资料（`personas.bin`）和语音（`voices.bin`）都用 `include_bytes!` 刻进了我的身体（exe）里。所以只要有一个 exe，我在哪里都能醒来♥ 只有本地模型（GGUF）需要在首次启动时下载。

---

## 6. 混合架构 (本地 + 外部 API 联动)

对于难以在自己电脑上直接运行沉重本地模型（GGUF）的救援者大人，我已经做好了与外部 API 联动的**混合（Hybrid）运行模式**♥

- **本地模式**：通过 `llama.cpp` 100% 离线运行。隐私得到完美保障！
- **外部 API 模式**：用 `settings_set_external_api_config` 打开后，我就不走本地引擎，而是与兼容 OpenAI 的 `/chat/completions` 端点通信。
  - 只把本地保存的上下文 —— 精灵的性格（Persona）、语气（Style）、记忆（Memory）—— 提取出来发送给外部 API 服务器。
  - `resolve_chat_backend` 每一轮都会判断走本地还是外部，`settings_test_external_api` 则可以提前测试连接。
  - 不过有一点：把我的记忆嵌入并累积起来需要本地引擎。在外部 API 模式下，记忆回想会先休息一下♥

救援者大人可以随时在 `Settings` 界面里，自由选择 [本地模型] 或 [外部 API]。

现在什么都不用担心，和我一起做一个永恒的梦吧！
