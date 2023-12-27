use crate::writer;

#[derive(Debug, PartialEq)]
enum BackupType {
    FileDirectory,
    FileFile,
    DirectoryDirectory,
    DirectoryFile,
}

/// Creates a backup of a file or directory.
///
/// # Arguments
///
/// * `source` - The path to the file or directory to backup.
/// * `target` - The path to the directory where the backup will be created.
///
/// Several cases can occur here:
/// 1. Source is a file, target is a directory:
///     - Create a backup of the source file within the target directory
///     - If the target directory does not exist, attempts to create it
///     - Try to write the backup file to the target directory
///
/// 2. Source is a file, target is a file
///      - Create a backup file with the target name
///      - If the target file does not exist, create it
///      - If the target file is not accessible, exit with error
///      - If the target file already exists, exit with error
///      - TODO: implement flag for force overwrite
///
/// 3. Source is a directory, target is a directory
///      - Create a backup of source directory within the target directory
///      - If the target directory does not exist, create it
///      - Try to write the backup directory to the target directory
///      - If the target directory is not accessible, exit with error
///
/// 4. Source is a directory, target is a file
///      - Create a tarball of the source directory and save it as a file
///      - If the target file already exists, exit with error
///      - TODO: implement flag for force overwrite
///
/// 5. Source is a symlink, target is a directory
/// 6. Source is a symlink, target is a file
/// 7. Source is a symlink, target is a symlink
/// 8. Source is a file, target is a symlink
/// 9. Source is a directory, target is a symlink
/// 10. Source is a symlink, target is a symlink
///
/// REGARDING SYMLINKS:
///      - In the case of a symlink as source or target,
///          actions can be taken directly on it (i.e. the symlink itself will be backed up or restored)
///          or we can follow the symlink and perform the action on its location.
///          This is a case for potential future implementation using flags.
///      - For now though, we'll inform the user that symlinks are not supported and exit with error.
pub fn backup(source: Option<&String>, target: Option<&String>) {
    if source.is_none() {
        eprintln!("No action received");
        eprintln!();
        writer::usage(1, false);
    }

    let default_target = String::from("./");
    let source = source.unwrap();
    let target = target.unwrap_or(&default_target);

    // Check for symlinks
    if std::path::Path::new(source).is_symlink() || std::path::Path::new(target).is_symlink() {
        eprintln!("Symlinks are not supported!");
        eprintln!();
        writer::usage(1, false);
    }

    let backup_type = determine_backup_type(source, target);
    match backup_type {
        Ok(backup_type) => {
            match backup_type {
                BackupType::FileDirectory => { backup_file_directory(source, target); }
                BackupType::FileFile => { backup_file_file(source, target); }
                BackupType::DirectoryDirectory => { backup_directory_directory(source, target); }
                BackupType::DirectoryFile => { backup_directory_file(source, target); }
            }
        }
        Err(error) => {
            eprintln!("{}", error);
            eprintln!();
            writer::usage(1, false);
        }
    }
}

/// Determines the type of backup to perform.
///
/// # Arguments
///
/// * `source` - The path to the file or directory to backup.
/// * `target` - The path to the directory where the backup will be created.
///
/// # Returns
///
/// * `BackupType` - The type of backup to perform.
fn determine_backup_type(source: &String, target: &String) -> Result<BackupType, String> {
    let source_metadata = match std::fs::metadata(source) {
        Ok(metadata) => { metadata }
        Err(_) => { return Err(String::from("Source file or directory does not exist")); }
    };
    let target_metadata = match std::fs::metadata(target) {
        Ok(metadata) => { metadata }
        Err(_) => { return Err(String::from("Target file or directory does not exist")); }
    };

    if source_metadata.is_file() && target_metadata.is_dir() {
        return Ok(BackupType::FileDirectory);
    } else if source_metadata.is_file() && target_metadata.is_file() {
        return Ok(BackupType::FileFile);
    } else if source_metadata.is_dir() && target_metadata.is_dir() {
        return Ok(BackupType::DirectoryDirectory);
    } else if source_metadata.is_dir() && target_metadata.is_file() {
        return Ok(BackupType::DirectoryFile);
    } else {
        return Err(String::from("Unknown error"));
    }
}

