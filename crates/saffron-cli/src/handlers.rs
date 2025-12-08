use crate::cli::*;
use crate::history::{HistoryEntry, HistoryRequest, HistoryResponse};
use crate::output::*;
use crate::storage::Storage;
use colored::Colorize;
use saffron_core::domain::collection::{Collection, SavedRequest, SerializableRequest};
use saffron_core::domain::environment::Environment;
use saffron_core::domain::request::{HttpMethod, HttpRequest, RequestBody};
use saffron_http::{HttpClient, HttpClientConfig};
use std::collections::HashMap;
use std::time::Instant;

#[allow(clippy::too_many_arguments)]
pub fn handle_send(
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
    json: Option<String>,
    data: Vec<(String, String)>,
    timeout: Option<u64>,
    follow_redirects: bool,
    env: Option<String>,
    verbose: bool,
) {
    let storage = match Storage::new() {
        Ok(s) => s,
        Err(e) => {
            print_error(&format!("Failed to initialize storage: {}", e));
            return;
        }
    };

    let env_set = storage.load_environment_set().unwrap_or_default();
    let resolved_url = if let Some(ref env_name) = env {
        if let Some(environment) = env_set.get(env_name) {
            environment.resolve_template(&url)
        } else {
            print_error(&format!("Environment '{}' not found", env_name));
            return;
        }
    } else {
        url
    };

    let http_method = match method.to_uppercase().as_str() {
        "GET" => HttpMethod::Get,
        "POST" => HttpMethod::Post,
        "PUT" => HttpMethod::Put,
        "PATCH" => HttpMethod::Patch,
        "DELETE" => HttpMethod::Delete,
        "HEAD" => HttpMethod::Head,
        "OPTIONS" => HttpMethod::Options,
        _ => {
            print_error(&format!("Invalid HTTP method: {}", method));
            return;
        }
    };

    let mut request = HttpRequest::new(http_method, &resolved_url);

    for (key, value) in headers {
        let resolved_key = if let Some(ref env_name) = env {
            if let Some(environment) = env_set.get(env_name) {
                environment.resolve_template(&key)
            } else {
                key
            }
        } else {
            key
        };

        let resolved_value = if let Some(ref env_name) = env {
            if let Some(environment) = env_set.get(env_name) {
                environment.resolve_template(&value)
            } else {
                value
            }
        } else {
            value
        };

        request = request.with_header(&resolved_key, &resolved_value);
    }

    if let Some(json_body) = json {
        let resolved_body = if let Some(ref env_name) = env {
            if let Some(environment) = env_set.get(env_name) {
                environment.resolve_template(&json_body)
            } else {
                json_body
            }
        } else {
            json_body
        };
        request = request.with_json_body(&resolved_body);
    } else if !data.is_empty() {
        let form_data: HashMap<String, String> = data.into_iter().collect();
        request = request.with_body(RequestBody::FormUrlEncoded(form_data));
    } else if let Some(text_body) = body {
        let resolved_body = if let Some(ref env_name) = env {
            if let Some(environment) = env_set.get(env_name) {
                environment.resolve_template(&text_body)
            } else {
                text_body
            }
        } else {
            text_body
        };
        request = request.with_text_body(&resolved_body);
    }

    if let Some(t) = timeout {
        request = request.with_timeout(t);
    }

    request = request.follow_redirects(follow_redirects);

    let config = HttpClientConfig {
        timeout_seconds: timeout.unwrap_or(30),
        follow_redirects,
        ..Default::default()
    };

    let client = HttpClient::with_config(config);

    let start = Instant::now();
    match client.send(&request) {
        Ok(response) => {
            let duration_ms = start.elapsed().as_millis() as u64;

            let history_request = HistoryRequest {
                method: request.method.as_str().to_string(),
                url: request.url.clone(),
                headers: request
                    .headers
                    .iter()
                    .map(|h| (h.name.clone(), h.value.clone()))
                    .collect(),
                body: match &request.body {
                    RequestBody::None => None,
                    RequestBody::Text(t) => Some(t.clone()),
                    RequestBody::Json(j) => Some(j.clone()),
                    _ => Some("<complex body>".to_string()),
                },
            };

            let history_response = HistoryResponse::from_response(&response);
            let entry = HistoryEntry::new(history_request, history_response, duration_ms);

            if let Err(e) = storage.save_history_entry(&entry) {
                eprintln!("Warning: Failed to save to history: {}", e);
            }

            print_response(&response, verbose);
        }
        Err(e) => print_error(&format!("Request failed: {}", e)),
    }
}

