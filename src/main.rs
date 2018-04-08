#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate structopt;
extern crate pulldown_cmark;

use std::io::{Result, Write};
use std::fs::File;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use pulldown_cmark::{html, Options, Parser, OPTION_ENABLE_TABLES};


#[derive(StructOpt, Debug)]
#[structopt(name = "cmark")]
/// CommonMark to html converter
struct Opt {
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: Option<PathBuf>,
    #[structopt(parse(from_os_str))]
    src: PathBuf,
}

fn main() {
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Info)
        .init();

    let opt = Opt::from_args();

    let output = match opt.output {
        Some(ref path) => path.clone(),
        None => opt.src.with_extension("html"),
    };

    info!("src: {:?}", opt.src);
    info!("output: {:?}", output);

    let src_raw = ::std::fs::read(opt.src).expect("can not read src file");
    let html = markdown_to_html(::std::str::from_utf8(&src_raw).expect("src file encoding not utf8"));
    write_file(&output, html.as_bytes()).expect("write output error");
}


/// the rendered html content of post body port
fn markdown_to_html(content: &str) -> String {
    let mut opts = Options::empty();
    opts.insert(OPTION_ENABLE_TABLES);
    let mut s = String::with_capacity(content.len() * 3 / 2);
    s.push_str(r#"<htm><head><meta charset="utf-8"></head><body>"#);
    let p = Parser::new_ext(content, opts);
    html::push_html(&mut s, p);
    s.push_str(r#"</body></html>"#);
    s
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
