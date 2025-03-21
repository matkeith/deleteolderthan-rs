use std::env;
use std::fs;
use std::path::Path;
use std::thread;
use std::time::{Duration, SystemTime};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 4 {
        eprintln!("Usage: <path_to_folder> <hours_old> <interval_hours>");
        return;
    }
    
    let path = &args[1];
    let hours: u64 = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid number of hours");
            return;
        }
    };
    
    let interval_hours: u64 = match args[3].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid number of interval hours");
            return;
        }
    };
    
    let run_once = interval_hours == 0;
    
    loop {
        let cutoff = SystemTime::now() - Duration::from_secs(hours * 60 * 60);
        
        match process_dir(Path::new(&path), cutoff) {
            Ok(_) => println!("All matching files and folders deleted successfully."),
            Err(e) => eprintln!("Error: {}", e),
        }
        
        if run_once {
            break;
        } else {
            thread::sleep(Duration::from_secs(interval_hours * 60 * 60));
        }
    }
}

fn process_dir(dir: &Path, cutoff: SystemTime) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if is_older_than(&path, cutoff)? {
                    println!("Directory: {}", path.display());
                    fs::remove_dir_all(&path)?;
                    println!("Deleted directory: {}", path.display());
                } else {
                    process_dir(&path, cutoff)?;
                }
            } else {
                if is_older_than(&path, cutoff)? {
                    println!("File: {}", path.display());
                    fs::remove_file(&path)?;
                    println!("Deleted file: {}", path.display());
                }
            }
        }
    }
    Ok(())
}

fn is_older_than(path: &Path, cutoff: SystemTime) -> std::io::Result<bool> {
    let metadata = fs::metadata(path)?;
    if let Ok(modified) = metadata.modified() {
        return Ok(modified < cutoff);
    }
    Ok(false)
}