pub fn handle_collection(action: CollectionAction) {
    let storage = match Storage::new() {
        Ok(s) => s,
        Err(e) => {
            print_error(&format!("Failed to initialize storage: {}", e));
            return;
        }
    };

    match action {
        CollectionAction::New { name, description } => {
            let collection = Collection {
                name: name.clone(),
                description,
                folders: Vec::new(),
                requests: Vec::new(),
            };

            match storage.save_collection(&collection) {
                Ok(_) => print_success(&format!("Collection '{}' created", name)),
                Err(e) => print_error(&format!("Failed to create collection: {}", e)),
            }
        }

        CollectionAction::List => match storage.list_collections() {
            Ok(collections) => {
                if collections.is_empty() {
                    print_info("No collections found");
                } else {
                    println!("\n{}:", "Collections".bold().cyan());
                    for name in collections {
                        println!("  • {}", name);
                    }
                    println!();
                }
            }
            Err(e) => print_error(&format!("Failed to list collections: {}", e)),
        },

        CollectionAction::Show { name } => match storage.load_collection(&name) {
            Ok(collection) => {
                println!("\n{}: {}", "Collection".bold().cyan(), collection.name);
                if let Some(desc) = &collection.description {
                    println!("{}: {}", "Description".bold(), desc);
                }
                println!("\n{}:", "Requests".bold().cyan());
                if collection.requests.is_empty() {
                    println!("  {}", "(no requests)".bright_black());
                } else {
                    for req in &collection.requests {
                        println!("  • {} - {}", req.name, req.request.url);
                    }
                }
                println!();
            }
            Err(e) => print_error(&format!("Failed to load collection: {}", e)),
        },

        CollectionAction::Add {
            collection,
            name,
            url,
            method,
            header,
            body,
            description,
        } => {
            let mut coll = match storage.load_collection(&collection) {
                Ok(c) => c,
                Err(_) => {
                    print_error(&format!("Collection '{}' not found", collection));
                    return;
                }
            };

            let http_method = match method.to_uppercase().as_str() {
                "GET" => HttpMethod::Get,
                "POST" => HttpMethod::Post,
                "PUT" => HttpMethod::Put,
                "PATCH" => HttpMethod::Patch,
                "DELETE" => HttpMethod::Delete,
                "HEAD" => HttpMethod::Head,
                "OPTIONS" => HttpMethod::Options,
                _ => {
                    print_error(&format!("Invalid HTTP method: {}", method));
                    return;
                }
            };

            let mut request = HttpRequest::new(http_method, &url);
            for (key, value) in header {
                request = request.with_header(&key, &value);
            }
            if let Some(b) = body {
                request = request.with_text_body(&b);
            }

            let saved_request = SavedRequest {
                id: uuid::Uuid::new_v4().to_string(),
                name: name.clone(),
                description,
                request: SerializableRequest::from_request(&request),
            };

            coll.requests.push(saved_request);

            match storage.save_collection(&coll) {
                Ok(_) => print_success(&format!(
                    "Request '{}' added to collection '{}'",
                    name, collection
                )),
                Err(e) => print_error(&format!("Failed to save collection: {}", e)),
            }
        }

        CollectionAction::Delete { name } => match storage.delete_collection(&name) {
            Ok(_) => print_success(&format!("Collection '{}' deleted", name)),
            Err(e) => print_error(&format!("Failed to delete collection: {}", e)),
        },

        CollectionAction::Export { name, output } => match storage.load_collection(&name) {
            Ok(collection) => {
                let json = match serde_json::to_string_pretty(&collection) {
                    Ok(j) => j,
                    Err(e) => {
                        print_error(&format!("Failed to serialize collection: {}", e));
                        return;
                    }
                };
                match std::fs::write(&output, json) {
                    Ok(_) => print_success(&format!("Collection exported to '{}'", output)),
                    Err(e) => print_error(&format!("Failed to write file: {}", e)),
                }
            }
            Err(e) => print_error(&format!("Failed to load collection: {}", e)),
        },

        CollectionAction::Import { input } => {
            let contents = match std::fs::read_to_string(&input) {
                Ok(c) => c,
                Err(e) => {
                    print_error(&format!("Failed to read file: {}", e));
                    return;
                }
            };

            let collection: Collection = match serde_json::from_str(&contents) {
                Ok(c) => c,
                Err(e) => {
                    print_error(&format!("Failed to parse collection: {}", e));
                    return;
                }
            };

            match storage.save_collection(&collection) {
                Ok(_) => print_success(&format!("Collection '{}' imported", collection.name)),
                Err(e) => print_error(&format!("Failed to save collection: {}", e)),
            }
        }
    }
}

