use clap::Parser;
use std::error::Error;
use std::fmt;

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
    let mut html = String::from("<!DOCTYPE html>\n");

    html.push_str("<html>\n");
    html.push_str("<head>\n");
    html.push_str("<title>");
    html.push_str(file_name.replace(".md", "").as_str());
    html.push_str("</title>\n");
    html.push_str("</head>\n");
    html.push_str("<body>\n");
    html.push_str("</body>\n");
    html.push_str("</html>\n");

    Ok(html)
}
