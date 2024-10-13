use std::error::Error;
use std::fs;

use file_operations::copy_dir_all;
use tera::Context;
use tera::Tera;

mod file_operations;

fn main() {
    let _ = build_html_templates();
    let _ = copy_dir_all("../src/js", "../dist/js");
    let _ = copy_dir_all("../src/css", "../dist/css");
    let _ = copy_dir_all("../src/vendor", "../dist/vendor");
    let _ = fs::copy("../src/favicon.ico", "../dist/favicon.ico");
}

fn build_html_templates() -> Result<(), Box<dyn Error>> {
    let tera = Tera::new("../src/templates/**/*.html")?;
    let context = Context::new();
    fs::write("../dist/index.html", tera.render("index.html", &context)?)?;
    Ok(())
}
