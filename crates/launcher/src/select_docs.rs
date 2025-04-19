use crate::settings::{DOCS_BASE_URL, DOCS_MANIFEST_URL};
use anyhow::Context;
use dialoguer::theme::ColorfulTheme;
use dialoguer::MultiSelect;
use reqwest::blocking::get;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct DocConfig {
    pub versions: Vec<String>,
}

pub fn get_doc_versions() -> anyhow::Result<DocConfig> {
    let response = get(DOCS_MANIFEST_URL)?.error_for_status()?.json()?;

    Ok(response)
}


pub fn select_languages(versions: &[String]) -> anyhow::Result<Vec<String>> {
    let theme = ColorfulTheme::default();

    let selections = MultiSelect::with_theme(&theme)
        .with_prompt("Select the documentation languages (press the SPACE bar to multi select)")
        .items(versions)
        .interact()?;

    Ok(versions
        .iter()
        .enumerate()
        .filter(|(i, _)| selections.contains(i))
        .map(|(_, lang)| lang.clone())
        .collect())
}

pub fn download_docs(lang: &str) -> anyhow::Result<String> {
    let url = DOCS_BASE_URL.replace("{lang}", lang);
    let content = get(&url)?.error_for_status()?.text()?;

    Ok(content)
}

pub fn create_dir(path: &str) -> anyhow::Result<()> {
    if !Path::new(path).exists() {
        fs::create_dir(path).with_context(|| format!("Couldn't create folder {}", path))?;
    }
    Ok(())
}

pub fn create_docs_dir(config_dir: &str) -> anyhow::Result<PathBuf> {
    let docs_dir = Path::new(config_dir).join("docs");
    create_dir(docs_dir.to_str().unwrap())?;
    Ok(docs_dir)
}

pub fn save_docs(docs_dir: &Path, lang: &str, content: &str) -> anyhow::Result<()> {
    let filename = format!("README.{}.md", lang);
    let mut file = File::create(docs_dir.join(filename))?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
