use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::ffi::OsStr;
use pulldown_cmark::{Parser, Options, html};
use clap::{App, Arg};
use glob::glob;

fn convert_md_to_html(md_files: Vec<String>, output_dir: &str) -> io::Result<()> {
    if !Path::new(output_dir).exists() {
        fs::create_dir(output_dir)?;
    }

    let mut index_content = String::from(
        "<html><head><title>Index of Newbies Bites Part II</title></head><body><h1>Index of Newbie Bites Part II</h1><ul>"
    );

    for md_file in md_files {
        let subdir_name = Path::new(&md_file)
            .parent()
            .and_then(Path::file_name)
            .and_then(OsStr::to_str)
            .unwrap_or("");

        if !subdir_name.chars().next().unwrap_or(' ').is_digit(10) {
            continue;
        }

        let html_file_name = format!("{}.html", subdir_name);
        let html_file_path = Path::new(output_dir).join(&html_file_name);

        let md_content = fs::read_to_string(&md_file)?;
        let mut html_content = String::new();
        let parser = Parser::new_ext(&md_content, Options::empty());
        html::push_html(&mut html_content, parser);

        let mut html_file = File::create(html_file_path)?;
        write!(
            html_file,
            "<html><head><title>{}</title></head><body>{}</body></html>",
            subdir_name, html_content
        )?;

        index_content.push_str(&format!(
            "<li><a href=\"{}\">{}</a></li>\n",
            html_file_name, subdir_name
        ));
    }

    index_content.push_str("</ul></body></html>");

    let index_file_path = Path::new(output_dir).join("index.html");
    let mut index_file = File::create(index_file_path)?;
    write!(index_file, "{}", index_content)?;

    println!("HTML pages and index generated in {}", output_dir);

    Ok(())
}

fn main() -> io::Result<()> {
    let matches = App::new("Markdown to HTML Converter")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Converts Markdown files to HTML and generates an index")
        .arg(
            Arg::new("directory")
                .short('d')
                .long("directory")
                .value_name("DIRECTORY")
                .help("Specifies the directory to search for Markdown files")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let directory = matches.value_of("directory").unwrap();
    let pattern = format!("{}/[0-9][0-9]_*/*.md", directory);

    let md_files: Vec<String> = glob(&pattern)
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .filter_map(|path| path.to_str().map(String::from))
        .collect();

    let output_dir = "html_pages";
    fs::create_dir_all(output_dir)?;

    convert_md_to_html(md_files, output_dir)
}

