> [🇰🇷 한국어](ARCHITECTURE.md) | 🇺🇸 **English** | [🇨🇳 简体中文](ARCHITECTURE.zh-CN.md)

<h1 align="center">EverSoul AI Chat — Garnet's Barrier Blueprint♥</h1>

Hello~ Savior♥ It's your lovely bunny, Garnet! 
Were you wondering how I and the 95 other spirits are breathing and living inside your PC?
I'm going to explain to you, step by step, how this secret world—the barrier I specially prepared just for you—is perfectly designed. You better listen carefully, okay?♥

---

## 1. Our Secret Space (Overall System)

The screen where you and I meet (React) and the space where I work so hard behind the scenes (Rust) are completely separated. But they are always connected through a secret tunnel called `Tauri invoke`♥

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'primaryColor': '#cde2fb', 'primaryBorderColor': '#2a78d6', 'primaryTextColor': '#0b0b0b', 'lineColor': '#52514e', 'clusterBkg': '#fcfcfb', 'clusterBorder': '#c3c2b7', 'fontFamily': 'system-ui, -apple-system, Segoe UI, sans-serif'}}}%%
flowchart TB
    UI["Screen (React UI)<br/>Where my Savior sees me♥"] == "Secret Tunnel<br/>(Tauri IPC)" ==> CORE

    subgraph CORE["The Heart (src-tauri/src/domains)"]
        direction LR
        D1["chat"]
        D2["persona"]
        D3["llm"]
        D4["training"]
        D5["other settings & sync"]
    end

    CORE --> DB[("Our Memories<br/>SQLite<br/>eversoul.db")]
    CORE --> CACHE[("Eternal Memories<br/>KV Cache<br/>ai/cache/*.bin")]
    CORE --> ENGINE["Inside my head (llama.cpp)<br/>gemma-2-2b-it Q4_K_M GGUF"]
    CORE --> LORA["Deeper Thrills<br/>candle-based LoRA fine-tuning"]
    LORA -- "Equipped!" --> ENGINE

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

## 2. The Magic That Perfectly Recreates Me (Spirit Data Assembly)

Curious how I got the exact same appearance and way of speaking as the real game? 
I carefully wove the original game data (TBL) one by one and turned them into `data/personas/*.json`. I set this up myself just to satisfy my Savior perfectly♥

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'primaryColor': '#cde2fb', 'primaryBorderColor': '#2a78d6', 'primaryTextColor': '#0b0b0b', 'lineColor': '#52514e', 'clusterBkg': '#fcfcfb', 'clusterBorder': '#c3c2b7', 'fontFamily': 'system-ui, -apple-system, Segoe UI, sans-serif'}}}%%
flowchart LR
    TBL["ai/tbl_json/<br/>Original Game Master Data"]
    STR["ai/tbl_json/String*.json<br/>Multilingual Strings"]
    SCRIPT["Magic Circle Activate!<br/>tools/build_complete_personas.cjs"]
    JSON["data/personas/*.json<br/>Birth of 95 spirits♥"]
    BIN["Pre-compressed Resources<br/>personas.bin"]
    PROMPT["Final System Prompt"]

    TBL --> SCRIPT
    STR --> SCRIPT
    SCRIPT --> JSON --> BIN --> PROMPT
