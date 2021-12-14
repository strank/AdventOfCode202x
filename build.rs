/// generate a source file that loads all rs files found in certain subdirectories
/// as modules and exposes their run functions
use glob::glob;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;

fn write_generated_module(
    file: &mut fs::File,
    entries: Vec<String>,
    filemap: HashMap<String, Vec<PathBuf>>,
) -> Result<(), io::Error> {
    writeln!(file, "// GENERATED BY CARGO BUILD SCRIPT ../build.rs")?;
    writeln!(file, "// DO NOT EDIT!")?;
    writeln!(file)?;
    for year_entry in &entries {
        writeln!(file, "// year {}", year_entry)?;
        for path_buf in &filemap[year_entry] {
            let path_str = path_buf.to_str().unwrap().replace("\\", "/");
            writeln!(file, "#[path = \"../{}\"]", path_str)?;
            writeln!(
                file,
                "mod year{}{};",
                year_entry,
                path_str.get(5..10).unwrap()
            )?;
        }
    }
    writeln!(file)?;
    writeln!(file, "pub fn get_years() -> [usize; {}] {{", entries.len())?;
    writeln!(file, "    [{}]", entries.join(", "))?;
    writeln!(file, "}}")?;
    writeln!(file)?;
    writeln!(file, "/// Return an array of 26 run functions")?;
    writeln!(file, "/// (26 rather than 25 for clean 1-based indexing)")?;
    writeln!(
        file,
        "pub fn get_days(year: usize) -> [Option<fn() -> ()>; 26] {{"
    )?;
    writeln!(
        file,
        "    let mut days: [Option<fn() -> ()>; 26] = [None; 26];"
    )?;
    writeln!(file, "    match year {{")?;
    for year_entry in &entries {
        writeln!(file, "        {} => {{", year_entry)?;
        for path_buf in &filemap[year_entry] {
            let path_str = path_buf.to_str().unwrap().replace("\\", "/");
            writeln!(
                file,
                "            days[{}] = Some(year{}{}::run);",
                path_str.get(8..10).unwrap().trim_start_matches('0'),
                year_entry,
                path_str.get(5..10).unwrap()
            )?;
        }
        writeln!(file, "        }}")?;
    }
    writeln!(file, "        _ => {{}}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "    days")?;
    writeln!(file, "}}")?;
    Ok(())
}

fn generate_module(
    entries: Vec<String>,
    filemap: HashMap<String, Vec<PathBuf>>,
) -> Result<(), io::Error> {
    let mut module = fs::File::create("src/generated.rs")?;
    write_generated_module(&mut module, entries, filemap)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let mut entries: Vec<String> = fs::read_dir(".")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?
        // filter to leave only directories whose names are year-like integers:
        .into_iter()
        .filter(|e| {
            e.is_dir()
                && e.file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap_or_default()
                    > 1900
        })
        .map(|e| e.file_name().unwrap().to_str().unwrap().to_owned())
        .collect();
    // Ensure alphabetical ordering
    entries.sort();
    // get a list of rust modules for each of the year directories:
    let mod_files: HashMap<_, _> = entries
        .iter()
        .filter_map(|e| {
            let year = e.to_owned();
            let glob_result = glob(&(e.to_owned() + "/**/*.rs"));
            match glob_result {
                Ok(paths) => {
                    let path_list: Vec<_> = paths
                        .filter_map(|p| match p {
                            Ok(pb) => Some(pb),
                            _ => None,
                        })
                        .collect();
                    if !path_list.is_empty() {
                        Some((year, path_list))
                    } else {
                        None
                    }
                }
                _ => None,
            }
        })
        .collect();
    for (year_entry, path_vec) in &mod_files {
        println!("cargo:rerun-if-changed={}", year_entry);
        // rerun-if-changed should check down the directory, but just in case:
        for path_buf in path_vec {
            println!("cargo:rerun-if-changed={}", path_buf.display());
        }
    }
    if let Err(e) = generate_module(entries, mod_files) {
        eprintln!("Error: {}", e);
    }
    Ok(())
}
