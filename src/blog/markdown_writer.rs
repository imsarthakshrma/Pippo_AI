use std::fs;
use std::path::Path;
use anyhow::Result;
use chrono::Utc;

pub struct BlogWriter;

impl BlogWriter {
    pub fn write_entry(agent_name: &str, content: &str) -> Result<String> {
        let date = Utc::now().format("%Y-%m-%d").to_string();
        let filename = format!("{}-pippo-{}.md", date, agent_name.to_lowercase());
        let path = Path::new("blog").join(&filename);
        
        fs::create_dir_all("blog")?;
        fs::write(&path, content)?;
        
        Ok(path.to_string_lossy().into_owned())
    }
}
