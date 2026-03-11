use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::Utc;
use std::sync::Arc;
use std::sync::Mutex;
use crate::core::memory_bank::MemoryBank;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction {
    pub user_input: String,
    pub ai_response: String,
    pub user_feedback: bool,
    pub task_result: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonLearned {
    pub content: String,
    pub context: String,
    pub importance: i32,
    pub created_at: String,
}

#[derive(Clone)]
pub struct Reflector {
    history: Arc<Mutex<Vec<Interaction>>>,
}

impl Reflector {
    pub fn new() -> Self {
        Self {
            history: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn analyze_interaction(
        &self,
        user_input: String,
        ai_response: String,
        user_feedback: bool,
        task_result: String,
    ) -> Option<LessonLearned> {
        let interaction = Interaction {
            user_input: user_input.clone(),
            ai_response: ai_response.clone(),
            user_feedback,
            task_result: task_result.clone(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        self.history.lock().unwrap().push(interaction);

        if !user_feedback {
            return Some(LessonLearned {
                content: format!("Task failed: {}. Need improvement.", task_result),
                context: user_input,
                importance: 3,
                created_at: chrono::Utc::now().to_rfc3339(),
            });
        }

        if task_result.contains("error") || task_result.contains("failed") {
            return Some(LessonLearned {
                content: format!("Encountered error: {}. Should handle this case better.", task_result),
                context: user_input,
                importance: 2,
                created_at: chrono::Utc::now().to_rfc3339(),
            });
        }

        None
    }

    pub fn get_interaction_history(&self) -> Vec<Interaction> {
        self.history.lock().unwrap().clone()
    }

    pub fn analyze_patterns(&self) -> Vec<String> {
        let history = self.history.lock().unwrap();
        let mut patterns = HashMap::new();

        for interaction in history.iter() {
            if !interaction.user_feedback {
                for word in interaction.user_input.split_whitespace() {
                    let count = patterns.entry(word.to_lowercase()).or_insert(0);
                    *count += 1;
                }
            }
        }

        let mut sorted_patterns: Vec<_> = patterns.into_iter().collect();
        sorted_patterns.sort_by(|a, b| b.1.cmp(&a.1));
        
        sorted_patterns
            .into_iter()
            .filter(|(_, count)| *count >= 2)
            .map(|(word, count)| format!("{} ({} occurrences)", word, count))
            .collect()
    }
}

pub struct EvolutionEngine {
    memory_bank: Arc<MemoryBank>,
}

impl Clone for EvolutionEngine {
    fn clone(&self) -> Self {
        Self {
            memory_bank: Arc::clone(&self.memory_bank),
        }
    }
}

impl EvolutionEngine {
    pub fn new(memory_bank: Arc<MemoryBank>) -> Self {
        Self { memory_bank }
    }

    pub async fn propose_updates(&self) -> Result<Vec<EvolutionProposal>, String> {
        let memories = self.memory_bank.get_all_memories(100).await
            .map_err(|e| e.to_string())?;

        let mut proposals = Vec::new();
        let mut skill_suggestions = HashMap::new();

        for memory in memories {
            let content_lower = memory.content.to_lowercase();

            if content_lower.contains("excel") || content_lower.contains("spreadsheet") {
                *skill_suggestions.entry("Excel Expert".to_string()).or_insert(0) += memory.importance;
            }
            if content_lower.contains("pdf") {
                *skill_suggestions.entry("PDF Tools".to_string()).or_insert(0) += memory.importance;
            }
            if content_lower.contains("code") || content_lower.contains("programming") {
                *skill_suggestions.entry("Code Assistant".to_string()).or_insert(0) += memory.importance;
            }
            if content_lower.contains("web") || content_lower.contains("html") || content_lower.contains("css") {
                *skill_suggestions.entry("Web Design".to_string()).or_insert(0) += memory.importance;
            }
        }

        for (skill_name, score) in skill_suggestions {
            if score >= 3 {
                proposals.push(EvolutionProposal {
                    proposal_type: ProposalType::NewSkill,
                    title: format!("Add '{}' skill", skill_name),
                    description: format!(
                        "Based on your recent interactions, you frequently work with {}. Consider adding a specialized skill for this.",
                        skill_name
                    ),
                    priority: if score >= 5 { "High" } else { "Medium" }.to_string(),
                    action: format!("Create skill: {}", skill_name),
                });
            }
        }

        Ok(proposals)
    }

    pub fn analyze_soul_evolution(&self, soul_content: &str) -> Vec<String> {
        let mut suggestions = Vec::new();

        if soul_content.len() < 200 {
            suggestions.push("Consider expanding the soul description with more details about capabilities.".to_string());
        }

        if !soul_content.contains("友好") && !soul_content.contains("friendly") {
            suggestions.push("Consider adding personality traits to make interactions more engaging.".to_string());
        }

        if !soul_content.contains("学习") && !soul_content.contains("learn") {
            suggestions.push("Consider emphasizing the ability to learn and adapt from interactions.".to_string());
        }

        suggestions
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionProposal {
    pub proposal_type: ProposalType,
    pub title: String,
    pub description: String,
    pub priority: String,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProposalType {
    NewSkill,
    SoulUpdate,
    AgentUpdate,
    MemoryCleanup,
}
