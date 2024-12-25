use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;
use std::time::{SystemTime,Duration};

static INSTALLED_FILES : &str = "installierte_programme.txt";

/// Searches for a specific term within a file and returns matching lines as tuples of name and path.
///
/// This function opens the specified file and reads it line by line, searching for lines that contain
/// the given search term. Each matching line is split into a name and path, which are returned as a vector
/// of tuples.
///
/// # Parameters
///
/// * `file_path` - A string slice that holds the path to the file to be searched.
/// * `search_term` - A string slice representing the term to search for within the file.
///
/// # Returns
///
/// * `io::Result<Vec<(String, String)>>` - Returns a vector of tuples, where each tuple contains the name
///   and path extracted from lines that contain the search term. If an error occurs while opening or reading
///   the file, an `io::Error` is returned.
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

/// Recursively searches for executable files with the `.exe` extension within a specified directory.
///
/// This function traverses the given directory and its subdirectories to find files with the `.exe` extension.
/// It collects the names and paths of these files and returns them as a vector of tuples.
///
/// # Parameters
///
/// * `dir` - A reference to a `PathBuf` representing the directory to search for executable files.
///
/// # Returns
///
/// * `io::Result<Vec<(String, String)>>` - Returns a vector of tuples, where each tuple contains the name
///   and path of an executable file. If an error occurs during directory traversal, an `io::Error` is returned.
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


/// Writes the names and paths of all executable files found in the specified directories
/// to a file named "installierte_programme.txt".
///
/// This function searches for `.exe` files within the "C:\\Program Files" and
/// "C:\\Program Files (x86)" directories. It collects the names and paths of these
/// files and writes them to a file for later reference.
///
/// # Returns
///
/// * `io::Result<()>` - Returns `Ok(())` if the operation is successful, or an `io::Error`
///   if an error occurs during file creation or writing.
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

    let mut file = File::create(INSTALLED_FILES)?;
    for (name, path) in &results {
        writeln!(file, "Name: {}, Pfad: {}", name, path)?;
    }

    println!("Ergebnisse wurden in 'installierte_programme.txt' gespeichert.");
    Ok(())
}


/// Determines whether the file "installierte_programme.txt" should be updated based on its age.
///
/// This function checks the last modified timestamp of the file and compares it to the current
/// time. If the file is older than 24 hours, or if there is an error accessing the file's
/// metadata, the function will return `true`, indicating that the file should be updated.
///
/// # Returns
///
/// * `io::Result<bool>` - Returns `Ok(true)` if the file should be updated (either because it
///   is older than 24 hours or an error occurred while accessing its metadata), or `Ok(false)`
///   if the file is less than 24 hours old.
pub fn should_update_file() -> io::Result<bool> {
    if let Ok(metadata) = fs::metadata(INSTALLED_FILES) {
        if let Ok(modified) = metadata.modified() {
            let now = SystemTime::now();
            match now.duration_since(modified) {
                Ok(age) => return Ok(age > Duration::from_secs(24 * 60 * 60)),
                Err(_) => return Ok(true),
            }
        }
    }
    Ok(true) // Datei existiert nicht oder Fehler beim Lesen des Zeitstempels
}