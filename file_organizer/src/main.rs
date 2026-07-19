use std::{env, ffi, fs, time};

mod traits;
use traits::print_format::fmt_to_print;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: file_organizer <directory>");
        return;
    }
    println!("Args are: {}, {}", args[0], args[1]);
    let directory = &args[1];
    let mut complete_list: Vec<(String, String, String, String)> = Vec::new();

    let dir_entries = fs::read_dir(directory).unwrap();
    for entry in dir_entries {
        let entry = entry.unwrap_or_else(|error| {
            panic!("Faced error getting directory entry: {error}");
        });
        let name = entry.file_name();
        let file_type = entry.file_type().unwrap_or_else(|error| {
            panic!("Error reading file type: {error}");
        });
        let metadata = entry.metadata().unwrap_or_else(|error| {
            panic!("Error reading file metadata: {error}");
        });
        let file_size = metadata.len();
        let last_modified = metadata.modified().unwrap();
        let file_stats = (name, file_type, file_size, last_modified);
        add_to_list(&mut complete_list, file_stats);
    }
    let name_width = calculate_name_width(&complete_list);

    sort_by_name(&mut complete_list);
    print_table(&complete_list, name_width);

    sort_by_size(&mut complete_list);
    print_table(&complete_list, name_width);

    sort_by_last_modified(&mut complete_list, SortOrder::Ascending);
    print_table(&complete_list, name_width);

    sort_by_last_modified(&mut complete_list, SortOrder::Descending);
    print_table(&complete_list, name_width);
}

fn print_table_header(name_width: usize) {
    print_bar(name_width);
    println!(
        "Name{:<w$} Type       Size       Last modified",
        "",
        w = name_width - 4
    );
    print_bar(name_width);
}

fn print_table(list: &Vec<(String, String, String, String)>, name_width: usize) {
    print_table_header(name_width);
    for item in list {
        println!(
            "{:<name_width$} {:<10} {:<10} {}",
            item.0,
            item.1,
            item.2,
            item.3,
            name_width = name_width
        );
    }
    print_bar(name_width);
}

fn print_bar(name_width: usize) {
    println!(
        "----------------------------------------------{:-<w$}",
        "",
        w = name_width - 4
    );
}

fn calculate_name_width(list: &Vec<(String, String, String, String)>) -> usize {
    list.iter().map(|item| item.0.len()).max().unwrap_or(20)
}

fn add_to_list(
    list: &mut Vec<(String, String, String, String)>,
    file_stats: (ffi::OsString, fs::FileType, u64, time::SystemTime),
) {
    list.push((
        fmt_to_print(file_stats.0),
        fmt_to_print(file_stats.1),
        fmt_to_print(file_stats.2),
        fmt_to_print(file_stats.3),
    ));
}

fn sort_by_name(list: &mut Vec<(String, String, String, String)>) {
    list.sort_by(|a, b| a.0.cmp(&b.0))
}

fn sort_by_size(list: &mut Vec<(String, String, String, String)>) {
    list.sort_by(|a, b| a.2.cmp(&b.2))
}

enum SortOrder {
    Ascending,
    Descending,
}

fn sort_by_last_modified(list: &mut Vec<(String, String, String, String)>, order: SortOrder) {
    match order {
        SortOrder::Ascending => list.sort_by(|a, b| a.3.cmp(&b.3)),
        SortOrder::Descending => list.sort_by(|a, b| b.3.cmp(&a.3)),
    }
}
