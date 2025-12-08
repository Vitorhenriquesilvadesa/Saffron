use saffron_core::domain::environment::{Environment, EnvironmentSet};

#[test]
fn test_environment_new() {
    let env = Environment::new("development");
    assert_eq!(env.name, "development");
    assert!(env.variables.is_empty());
}

#[test]
fn test_environment_set_get() {
    let mut env = Environment::new("test");
    env.set("api_key", "secret123");
    env.set("base_url", "https://api.example.com");

    assert_eq!(env.get("api_key"), Some("secret123"));
    assert_eq!(env.get("base_url"), Some("https://api.example.com"));
    assert_eq!(env.get("missing"), None);
}

#[test]
fn test_environment_remove() {
    let mut env = Environment::new("test");
    env.set("key1", "value1");
    env.set("key2", "value2");

    assert_eq!(env.remove("key1"), Some("value1".to_string()));
    assert_eq!(env.get("key1"), None);
    assert_eq!(env.get("key2"), Some("value2"));
}

#[test]
fn test_environment_contains() {
    let mut env = Environment::new("test");
    env.set("exists", "value");

    assert!(env.contains("exists"));
    assert!(!env.contains("missing"));
}

#[test]
fn test_environment_resolve_template() {
    let mut env = Environment::new("test");
    env.set("host", "example.com");
    env.set("port", "8080");
    env.set("path", "api/v1");

    let template = "https://{{host}}:{{port}}/{{path}}/users";
    let resolved = env.resolve_template(template);

    assert_eq!(resolved, "https://example.com:8080/api/v1/users");
}

#[test]
fn test_environment_resolve_template_no_variables() {
    let env = Environment::new("test");
    let template = "https://example.com/api";
    let resolved = env.resolve_template(template);

    assert_eq!(resolved, "https://example.com/api");
}

#[test]
fn test_environment_resolve_template_missing_variable() {
    let mut env = Environment::new("test");
    env.set("host", "example.com");

    let template = "https://{{host}}/{{missing}}/users";
    let resolved = env.resolve_template(template);

    assert_eq!(resolved, "https://example.com/{{missing}}/users");
}

#[test]
fn test_environment_resolve_request_url() {
    let mut env = Environment::new("test");
    env.set("base_url", "https://api.example.com");
    env.set("version", "v2");

    let url = "{{base_url}}/{{version}}/users";
    let resolved = env.resolve_request_url(url);

    assert_eq!(resolved, "https://api.example.com/v2/users");
}

#[test]
fn test_environment_resolve_header_value() {
    let mut env = Environment::new("test");
    env.set("token", "Bearer abc123");

    let value = "{{token}}";
    let resolved = env.resolve_header_value(value);

    assert_eq!(resolved, "Bearer abc123");
}

#[test]
fn test_environment_set_new() {
    let env_set = EnvironmentSet::new();
    assert!(env_set.active.is_none());
    assert!(env_set.environments.is_empty());
}

#[test]
fn test_environment_set_add() {
    let mut env_set = EnvironmentSet::new();
    let env1 = Environment::new("dev");
    let env2 = Environment::new("prod");

    env_set.add(env1);
    env_set.add(env2);

    assert_eq!(env_set.environments.len(), 2);
}

#[test]
fn test_environment_set_get_mut() {
    let mut env_set = EnvironmentSet::new();
    env_set.add(Environment::new("dev"));

    if let Some(env) = env_set.get_mut("dev") {
        env.set("new_key", "new_value");
    }

    assert_eq!(
        env_set.get("dev").unwrap().get("new_key"),
        Some("new_value")
    );
}

#[test]
fn test_environment_set_remove() {
    let mut env_set = EnvironmentSet::new();
    env_set.add(Environment::new("dev"));
    env_set.add(Environment::new("prod"));

    let removed = env_set.remove("dev");
    assert!(removed.is_some());
    assert_eq!(removed.unwrap().name, "dev");
    assert_eq!(env_set.environments.len(), 1);
}

#[test]
fn test_environment_set_active() {
    let mut env_set = EnvironmentSet::new();
    env_set.add(Environment::new("dev"));
    env_set.add(Environment::new("prod"));

    env_set.set_active("dev");
    assert_eq!(env_set.active, Some("dev".to_string()));
}

#[test]
fn test_environment_set_get_active() {
    let mut env_set = EnvironmentSet::new();
    let mut env = Environment::new("dev");
    env.set("test_key", "test_value");
    env_set.add(env);
    env_set.set_active("dev");

    let active = env_set.get_active();
    assert!(active.is_some());
    assert_eq!(active.unwrap().name, "dev");
    assert_eq!(active.unwrap().get("test_key"), Some("test_value"));
}

#[test]
fn test_environment_set_get_active_none() {
    let env_set = EnvironmentSet::new();
    assert!(env_set.get_active().is_none());
}

#[test]
fn test_environment_set_get_active_mut() {
    let mut env_set = EnvironmentSet::new();
    env_set.add(Environment::new("dev"));
    env_set.set_active("dev");

    if let Some(env) = env_set.get_active_mut() {
        env.set("modified", "yes");
    }

    assert_eq!(env_set.get_active().unwrap().get("modified"), Some("yes"));
}

#[test]
fn test_environment_multiple_variables() {
    let mut env = Environment::new("test");
    env.set("var1", "value1");
    env.set("var2", "value2");
    env.set("var3", "value3");

    let template = "{{var1}}-{{var2}}-{{var3}}";
    let resolved = env.resolve_template(template);

    assert_eq!(resolved, "value1-value2-value3");
}

#[test]
fn test_environment_nested_braces() {
    let mut env = Environment::new("test");
    env.set("key", "value");

    let template = "{{key}} and {{{{nested}}}}";
    let resolved = env.resolve_template(template);

    assert_eq!(resolved, "value and {{{{nested}}}}");
}
