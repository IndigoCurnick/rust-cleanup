use std::{env, fs, io, path::PathBuf};

use log::info;
use walkdir::WalkDir;

fn main() {
    simple_logger::init().unwrap();

    let pwd = env::current_dir().expect("No current working directory");

    let mut num_cleaned = 0;
    let mut total_bytes_cleaned = 0;

    for entry in WalkDir::new(pwd) {
        let entry = entry.unwrap();
        // Check if the entry is a directory
        let path = entry.path();
        let folder_name = match path.file_name() {
            Some(x) => x.to_str().expect("Could not convert OsString to str"),
            None => continue,
        };

        if path.is_dir() {
            // Print the path of the directory

            // Does directory have a Cargo.toml file?
            let toml_path = path.join("Cargo.toml");

            if !toml_path.exists() {
                continue;
            }

            let target_path = path.join("target");

            if !target_path.exists() {
                info!(
                    "Rust project `{}` has no `target` folder - skipping",
                    folder_name
                );
                continue;
            }

            let target_size =
                get_directory_size(&target_path).expect("Could not identify directory size");

            info!(
                "Cleaning folder `{}` to free {} bytes",
                folder_name, target_size
            );

            delete_directory(&target_path).expect(&format!(
                "Could not delete target folder for project {}",
                folder_name
            ));

            num_cleaned += 1;
            total_bytes_cleaned += target_size;
        }
    }

    info!(
        "Cleaning complete - cleaned a total of {} Rust projects to free {:.2} GB",
        num_cleaned,
        bytes_to_gb(total_bytes_cleaned)
    );
}

fn get_directory_size(path: &PathBuf) -> Result<u64, std::io::Error> {
    let mut total_size = 0;

    // Iterate over each entry in the directory
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_file() {
            // If the entry is a file, add its size to the total
            total_size += metadata.len();
        } else if metadata.is_dir() {
            // If the entry is a directory, recursively call the function
            total_size += get_directory_size(&entry.path())?;
        }
        // Ignore other types of entries (e.g., symbolic links)
    }

    Ok(total_size)
}

fn bytes_to_gb(bytes: u64) -> f64 {
    // Convert bytes to gigabytes
    let gb = bytes as f64 / 1024.0_f64.powf(3.0_f64);
    return gb;
}

fn delete_directory(dir: &PathBuf) -> io::Result<()> {
    // Iterate over each entry in the directory
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let entry_path = entry.path();

        // Check if the entry is a directory
        if entry_path.is_dir() {
            // Recursively delete the directory
            delete_directory(&entry_path)?;
        } else {
            // Delete the file
            fs::remove_file(&entry_path)?;
        }
    }

    // Finally, remove the directory itself
    fs::remove_dir(dir)?;

    Ok(())
}
