use chrono::{DateTime, Utc};
use std::{ffi::OsString, fs::FileType, time::SystemTime};

pub trait PrintFormat {
    fn display(self) -> String;
}

impl PrintFormat for OsString {
    fn display(self) -> String {
        if let Ok(val) = self.clone().into_string() {
            val.to_owned()
        } else {
            self.to_string_lossy().to_string()
        }
    }
}

impl PrintFormat for FileType {
    fn display(self) -> String {
        let file_type_str = if self.is_dir() {
            "dir"
        } else if self.is_file() {
            "file"
        } else if self.is_symlink() {
            "symlink"
        } else {
            panic!("Not a dir or a file. WEIRD!!");
        };
        String::from(file_type_str)
    }
}

impl PrintFormat for u64 {
    fn display(self) -> String {
        let mut bytes = self as f32;
        let units = vec!["B", "KB", "MB"];
        for unit in units {
            if bytes < 1024f32 {
                return format!("{:0.0} {}", bytes, unit);
            }
            bytes /= 1024f32;
        }
        format!("{}GB", bytes)
    }
}

impl PrintFormat for SystemTime {
    fn display(self) -> String {
        let fmt_datetime: DateTime<Utc> = self.into();
        fmt_datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

pub fn fmt_to_print<T: PrintFormat>(item: T) -> String {
    item.display()
}
