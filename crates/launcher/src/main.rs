mod select_docs;
mod select_user_settings;
mod settings;

use anyhow::Result;
use console::style;

fn main() -> Result<()> {
    println!(
        "{}",
        style("FRO Calculator Launcher").bold().blue().underlined()
    );

    let config_dir = "configurate";
    select_docs::create_dir(config_dir)?;

    let doc_config = select_docs::get_doc_versions()?;

    let selections = select_docs::select_languages(&doc_config.versions)?;

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
        style("The installation has been completed successfully!")
            .bold()
            .green()
    );
    println!("{}", style("Press Enter to exit...").dim());
    let _ = std::io::stdin().read_line(&mut String::new())?;

    Ok(())
}