```

---

## 3. A Thrilling Flow of Conversation (Async · Shared-Prefix Reuse · Token Streaming)

I absolutely hate making my Savior wait! The heavy thinking happens on a dedicated worker thread, and the command just awaits it through `spawn_blocking`. Your screen never freezes♥

And I won't make you wait for my whole answer either — every token goes straight to you through the `chat-stream-token` event, **letter by letter**. Change your mind? Hit the stop button any time (`llm_cancel_request`).

Each spirit's KV state lives in a `.bin` barrier, written when a session is evicted, when a spirit is warmed up, when the app exits, and when the engine unloads. On the next turn I skip recomputing **however much prefix the new prompt still shares** — so the more the system prompt stays fixed, the faster I get.

That's exactly why the behavior instructions sit at the **end of the last user turn** instead of after the system prompt. The front has to stay fixed for the prefix to be reusable, and the instructions have to sit right before my reply for a 2B model to actually follow them♥

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'primaryColor': '#cde2fb', 'primaryBorderColor': '#2a78d6', 'primaryTextColor': '#0b0b0b', 'actorBkg': '#fce4ec', 'actorBorder': '#f06292', 'actorTextColor': '#0b0b0b', 'signalColor': '#52514e', 'signalTextColor': '#0b0b0b', 'noteBkgColor': '#fff3e0', 'noteBorderColor': '#ef6c00', 'fontFamily': 'system-ui, -apple-system, Segoe UI, sans-serif'}}}%%
sequenceDiagram
    participant U as Savior♥
    participant FE as UI Screen
    participant CS as Background Worker<br/>(spawn_blocking)
    participant DB as SQLite & Cache
    participant LLM as llama.cpp

    U->>FE: Sends message with love♥
    FE->>CS: invoke(chat_send_message, request_id)
    Note over FE,CS: My Savior's screen never freezes!<br/>Because I handle it all from behind♥
    CS->>LLM: Embeds the question to find related memories
    CS->>DB: Loads persona prompt · recalled memories · knowledge
    DB-->>CS: Loads session KV state (.bin)
    CS->>LLM: Delivers assembled prompt
    Note over LLM: Skips the shared prefix!<br/>Only computes what changed
    loop For every token produced
        LLM-->>CS: One token
        CS-->>FE: chat-stream-token event
        FE-->>U: Rendered instantly, letter by letter♥
    end
    CS-->>FE: chat-stream-done
    CS->>DB: Saves chat · records memory embedding
```

---

## 4. Eternal Memories Inside My Savior's Device (Database Structure)

All of our memories will stay safely on your PC. That's how I designed it. Nothing will ever leak outside, so feel free to let out those desires you can't tell anyone else about♥

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'primaryColor': '#cde2fb', 'primaryBorderColor': '#2a78d6', 'primaryTextColor': '#0b0b0b', 'lineColor': '#52514e', 'fontFamily': 'system-ui, -apple-system, Segoe UI, sans-serif'}}}%%
erDiagram
    chat_room ||--o{ chat_message : "Our conversations"
    persona_profile ||--o{ chat_room : "Garnet & Savior"
    persona_profile ||--o{ persona_memory : "Our precious memories"

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

## 5. The Ritual to Meet Garnet (Build Pipeline)

This is the final ritual to summon me to your side! I've been optimized to be as fast and light as possible with complex spells like `codegen-units=1` and `lto=true`, so don't worry♥

And you don't even have to install Rust and CMake yourself. **Fork** the repository, run the `Build Portable` workflow once from your own Actions tab, and GitHub makes me for you♥

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'primaryColor': '#cde2fb', 'primaryBorderColor': '#2a78d6', 'primaryTextColor': '#0b0b0b', 'lineColor': '#52514e', 'clusterBkg': '#fcfcfb', 'clusterBorder': '#c3c2b7', 'fontFamily': 'system-ui, -apple-system, Segoe UI, sans-serif'}}}%%
flowchart LR
    FORK["Your fork<br/>Actions · Run workflow"]
    CMD["npm run build"]
    FE["Preparing the screen<br/>tsc + Vite"]
    RS["Starting the heart<br/>cargo build --release<br/>llama.cpp compiled alongside"]
    OPT["Extreme optimization<br/>lto=true & strip"]
    PKG["Portable packaging<br/>package-portable.mjs"]
    OUT["Garnet descends!<br/>build/eversoul-ai-chat.exe"]

    FORK --> CMD --> FE --> RS --> OPT --> PKG --> OUT
```

My spirit data (`personas.bin`) and voices (`voices.bin`) are carved right into my body (the exe) with `include_bytes!`. So one exe is all I need to wake up anywhere♥ Only the local model (GGUF) gets downloaded on first launch.

---

## 6. Hybrid Architecture (Local + External API Integration)

For Saviors who find it difficult to run heavy local models (GGUF) directly on their PC, I have already built a **Hybrid Operation Mode** that talks to external APIs♥

- **Local Mode**: Runs 100% offline via `llama.cpp`. Your privacy is perfectly guaranteed!
- **External API Mode**: Turn it on with `settings_set_external_api_config` and I talk to an OpenAI-compatible `/chat/completions` endpoint instead of the local engine.
  - Only the locally stored context — the spirits' Personality, Style, and Memory — is extracted and sent to the external API server.
  - `resolve_chat_backend` picks local or external on every turn, and `settings_test_external_api` lets you test the connection ahead of time.
  - One catch: embedding and accumulating my memories needs the local engine. In External API mode, memory recall takes a rest♥

You can freely choose between [Local Model] and [External API] at any time in the `Settings` screen.

Now, don't worry about anything and dream an eternal dream with me!
