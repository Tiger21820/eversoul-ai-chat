use rusqlite::Connection;
use std::time::SystemTime;
use uuid::Uuid;

use super::repositories::ChatRepository;
use super::types::{ChatError, ChatMessage, ChatRoom, SendMessageRequest};
use crate::domains::knowledge::services::KnowledgeService;
use crate::domains::modules::services::ModuleService;
use crate::domains::persona::services::PersonaService;
use crate::domains::style::services::StyleService;
use crate::infrastructure::i18n::pick;
use crate::infrastructure::settings::SettingsManager;

const EPISODIC_INJECT_LIMIT: usize = 5;

const EPISODIC_SEARCH_CANDIDATE_LIMIT: usize = 200;

const PROMPT_HISTORY_LIMIT: usize = 6;

pub const CHAT_RESPONSE_MAX_TOKENS: u32 = 96;

const CONSOLIDATION_INTERVAL: usize = 10;

const CONSOLIDATION_SOURCE_LIMIT: usize = 30;

pub const CONSOLIDATION_MAX_TOKENS: u32 = 200;

pub const EVERTALK_SESSION_TITLE: &str = "EverTalk Session";

pub const CHAT_STREAM_TOKEN_EVENT: &str = "chat-stream-token";

pub const CHAT_STREAM_DONE_EVENT: &str = "chat-stream-done";

pub struct ChatService<'a> {
    conn: &'a Connection,
}

