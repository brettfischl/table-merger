use std::env;
use std::fs;

#[derive(Clone)]
struct Column {
    name: String,
    datatype: String,
}

#[derive(Clone)]
struct Table {
    name: String,
    columns: Vec<String>,
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let target_table_file = &args[2];
    println!("In file {}", target_table_file);

    let source_table_dir = &args[3];
    println!("source tables {}", source_table_dir);

    let target_table = read_sql_file(target_table_file);


    let paths = fs::read_dir(source_table_dir).unwrap();
    for path in paths {
        let table = read_sql_file(&path.unwrap().path().display().to_string());

        println!("Table name: {}", table.name);
        let (matched_columns, unmatched_columns) = matching_columns(table, target_table.clone());

        println!("UNMATCHED COLUMNS: {}", unmatched_columns.join(",\n"));
        println!("MATCHED COLUMNS: {}", matched_columns.join(",\n"));
    }
}

fn read_sql_file(file_path: &String) -> Table {
    let fp = file_path;
    let dll = fs::read_to_string(fp.clone())
        .expect("Something went wrong reading the file");

    let table = Table {
        name: fp.to_string(),
        columns: parse_dll(dll)
    };

    return table
}

fn parse_dll(dll:String) -> Vec<String> {
    let mut columns = Vec::new();

    let dll_lines = dll.lines();
    for mut line in dll_lines {

        match line {
            ");" => continue,
            "" => continue,
            line if line.starts_with("CREATE TABLE") => continue,
            _ => ""
        };

        let line_parts = line.split_whitespace().collect::<Vec<&str>>();
        line = line_parts[0];
        
        columns.push(line.to_string());
    }

    return columns
}

fn matching_columns(src_table: Table, dest_table: Table) -> (Vec<String>, Vec<String>) {
    let mut matched_columns = Vec::new();
    let mut unmatched_columns = Vec::new();

    for src_col in src_table.columns.clone() {
        if dest_table.columns.contains(&src_col) {
            matched_columns.push(src_col);
        } else {
            unmatched_columns.push(src_col);
        }
    }

    for dest_col in dest_table.columns {
        if !src_table.columns.contains(&dest_col) {
            unmatched_columns.push(dest_col);
        }
    }

    return (matched_columns, unmatched_columns);
}