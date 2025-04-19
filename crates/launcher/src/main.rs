extern crate rust_i18n;

mod i18n_lang;
mod select_docs;
mod select_user_settings;
mod settings;

use crate::i18n_lang::select_language;
use anyhow::Result;
use console::style;
use rust_i18n::t;

rust_i18n::i18n!();

fn main() -> Result<()> {
    println!(
        "{}",
        style("FRO Calculator Launcher").bold().blue().underlined()
    );

    let lang = select_language()?;
    rust_i18n::set_locale(&lang);

    let config_dir = "configurate";
    select_docs::create_dir(config_dir)?;

    let selections = select_docs::select_languages()?;

    let docs_dir = select_docs::create_docs_dir(config_dir)?;
    let mut installed_langs = Vec::new();

    for lang in selections {
        let content = select_docs::download_docs(&lang)?;
        select_docs::save_docs(&docs_dir, &lang, &content)?;
        installed_langs.push(lang);
    }

    select_user_settings::create_user_config(config_dir, &installed_langs)?;

    println!(
        "\n{}",
        style(t!("The installation has been completed successfully!"))
            .bold()
            .green()
    );
    println!("{}", style(t!("Press Enter to exit...")).dim());
    let _ = std::io::stdin().read_line(&mut String::new())?;

    Ok(())
}
