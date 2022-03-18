use std::fs::{self, ReadDir};
use filetime::FileTime;
use std::io::Error;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    let path_name = "C:\\data";
    let data_dir = fs::read_dir(path_name);
    list_dir_contents();
    let _ = delete_dir_contents(data_dir);
    sleep(Duration::from_secs(10));
    main();
}

fn delete_dir_contents(read_dir_res: Result<ReadDir, Error>) {
    if let Ok(dir) = read_dir_res {
        for entry in dir {
            if let Ok(entry) = entry {
                let path = entry.path();
                if check_age(entry.path()) {
                    if path.is_dir() {
                        fs::remove_dir_all(path).ok();
                    } else {
                        fs::remove_file(path).ok();
                    }
                };
            };
        }
    };
}

fn list_dir_contents() {
    let files = fs::read_dir("C:\\data").unwrap();
    let found_files = files.count() > 0;
    if found_files {
        println!("{} files found.", fs::read_dir("C:\\data").unwrap().count());
    } else {
        println!("No files in this directory.");
    }
}

fn check_age(path: std::path::PathBuf) -> bool {
    // Check if file is 60 seconds or older, return true if older.
    let metadata = fs::metadata(&path).unwrap();
    let mod_time = FileTime::from_last_modification_time(&metadata).seconds();
    let now = FileTime::now().seconds();
    let difference = now - mod_time;
    // println!("{:?} is {} seconds old.", path, difference);
    if difference >= 60 {
        return true
    };
    return false;
}