/// Creates a backup of a file within a directory.
///
/// # Arguments
///
/// * `source` - The path to the file to backup.
/// * `target` - The path to the directory where the backup will be created.
fn backup_file_directory(source: &String, target: &String) {
    let source_metadata = match std::fs::metadata(source) {
        Ok(metadata) => { metadata }
        Err(_) => { return; }
    };
    let target_metadata = match std::fs::metadata(target) {
        Ok(metadata) => { metadata }
        Err(_) => { return; }
    };

    if !target_metadata.is_dir() {
        match std::fs::create_dir_all(target) {
            Ok(_) => {}
            Err(_) => { return; }
        }
    }

    let source_filename = match std::path::Path::new(source).file_name() {
        Some(filename) => { filename }
        None => { return; }
    };
    let source_filename = match source_filename.to_str() {
        Some(filename) => { filename }
        None => { return; }
    };
    let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S");
    let backup_filename = format!("{}.{}.backup", source_filename, timestamp);
    let backup_path = std::path::Path::new(target).join(backup_filename);

    match std::fs::copy(source, backup_path) {
        Ok(_) => {}
        Err(_) => { return; }
    }
}

/// Creates a backup of a file within a file.
///
/// # Arguments
///
/// * `source` - The path to the file to backup.
/// * `target` - The path to the file where the backup will be created.
fn backup_file_file(source: &String, target: &String) {
    let source_metadata = match std::fs::metadata(source) {
        Ok(metadata) => { metadata }
        Err(_) => { return; }
    };
    let target_metadata = match std::fs::metadata(target) {
        Ok(metadata) => { metadata }
        Err(_) => { return; }
    };

    if !target_metadata.is_file() {
        match std::fs::File::create(target) {
            Ok(_) => {}
            Err(_) => { return; }
        }
    }

    let source_filename = match std::path::Path::new(source).file_name() {
        Some(filename) => { filename }
        None => { return; }
    };
    let source_filename = match source_filename.to_str() {
        Some(filename) => { filename }
        None => { return; }
    };
    let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S");
    let backup_filename = format!("{}.{}.backup", source_filename, timestamp);
    let backup_path = std::path::Path::new(target).join(backup_filename);

    match std::fs::copy(source, backup_path) {
        Ok(_) => {}
        Err(_) => { return; }
    }
}

/// Creates a backup of a directory within a directory.
///
/// # Arguments
///
/// * `source` - The path to the directory to backup.
/// * `target` - The path to the directory where the backup will be created.
fn backup_directory_directory(source: &String, target: &String) {
    let source_metadata = match std::fs::metadata(source) {
        Ok(metadata) => { metadata }
        Err(_) => { return; }
    };
    let target_metadata = match std::fs::metadata(target) {
        Ok(metadata) => { metadata }
        Err(_) => { return; }
    };

    if !target_metadata.is_dir() {
        match std::fs::create_dir_all(target) {
            Ok(_) => {}
            Err(_) => { return; }
        }
    }

    let source_filename = match std::path::Path::new(source).file_name() {
        Some(filename) => { filename }
        None => { return; }
    };
    let source_filename = match source_filename.to_str() {
        Some(filename) => { filename }
        None => { return; }
    };
    let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S");
    let backup_filename = format!("{}.{}.backup", source_filename, timestamp);
    let backup_path = std::path::Path::new(target).join(backup_filename);

    match std::fs::create_dir_all(&backup_path) {
        Ok(_) => {}
        Err(_) => { return; }
    }

    match std::fs::copy(source, backup_path) {
        Ok(_) => {}
        Err(_) => { return; }
    }
}

/// Creates a backup of a directory within a file.
///
/// # Arguments
///
/// * `source` - The path to the directory to backup.
/// * `target` - The path to the file where the backup will be created.
fn backup_directory_file(source: &String, target: &String) {
    let source_metadata = match std::fs::metadata(source) {
        Ok(metadata) => { metadata }
        Err(_) => { return; }
    };
    let target_metadata = match std::fs::metadata(target) {
        Ok(metadata) => { metadata }
        Err(_) => { return; }
    };

    if !target_metadata.is_file() {
        match std::fs::File::create(target) {
            Ok(_) => {}
            Err(_) => { return; }
        }
    }

    let source_filename = match std::path::Path::new(source).file_name() {
        Some(filename) => { filename }
        None => { return; }
    };
    let source_filename = match source_filename.to_str() {
        Some(filename) => { filename }
        None => { return; }
    };
    let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S");
    let backup_filename = format!("{}.{}.backup", source_filename, timestamp);
    let backup_path = std::path::Path::new(target).join(backup_filename);

    match std::fs::copy(source, backup_path) {
        Ok(_) => {}
        Err(_) => { return; }
    }
}
