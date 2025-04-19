use crate::settings::SUPPORTED_LANGUAGES;
use anyhow::Result;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;

pub fn select_language() -> Result<String> {
    let langs = ["English 🇺🇲", "Русский 🇷🇺"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose language")
        .items(&langs)
        .default(0)
        .interact()?;

    Ok(SUPPORTED_LANGUAGES[selection].to_string())
}
