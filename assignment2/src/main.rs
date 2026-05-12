use std::env;
use std::fs::{metadata, read_dir};
use std::os::unix::fs::PermissionsExt;
use std::process;

fn permissions_to_rwx(mode: u32) -> String {
    let mut rwx = String::with_capacity(9);

    for shift in [6, 3, 0] {
        let group_bits = (mode >> shift) & 0o7;
        rwx.push(if group_bits & 0o4 != 0 { 'r' } else { '-' });
        rwx.push(if group_bits & 0o2 != 0 { 'w' } else { '-' });
        rwx.push(if group_bits & 0o1 != 0 { 'x' } else { '-' });
    }

    rwx
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(format!("Usage: {} <directory_path>", args[0]));
    }

    let directory = &args[1];
    let dir_metadata = metadata(directory)
        .map_err(|err| format!("Failed to read metadata for '{}': {}", directory, err))?;

    if !dir_metadata.is_dir() {
        return Err(format!("'{}' is not a valid directory.", directory));
    }

    println!("Listing files in: {}", directory);

    let entries =
        read_dir(directory).map_err(|err| format!("Failed to read directory '{}': {}", directory, err))?;

    for entry_result in entries {
        let entry = match entry_result {
            Ok(value) => value,
            Err(err) => {
                eprintln!("Skipping an entry due to read error: {}", err);
                continue;
            }
        };

        let path = entry.path();
        let file_metadata = match metadata(&path) {
            Ok(value) => value,
            Err(err) => {
                eprintln!("Skipping '{}': {}", path.display(), err);
                continue;
            }
        };

        if !file_metadata.is_file() {
            continue;
        }

        let mode = file_metadata.permissions().mode() & 0o777;
        let rwx = permissions_to_rwx(mode);
        let octal_mode = format!("{:03o}", mode);
        let file_name = path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.display().to_string());

        println!("File: {}", file_name);
        println!("  Size (bytes): {}", file_metadata.len());
        println!("  Permissions: {}", rwx);
        println!("  Mode (octal): {}", octal_mode);
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        process::exit(1);
    }
}
