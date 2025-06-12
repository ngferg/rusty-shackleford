use clap::Parser;
use std::error::Error;
use std::{fmt, fs};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_name: String,
}

#[derive(Debug)]
struct MDToHtmlError {
    reason: String,
}

impl Error for MDToHtmlError {}

impl fmt::Display for MDToHtmlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to convert md to html because {}", self.reason)
    }
}

fn main() {
    let args = Args::parse();

    let file_name = args.file_name;
    println!("Looking at {}", file_name);

    match convert_md_to_html(&file_name) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("Fail! {}", e),
    }
}

fn convert_md_to_html(file_name: &String) -> Result<String, MDToHtmlError> {
    if !file_name.ends_with(".md") {
        return Err(MDToHtmlError {
            reason: String::from("file name must end in .md"),
        });
    }
    let md_lines = fs::read_to_string(file_name);

    if let Err(_) = md_lines {
        return Err(MDToHtmlError {
            reason: String::from("failed to open .md file"),
        });
    }
    let md_lines = md_lines.unwrap();
    let md_lines = md_lines.lines();

    let mut html = String::from("<!DOCTYPE html>\n");

    html.push_str("<html>\n");
    html.push_str("<head>\n");
    html.push_str("<title>");
    html.push_str(file_name.replace(".md", "").as_str());
    html.push_str("</title>\n");
    html.push_str("</head>\n");
    html.push_str("<body>\n");
    html.push_str(build_html_body(md_lines.collect()).as_str());
    html.push_str("</body>\n");
    html.push_str("</html>\n");

    Ok(html)
}

fn build_html_body(md_lines: Vec<&str>) -> String {
    md_lines
        .iter()
        .map(|line| String::from(*line).trim().to_string())
        .map(|line| {
            match line {
                _ if line.is_empty() => String::from("<br/>\n"),
                _ if line.starts_with("#") => {
                    let mut h:usize = 0;
                    let mut chars = line.chars();
                    while chars.next() == Some('#') && h < 6 {
                        h += 1;
                    }
                    format!("<h{}>{}</h{}>\n", h, line[h..].to_string().trim(), h).to_string()
                },
                _ if line.starts_with("---") => String::from("<hr/>\n"),
                _ => format!("<p>{line}</p>\n"),
            }
        })
        .collect()
}
