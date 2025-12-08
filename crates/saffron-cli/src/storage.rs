use crate::history::HistoryEntry;
use saffron_core::domain::collection::Collection;
use saffron_core::domain::environment::EnvironmentSet;
use std::fs;
use std::io;
use std::path::PathBuf;

pub struct Storage {
    base_path: PathBuf,
}

impl Storage {
    pub fn new() -> io::Result<Self> {
        let base_path = dirs::home_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?
            .join(".saffron");

        if !base_path.exists() {
            fs::create_dir_all(&base_path)?;
        }

        Ok(Self { base_path })
    }

    pub fn with_path(path: PathBuf) -> io::Result<Self> {
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }
        Ok(Self { base_path: path })
    }

    pub fn collections_dir(&self) -> PathBuf {
        let dir = self.base_path.join("collections");
        if !dir.exists() {
            let _ = fs::create_dir_all(&dir);
        }
        dir
    }

    pub fn environments_dir(&self) -> PathBuf {
        let dir = self.base_path.join("environments");
        if !dir.exists() {
            let _ = fs::create_dir_all(&dir);
        }
        dir
    }

    pub fn save_collection(&self, collection: &Collection) -> io::Result<()> {
        let file_name = format!("{}.json", sanitize_filename(&collection.name));
        let path = self.collections_dir().join(file_name);
        let json = serde_json::to_string_pretty(collection)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn load_collection(&self, name: &str) -> io::Result<Collection> {
        let file_name = format!("{}.json", sanitize_filename(name));
        let path = self.collections_dir().join(file_name);
        let contents = fs::read_to_string(path)?;
        let collection = serde_json::from_str(&contents)?;
        Ok(collection)
    }

    pub fn list_collections(&self) -> io::Result<Vec<String>> {
        let dir = self.collections_dir();
        let mut collections = Vec::new();

        if dir.exists() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("json")
                    && let Some(name) = path.file_stem().and_then(|s| s.to_str())
                {
                    collections.push(name.to_string());
                }
            }
        }

        Ok(collections)
    }

    pub fn load_collections(&self) -> io::Result<Vec<Collection>> {
        let names = self.list_collections()?;
        let mut collections = Vec::new();

        for name in names {
            if let Ok(collection) = self.load_collection(&name) {
                collections.push(collection);
            }
        }

        Ok(collections)
    }

    pub fn delete_collection(&self, name: &str) -> io::Result<()> {
        let file_name = format!("{}.json", sanitize_filename(name));
        let path = self.collections_dir().join(file_name);
        fs::remove_file(path)?;
        Ok(())
    }

    pub fn save_environment_set(&self, env_set: &EnvironmentSet) -> io::Result<()> {
        let path = self.environments_dir().join("environments.json");
        let json = serde_json::to_string_pretty(env_set)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn load_environment_set(&self) -> io::Result<EnvironmentSet> {
        let path = self.environments_dir().join("environments.json");
        if !path.exists() {
            return Ok(EnvironmentSet::new());
        }
        let contents = fs::read_to_string(path)?;
        let env_set = serde_json::from_str(&contents)?;
        Ok(env_set)
    }

    pub fn history_file(&self) -> PathBuf {
        self.base_path.join("history.json")
    }

    pub fn save_history_entry(&self, entry: &HistoryEntry) -> io::Result<()> {
        let mut history = self.load_history()?;
        history.insert(0, entry.clone());

        if history.len() > 100 {
            history.truncate(100);
        }

        let json = serde_json::to_string_pretty(&history)?;
        fs::write(self.history_file(), json)?;
        Ok(())
    }

    pub fn load_history(&self) -> io::Result<Vec<HistoryEntry>> {
        let path = self.history_file();
        if !path.exists() {
            return Ok(Vec::new());
        }
        let contents = fs::read_to_string(path)?;
        let history = serde_json::from_str(&contents)?;
        Ok(history)
    }

    pub fn clear_history(&self) -> io::Result<()> {
        let path = self.history_file();
        if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .collect()
}

impl Default for Storage {
    fn default() -> Self {
        Self::new().expect("Failed to create storage")
    }
}
