extern crate log;

use std::fs::File;
use std::io::Read;

mod bot;
mod bridge;
mod client;
mod projects;
mod iot;
mod other;
mod sdk;
mod server;

fn main() {
    // 1. Open and parse the toml containing all data
    const PROJECT_DATA_PATH: &str = "./projects.toml";
    let mut file = File::open(PROJECT_DATA_PATH)
        .expect("Unable to open file");
    let mut template = String::new();
    file.read_to_string(&mut template)
        .expect("Unable to read file");

    let projects: projects::Projects = toml::from_str(&template)
        .expect("Unable to parse config file");

    // 2. Push to twim-config
    //   a. Open & parse twim-config toml file, and for each project:
    //   b. Find & update (or add) the entry
    //   c. Write the file to disk

    // 3. Push to matrix.org
    //   a. For each project, generate the markdown content
    //   b. Find the file for the project, or create it if it doesn't exist
    //   c. Override content of the file with generated markdown

    // 4. Push to matrix.to
    //   a. For each project, find the project's file or create one if needed
    //   b. Find the following functions and update their body
    //     i.    id()
    //     ii.   platforms()
    //     iii.  icon()
    //     iv.   name()
    //     v.    desccription()
    //     vi.   homepage()
    //     vii.  author()
    //     viii. getMaturity()
    //     ix.   getInstallLinks()
    //   c. Write the file to disk
}
