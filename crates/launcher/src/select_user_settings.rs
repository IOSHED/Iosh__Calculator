use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct UserConfig {
    path_file_help: String,
    commands: Commands,
    output_line_history: u8,
    max_size_history: u8,
    max_number_variable: u8,
}

#[derive(Debug, Serialize)]
pub struct Commands {
    end: String,
    help: String,
    history: String,
    empty_input: String,
}

pub fn create_user_config(config_dir: &str, langs: &[String]) -> anyhow::Result<()> {
    let theme = ColorfulTheme::default();
    let mut config = create_default_config(langs);

    if Confirm::with_theme(&theme)
        .with_prompt("Do you want to customize the settings?")
        .default(false)
        .interact()?
    {
        customize_settings(&mut config, &theme)?;
    }

    save_config(config_dir, &config)
}

fn create_default_config(langs: &[String]) -> UserConfig {
    let default_lang = langs.first().map(|s| s.as_str()).unwrap_or("en");

    UserConfig {
        path_file_help: format!("./configurate/docs/README.{}.md", default_lang),
        commands: Commands {
            end: "/end".to_string(),
            help: "/help".to_string(),
            history: "/history".to_string(),
            empty_input: "".to_string(),
        },
        output_line_history: 10,
        max_size_history: 50,
        max_number_variable: 50,
    }
}

fn customize_settings(config: &mut UserConfig, theme: &ColorfulTheme) -> anyhow::Result<()> {
    config.path_file_help = Input::with_theme(theme)
        .with_prompt("Help file path")
        .default(config.path_file_help.clone())
        .interact()?;

    config.output_line_history = Input::with_theme(theme)
        .with_prompt("Number of history lines to show")
        .default(config.output_line_history.to_string())
        .validate_with(|input: &String| match input.parse::<u8>() {
            Ok(_) => Ok(()),
            Err(_) => Err("Please enter a number between 0 and 255"),
        })
        .interact()?
        .parse()?;

    config.max_size_history = Input::with_theme(theme)
        .with_prompt("Maximum history size")
        .default(config.max_size_history.to_string())
        .validate_with(|input: &String| match input.parse::<u8>() {
            Ok(_) => Ok(()),
            Err(_) => Err("Please enter a number between 0 and 255"),
        })
        .interact()?
        .parse()?;

    config.max_number_variable = Input::with_theme(theme)
        .with_prompt("Maximum number of variables")
        .default(config.max_number_variable.to_string())
        .validate_with(|input: &String| match input.parse::<u8>() {
            Ok(_) => Ok(()),
            Err(_) => Err("Please enter a number between 0 and 255"),
        })
        .interact()?
        .parse()?;

    println!("\nCustomizing commands:");
    config.commands.end = Input::with_theme(theme)
        .with_prompt("Command to end session")
        .default(config.commands.end.clone())
        .interact()?;

    config.commands.help = Input::with_theme(theme)
        .with_prompt("Command to show help")
        .default(config.commands.help.clone())
        .interact()?;

    config.commands.history = Input::with_theme(theme)
        .with_prompt("Command to show history")
        .default(config.commands.history.clone())
        .interact()?;

    Ok(())
}

fn save_config(config_dir: &str, config: &UserConfig) -> anyhow::Result<()> {
    let config_path = Path::new(config_dir).join("user.json");
    let mut file = File::create(config_path)?;
    let json = serde_json::to_string_pretty(config)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
