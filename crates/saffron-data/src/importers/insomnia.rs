use super::{ImportError, ImportFormat, ImportResult, ImportedCollection, ImportedRequest};
use crate::json::{Json, JsonElement};
use crate::parse::Parse;
use std::collections::HashMap;

/// Insomnia export format (v4)
#[derive(Debug)]
pub struct InsomniaExport {
    pub version: String,
    pub resources: Vec<InsomniaResource>,
}

#[derive(Debug, Clone)]
pub struct InsomniaResource {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub resource_type: InsomniaResourceType,
}

#[derive(Debug, Clone)]
pub enum InsomniaResourceType {
    Workspace {
        description: Option<String>,
    },
    RequestGroup {
        description: Option<String>,
    },
    Request {
        method: String,
        url: String,
        headers: Vec<(String, String)>,
        body: Option<String>,
        description: Option<String>,
    },
    Environment {
        data: HashMap<String, String>,
    },
}

pub struct InsomniaImporter;

impl ImportFormat for InsomniaImporter {
    type Source = InsomniaExport;

    fn can_import(content: &str) -> bool {
        // Check if it looks like Insomnia format
        content.contains("\"__export_format\"") && content.contains("\"resources\"")
    }

    fn parse(content: &str) -> ImportResult<Self::Source> {
        let json = Json::parse(content).map_err(|e| ImportError::ParseError(e.to_string()))?;

        let obj = match json.root {
            JsonElement::Object(map) => map,
            _ => return Err(ImportError::InvalidFormat("Root must be an object".into())),
        };

        // Get version
        let version = obj
            .get("__export_format")
            .and_then(|v| match v {
                JsonElement::Number(n) => Some(n.to_string()),
                JsonElement::String(s) => Some(s.clone()),
                _ => None,
            })
            .ok_or_else(|| ImportError::MissingField("__export_format".into()))?;

        if version != "4" {
            return Err(ImportError::UnsupportedVersion(format!(
                "Insomnia v{} (only v4 supported)",
                version
            )));
        }

        // Get resources array
        let resources_json = obj
            .get("resources")
            .ok_or_else(|| ImportError::MissingField("resources".into()))?;

        let resources_array = match resources_json {
            JsonElement::Array(arr) => arr,
            _ => {
                return Err(ImportError::InvalidFormat(
                    "resources must be an array".into(),
                ));
            }
        };

        let mut resources = Vec::new();

        for resource_json in resources_array {
            let resource_obj = match resource_json {
                JsonElement::Object(map) => map,
                _ => continue,
            };

            let id = get_string(&resource_obj, "_id")?;
            let name = get_string(&resource_obj, "name")?;
            let parent_id = get_optional_string(&resource_obj, "parentId");
            let type_str = get_string(&resource_obj, "_type")?;

            let resource_type = match type_str.as_str() {
                "workspace" => InsomniaResourceType::Workspace {
                    description: get_optional_string(&resource_obj, "description"),
                },
                "request_group" => InsomniaResourceType::RequestGroup {
                    description: get_optional_string(&resource_obj, "description"),
                },
                "request" => {
                    let method = get_string(&resource_obj, "method")?;
                    let url = get_string(&resource_obj, "url")?;
                    let description = get_optional_string(&resource_obj, "description");

                    // Parse headers
                    let mut headers = Vec::new();
                    if let Some(JsonElement::Array(headers_arr)) = resource_obj.get("headers") {
                        for header_json in headers_arr {
                            if let JsonElement::Object(header_obj) = header_json {
                                if let (Some(name), Some(value)) = (
                                    get_optional_string(&header_obj, "name"),
                                    get_optional_string(&header_obj, "value"),
                                ) {
                                    headers.push((name, value));
                                }
                            }
                        }
                    }

                    // Parse body
                    let body = if let Some(JsonElement::Object(body_obj)) = resource_obj.get("body")
                    {
                        get_optional_string(&body_obj, "text")
                    } else {
                        None
                    };

                    InsomniaResourceType::Request {
                        method,
                        url,
                        headers,
                        body,
                        description,
                    }
                }
                "environment" => {
                    let mut data = HashMap::new();
                    if let Some(JsonElement::Object(data_obj)) = resource_obj.get("data") {
                        for (key, value) in data_obj {
                            if let JsonElement::String(s) = value {
                                data.insert(key.clone(), s.clone());
                            }
                        }
                    }
                    InsomniaResourceType::Environment { data }
                }
                _ => continue, // Skip unknown types
            };

            resources.push(InsomniaResource {
                id,
                name,
                parent_id,
                resource_type,
            });
        }

        Ok(InsomniaExport { version, resources })
    }

    fn convert(source: Self::Source) -> ImportResult<Vec<ImportedCollection>> {
        let mut collections = Vec::new();
        let mut workspaces: HashMap<String, (String, Option<String>)> = HashMap::new();
        let mut requests_by_parent: HashMap<String, Vec<InsomniaResource>> = HashMap::new();

        // First pass: organize resources
        for resource in source.resources {
            match &resource.resource_type {
                InsomniaResourceType::Workspace { description } => {
                    workspaces.insert(
                        resource.id.clone(),
                        (resource.name.clone(), description.clone()),
                    );
                }
                InsomniaResourceType::Request { .. }
                | InsomniaResourceType::RequestGroup { .. } => {
                    let parent = resource.parent_id.clone().unwrap_or_default();
                    requests_by_parent
                        .entry(parent)
                        .or_insert_with(Vec::new)
                        .push(resource.clone());
                }
                InsomniaResourceType::Environment { .. } => {
                    // TODO: Handle environments in future
                }
            }
        }

        // Second pass: create collections
        for (workspace_id, (workspace_name, description)) in workspaces {
            let mut requests = Vec::new();

            // Add requests from this workspace
            if let Some(resources) = requests_by_parent.get(&workspace_id) {
                for resource in resources {
                    if let InsomniaResourceType::Request {
                        method,
                        url,
                        headers,
                        body,
                        description,
                    } = &resource.resource_type
                    {
                        let req = ImportedRequest {
                            id: resource.id.clone(),
                            name: resource.name.clone(),
                            description: description.clone(),
                            method: method.clone(),
                            url: url.clone(),
                            headers: headers.clone(),
                            body: body.clone(),
                        };
                        requests.push(req);
                    }
                }
            }

            collections.push(ImportedCollection {
                name: workspace_name,
                description,
                requests,
            });
        }

        Ok(collections)
    }
}

// Helper functions
fn get_string(obj: &HashMap<String, JsonElement>, key: &str) -> ImportResult<String> {
    obj.get(key)
        .and_then(|v| match v {
            JsonElement::String(s) => Some(s.clone()),
            _ => None,
        })
        .ok_or_else(|| ImportError::MissingField(key.into()))
}

fn get_optional_string(obj: &HashMap<String, JsonElement>, key: &str) -> Option<String> {
    obj.get(key).and_then(|v| match v {
        JsonElement::String(s) => Some(s.clone()),
        _ => None,
    })
}