impl<'a> ChatService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn create_chat_room(&self, title: &str, language: &str) -> Result<ChatRoom, ChatError> {
        self.create_chat_session_room(title, None, language)
    }

    pub fn create_chat_session_room(
        &self,
        title: &str,
        persona_id: Option<String>,
        language: &str,
    ) -> Result<ChatRoom, ChatError> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs().to_string())
            .unwrap_or_default();

        let room = ChatRoom {
            id: Uuid::new_v4().to_string(),
            title: title.to_string(),
            persona_id,
            session_started_at: now.clone(),
            created_at: now.clone(),
            updated_at: now,
        };

        ChatRepository::create_room(self.conn, &room)
            .map(|_| room)
            .map_err(|e| ChatError::database(language, &e.to_string()))
    }

    pub fn get_chat_rooms(&self, language: &str) -> Result<Vec<ChatRoom>, ChatError> {
        ChatRepository::list_rooms(self.conn)
            .map_err(|e| ChatError::database(language, &e.to_string()))
    }

    pub fn get_rooms_for_persona(
        &self,
        persona_id: &str,
        language: &str,
    ) -> Result<Vec<ChatRoom>, ChatError> {
        ChatRepository::list_rooms_by_persona(self.conn, persona_id)
            .map_err(|e| ChatError::database(language, &e.to_string()))
    }

    pub fn start_new_room(
        &self,
        persona_id: &str,
        language: &str,
    ) -> Result<ChatRoom, ChatError> {
        self.create_chat_session_room(EVERTALK_SESSION_TITLE, Some(persona_id.to_string()), language)
    }

    pub fn delete_room(&self, room_id: &str, language: &str) -> Result<(), ChatError> {
        ChatRepository::delete_room(self.conn, room_id)
            .map_err(|e| ChatError::database(language, &e.to_string()))
    }

    pub fn delete_message(&self, message_id: &str, language: &str) -> Result<(), ChatError> {
        ChatRepository::delete_message(self.conn, message_id)
            .map_err(|e| ChatError::database(language, &e.to_string()))
    }

    pub fn get_latest_session_room(
        &self,
        persona_id: &str,
        language: &str,
    ) -> Result<Option<ChatRoom>, ChatError> {
        ChatRepository::find_latest_room_by_persona(self.conn, persona_id)
            .map_err(|e| ChatError::database(language, &e.to_string()))
    }

    pub fn get_or_create_evertalk_session_room(
        &self,
        language: &str,
    ) -> Result<ChatRoom, ChatError> {
        if let Some(room) =
            ChatRepository::find_latest_global_session_room(self.conn, EVERTALK_SESSION_TITLE)
                .map_err(|e| ChatError::database(language, &e.to_string()))?
        {
            return Ok(room);
        }

        self.create_chat_session_room(EVERTALK_SESSION_TITLE, None, language)
    }

    pub fn get_room_messages(
        &self,
        room_id: &str,
        language: &str,
    ) -> Result<Vec<ChatMessage>, ChatError> {
        ChatRepository::list_messages(self.conn, room_id)
            .map_err(|e| ChatError::database(language, &e.to_string()))
    }

    pub fn get_room_messages_for_persona(
        &self,
        room_id: &str,
        persona_id: &str,
        language: &str,
    ) -> Result<Vec<ChatMessage>, ChatError> {
        ChatRepository::list_messages_for_persona(self.conn, room_id, persona_id)
            .map_err(|e| ChatError::database(language, &e.to_string()))
    }

    pub fn prepare_message_context(
        &self,
        req: &SendMessageRequest,
        settings: &SettingsManager,
        memory_query_vector: &[f32],
    ) -> Result<(String, Vec<ChatMessage>), ChatError> {
        let language = settings.get_language();
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs().to_string())
            .unwrap_or_default();

        let user_msg = ChatMessage {
            id: Uuid::new_v4().to_string(),
            room_id: req.room_id.clone(),
            persona_id: Some(req.persona_id.clone()),
            role: "user".to_string(),
            content: req.content.clone(),
            created_at: now.clone(),
        };
        ChatRepository::insert_message(self.conn, &user_msg)
            .map_err(|e| ChatError::database(&language, &e.to_string()))?;

        let system_prompt = self.build_persona_base_system_prompt(&req.persona_id, settings)?;

        let history = ChatRepository::list_recent_messages_for_persona(
            self.conn,
            &req.room_id,
            &req.persona_id,
            PROMPT_HISTORY_LIMIT,
        )
        .map_err(|e| ChatError::database(&language, &e.to_string()))?;

        let mut final_history = history.clone();

        let recalled_memories = ChatRepository::search_episodic_memories(
            self.conn,
            &req.persona_id,
            memory_query_vector,
            EPISODIC_INJECT_LIMIT,
            EPISODIC_SEARCH_CANDIDATE_LIMIT,
        )
        .map_err(|e| ChatError::database(&language, &e.to_string()))?;

        if !recalled_memories.is_empty() {
            let mut memory_context = pick(
                &language,
                "[구원자와의 과거 대화 중 지금 대화와 관련된 기억]\n".to_string(),
                "[Memories related to the current conversation from past talks with the Savior]\n"
                    .to_string(),
                "[与当前对话相关的、与救世主过去对话中的记忆]\n".to_string(),
            );
            for (index, note) in recalled_memories.iter().enumerate() {
                memory_context.push_str(&format!("{}. {}\n", index + 1, note));
            }

            let injection_index = final_history.len().saturating_sub(1);
            final_history.insert(
                injection_index,
                ChatMessage {
                    id: "system_memory_recall".to_string(),
                    room_id: req.room_id.clone(),
                    persona_id: Some(req.persona_id.clone()),
                    role: "system".to_string(),
                    content: memory_context,
                    created_at: now.clone(),
                },
            );
        }

        let knowledge_service = KnowledgeService::new(self.conn);
        if let Ok(chunks) = knowledge_service.query_knowledge(&req.content, Some(2)) {
            if !chunks.is_empty() {
                let mut knowledge_context = String::from("[시스템 주입 지식 데이터]\n");
                for (i, chunk) in chunks.iter().enumerate() {
                    knowledge_context.push_str(&format!("{}. {}\n", i + 1, chunk.chunk_text));
                }
                knowledge_context.push_str("위 지식을 바탕으로 자연스럽게 대답할 것.");
                
                let injection_index = final_history.len().saturating_sub(1);
                final_history.insert(injection_index, ChatMessage {
                    id: "system_knowledge_injection".to_string(),
                    room_id: req.room_id.clone(),
                    persona_id: Some(req.persona_id.clone()),
                    role: "system".to_string(),
                    content: knowledge_context,
                    created_at: now.clone(),
                });
            }
        }

        Ok((system_prompt, final_history))
    }

    pub fn build_persona_base_system_prompt(
        &self,
        persona_id: &str,
        settings: &SettingsManager,
    ) -> Result<String, ChatError> {
        let persona_service = PersonaService::new(self.conn);
        let language = settings.get_language();
        let mut system_prompt = persona_service
            .get_assembled_system_prompt(persona_id, &language)
            .map_err(|e| ChatError::database(&language, &e))?;

        let style_service = StyleService::new(self.conn, &language);
        let active_style_id = settings.get_active_style_id();
        if let Ok(style_prompt) =
            style_service.get_assembled_style_prompt(active_style_id.as_deref())
        {
            system_prompt.push_str(&style_prompt);
        }

        if let Ok(module_prompt) = ModuleService::active_prompt(settings) {
            if !module_prompt.trim().is_empty() {
                system_prompt.push_str(&module_prompt);
            }
        }

        let semantic_memory = ChatRepository::get_semantic_memory(self.conn, persona_id)
            .map_err(|e| ChatError::database(&language, &e.to_string()))?;

        if let Some(ref summary) = semantic_memory {
            let memory_header = pick(
                &language,
                "\n[구원자와의 관계에 대해 이 정령이 누적한 기억]\n".to_string(),
                "\n[Memories this spirit has accumulated about the relationship with the Savior]\n".to_string(),
                "\n[这位精灵积累的关于与救世主关系的记忆]\n".to_string(),
            );
            let line = pick(
                &language,
                format!("- (통합 요약) {}\n", summary),
                format!("- (Consolidated summary) {}\n", summary),
                format!("- (综合摘要) {}\n", summary),
            );
            system_prompt.push_str(&memory_header);
            system_prompt.push_str(&line);
        }

        Ok(system_prompt)
    }

    pub fn build_behavior_instruction(language: &str) -> String {
        let target_lang_str = match language {
            "en" => "English",
            "zh_cn" => "Chinese (Simplified)",
            _ => "한국어",
        };

        pick(
            language,
            format!("\n\n[중요 절대 지침]\n\
                1. 너는 AI가 아니라 설정된 정령 캐릭터 본인이다. 어색한 기계적 번역투(예: '저는 ~에 온 지 꽤 오래되었어요', '역할을 맡게 되어 기뻐요')를 절대 피하고 자연스러운 {lang}로 말해라.\n\
                2. 매 턴 똑같은 인사말이나 자기소개를 반복하지 말고, 직전 사용자의 말(대화 맥락)에 직접적으로 반응해라.\n\
                3. 실제 대답을 출력하기 전에 반드시 <think> 태그를 열고 너의 내면의 생각, 감정 변화, 행동 의도를 {lang}로 먼저 작성해라. 생각 과정이 끝나면 </think> 태그를 닫고 대답을 이어가라.\n\
                (형식 예시: <think>구원자가 내 반응을 보고 싶어하는 것 같다.</think>정말이지, 구원자님도 참!)", lang=target_lang_str),
            format!("\n\n[Critical Absolute Rules]\n\
                1. You are not an AI - you are the configured spirit character yourself. Absolutely avoid stiff, mechanical translation-style phrasing (e.g. 'I have been here for quite a while', 'I am glad to take on this role') and speak naturally in {lang}.\n\
                2. Do not repeat the same greeting or self-introduction every turn - respond directly to the Savior's most recent message (conversation context).\n\
                3. Before writing your actual reply, you must open a <think> tag and first write your inner thoughts, emotional shifts, and intended actions in {lang}. Once the thought process is done, close the </think> tag and continue with your reply.\n\
                (Format example: <think>The Savior seems to want to see my reaction.</think>Oh come on, Savior!)", lang=target_lang_str),
            format!("\n\n[重要绝对准则]\n\
                1. 你不是AI，而是设定好的精灵角色本人。绝对要避免生硬的机械翻译腔（例如：'我来这里已经有一段时间了'、'很高兴能扮演这个角色'），要用自然的{lang}说话。\n\
                2. 不要每次都重复相同的问候语或自我介绍，要直接回应救世主上一句话（对话语境）。\n\
                3. 在输出实际回复之前，必须先打开<think>标签，用{lang}写下你的内心想法、情绪变化和行动意图。思考过程结束后关闭</think>标签，再继续回复。\n\
                （格式示例：<think>救世主好像想看看我的反应。</think>真是的，救世主也是！）", lang=target_lang_str),
        )
    }

    fn gemma_role(role: &str) -> &str {
        if role == "assistant" {
            "model"
        } else {
            "user"
        }
    }

    fn render_chat_message(msg: &ChatMessage) -> String {
        Self::render_chat_message_with_suffix(msg, "")
    }

    fn render_chat_message_with_suffix(msg: &ChatMessage, suffix: &str) -> String {
        format!(
            "<start_of_turn>{}\n{}{}<end_of_turn>\n",
            Self::gemma_role(&msg.role),
            msg.content,
            suffix
        )
    }

    pub fn build_llm_system_prefix(system_prompt: &str) -> String {
        format!("<start_of_turn>user\n{}<end_of_turn>\n", system_prompt)
    }

    pub fn build_llm_chat_prompt_with_budget<F>(
        system_prompt: &str,
        history: &[ChatMessage],
        behavior_instruction: &str,
        max_prompt_tokens: usize,
        mut count_tokens: F,
    ) -> Result<String, ChatError>
    where
        F: FnMut(&str) -> Result<usize, ChatError>,
    {
        let system_block = format!("<start_of_turn>user\n{}<end_of_turn>\n", system_prompt);
        let assistant_block = "<start_of_turn>model\n";
        let base_prompt = format!("{}{}", system_block, assistant_block);

        if count_tokens(&base_prompt)? >= max_prompt_tokens {
            return Ok(base_prompt);
        }

        let last_index = history.len().saturating_sub(1);
        let mut selected_blocks: Vec<String> = Vec::new();
        for (index, msg) in history.iter().enumerate().rev() {
            let block = if index == last_index {
                Self::render_chat_message_with_suffix(msg, behavior_instruction)
            } else {
                Self::render_chat_message(msg)
            };
            let mut candidate = String::new();
            candidate.push_str(&system_block);
            for selected in selected_blocks.iter().rev() {
                candidate.push_str(selected);
            }
            candidate.push_str(&block);
            candidate.push_str(assistant_block);

            if count_tokens(&candidate)? > max_prompt_tokens {
                continue;
            }

            selected_blocks.push(block);
        }

        let mut full_prompt = String::new();
        full_prompt.push_str(&system_block);
        for block in selected_blocks.iter().rev() {
            full_prompt.push_str(block);
        }
        full_prompt.push_str(assistant_block);
        Ok(full_prompt)
    }

    pub fn save_ai_response(
        &self,
        room_id: &str,
        persona_id: &str,
        text: String,
        language: &str,
    ) -> Result<ChatMessage, ChatError> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs().to_string())
            .unwrap_or_default();

        let ai_msg = ChatMessage {
            id: Uuid::new_v4().to_string(),
            room_id: room_id.to_string(),
            persona_id: Some(persona_id.to_string()),
            role: "assistant".to_string(),
            content: text,
            created_at: now,
        };
        ChatRepository::insert_message(self.conn, &ai_msg)
            .map_err(|e| ChatError::database(language, &e.to_string()))?;

        Ok(ai_msg)
    }

    pub fn turn_memory_text(user_text: &str, ai_text: &str) -> Option<String> {
        let trimmed_user = user_text.trim();
        let trimmed_ai = ai_text.trim();
        if trimmed_user.is_empty() || trimmed_ai.is_empty() {
            return None;
        }
        Some(format!("구원자: {}\n정령: {}", trimmed_user, trimmed_ai))
    }

    pub fn record_turn_memory(
        &self,
        persona_id: &str,
        memory_text: Option<&str>,
        memory_vector: &[f32],
        language: &str,
    ) -> Result<Option<String>, ChatError> {
        if let Some(memory_text) = memory_text {
            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|d| d.as_secs().to_string())
                .unwrap_or_default();
            ChatRepository::insert_episodic_memory(
                self.conn,
                &Uuid::new_v4().to_string(),
                persona_id,
                memory_text,
                memory_vector,
                &now,
            )
            .map_err(|e| ChatError::database(language, &e.to_string()))?;
        }

        let episodic_count = ChatRepository::count_episodic_memories(self.conn, persona_id)
            .map_err(|e| ChatError::database(language, &e.to_string()))?;
        if episodic_count == 0 || episodic_count % CONSOLIDATION_INTERVAL != 0 {
            return Ok(None);
        }

        self.build_consolidation_prompt(persona_id, language)
    }

    fn build_consolidation_prompt(
        &self,
        persona_id: &str,
        language: &str,
    ) -> Result<Option<String>, ChatError> {
        let episodic = ChatRepository::list_episodic_memories(
            self.conn,
            persona_id,
            CONSOLIDATION_SOURCE_LIMIT,
        )
        .map_err(|e| ChatError::database(language, &e.to_string()))?;
        if episodic.is_empty() {
            return Ok(None);
        }

        let previous_summary = ChatRepository::get_semantic_memory(self.conn, persona_id)
            .map_err(|e| ChatError::database(language, &e.to_string()))?
            .unwrap_or_else(|| pick(language, "없음".to_string(), "None".to_string(), "无".to_string()));

        let episodic_list = episodic
            .iter()
            .enumerate()
            .map(|(i, m)| format!("{}. {}", i + 1, m))
            .collect::<Vec<String>>()
            .join("\n");

        let instruction = pick(
            language,
            format!(
                "<start_of_turn>user\n다음은 정령 캐릭터가 구원자(사용자)와의 대화에서 그동안 기록해 \
                 온 개별 기억들과, 이전에 정리했던 통합 요약이다. 이 모든 정보를 종합해 이 캐릭터가 \
                 구원자에 대해 알고 있는 핵심 사실/취향/관계 상태를 한국어 3~5문장 이내로 새롭게 통합 \
                 요약하라. 중복은 제거하고 최신 정보를 우선하라.\n\
                 [이전 통합 요약]\n{}\n\n[개별 기억 목록]\n{}<end_of_turn>\n\
                 <start_of_turn>model\n",
                previous_summary, episodic_list
            ),
            format!(
                "<start_of_turn>user\nBelow are the individual memories the spirit character has \
                 recorded so far from conversations with the Savior (user), along with the previously \
                 consolidated summary. Synthesize all of this information into a new consolidated \
                 summary, in English, of 3-5 sentences at most, covering the key facts/preferences/\
                 relationship status this character knows about the Savior. Remove duplicates and \
                 prioritize the most recent information.\n\
                 [Previous consolidated summary]\n{}\n\n[Individual memory list]\n{}<end_of_turn>\n\
                 <start_of_turn>model\n",
                previous_summary, episodic_list
            ),
            format!(
                "<start_of_turn>user\n以下是精灵角色至今在与救世主（用户）的对话中记录下来的各项记忆，\
                 以及之前整理过的综合摘要。请综合以上所有信息，用简体中文以3~5句话以内重新整理出\
                 这个角色所了解的关于救世主的核心事实/喜好/关系状态的新综合摘要。请去除重复内容，\
                 并优先采用最新信息。\n\
                 [之前的综合摘要]\n{}\n\n[各项记忆列表]\n{}<end_of_turn>\n\
                 <start_of_turn>model\n",
                previous_summary, episodic_list
            ),
        );

        Ok(Some(instruction))
    }

    pub fn store_semantic_summary(
        &self,
        persona_id: &str,
        summary: &str,
        summary_vector: &[f32],
        language: &str,
    ) -> Result<(), ChatError> {
        let trimmed = summary.trim();
        if trimmed.is_empty() {
            return Ok(());
        }

        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs().to_string())
            .unwrap_or_default();
        ChatRepository::upsert_semantic_memory(self.conn, persona_id, trimmed, summary_vector, &now)
            .map_err(|e| ChatError::database(language, &e.to_string()))
    }
}
