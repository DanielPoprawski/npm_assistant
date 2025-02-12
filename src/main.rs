use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let path = Path::new("E:\\");

    let mut folders = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() || !path.join("package.json").exists() {
            continue;
        }
        if let Some(name) = path.file_name() {
            let name_str = name.to_string_lossy();
            if name_str == "production" || name_str == "server-production" {
                continue;
            }
            folders.push((name_str.into_owned(), path));
        }
    }

    if folders.is_empty() {
        println!("Folder is empty");
        return Ok(());
    }

    for (i, (name, _)) in folders.iter().enumerate() {
        println!("{}. {}", i + 1, name);
    }

    println!("\nEnter the number of the project (1-{}):", folders.len());
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    if let Ok(selection) = input.trim().parse::<usize>() {
        if selection > 0 && selection <= folders.len() {
            let selected = &folders[selection - 1];
            println!("Selected: {}", selected.0);
            let mut child = std::process::Command::new("cmd")
                .args([
                    "/K",
                    "cd",
                    "/D",
                    "E:",
                    "&&",
                    "cd",
                    selected.1.to_str().unwrap(),
                    "&&",
                    "npm",
                    "run",
                    "dev",
                ])
                .spawn()?;

            match child.wait() {
                Ok(status) => {
                    if !status.success() {
                        println!("Failed to run npm dev");
                    }
                }
                Err(e) => println!("Error waiting for process: {}", e),
            }
        } else {
            println!("Invalid selection");
        }
    } else {
        println!("Invalid input");
    }

    Ok(())
}
