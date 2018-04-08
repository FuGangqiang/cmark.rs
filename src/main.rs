extern crate log;
extern crate env_logger;
#[macro_use]
extern crate structopt;
extern crate pulldown_cmark;

use std::fs;
use std::str::from_utf8;
use std::io::{Result, Write};
use std::fs::File;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use pulldown_cmark::{Options, Parser, OPTION_ENABLE_TABLES};
use pulldown_cmark::html::push_html;


#[derive(StructOpt, Debug)]
#[structopt(name = "cmark")]
/// CommonMark to html converter
struct Opt {
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: Option<PathBuf>,
    #[structopt(short = "c", long = "css", parse(from_os_str))]
    css: Option<PathBuf>,
    #[structopt(parse(from_os_str))]
    src: PathBuf,
}

fn main() {
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Warn)
        .init();

    let opt = Opt::from_args();
    let output = match opt.output {
        Some(ref path) => path.clone(),
        None => opt.src.with_extension("html"),
    };
    let style_raw: Vec<_>;
    let src_raw = fs::read(opt.src).expect("can not read src file");
    let markdown = from_utf8(&src_raw).expect("src file encoding not utf8");
    let style = match opt.css {
        Some(ref sp) => {
            style_raw = fs::read(sp).expect("can not read css file");
            from_utf8(&style_raw).expect("src file encoding not utf8")
        },
        None => "",
    };

    let mut html = String::with_capacity(markdown.len() * 2);
     html.push_str(r#"<html>
  <head>
    <meta charset="utf-8">
    <style>"#);

    html.push_str(style);

     html.push_str(r#"
    </style>
  </head>
  <body>"#);

    let mut parser_opts = Options::empty();
    parser_opts.insert(OPTION_ENABLE_TABLES);
    let parser = Parser::new_ext(markdown, parser_opts);
    push_html(&mut html, parser);

     html.push_str(r#"
  </body>
</html>"#);

    write_file(&output, html.as_bytes()).expect("write output error");
}


/// create the file of `path` and append content
///
/// if parent of `path` does not existed, create it first.
fn write_file(path: &Path, buf: &[u8]) -> Result<()> {
    if let Some(p) = path.parent() {
        ::std::fs::create_dir_all(p)?;
    }
    let mut file = File::create(path)?;
    Ok(file.write_all(buf)?)
}
