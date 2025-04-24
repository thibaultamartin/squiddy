//! This file handles fetching the project data from remote repositories.
use git2::Repository;
use libappstream::{FormatKind, Metadata, UrlKind};
use libappstream::prelude::{ComponentExt, MetadataExt};
use tempfile::TempDir;
use tracing::{error, info};

use crate::{bot, client, iot, other, sdk, server};
use crate::client::Features;
use crate::projects::{Maturity, Projects, Repositories, RepositoryMetdata};

pub fn clone_repositories(repositories: Repositories) -> Projects {
    let mut remote_projects = Projects::default();

    for repository_metadata in repositories.repositories {
        info!("Cloning repository: {}", repository_metadata.url);
        let tmp_dir = TempDir::new();

        match tmp_dir {
            Ok(dir) => {
                let repo = match Repository::clone(&repository_metadata.url, &dir) {
                    Ok(repo) => repo,
                    Err(e) => {
                        error!("failed to clone: {}", e);
                        continue;
                    }
                };

                info!("Cloned repository: {}", repo.path().display());

                process_repository(&repository_metadata, &repo, &mut remote_projects);

                info!("Processed repository: {}", repo.path().display());

                info!("Removing temp dir: {}", dir.path().display());
                if let Err(e) = dir.close() {
                    error!("failed to remove temp dir: {}", e);
                }
            }
            Err(e) => error!("failed to create temp dir: {}", e),
        }
    }
    remote_projects
}

fn process_repository(repository_metadata: &RepositoryMetdata, repo: &Repository, projects: &mut Projects) {
    // If we have an appstream we parse it
    if let Some(appstream_location) = &repository_metadata.appstream_location {
        info!("Processing appstream: {}", appstream_location);
        // Repo path
        let repo_path = repo.path().parent().unwrap();
        let appstream_path = repo_path.join(appstream_location);

        let appstream = Metadata::new();
        let gio_file = gio::File::for_path(&appstream_path);
        if let Err(e) = appstream.parse_file(&gio_file, FormatKind::Unknown) {
            error!("failed to parse appstream: {}", e);
            return;
        }

        // Convert the appstream to a project
        from_appstream(appstream, projects);
    }
}

