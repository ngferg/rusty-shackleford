use clap::Parser;
use md_to_html::convert_md_to_html;
use std::fs::File;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    file_name: String,
}

fn main() {
    let args = Args::parse();

    let file_name = args.file_name;
    println!("Looking at {}", file_name);

    match convert_md_to_html(&file_name) {
        Ok(s) => {
            println!("{}", s);
            let mut file = File::create(format!("target/{}", file_name.replace(".md", ".html")))
                .expect("Unable to write output file");
            file.write_all(s.as_bytes())
                .expect("Unable to write output file");
        }
        Err(e) => println!("Fail! {}", e),
    }
}
