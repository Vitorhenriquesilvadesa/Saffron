use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    pub name: String,
    pub variables: HashMap<String, String>,
}

impl Environment {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            variables: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.variables.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.variables.get(key).map(|s| s.as_str())
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.variables.remove(key)
    }

    pub fn contains(&self, key: &str) -> bool {
        self.variables.contains_key(key)
    }

    pub fn resolve_template(&self, template: &str) -> String {
        let mut result = template.to_string();

        for (key, value) in &self.variables {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, value);
        }

        result
    }

    pub fn resolve_request_url(&self, url: &str) -> String {
        self.resolve_template(url)
    }

    pub fn resolve_header_value(&self, value: &str) -> String {
        self.resolve_template(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentSet {
    pub active: Option<String>,
    pub environments: Vec<Environment>,
}

impl EnvironmentSet {
    pub fn new() -> Self {
        Self {
            active: None,
            environments: Vec::new(),
        }
    }

    pub fn add(&mut self, env: Environment) {
        self.environments.push(env);
    }

    pub fn get(&self, name: &str) -> Option<&Environment> {
        self.environments.iter().find(|e| e.name == name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Environment> {
        self.environments.iter_mut().find(|e| e.name == name)
    }

    pub fn remove(&mut self, name: &str) -> Option<Environment> {
        self.environments
            .iter()
            .position(|e| e.name == name)
            .map(|i| self.environments.remove(i))
    }

    pub fn set_active(&mut self, name: impl Into<String>) {
        self.active = Some(name.into());
    }

    pub fn get_active(&self) -> Option<&Environment> {
        self.active.as_ref().and_then(|name| self.get(name))
    }

    pub fn get_active_mut(&mut self) -> Option<&mut Environment> {
        if let Some(name) = self.active.clone() {
            self.get_mut(&name)
        } else {
            None
        }
    }
}

impl Default for EnvironmentSet {
    fn default() -> Self {
        Self::new()
    }
}
