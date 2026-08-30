use super::services::ChatService;
use super::services::{
    CHAT_RESPONSE_MAX_TOKENS, CHAT_STREAM_DONE_EVENT, CHAT_STREAM_TOKEN_EVENT,
    CONSOLIDATION_MAX_TOKENS,
};
use super::types::{ChatError, ChatMessage, ChatRoom, SendMessageRequest};
use crate::domains::auth::commands::DbState;
use crate::domains::llm::commands::LlmState;
use crate::domains::settings::commands::{ApiKeyState, SettingsState};
use crate::infrastructure::external_ai::infer_chat;
use crate::infrastructure::i18n::pick;
use tauri::{AppHandle, Manager, State};

fn command_language(settings_state: &State<'_, SettingsState>) -> Result<String, ChatError> {
    Ok(settings_state
        .inner()
        .0
        .lock()
        .map_err(|e| ChatError::unknown("ko", &e.to_string()))?
        .get_language())
}

#[tauri::command(rename_all = "snake_case")]
pub fn chat_create_room(
    db_state: State<'_, DbState>,
    settings_state: State<'_, SettingsState>,
    title: String,
) -> Result<ChatRoom, ChatError> {
    let language = command_language(&settings_state)?;
    let conn = db_state
        .inner()
        .0
        .get()
        .map_err(|e| ChatError::database(&language, &e.to_string()))?;
    let service = ChatService::new(&conn);
    service.create_chat_room(&title, &language)
}

#[tauri::command(rename_all = "snake_case")]
pub fn chat_create_session_room(
    db_state: State<'_, DbState>,
    settings_state: State<'_, SettingsState>,
    title: String,
    persona_id: String,
) -> Result<ChatRoom, ChatError> {
    let language = command_language(&settings_state)?;
    let conn = db_state
        .inner()
        .0
        .get()
        .map_err(|e| ChatError::database(&language, &e.to_string()))?;
    let service = ChatService::new(&conn);
    service.create_chat_session_room(&title, Some(persona_id), &language)
}

#[tauri::command(rename_all = "snake_case")]
pub fn chat_list_rooms(
    db_state: State<'_, DbState>,
    settings_state: State<'_, SettingsState>,
) -> Result<Vec<ChatRoom>, ChatError> {
    let language = command_language(&settings_state)?;
    let conn = db_state
        .inner()
        .0
        .get()
        .map_err(|e| ChatError::database(&language, &e.to_string()))?;
    let service = ChatService::new(&conn);
    service.get_chat_rooms(&language)
}

#[tauri::command(rename_all = "snake_case")]
pub fn chat_get_latest_session_room(
    db_state: State<'_, DbState>,
    settings_state: State<'_, SettingsState>,
    persona_id: String,
) -> Result<Option<ChatRoom>, ChatError> {
    let language = command_language(&settings_state)?;
    let conn = db_state
        .inner()
        .0
        .get()
        .map_err(|e| ChatError::database(&language, &e.to_string()))?;
    let service = ChatService::new(&conn);
    service.get_latest_session_room(&persona_id, &language)
}

#[tauri::command(rename_all = "snake_case")]
pub fn chat_get_evertalk_session_room(
    db_state: State<'_, DbState>,
    settings_state: State<'_, SettingsState>,
) -> Result<ChatRoom, ChatError> {
    let language = command_language(&settings_state)?;
    let conn = db_state
        .inner()
        .0
        .get()
        .map_err(|e| ChatError::database(&language, &e.to_string()))?;
    let service = ChatService::new(&conn);
    service.get_or_create_evertalk_session_room(&language)
}

#[tauri::command(rename_all = "snake_case")]
pub fn chat_list_rooms_for_persona(
    db_state: State<'_, DbState>,
    settings_state: State<'_, SettingsState>,
    persona_id: String,
) -> Result<Vec<ChatRoom>, ChatError> {
    let language = command_language(&settings_state)?;
    let conn = db_state
        .inner()
        .0
        .get()
        .map_err(|e| ChatError::database(&language, &e.to_string()))?;
    let service = ChatService::new(&conn);
    service.get_rooms_for_persona(&persona_id, &language)
}

#[tauri::command(rename_all = "snake_case")]
pub fn chat_start_new_room(
    db_state: State<'_, DbState>,
    settings_state: State<'_, SettingsState>,
    persona_id: String,
) -> Result<ChatRoom, ChatError> {
    let language = command_language(&settings_state)?;
    let conn = db_state
        .inner()
        .0
        .get()
        .map_err(|e| ChatError::database(&language, &e.to_string()))?;
    let service = ChatService::new(&conn);
    service.start_new_room(&persona_id, &language)
}

