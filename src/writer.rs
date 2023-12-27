use std::{io::{self, Write}, process};

pub fn usage(exit_code: i32, is_stdout: bool) {
    let lines = vec![
        "Usage: backup <mode> <path/to/file/or/directory> [backup directory]",
        "Backup and restore files, directories or symlinks.",
        "",
        "Mode:",
        "    b, backup     Create a timestamped backup of the file or directory",
        "    r, restore    Restore the file or directory from a backup",
        "    h, help       Display this help message",
        "",
        "If the backup directory is not specified, the backup file will be generated in the current directory.",
        "If the backup directory does not exist, it will be created (assuming correct permissions are set).",
        "When performing a restore operation, the optional argument [backup directory] is ignored.",
        "",
        "The backup file or directory will be named as follows:",
        "    <backup directory>/<filename>.<timestamp>.backup",
        "",
        "Examples:",
        "    backup b /etc/hosts",
        "    backup b /etc/hosts /home/user/backups",
        "    backup r /home/user/backups/hosts.2018-01-01_00-00-00.backup",
    ];

    if is_stdout {
        bulk_write_stdout(lines);
    } else {
        bulk_write_stderr(lines);
    }

    process::exit(exit_code);
}

pub fn bulk_write_stdout(lines: Vec<&str>) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for line in lines {
        writeln!(handle, "{}", line).unwrap();
    }

    handle.flush().unwrap();
}

pub fn bulk_write_stderr(lines: Vec<&str>) {
    let stderr = io::stderr();
    let mut handle = stderr.lock();

    for line in lines {
        writeln!(handle, "{}", line).unwrap();
    }

    handle.flush().unwrap();
}
