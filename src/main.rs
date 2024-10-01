use std::env;
use std::fs;
use std::io::Write;

#[cfg(debug_assertions)]
const TEMPLATES: &str = "templates";

#[cfg(not(debug_assertions))]
const TEMPLATES: &str = "/usr/local/share/po2rs/templates";

fn main() {
    let args: Vec<String> = env::args().collect();
    let tera = tera::Tera::new(&format!("{}/*", TEMPLATES)).unwrap();
    let mut ctx = tera::Context::new();
    let langs = args[1].split(',').collect::<Vec<_>>();
    ctx.insert("langs", &langs);
    ctx.insert("trans", &langs.iter().map(|lang| {
        let parser = poreader::PoParser::new();
        let reader = parser.parse(fs::File::open(format!("src/i18n/messages.{}.po", lang)).unwrap()).unwrap();
        let mut items = Vec::new();
        for unit in reader {
            let unit = unit.unwrap();
            if !unit.locations().is_empty() {
                let message = unit.message();
                items.push((
                    message.get_id().to_string(),
                    if message.get_text() == "" {
                        message.get_id().to_string()
                    } else {
                        message.get_text().to_string()
                    }))
            }
        }
       (lang, items)
    }).collect::<Vec<_>>());
    fs::File::create("src/i18n.rs").unwrap()
        .write_all(tera.render("i18n.rs", &ctx).unwrap().as_bytes()).unwrap();
}