#[tauri::command(rename_all = "snake_case")]
pub fn chat_delete_room(
    db_state: State<'_, DbState>,
    settings_state: State<'_, SettingsState>,
    room_id: String,
) -> Result<(), ChatError> {
    let language = command_language(&settings_state)?;
    let conn = db_state
        .inner()
        .0
        .get()
        .map_err(|e| ChatError::database(&language, &e.to_string()))?;
    let service = ChatService::new(&conn);
    service.delete_room(&room_id, &language)
}

#[tauri::command(rename_all = "snake_case")]
pub fn chat_delete_message(
    db_state: State<'_, DbState>,
    settings_state: State<'_, SettingsState>,
    message_id: String,
) -> Result<(), ChatError> {
    let language = command_language(&settings_state)?;
    let conn = db_state
        .inner()
        .0
        .get()
        .map_err(|e| ChatError::database(&language, &e.to_string()))?;
    let service = ChatService::new(&conn);
    service.delete_message(&message_id, &language)
}

#[tauri::command(rename_all = "snake_case")]
pub fn chat_list_messages(
    db_state: State<'_, DbState>,
    settings_state: State<'_, SettingsState>,
    room_id: String,
) -> Result<Vec<ChatMessage>, ChatError> {
    let language = command_language(&settings_state)?;
    let conn = db_state
        .inner()
        .0
        .get()
        .map_err(|e| ChatError::database(&language, &e.to_string()))?;
    let service = ChatService::new(&conn);
    service.get_room_messages(&room_id, &language)
}

