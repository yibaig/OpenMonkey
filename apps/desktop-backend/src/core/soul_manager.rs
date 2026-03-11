use std::path::PathBuf;

#[derive(Clone)]
pub struct SoulManager {
    data_dir: PathBuf,
}

impl SoulManager {
    pub fn new(data_dir: PathBuf) -> Self {
        Self { data_dir }
    }

    pub fn get_soul_path(&self) -> PathBuf {
        self.data_dir.join("SOUL.md")
    }

    pub fn get_user_path(&self) -> PathBuf {
        self.data_dir.join("USER.md")
    }

    pub fn get_agents_path(&self) -> PathBuf {
        self.data_dir.join("AGENTS.md")
    }

    pub fn read_soul(&self) -> Result<String, std::io::Error> {
        std::fs::read_to_string(self.get_soul_path())
    }

    pub fn read_user(&self) -> Result<String, std::io::Error> {
        std::fs::read_to_string(self.get_user_path())
    }

    pub fn read_agents(&self) -> Result<String, std::io::Error> {
        std::fs::read_to_string(self.get_agents_path())
    }

    pub fn write_soul(&self, content: &str) -> Result<(), std::io::Error> {
        let path = self.get_soul_path();
        let tmp_path = path.with_extension("tmp");
        std::fs::write(&tmp_path, content)?;
        std::fs::rename(&tmp_path, &path)?;
        Ok(())
    }

    pub fn write_user(&self, content: &str) -> Result<(), std::io::Error> {
        let path = self.get_user_path();
        let tmp_path = path.with_extension("tmp");
        std::fs::write(&tmp_path, content)?;
        std::fs::rename(&tmp_path, &path)?;
        Ok(())
    }

    pub fn write_agents(&self, content: &str) -> Result<(), std::io::Error> {
        let path = self.get_agents_path();
        let tmp_path = path.with_extension("tmp");
        std::fs::write(&tmp_path, content)?;
        std::fs::rename(&tmp_path, &path)?;
        Ok(())
    }

    pub fn initialize_defaults(&self) -> Result<(), std::io::Error> {
        let soul_path = self.get_soul_path();
        let user_path = self.get_user_path();
        let agents_path = self.get_agents_path();

        if !soul_path.exists() {
            std::fs::write(&soul_path, include_str!("../../../../packages/core/soul/templates/SOUL.md"))?;
        }
        if !user_path.exists() {
            std::fs::write(&user_path, include_str!("../../../../packages/core/soul/templates/USER.md"))?;
        }
        if !agents_path.exists() {
            std::fs::write(&agents_path, include_str!("../../../../packages/core/soul/templates/AGENTS.md"))?;
        }

        Ok(())
    }
}
