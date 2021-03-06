use tera::{Tera, Context};
use config::Config;
use tera::Result;
use chrono::{Local, DateTime};

const TEMP_TEMPLATE_FILENAME: &str = "index.html";

fn build_context(config: &Config) -> Context {
    let mut context = Context::new();
    let now: DateTime<Local> = Local::now();
    context.add("config", &config);
    context.add("now", &now.to_rfc3339());
    context.add("version", &String::from(crate_version!()));
    return context;
}

pub fn get_errors(result: Result<String>) -> String {
    let errors: Vec<String> = result
        .expect_err("Tried to get errors on successful result")
        .iter()
        .map(|e| String::from(e.description()))
        .collect();
    return errors.join("\n");
}

pub fn render_template(content: &String, config: Config) -> Result<String> {
    let mut tera = Tera::default();
    tera.add_raw_template(TEMP_TEMPLATE_FILENAME, content).expect("Failed to add raw template");
    return tera.render(TEMP_TEMPLATE_FILENAME, &build_context(&config));
}
