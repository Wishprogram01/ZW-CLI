use clap::Parser;
use pulldown_cmark::{Parser as MarkdownParser, Options, html};
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(name = "md2html")]
#[command(about = "Menukar fail Markdown kepada HTML", long_about = None)]
struct Args {
    #[arg(short, long, help = "Path fail input markdown (.md)")]
    input: String,

    #[arg(short, long, help = "Path fail output HTML")]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();
    let input_path = Path::new(&args.input);

    if !input_path.exists() {
        eprintln!("Ralat: Fail input '{}' tidak wujud.", args.input);
        std::process::exit(1);
    }

    let markdown_input = fs::read_to_string(input_path).expect("Gagal membaca fail");

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = MarkdownParser::new_ext(&markdown_input, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let final_html = format!(
        r#"<!DOCTYPE html>
<html lang="ms">
<head>
    <meta charset="UTF-8">
    <title>{}</title>
    <style>
        body {{ font-family: sans-serif; line-height: 1.6; max-width: 800px; margin: 40px auto; padding: 0 20px; color: #333; }}
        code {{ background: #f4f4f4; padding: 2px 6px; border-radius: 4px; }}
        pre {{ background: #f4f4f4; padding: 15px; border-radius: 6px; }}
    </style>
</head>
<body>
{}
</body>
</html>"#,
        input_path.file_name().unwrap().to_string_lossy(),
        html_output
    );

    let output_path = args.output.unwrap_or_else(|| {
        format!("{}.html", input_path.file_stem().unwrap().to_string_lossy())
    });

    fs::write(&output_path, final_html).expect("Gagal menulis fail HTML");
    println!("Berjaya! Fail HTML dijana di: {}", output_path);
}