fn from_appstream(appstream: Metadata, projects: &mut Projects) {
    // Get component
    let component = appstream.component();
    match component {
        Some(component) => {
            let license = component.project_license();
            let name = component.name();
            let summary = component.summary();
            let description = component.description();
            let releases = component.load_releases(true);
            let matrix_type = component.custom_value("matrix_type");
            let matrix_room = component.custom_value("matrix_room");
            let repository_url = component.url(UrlKind::VcsBrowser);
            let homepage = component.url(UrlKind::Homepage);

            match matrix_type {
                Some(matrix_type) => {
                    // Construct the different project types based on the matrix_type
                    match matrix_type.as_str() {
                        "bot" => {
                            let bot = bot::Bot {
                                title: name.unwrap().to_string(),
                                description: summary.clone().unwrap().to_string(),
                                repository: Some(repository_url.unwrap().to_string()),
                                home: Some(homepage.unwrap().to_string()),
                                screenshot: None,
                                icon: None,
                                room: Some(matrix_room.unwrap().to_string()),
                                featured: false,
                                sort_order: None,
                                authors: vec![],
                                maturity: "".to_string(),
                                language: "".to_string(),
                                license: license.unwrap().to_string(),
                                full_description: description.unwrap().to_string(),
                            };
                            projects.bots.push(bot);
                        }
                        // SDK project type
                        "sdk" => {
                            let sdk = sdk::Sdk {
                                title: name.unwrap().to_string(),
                                description: summary.clone().unwrap().to_string(),
                                repository: Some(repository_url.unwrap().to_string()),
                                home: Some(homepage.unwrap().to_string()),
                                screenshot: None,
                                icon: None,
                                room: Some(matrix_room.unwrap().to_string()),
                                featured: false,
                                sort_order: None,
                                authors: vec![],
                                maturity: "".to_string(),
                                language: "".to_string(),
                                license: license.unwrap().to_string(),
                                full_description: description.unwrap().to_string(),
                            };
                            projects.sdks.push(sdk);
                        }
                        // Server project type
                        "server" => {
                            let server = server::Server {
                                title: name.unwrap().to_string(),
                                description: summary.clone().unwrap().to_string(),
                                repository: Some(repository_url.unwrap().to_string()),
                                home: Some(homepage.unwrap().to_string()),
                                screenshot: None,
                                icon: None,
                                room: Some(matrix_room.unwrap().to_string()),
                                featured: false,
                                sort_order: None,
                                authors: vec![],
                                maturity: "".to_string(),
                                language: "".to_string(),
                                license: license.unwrap().to_string(),
                                full_description: description.unwrap().to_string(),
                            };
                            projects.servers.push(server);
                        }
                        // iot project type
                        "iot" => {
                            let iot = iot::Iot {
                                title: name.unwrap().to_string(),
                                description: summary.clone().unwrap().to_string(),
                                repository: Some(repository_url.unwrap().to_string()),
                                home: Some(homepage.unwrap().to_string()),
                                screenshot: None,
                                icon: None,
                                room: Some(matrix_room.unwrap().to_string()),
                                featured: false,
                                sort_order: None,
                                authors: vec![],
                                maturity: "".to_string(),
                                language: "".to_string(),
                                license: license.unwrap().to_string(),
                                full_description: description.unwrap().to_string(),
                            };
                            projects.iots.push(iot);
                        }
                        // Client project type
                        "client" => {
                            let client = client::Client {
                                id: "".to_string(),
                                title: name.unwrap().to_string(),
                                slug: None,
                                description: summary.clone().unwrap().to_string(),
                                repository: Some(repository_url.unwrap().to_string()),
                                home: Some(homepage.unwrap().to_string()),
                                screenshot: None,
                                icon: None,
                                room: Some(matrix_room.unwrap().to_string()),
                                sdk: vec![],
                                platforms: vec![],
                                featured: false,
                                sort_order: None,
                                features: Features {
                                    e2ee: "".to_string(),
                                    widgets: "".to_string(),
                                    spaces: "".to_string(),
                                    room_directory: "".to_string(),
                                    read_receipts: "".to_string(),
                                    typing_indicators: "".to_string(),
                                    edits: "".to_string(),
                                    replies: "".to_string(),
                                    threads: "".to_string(),
                                    attachments: "".to_string(),
                                    multi_account: "".to_string(),
                                    registration: "".to_string(),
                                    calls: "".to_string(),
                                    reactions: "".to_string(),
                                    sso: "".to_string(),
                                    localised: "".to_string(),
                                },
                                appstore_details: None,
                                apple_associated_app_id: None,
                                playstore_app_id: None,
                                fdroid_app_id: None,
                                flathub_app_id: None,
                                authors: vec![],
                                maturity: Maturity::Alpha,
                                language: "".to_string(),
                                license: license.unwrap().to_string(),
                                full_description: description.unwrap().to_string(),
                                otherinstall_link: None,
                            };
                            projects.clients.push(client);
                        }
                        // Other project types
                        _ => {
                            let other = other::Other {
                                title: name.unwrap().to_string(),
                                description: summary.clone().unwrap().to_string(),
                                repository: Some(repository_url.unwrap().to_string()),
                                home: Some(homepage.unwrap().to_string()),
                                screenshot: None,
                                icon: None,
                                room: Some(matrix_room.unwrap().to_string()),
                                featured: false,
                                sort_order: None,
                                authors: vec![],
                                maturity: "".to_string(),
                                language: "".to_string(),
                                license: license.unwrap().to_string(),
                                full_description: description.unwrap().to_string(),
                            };
                            projects.others.push(other);
                        }
                    }
                }
                None => {
                    error!("No matrix_type found in appstream. This is required");
                }
            }
        }
        None => {
            error!("No component found in appstream");
        }
    }
}