#[tauri::command(rename_all = "snake_case")]
pub fn chat_list_messages_for_persona(
    db_state: State<'_, DbState>,
    settings_state: State<'_, SettingsState>,
    room_id: String,
    persona_id: String,
) -> Result<Vec<ChatMessage>, ChatError> {
    let language = command_language(&settings_state)?;
    let conn = db_state
        .inner()
        .0
        .get()
        .map_err(|e| ChatError::database(&language, &e.to_string()))?;
    let service = ChatService::new(&conn);
    service.get_room_messages_for_persona(&room_id, &persona_id, &language)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn chat_send_message(
    app_handle: AppHandle,
    db_state: State<'_, DbState>,
    llm_state: State<'_, LlmState>,
    settings_state: State<'_, SettingsState>,
    api_key_state: State<'_, ApiKeyState>,
    room_id: String,
    content: String,
    persona_id: String,
    request_id: String,
) -> Result<ChatMessage, ChatError> {
    let req = SendMessageRequest {
        room_id: room_id.clone(),
        content,
        persona_id,
    };

    let language = command_language(&settings_state)?;

    let external_config = api_key_state
        .inner()
        .0
        .lock()
        .map_err(|e| ChatError::unknown(&language, &e.to_string()))?
        .resolve_chat_backend();

    let engine_instance = if external_config.is_some() {
        None
    } else {
        let engine_lock = llm_state
            .inner()
            .0
            .lock()
            .map_err(|e| ChatError::unknown(&language, &e.to_string()))?;
        Some(
            engine_lock
                .as_ref()
                .cloned()
                .ok_or_else(|| ChatError::llm_engine_not_loaded(&language))?,
        )
    };

    let memory_query_vector = match engine_instance.clone() {
        Some(engine) => {
            let embed_language = language.clone();
            let embed_content = req.content.clone();
            tauri::async_runtime::spawn_blocking(move || {
                engine.embed_text(&embed_content).map_err(|e| {
                    ChatError::llm_inference_failed(&embed_language, &e.to_string())
                })
            })
            .await
            .map_err(|e| {
                ChatError::unknown(
                    &language,
                    &pick(
                        &language,
                        format!("스레드 패닉: {}", e),
                        format!("Thread panicked: {}", e),
                        format!("线程意外终止：{}", e),
                    ),
                )
            })??
        }
        None => Vec::new(),
    };

    let (system_prompt, history) = {
        let conn = db_state
            .inner()
            .0
            .get()
            .map_err(|e| ChatError::database(&language, &e.to_string()))?;
        let settings = settings_state
            .inner()
            .0
            .lock()
            .map_err(|e| ChatError::unknown(&language, &e.to_string()))?;
        let service = ChatService::new(&conn);
        service.prepare_message_context(&req, &settings, &memory_query_vector)?
    };

    let ai_text = if let Some(config) = external_config {
        infer_chat(&config, &system_prompt, &history, CHAT_RESPONSE_MAX_TOKENS)
            .await
            .map_err(|e| ChatError::llm_inference_failed(&language, &e.to_string()))?
    } else {
        let engine_instance = engine_instance
            .ok_or_else(|| ChatError::llm_engine_not_loaded(&language))?;

        let inference_language = language.clone();
        let inference_persona_id = req.persona_id.clone();
        let inference_system_prompt = system_prompt.clone();
        let inference_history = history.clone();
        let inference_request_id = request_id.clone();
        let stream_target = (
            app_handle.clone(),
            CHAT_STREAM_TOKEN_EVENT.to_string(),
            CHAT_STREAM_DONE_EVENT.to_string(),
        );

        tauri::async_runtime::spawn_blocking(move || {
            let response_max_tokens = engine_instance
                .profile()
                .max_tokens
                .min(CHAT_RESPONSE_MAX_TOKENS);
            let max_prompt_tokens = (engine_instance.profile().context_size as usize)
                .saturating_sub(response_max_tokens as usize);
            let behavior_instruction =
                ChatService::build_behavior_instruction(&inference_language);
            let full_prompt = ChatService::build_llm_chat_prompt_with_budget(
                &inference_system_prompt,
                &inference_history,
                &behavior_instruction,
                max_prompt_tokens,
                |text| {
                    engine_instance.count_tokens(text).map_err(|e| {
                        ChatError::llm_inference_failed(&inference_language, &e.to_string())
                    })
                },
            )?;

            engine_instance
                .infer_with_request(
                    &inference_request_id,
                    &full_prompt,
                    Some(response_max_tokens),
                    Some(&inference_persona_id),
                    Some(stream_target),
                )
                .map_err(|e| ChatError::llm_inference_failed(&inference_language, &e.to_string()))
        })
        .await
        .map_err(|e| {
            ChatError::unknown(
                &language,
                &pick(
                    &language,
                    format!("스레드 패닉: {}", e),
                    format!("Thread panicked: {}", e),
                    format!("线程意外终止：{}", e),
                ),
            )
        })??
    };

    let ai_msg = {
        let conn = db_state
            .inner()
            .0
            .get()
            .map_err(|e| ChatError::database(&language, &e.to_string()))?;
        let service = ChatService::new(&conn);
        service.save_ai_response(&room_id, &req.persona_id, ai_text.clone(), &language)?
    };

    let background_persona_id = req.persona_id.clone();
    let background_content = req.content.clone();
    let background_language = language.clone();
    std::thread::spawn(move || {
        let engine_instance = match app_handle.state::<LlmState>().inner().0.lock() {
            Ok(engine_lock) => engine_lock.as_ref().cloned(),
            Err(err) => {
                eprintln!(
                    "{}",
                    pick(
                        &background_language,
                        format!("정령 기억 통합 LLM 잠금 실패: {}", err),
                        format!("Failed to acquire LLM lock for persona memory consolidation: {}", err),
                        format!("精灵记忆整合 LLM 加锁失败：{}", err),
                    )
                );
                None
            }
        };

        let Some(engine_instance) = engine_instance else {
            eprintln!(
                "{}",
                pick(
                    &background_language,
                    "정령 누적 기억을 기록하려면 로컬 임베딩 엔진이 필요합니다.".to_string(),
                    "Recording accumulated persona memory requires the local embedding engine."
                        .to_string(),
                    "记录精灵累积记忆需要本地嵌入引擎。".to_string(),
                )
            );
            return;
        };

        let turn_memory = ChatService::turn_memory_text(&background_content, &ai_text);
        let turn_memory_vector = match turn_memory.as_deref() {
            Some(text) => match engine_instance.embed_text(text) {
                Ok(vector) => vector,
                Err(err) => {
                    eprintln!(
                        "{}",
                        pick(
                            &background_language,
                            format!("정령 turn 기억 임베딩 실패: {}", err),
                            format!("Failed to embed persona turn memory: {}", err),
                            format!("精灵回合记忆嵌入失败：{}", err),
                        )
                    );
                    return;
                }
            },
            None => Vec::new(),
        };

        let consolidation_prompt = match app_handle.state::<DbState>().inner().0.get() {
            Ok(conn) => match ChatService::new(&conn).record_turn_memory(
                &background_persona_id,
                turn_memory.as_deref(),
                &turn_memory_vector,
                &background_language,
            ) {
                Ok(prompt) => prompt,
                Err(err) => {
                    eprintln!(
                        "{}",
                        pick(
                            &background_language,
                            format!("정령 누적 기억 처리 실패: {}", err),
                            format!("Failed to process accumulated persona memory: {}", err),
                            format!("精灵累积记忆处理失败：{}", err),
                        )
                    );
                    None
                }
            },
            Err(err) => {
                eprintln!(
                    "{}",
                    pick(
                        &background_language,
                        format!("정령 누적 기억 DB 잠금 실패: {}", err),
                        format!("Failed to acquire DB lock for accumulated persona memory: {}", err),
                        format!("精灵累积记忆数据库加锁失败：{}", err),
                    )
                );
                None
            }
        };

        let Some(prompt) = consolidation_prompt else {
            return;
        };

        let consolidated = match engine_instance.infer(&prompt, Some(CONSOLIDATION_MAX_TOKENS), None)
        {
            Ok(consolidated_text) => {
                let trimmed = consolidated_text.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    match engine_instance.embed_text(trimmed) {
                        Ok(vector) => Some((trimmed.to_string(), vector)),
                        Err(err) => {
                            eprintln!(
                                "{}",
                                pick(
                                    &background_language,
                                    format!("정령 기억 임베딩 실패: {}", err),
                                    format!("Failed to embed persona memory: {}", err),
                                    format!("精灵记忆嵌入失败：{}", err),
                                )
                            );
                            None
                        }
                    }
                }
            }
            Err(err) => {
                eprintln!(
                    "{}",
                    pick(
                        &background_language,
                        format!("정령 기억 통합 추론 실패: {}", err),
                        format!("Failed to run inference for persona memory consolidation: {}", err),
                        format!("精灵记忆整合推理失败：{}", err),
                    )
                );
                None
            }
        };

        let Some((consolidated_text, consolidated_vector)) = consolidated else {
            return;
        };

        match app_handle.state::<DbState>().inner().0.get() {
            Ok(conn) => {
                if let Err(err) = ChatService::new(&conn).store_semantic_summary(
                    &background_persona_id,
                    &consolidated_text,
                    &consolidated_vector,
                    &background_language,
                ) {
                    eprintln!(
                        "{}",
                        pick(
                            &background_language,
                            format!("정령 통합 기억 저장 실패: {}", err),
                            format!("Failed to save consolidated persona memory: {}", err),
                            format!("精灵整合记忆保存失败：{}", err),
                        )
                    );
                }
            }
            Err(err) => eprintln!(
                "{}",
                pick(
                    &background_language,
                    format!("정령 통합 기억 저장 DB 잠금 실패: {}", err),
                    format!("Failed to acquire DB lock to save consolidated persona memory: {}", err),
                    format!("精灵整合记忆保存数据库加锁失败：{}", err),
                )
            ),
        }
    });

    Ok(ai_msg)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn chat_prepare_persona_cache(
    app_handle: tauri::AppHandle,
    db_state: State<'_, DbState>,
    llm_state: State<'_, LlmState>,
    settings_state: State<'_, SettingsState>,
    persona_id: String,
) -> Result<bool, ChatError> {
    let language = command_language(&settings_state)?;
    let system_prompt = {
        let conn = db_state
            .inner()
            .0
            .get()
            .map_err(|e| ChatError::database(&language, &e.to_string()))?;
        let settings = settings_state
            .inner()
            .0
            .lock()
            .map_err(|e| ChatError::unknown(&language, &e.to_string()))?;
        let service = ChatService::new(&conn);
        service.build_persona_base_system_prompt(&persona_id, &settings)?
    };
    let system_prefix = ChatService::build_llm_system_prefix(&system_prompt);

    let engine_instance = {
        let engine_lock = llm_state
            .inner()
            .0
            .lock()
            .map_err(|e| ChatError::unknown(&language, &e.to_string()))?;
        engine_lock
            .as_ref()
            .cloned()
            .ok_or_else(|| ChatError::llm_engine_not_loaded(&language))?
    };

    let spawn_language = language.clone();
    tauri::async_runtime::spawn_blocking(move || {
        engine_instance
            .warm_persona(&persona_id, &system_prefix, app_handle)
            .map_err(|e| ChatError::llm_inference_failed(&spawn_language, &e.to_string()))
    })
    .await
    .map_err(|e| {
        ChatError::unknown(
            &language,
            &pick(
                &language,
                format!("스레드 패닉: {}", e),
                format!("Thread panicked: {}", e),
                format!("线程意外终止：{}", e),
            ),
        )
    })??;

    Ok(true)
}
