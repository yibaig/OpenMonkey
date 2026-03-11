use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OmSkill {
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub triggers: Vec<String>,
    pub instructions: String,
    pub tools_required: Vec<String>,
    pub examples: Vec<String>,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct SkillYaml {
    name: String,
    description: String,
    version: Option<String>,
    author: Option<String>,
    triggers: Vec<String>,
    tools_required: Option<Vec<String>>,
    examples: Option<Vec<String>>,
}

#[derive(Clone)]
pub struct SkillAdapter {
    skills_dir: PathBuf,
}

impl SkillAdapter {
    pub fn new(skills_dir: PathBuf) -> Self {
        Self { skills_dir }
    }

    pub async fn parse_external(&self, source: &str) -> Result<OmSkill, String> {
        let content = if source.starts_with("http://") || source.starts_with("https://") {
            self.fetch_url(source).await?
        } else {
            source.to_string()
        };

        let skill = self.parse_yaml(&content)?;
        Ok(skill)
    }

    async fn fetch_url(&self, url: &str) -> Result<String, String> {
        let response = reqwest::get(url).await.map_err(|e| e.to_string())?;
        let content = response.text().await.map_err(|e| e.to_string())?;
        Ok(content)
    }

    fn parse_yaml(&self, content: &str) -> Result<OmSkill, String> {
        let trimmed = content.trim();
        
        // Check if content starts with ---
        if !trimmed.starts_with("---") {
            return Err("Invalid skill format: missing YAML frontmatter".to_string());
        }

        // Find the second --- that closes the YAML frontmatter
        let after_first_marker = &trimmed[3..]; // Skip first ---
        let Some(second_marker_pos) = after_first_marker.find("---") else {
            return Err("Invalid skill format: YAML frontmatter not properly closed".to_string());
        };

        let yaml_content = after_first_marker[..second_marker_pos].trim();
        let instructions = after_first_marker[second_marker_pos + 3..].trim();

        // Parse YAML
        let skill_yaml: SkillYaml = serde_yaml::from_str(yaml_content)
            .map_err(|e| format!("Failed to parse YAML: {}", e))?;

        Ok(OmSkill {
            name: skill_yaml.name,
            description: skill_yaml.description,
            version: skill_yaml.version.unwrap_or("1.0.0".to_string()),
            author: skill_yaml.author.unwrap_or("Unknown".to_string()),
            triggers: skill_yaml.triggers,
            instructions: instructions.to_string(),
            tools_required: skill_yaml.tools_required.unwrap_or_default(),
            examples: skill_yaml.examples.unwrap_or_default(),
            enabled: true,
        })
    }

    pub async fn validate_and_fix(&self, skill_path: &str) -> Result<String, String> {
        let path = PathBuf::from(skill_path);
        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        
        let skill = self.parse_yaml(&content)?;
        
        if skill.triggers.is_empty() {
            return Err("Skill must have at least one trigger".to_string());
        }

        if skill.instructions.is_empty() {
            return Err("Skill must have instructions".to_string());
        }

        Ok("Skill validated successfully".to_string())
    }

    pub fn save_skill(&self, skill: &OmSkill) -> Result<String, String> {
        let filename = format!("om_skill_{}.md", slug::slugify(&skill.name));
        let path = self.skills_dir.join(&filename);

        let yaml_content = format!(
            r#"---
name: {}
description: {}
version: {}
author: {}

triggers:
{}
tools_required:
{}
examples:
{}
---

{}"#,
            skill.name,
            skill.description,
            skill.version,
            skill.author,
            skill.triggers.iter().map(|t| format!("  - \"{}\"", t)).collect::<Vec<_>>().join("\n"),
            skill.tools_required.iter().map(|t| format!("  - \"{}\"", t)).collect::<Vec<_>>().join("\n"),
            skill.examples.iter().map(|e| format!("  - \"{}\"", e)).collect::<Vec<_>>().join("\n"),
            skill.instructions
        );

        let tmp_path = path.with_extension("tmp");
        fs::write(&tmp_path, yaml_content).map_err(|e| e.to_string())?;
        fs::rename(&tmp_path, &path).map_err(|e| e.to_string())?;

        Ok(path.to_string_lossy().to_string())
    }

    pub fn list_skills(&self) -> Result<Vec<OmSkill>, String> {
        let mut skills = Vec::new();

        if !self.skills_dir.exists() {
            return Ok(skills);
        }

        for entry in fs::read_dir(&self.skills_dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
                if let Ok(skill) = self.parse_yaml(&content) {
                    skills.push(skill);
                }
            }
        }

        Ok(skills)
    }

    pub fn enable_skill(&self, skill_name: &str, enabled: bool) -> Result<(), String> {
        let filename = format!("om_skill_{}.md", slug::slugify(skill_name));
        let path = self.skills_dir.join(&filename);

        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let mut skill = self.parse_yaml(&content)?;
        skill.enabled = enabled;

        self.save_skill(&skill)?;
        Ok(())
    }

    pub fn delete_skill(&self, skill_name: &str) -> Result<(), String> {
        let filename = format!("om_skill_{}.md", slug::slugify(skill_name));
        let path = self.skills_dir.join(&filename);

        fs::remove_file(&path).map_err(|e| e.to_string())?;
        Ok(())
    }
}