pub fn handle_env(action: EnvAction) {
    let storage = match Storage::new() {
        Ok(s) => s,
        Err(e) => {
            print_error(&format!("Failed to initialize storage: {}", e));
            return;
        }
    };

    let mut env_set = storage.load_environment_set().unwrap_or_default();

    match action {
        EnvAction::List => {
            let envs: Vec<String> = env_set
                .environments
                .iter()
                .map(|e| e.name.clone())
                .collect();
            if envs.is_empty() {
                print_info("No environments found");
            } else {
                println!("\n{}:", "Environments".bold().cyan());
                for name in envs {
                    let marker = if env_set.get_active().map(|e| &e.name) == Some(&name) {
                        "* ".green()
                    } else {
                        "  ".normal()
                    };
                    println!("{}• {}", marker, name);
                }
                println!();
            }
        }

        EnvAction::Set { name, variables } => {
            let vars: HashMap<String, String> = variables.into_iter().collect();
            let environment = Environment {
                name: name.clone(),
                variables: vars,
            };
            env_set.add(environment);

            match storage.save_environment_set(&env_set) {
                Ok(_) => print_success(&format!("Environment '{}' saved", name)),
                Err(e) => print_error(&format!("Failed to save environment: {}", e)),
            }
        }

        EnvAction::Show { name } => {
            if let Some(env) = env_set.get(&name) {
                println!("\n{}: {}", "Environment".bold().cyan(), env.name);
                println!("\n{}:", "Variables".bold().cyan());
                if env.variables.is_empty() {
                    println!("  {}", "(no variables)".bright_black());
                } else {
                    for (key, value) in &env.variables {
                        println!("  {} = {}", key.bright_white(), value);
                    }
                }
                println!();
            } else {
                print_error(&format!("Environment '{}' not found", name));
            }
        }

        EnvAction::Delete { name } => {
            env_set.remove(&name);
            match storage.save_environment_set(&env_set) {
                Ok(_) => print_success(&format!("Environment '{}' deleted", name)),
                Err(e) => print_error(&format!("Failed to save changes: {}", e)),
            }
        }

        EnvAction::Use { name } => {
            if env_set.get(&name).is_some() {
                env_set.set_active(&name);
                match storage.save_environment_set(&env_set) {
                    Ok(_) => print_success(&format!("Active environment set to '{}'", name)),
                    Err(e) => print_error(&format!("Failed to save changes: {}", e)),
                }
            } else {
                print_error(&format!("Environment '{}' not found", name));
            }
        }
    }
}

