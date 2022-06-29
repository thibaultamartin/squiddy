use std::fs;

use crate::client::Client;

mod bot;
mod bridge;
mod client;
mod iot;
mod other;
mod projects;
mod sdk;
mod server;
mod twim_config;

fn main() {
    const PROJECT_DATA_PATH: &str = "./data/projects.toml";
    const TWIM_CONFIG_PATH: &str = "../twim-config/config.toml";
    const MATRIXDOTORG_PROJECTS_PATH: &str = "../matrix.org/gatsby/content/projects";
    const MATRIXTO_PROJECTS_PATH: &str = "../matrix.to/src/open/clients";
    let projects_file = fs::read(PROJECT_DATA_PATH).expect("Unable to open master data file");

    let projects: projects::Projects =
        toml::from_slice(&projects_file).expect("Unable to parse master data file");

    let twim_config_file = fs::read(TWIM_CONFIG_PATH).expect("Unable to open twim config file");
    let mut twim_config: twim_config::Config =
        toml::from_slice(&twim_config_file).expect("Unable to parse twim-config file");

    let mut twim_projects_matched = 0;
    let mut twim_projects_added = 0;

    println!(
        "TWIM Config contains {} projects",
        twim_config.projects.len()
    );

    for bot in &projects.bots {
        let mut found = false;
        for twim_project in &mut twim_config.projects {
            if bot.title == twim_project.title {
                println!("Found {} in data and twim-config", twim_project.title);
                twim_projects_matched += 1;
                *twim_project = twim_config::Project::from(bot);
                found = true;
                break;
            }
        }

        if !found {
            println!("Didn't find {} in twim-config, adding", bot.title);
            twim_projects_added += 1;
            twim_config.projects.push(twim_config::Project::from(bot));
        }

        let matrixdotorg_project_path =
            format!("{}/bots/{}", MATRIXDOTORG_PROJECTS_PATH, bot.filename());
        fs::write(&matrixdotorg_project_path, bot.to_markdown()).unwrap_or_else(|_| {
            panic!("Could not write project file {}", matrixdotorg_project_path)
        });
    }

    for bridge in &projects.bridges {
        let mut found = false;
        for twim_project in &mut twim_config.projects {
            if bridge.title == twim_project.title {
                println!("Found {} in data and twim-config", twim_project.title);
                twim_projects_matched += 1;
                *twim_project = twim_config::Project::from(bridge);
                found = true;
                break;
            }
        }

        if !found {
            println!("Didn't find {} in twim-config, adding", bridge.title);
            twim_projects_added += 1;
            twim_config
                .projects
                .push(twim_config::Project::from(bridge));
        }

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
        let mut found = false;
        for twim_project in &mut twim_config.projects {
            if client.title == twim_project.title {
                println!("Found {} in data and twim-config", twim_project.title);
                twim_projects_matched += 1;
                *twim_project = twim_config::Project::from(client);
                found = true;
                break;
            }
        }

        if !found {
            println!("Didn't find {} in twim-config, adding", client.title);
            twim_projects_added += 1;
            twim_config
                .projects
                .push(twim_config::Project::from(client));
        }

        let matrixdotorg_project_path = format!(
            "{}/clients/{}",
            MATRIXDOTORG_PROJECTS_PATH,
            client.filename()
        );
        fs::write(&matrixdotorg_project_path, client.to_markdown()).unwrap_or_else(|_| {
            panic!("Could not write project file {}", matrixdotorg_project_path)
        });

        let matrixto_project_path = format!(
            "{}/{}-data.js",
            MATRIXTO_PROJECTS_PATH,
            client.matrixto_filename()
        );
        fs::write(
            &matrixto_project_path,
            Client::matrixto_join_file(client.id.clone(), projects.clients.clone()),
        )
        .unwrap_or_else(|_| panic!("Could not write project file {}", matrixto_project_path));

        // TODO update index.js after the loop
    }

    for iot in &projects.iots {
        let mut found = false;
        for twim_project in &mut twim_config.projects {
            if iot.title == twim_project.title {
                println!("Found {} in data and twim-config", twim_project.title);
                twim_projects_matched += 1;
                *twim_project = twim_config::Project::from(iot);
                found = true;
                break;
            }
        }

        if !found {
            println!("Didn't find {} in twim-config, adding", iot.title);
            twim_projects_added += 1;
            twim_config.projects.push(twim_config::Project::from(iot));
        }

        let matrixdotorg_project_path =
            format!("{}/iot/{}", MATRIXDOTORG_PROJECTS_PATH, iot.filename());
        fs::write(&matrixdotorg_project_path, iot.to_markdown()).unwrap_or_else(|_| {
            panic!("Could not write project file {}", matrixdotorg_project_path)
        });
    }

    for other in &projects.others {
        let mut found = false;
        for twim_project in &mut twim_config.projects {
            if other.title == twim_project.title {
                println!("Found {} in data and twim-config", twim_project.title);
                twim_projects_matched += 1;
                *twim_project = twim_config::Project::from(other);
                found = true;
                break;
            }
        }

        if !found {
            println!("Didn't find {} in twim-config, adding", other.title);
            twim_projects_added += 1;
            twim_config.projects.push(twim_config::Project::from(other));
        }

        let matrixdotorg_project_path =
            format!("{}/other/{}", MATRIXDOTORG_PROJECTS_PATH, other.filename());
        fs::write(&matrixdotorg_project_path, other.to_markdown()).unwrap_or_else(|_| {
            panic!("Could not write project file {}", matrixdotorg_project_path)
        });
    }

    for sdk in &projects.sdks {
        let mut found = false;
        for twim_project in &mut twim_config.projects {
            if sdk.title == twim_project.title {
                println!("Found {} in data and twim-config", twim_project.title);
                twim_projects_matched += 1;
                *twim_project = twim_config::Project::from(sdk);
                found = true;
                break;
            }
        }

        if !found {
            println!("Didn't find {} in twim-config, adding", sdk.title);
            twim_projects_added += 1;
            twim_config.projects.push(twim_config::Project::from(sdk));
        }

        let matrixdotorg_project_path =
            format!("{}/sdks/{}", MATRIXDOTORG_PROJECTS_PATH, sdk.filename());
        fs::write(&matrixdotorg_project_path, sdk.to_markdown()).unwrap_or_else(|_| {
            panic!("Could not write project file {}", matrixdotorg_project_path)
        });
    }

    for server in &projects.servers {
        let mut found = false;
        for twim_project in &mut twim_config.projects {
            if server.title == twim_project.title {
                println!("Found {} in data and twim-config", twim_project.title);
                twim_projects_matched += 1;
                *twim_project = twim_config::Project::from(server);
                found = true;
                break;
            }
        }

        if !found {
            println!("Didn't find {} in twim-config, adding", server.title);
            twim_projects_added += 1;
            twim_config
                .projects
                .push(twim_config::Project::from(server));
        }

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

    println!(
        "{} of them are not known in the meta repository",
        twim_config.projects.len() - twim_projects_matched
    );

    println!(
        "TWIM-Config now contains {} projects",
        twim_config.projects.len()
    );
    println!("{} of them were updated", twim_projects_matched);
    println!("{} were just added", twim_projects_added);
}
