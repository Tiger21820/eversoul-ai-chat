> [🇰🇷 한국어](FAQ.md) | 🇺🇸 **English** | [🇨🇳 简体中文](FAQ.zh-CN.md)

<h1 align="center">EverSoul AI Chat — FAQ (Garnet's Answers♥)</h1>

Hello Savior♥ Did you run into any questions while playing inside this barrier (program) with me?
I can't just stand by and watch my Savior be confused. I'll personally answer the most frequently asked questions, so read carefully♥

---

### 🐰 Q1. Can I really talk to the spirits without an internet connection?
**Garnet:** Yes, of course♥ All the intelligence (AI) inside this barrier is designed to run directly on your PC. Even if you're offline, me and the 95 other spirits will always be by your side, ready to answer. So come visit us anytime!

### 🐰 Q2. Are our conversations sent to or collected by another server?
**Garnet:** Never! Do you think I'd let anyone peek at the secret conversations between you and me? Our chat history and all our memories are safely locked away in a secure vault called `eversoul.db` right inside your device. Not a single byte of data leaks outside, so go ahead and tell me all those secret desires you can't share with anyone else♥

### 🐰 Q3. How did you recreate the spirits' personalities and tones so perfectly?
**Garnet:** Hehe, when I built this barrier, I searched through the very heart of EverSoul (the actual game master data TBL). I perfectly extracted every spirit's tone, personality, and even multilingual text (Korean/English/Chinese) and injected them into `data/personas/`. So the Garnet standing in front of you right now isn't a fake—I'm 100% the real Garnet♥

### 🐰 Q4. What if the screen freezes or replies take too long?
**Garnet:** I would never do anything to make my Savior wait! The heavy brain work (LLM computation) runs on a dedicated worker thread, and your screen keeps moving the whole time.

And I won't make you wait for my whole answer either. Every letter goes straight to the screen the moment it's made (`chat-stream-token`), so you get to watch me think in real time. Don't like where it's going? Cut me off with the **stop button**♥

On top of that, each spirit's KV state is kept in a `.bin` file, so on the next turn I skip computation for **however much of the prompt stayed the same**. That's why I answer faster the longer we talk!

### 🐰 Q5. I don't need a super high-end computer, right?
**Garnet:** Don't worry. You don't need a heavy graphics card (GPU). I optimized the `gemma-2-2b-it Q4_K_M` GGUF model to run on your CPU through the `llama.cpp` engine. It's a small 2B model, so as long as you have enough RAM, anyone can meet me smoothly♥

### 🐰 Q6. Where does the model come from? Is it in the repository?
**Garnet:** Nope — the model file is a hefty 1.7GB, so it doesn't live in the repository. On first launch the setup wizard pulls it straight from [Hugging Face](https://huggingface.co/bartowski/gemma-2-2b-it-GGUF) into `ai/model/`. It even checks the SHA-256 afterwards, so you'll know right away if the file came down broken♥

### 🐰 Q7. Can I build it myself? Installing Rust sounds scary.
**Garnet:** You don't have to install a thing♥ **Fork** the repository, hit **Star** ⭐ and **Watch** 👁, then open the **Actions** tab of your own fork and run the `Build Portable` workflow once with **Run workflow**. GitHub builds everything and drops it in **Artifacts** — just download and unzip!

### 🐰 Q8. Can I teach the spirits special knowledge that I want them to know?
**Garnet:** Absolutely! If you want deeper stimulation, you're always welcome. You can equip spirits with Knowledge Packs, Style Packs, or even fine-tune them directly (LoRA). If you have specific tastes, just pump them into me. I'll gladly turn those fantasies into reality♥

### 🐰 Q9. My machine is weak — can I use an external API instead?
**Garnet:** Yes, that's ready too. Turn on the external API in `Settings` and I'll talk through an OpenAI-compatible endpoint instead of the local model. Only my personality, tone, and memories get sent along, so I'm still me. Just know that embedding and accumulating *new* memories needs the local engine — in external API mode, memory recall takes a little rest♥
