use std::{env, path::PathBuf};

use rust_search::SearchBuilder;

fn get_manifest_path() -> Vec<String> {
    let p = get_current_working_dir().unwrap();

    let search: Vec<String> = SearchBuilder::default()
        .location(p.to_str().unwrap())
        .search_input("AndroidManifest")
        .limit(1) // results to return
        .ext("xml")
        .strict()
        .depth(10)
        .ignore_case()
        .hidden()
        .build()
        .collect();

    search
}

fn main() {
    let manifest_path = get_manifest_path();

    if manifest_path.is_empty() {
        eprintln!("Manifest File not found");
        return;
    }

    let manifest_path = manifest_path.first().unwrap();

    println!("{}", manifest_path);
}

fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}
