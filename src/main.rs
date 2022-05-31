extern crate log;

use std::fs::{File, self};

mod bot;
mod bridge;
mod client;
mod projects;
mod iot;
mod other;
mod sdk;
mod server;
mod twim_config;

fn main() {
    // 1. Open and parse the toml containing all data
    const PROJECT_DATA_PATH: &str = "./projects.toml";
    const TWIM_CONFIG_PATH: &str = "../twim-config/config.toml";
    let projects_file = fs::read(PROJECT_DATA_PATH)
        .expect("Unable to open master data file");

    let projects: projects::Projects = toml::from_slice(&projects_file)
        .expect("Unable to parse master data file");

    // 2. Push to twim-config
    //   a. Open & parse twim-config toml file, and for each project:
    //   b. Find & update (or add) the entry
    //   c. Write the file to disk
    let  twim_config_file = fs::read(TWIM_CONFIG_PATH)
        .expect("Unable to open twim config file");
    let mut twim_config: twim_config::Config = toml::from_slice(&twim_config_file)
        .expect("Unable to parse twim-config file");
    
    let mut projects_matched = 0;
    for twim_project in &mut twim_config.projects {
        for bot in &projects.bots {
            if bot.title == twim_project.title {
                println!("Found {} in data and twim-config", twim_project.title);
                projects_matched = projects_matched + 1;
                *twim_project = twim_config::Project::from_bot(&bot);
                break;
            }
        }
        for bridge in &projects.bridges {
            if bridge.title == twim_project.title {
                println!("Found {} in data and twim-config", twim_project.title);
              projects_matched = projects_matched + 1;
              break;
            }
        }
    }

    println!("TWIM Config contains {} projects", twim_config.projects.len());
    println!("{} of them are not known in the meta repository", twim_config.projects.len() - projects_matched);
    println!("{} of them were updated", projects_matched);

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
