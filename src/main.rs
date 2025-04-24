use std::{fs, path::Path};

use tracing::{info, warn};

use crate::client::Client;
use crate::git::clone_repositories;
use crate::twim_config::Project;

mod bot;
mod bridge;
mod client;
mod iot;
mod other;
mod projects;
mod sdk;
mod server;
mod twim_config;
mod git;

const PROJECT_DATA_PATH: &str = "./data/projects.toml";
const REPOSITORIES_CONFIG_PATH: &str = "./data/repositories.toml";
const TWIM_CONFIG_PATH: &str = "../twim-config/config.toml";
const MATRIXDOTORG_PROJECTS_PATH: &str = "../matrix.org/gatsby/content/projects";
const MATRIXTO_PROJECTS_PATH: &str = "../matrix.to/src/open/clients";

fn main() {
    tracing_subscriber::fmt::init();

    // Get locally stored projects data
    let projects_file = fs::read_to_string(PROJECT_DATA_PATH).expect("Unable to open master data file");

    let projects: projects::Projects =
        toml::from_str(&projects_file).expect("Unable to parse master data file");

    // Prepare fetching the repositories and projects data
    let repositories_file =
        fs::read_to_string(REPOSITORIES_CONFIG_PATH).expect("Unable to open repositories file");

    let repositories: projects::Repositories =
        toml::from_str(&repositories_file).expect("Unable to parse repositories file");

    let remote_projects = clone_repositories(repositories);

    // TODO: Merge the fetched data on top of the existing data

    process_projects(projects);
}

// Check if the project is already in the twim-config
fn insert_or_update_twim_config(twim_config: &mut twim_config::Config, project: Project, twim_projects_matched: &mut usize, twim_projects_added: &mut usize) {
    let mut found = false;
    for twim_project in &mut twim_config.projects {
        if twim_project.title == project.title {
            *twim_project = project.clone();
            found = true;
            break;
        }
    }

    if !found {
        warn!("Didn't find {} in twim-config, adding", project.title);
        *twim_projects_added += 1;
        twim_config
            .projects
            .push(project);
    } else {
        *twim_projects_matched += 1;
    }
}