pub fn handle_history(action: HistoryAction) {
    let storage = match Storage::new() {
        Ok(s) => s,
        Err(e) => {
            print_error(&format!("Failed to initialize storage: {}", e));
            return;
        }
    };

    match action {
        HistoryAction::List { limit } => {
            let history = match storage.load_history() {
                Ok(h) => h,
                Err(e) => {
                    print_error(&format!("Failed to load history: {}", e));
                    return;
                }
            };

            if history.is_empty() {
                print_info("No history entries found");
                return;
            }

            println!("\n{}:", "Request History".bold().cyan());
            for (i, entry) in history.iter().take(limit).enumerate() {
                let status_color = if entry.response.status < 300 {
                    entry.response.status.to_string().green()
                } else if entry.response.status < 400 {
                    entry.response.status.to_string().yellow()
                } else {
                    entry.response.status.to_string().red()
                };

                println!(
                    "\n  {} {} {} {} {}",
                    format!("[{}]", i + 1).bright_black(),
                    entry.request.method.bright_white(),
                    entry.request.url,
                    status_color,
                    format!("({}ms)", entry.duration_ms).bright_black()
                );
                println!("     {}", entry.format_timestamp().bright_black());
            }
            println!();
        }

        HistoryAction::Show { id } => {
            let history = match storage.load_history() {
                Ok(h) => h,
                Err(e) => {
                    print_error(&format!("Failed to load history: {}", e));
                    return;
                }
            };

            let entry = if let Ok(index) = id.parse::<usize>() {
                if index > 0 && index <= history.len() {
                    &history[index - 1]
                } else {
                    print_error(&format!("Invalid index: {}", index));
                    return;
                }
            } else {
                match history.iter().find(|e| e.id == id) {
                    Some(e) => e,
                    None => {
                        print_error(&format!("History entry not found: {}", id));
                        return;
                    }
                }
            };

            println!(
                "\n{}: {}",
                "Request".bold().cyan(),
                entry.format_timestamp()
            );
            println!(
                "  {} {}",
                entry.request.method.bright_white(),
                entry.request.url
            );

            if !entry.request.headers.is_empty() {
                println!("\n{}:", "Headers".bold());
                for (name, value) in &entry.request.headers {
                    println!("  {}: {}", name.bright_black(), value);
                }
            }

            if let Some(body) = &entry.request.body {
                println!("\n{}:", "Body".bold());
                println!("{}", body);
            }

            println!(
                "\n{}: {} {}",
                "Response".bold().cyan(),
                entry.response.status,
                entry.response.status_text
            );
            println!("  Duration: {}ms", entry.duration_ms);

            println!("\n{}:", "Preview".bold());
            println!("{}", entry.response.body_preview);
            println!();
        }

        HistoryAction::Rerun { id, verbose } => {
            let history = match storage.load_history() {
                Ok(h) => h,
                Err(e) => {
                    print_error(&format!("Failed to load history: {}", e));
                    return;
                }
            };

            let entry = if let Ok(index) = id.parse::<usize>() {
                if index > 0 && index <= history.len() {
                    &history[index - 1]
                } else {
                    print_error(&format!("Invalid index: {}", index));
                    return;
                }
            } else {
                match history.iter().find(|e| e.id == id) {
                    Some(e) => e,
                    None => {
                        print_error(&format!("History entry not found: {}", id));
                        return;
                    }
                }
            };

            println!(
                "Rerunning: {} {}\n",
                entry.request.method, entry.request.url
            );

            let method = match entry.request.method.to_uppercase().as_str() {
                "GET" => HttpMethod::Get,
                "POST" => HttpMethod::Post,
                "PUT" => HttpMethod::Put,
                "PATCH" => HttpMethod::Patch,
                "DELETE" => HttpMethod::Delete,
                "HEAD" => HttpMethod::Head,
                "OPTIONS" => HttpMethod::Options,
                _ => {
                    print_error("Invalid HTTP method in history");
                    return;
                }
            };

            let mut request = HttpRequest::new(method, &entry.request.url);

            for (key, value) in &entry.request.headers {
                request = request.with_header(key, value);
            }

            if let Some(body) = &entry.request.body {
                request = request.with_text_body(body);
            }

            let client = HttpClient::new();
            let start = Instant::now();

            match client.send(&request) {
                Ok(response) => {
                    let duration_ms = start.elapsed().as_millis() as u64;

                    let new_history_request = HistoryRequest {
                        method: entry.request.method.clone(),
                        url: entry.request.url.clone(),
                        headers: entry.request.headers.clone(),
                        body: entry.request.body.clone(),
                    };

                    let history_response = HistoryResponse::from_response(&response);
                    let new_entry =
                        HistoryEntry::new(new_history_request, history_response, duration_ms);

                    if let Err(e) = storage.save_history_entry(&new_entry) {
                        eprintln!("Warning: Failed to save to history: {}", e);
                    }

                    print_response(&response, verbose);
                }
                Err(e) => print_error(&format!("Request failed: {}", e)),
            }
        }

        HistoryAction::Clear => match storage.clear_history() {
            Ok(_) => print_success("History cleared"),
            Err(e) => print_error(&format!("Failed to clear history: {}", e)),
        },
    }
}
