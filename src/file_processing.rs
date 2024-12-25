// file_processing.rs
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

pub fn search_in_file(file_path: &str, search_term: &str) -> io::Result<Vec<(String, String)>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let results: Vec<(String, String)> = reader
        .lines()
        .filter_map(Result::ok)
        .filter(|line| line.contains(search_term))
        .map(|line| {
            let parts: Vec<&str> = line.split(", Pfad: ").collect();
            let name = parts.get(0).unwrap_or(&"").trim().to_string();
            let path = parts.get(1).unwrap_or(&"").trim().to_string();
            (name, path)
        })
        .collect();

    Ok(results)
}

pub fn find_exe_files(dir: &PathBuf) -> io::Result<Vec<(String, String)>> {
    let results = fs::read_dir(dir)?
        .filter_map(|entry| entry.ok())
        .flat_map(|entry| {
            let path = entry.path();
            if path.is_dir() {
                find_exe_files(&path).unwrap_or_else(|e| {
                    eprintln!("Fehler beim Zugriff auf Verzeichnis {}: {}", path.display(), e);
                    Vec::new()
                })
            } else if path.extension().and_then(|s| s.to_str()) == Some("exe") {
                if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                    vec![(file_name.to_string(), path.to_string_lossy().to_string())]
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            }
        })
        .collect();
    Ok(results)
}

pub fn write_exe_files_to_file() -> io::Result<()> {
    let program_files = vec![
        PathBuf::from("C:\\Program Files"),
        PathBuf::from("C:\\Program Files (x86)"),
    ];

    let results: Vec<_> = program_files
        .iter()
        .filter_map(|dir| {
            if dir.exists() {
                find_exe_files(dir).ok()
            } else {
                None
            }
        })
        .flatten()
        .collect();

    let mut file = File::create("installierte_programme.txt")?;
    for (name, path) in &results {
        writeln!(file, "Name: {}, Pfad: {}", name, path)?;
    }

    println!("Ergebnisse wurden in 'installierte_programme.txt' gespeichert.");
    Ok(())
}
