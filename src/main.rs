mod dba;
mod templates;
#[macro_use]
mod macros;

use anyhow::Result;
use clap::Parser;
use dba::Dba;
use handlebars::Handlebars;
use std::fs;
use std::path::PathBuf;
use templates::BuiltInTemplate;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to the output file
    #[clap(short = 'o', long = "output")]
    output_path: PathBuf,
    #[cfg(not(feature = "built_in"))]
    /// Path to the Handlebars template file
    #[clap(short = 't', long = "template")]
    template_path: PathBuf,
    /// Path to the xcresult
    #[clap(short = 'p', long = "xcresult")]
    xcresult_path: PathBuf,

    #[cfg(feature = "built_in")]
    /// Built-in template
    #[clap(short = 't', long = "template")]
    built_in_template: BuiltInTemplate,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let reg = Handlebars::new();
    let dba = Dba::new(&args.xcresult_path.display().to_string())?;

    let test_results = dba.get_test_results()?;
    let params: serde_json::Value = serde_json::to_value(&test_results)?;
    #[cfg(not(feature = "built_in"))]
    let content = fs::read_to_string(args.template_path)?;
    #[cfg(feature = "built_in")]
    let content = args.built_in_template.template();

    let content = reg.render_template(&content, &params)?;

    fs::write(args.output_path, content)?;
    Ok(())
}