fn process_projects(projects: projects::Projects) {
    let twim_config_file = fs::read_to_string(TWIM_CONFIG_PATH).expect("Unable to open twim config file");
    let mut twim_config: twim_config::Config =
        toml::from_str(&twim_config_file).expect("Unable to parse twim-config file");

    let mut twim_projects_matched = 0;
    let mut twim_projects_added = 0;

    info!(
        "TWIM Config contains {} projects",
        twim_config.projects.len()
    );

    for bot in &projects.bots {
        insert_or_update_twim_config(&mut twim_config, twim_config::Project::from(bot), &mut twim_projects_matched, &mut twim_projects_added);

        let matrixdotorg_project_path =
            format!("{}/bots/{}", MATRIXDOTORG_PROJECTS_PATH, bot.filename());
        fs::write(&matrixdotorg_project_path, bot.to_markdown()).unwrap_or_else(|_| {
            panic!("Could not write project file {}", matrixdotorg_project_path)
        });
    }

    for bridge in &projects.bridges {
        insert_or_update_twim_config(&mut twim_config, twim_config::Project::from(bridge), &mut twim_projects_matched, &mut twim_projects_added);

        let matrixdotorg_project_path = format!(
            "{}/bridges/{}",
            MATRIXDOTORG_PROJECTS_PATH,
            bridge.filename()
        );
        fs::write(&matrixdotorg_project_path, bridge.to_markdown()).unwrap_or_else(|_| {
            panic!("Could not write project file {}", matrixdotorg_project_path)
        });
    }

    for client in &projects.clients {
        insert_or_update_twim_config(&mut twim_config, twim_config::Project::from(client), &mut twim_projects_matched, &mut twim_projects_added);

        let matrixdotorg_project_path = format!(
            "{}/clients/{}",
            MATRIXDOTORG_PROJECTS_PATH,
            client.filename()
        );
        fs::write(&matrixdotorg_project_path, client.to_markdown()).unwrap_or_else(|_| {
            panic!("Could not write project file {}", matrixdotorg_project_path)
        });

        let matrixto_file_path = format!(
            "{}/{}.js",
            MATRIXTO_PROJECTS_PATH,
            client.matrixto_filename()
        );

        let matrixto_data_file_path = format!(
            "{}/{}-data.js",
            MATRIXTO_PROJECTS_PATH,
            client.matrixto_filename()
        );
        fs::write(
            &matrixto_data_file_path,
            Client::matrixto_join_file(client.id.clone(), projects.clients.clone()),
        )
            .unwrap_or_else(|_| {
                panic!(
                    "Could not write project data file {}",
                    matrixto_data_file_path
                )
            });

        if !Path::new(&matrixto_file_path).exists() {
            fs::write(&matrixto_file_path, client.matrixto_template_file())
                .unwrap_or_else(|_| panic!("Could not write project file {}", matrixto_file_path));
        }
    }

    let matrixto_index_path = format!("{}/index.js", MATRIXTO_PROJECTS_PATH);
    fs::write(
        &matrixto_index_path,
        Client::matrixto_index(projects.clients.clone()),
    )
        .unwrap_or_else(|_| panic!("Could not write matrix.to index.js file"));

    for iot in &projects.iots {
        insert_or_update_twim_config(&mut twim_config, twim_config::Project::from(iot), &mut twim_projects_matched, &mut twim_projects_added);

        let matrixdotorg_project_path =
            format!("{}/iot/{}", MATRIXDOTORG_PROJECTS_PATH, iot.filename());
        fs::write(&matrixdotorg_project_path, iot.to_markdown()).unwrap_or_else(|_| {
            panic!("Could not write project file {}", matrixdotorg_project_path)
        });
    }

    for other in &projects.others {
        insert_or_update_twim_config(&mut twim_config, twim_config::Project::from(other), &mut twim_projects_matched, &mut twim_projects_added);

        let matrixdotorg_project_path =
            format!("{}/other/{}", MATRIXDOTORG_PROJECTS_PATH, other.filename());
        fs::write(&matrixdotorg_project_path, other.to_markdown()).unwrap_or_else(|_| {
            panic!("Could not write project file {}", matrixdotorg_project_path)
        });
    }

    for sdk in &projects.sdks {
        insert_or_update_twim_config(&mut twim_config, twim_config::Project::from(sdk), &mut twim_projects_matched, &mut twim_projects_added);

        let matrixdotorg_project_path =
            format!("{}/sdks/{}", MATRIXDOTORG_PROJECTS_PATH, sdk.filename());
        fs::write(&matrixdotorg_project_path, sdk.to_markdown()).unwrap_or_else(|_| {
            panic!("Could not write project file {}", matrixdotorg_project_path)
        });
    }

    for server in &projects.servers {
        insert_or_update_twim_config(&mut twim_config, twim_config::Project::from(server), &mut twim_projects_matched, &mut twim_projects_added);

        let matrixdotorg_project_path = format!(
            "{}/servers/{}",
            MATRIXDOTORG_PROJECTS_PATH,
            server.filename()
        );
        fs::write(&matrixdotorg_project_path, server.to_markdown()).unwrap_or_else(|_| {
            panic!("Could not write project file {}", matrixdotorg_project_path)
        });
    }

    fs::write(TWIM_CONFIG_PATH, toml::to_string(&twim_config).unwrap())
        .expect("Unable to write to twim-config");

    info!(
        "{} of them are not known in the meta repository",
        twim_config.projects.len() - twim_projects_matched
    );

    info!(
        "TWIM-Config now contains {} projects",
        twim_config.projects.len()
    );
    info!("{} of them were updated", twim_projects_matched);
    info!("{} were just added", twim_projects_added);
}