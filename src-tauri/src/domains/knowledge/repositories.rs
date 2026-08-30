use super::types::KnowledgePayload;
use rusqlite::{params, Connection, Result};

pub struct KnowledgeRepository;

impl KnowledgeRepository {
    pub fn insert_chunk(conn: &Connection, payload: &KnowledgePayload) -> Result<()> {
        conn.execute(
            "INSERT OR REPLACE INTO knowledge_chunk (id, document_name, chunk_text, created_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                payload.id,
                payload.document_name,
                payload.chunk_text,
                payload.created_at
            ],
        )?;
        Ok(())
    }

    const MIN_TERM_LENGTH: usize = 2;

    const MAX_TERMS: usize = 8;

    fn search_terms(query: &str) -> Vec<String> {
        let mut terms: Vec<String> = query
            .split(|c: char| !c.is_alphanumeric())
            .filter(|term| term.chars().count() >= Self::MIN_TERM_LENGTH)
            .map(|term| term.to_lowercase())
            .collect();
        terms.dedup();
        terms.truncate(Self::MAX_TERMS);
        terms
    }

    pub fn search_chunks(
        conn: &Connection,
        query: &str,
        limit: usize,
    ) -> Result<Vec<KnowledgePayload>> {
        let terms = Self::search_terms(query);
        if terms.is_empty() {
            return Ok(Vec::new());
        }

        let conditions = terms
            .iter()
            .map(|_| "lower(chunk_text) LIKE ?")
            .collect::<Vec<&str>>()
            .join(" OR ");
        let sql = format!(
            "SELECT id, document_name, chunk_text, created_at FROM knowledge_chunk WHERE {conditions}"
        );
        let patterns: Vec<String> = terms.iter().map(|term| format!("%{term}%")).collect();

        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(rusqlite::params_from_iter(patterns.iter()), |row| {
            Ok(KnowledgePayload {
                id: row.get(0)?,
                document_name: row.get(1)?,
                chunk_text: row.get(2)?,
                created_at: row.get(3)?,
            })
        })?;

        let mut scored = Vec::new();
        for item in rows {
            if let Ok(chunk) = item {
                let lowered = chunk.chunk_text.to_lowercase();
                let score = terms
                    .iter()
                    .filter(|term| lowered.contains(term.as_str()))
                    .count();
                scored.push((score, chunk));
            }
        }

        scored.sort_by(|left, right| right.0.cmp(&left.0));
        Ok(scored
            .into_iter()
            .take(limit)
            .map(|(_, chunk)| chunk)
            .collect())
    }
}
