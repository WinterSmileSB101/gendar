use std::{
    collections::HashMap,
    fs::File,
    io::{Cursor, Read, Result, Write},
};

use tera::{Context, Tera};
use zip::{write::SimpleFileOptions, ZipArchive, ZipWriter};

fn main() -> Result<()> {
    storage_temp_gen_to_mem("temp.zip")?;
    Ok(())
}

fn unzip_to_memory(zip_data: &[u8]) -> Result<HashMap<String, String>> {
    let reader = Cursor::new(zip_data);
    let mut archive = ZipArchive::new(reader)?;
    let mut files = HashMap::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;
        files.insert(file.name().to_string(), contents);
    }

    Ok(files)
}

fn process_templates(
    files: &HashMap<String, String>,
    tera: &mut Tera,
    context: &Context,
) -> HashMap<String, String> {
    let mut processed_files = HashMap::new();

    for (name, contents) in files {
        if let Ok(rendered_content) = tera.render_str(contents, context) {
            processed_files.insert(name.clone(), rendered_content);
        }
    }

    processed_files
}

fn zip_from_memory(files: &HashMap<String, String>) -> Result<Vec<u8>> {
    let mut buffer = Cursor::new(Vec::new());
    let mut zip = ZipWriter::new(&mut buffer);

    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

    for (name, content) in files {
        zip.start_file(name, options)?;
        zip.write_all(content.as_bytes())?;
    }

    zip.finish()?;
    Ok(buffer.into_inner())
}

fn storage_temp_gen_to_mem<P: AsRef<std::path::Path>>(path: P) -> Result<()> {
    let zip_data = std::fs::read(path)?;

    let unzip_files = unzip_to_memory(&zip_data)?;

    let mut tera = Tera::default();

    for (name, content) in &unzip_files {
        tera.add_raw_template(name, content).unwrap();
    }

    let mut context = Context::new();
    context.insert("name", "Rust Mem Temp");

    let processed_files = process_templates(&unzip_files, &mut tera, &context);

    let zipped_output = zip_from_memory(&processed_files)?;

    std::fs::write("processed_templates.zip", zipped_output)?;

    println!("Template generated file name: processed_templates.zip");

    Ok(())
}

#[warn(dead_code)]
fn storage_temp_gen_to_storage<P: AsRef<std::path::Path>>(path: P) -> Result<()> {
    let file = File::open(path)?;
    let mut archive = ZipArchive::new(file)?;

    let mut templates = HashMap::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        templates.insert(file.name().to_string(), contents);
    }

    let mut tera = Tera::default();

    for (name, contents) in templates {
        tera.add_raw_template(&name, &contents).unwrap();
    }

    let mut context = Context::new();
    context.insert("name", "Rust Temp");

    let rendered = tera.render("hello.html", &context).unwrap();

    println!("Result: \r\n{}", rendered);

    Ok(())
}
