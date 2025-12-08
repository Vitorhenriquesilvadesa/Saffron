use saffron_core::domain::collection::{Collection, Folder, SavedRequest};
use saffron_core::domain::request::HttpRequest;

#[test]
fn test_collection_new() {
    let collection = Collection::new("API Tests");
    assert_eq!(collection.name, "API Tests");
    assert!(collection.description.is_none());
    assert!(collection.folders.is_empty());
    assert!(collection.requests.is_empty());
}

#[test]
fn test_collection_with_description() {
    let collection =
        Collection::new("API Tests").with_description("Collection for testing REST APIs");

    assert_eq!(collection.name, "API Tests");
    assert_eq!(
        collection.description,
        Some("Collection for testing REST APIs".to_string())
    );
}

#[test]
fn test_collection_add_request() {
    let mut collection = Collection::new("Test");
    let request = HttpRequest::get("https://example.com");
    let saved = SavedRequest::new("req1", "Get Example", &request);

    collection.add_request(saved);
    assert_eq!(collection.requests.len(), 1);
    assert_eq!(collection.requests[0].id, "req1");
}

#[test]
fn test_collection_add_folder() {
    let mut collection = Collection::new("Test");
    let folder = Folder::new("Auth");

    collection.add_folder(folder);
    assert_eq!(collection.folders.len(), 1);
    assert_eq!(collection.folders[0].name, "Auth");
}

#[test]
fn test_collection_find_request() {
    let mut collection = Collection::new("Test");
    let request = HttpRequest::get("https://example.com");
    let saved = SavedRequest::new("req1", "Test Request", &request);

    collection.add_request(saved);

    let found = collection.find_request("req1");
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, "req1");

    let not_found = collection.find_request("missing");
    assert!(not_found.is_none());
}

#[test]
fn test_collection_find_request_in_folder() {
    let mut collection = Collection::new("Test");
    let mut folder = Folder::new("API");
    let request = HttpRequest::get("https://example.com");
    let saved = SavedRequest::new("req1", "Test", &request);

    folder.add_request(saved);
    collection.add_folder(folder);

    let found = collection.find_request("req1");
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, "req1");
}

#[test]
fn test_folder_new() {
    let folder = Folder::new("Auth");
    assert_eq!(folder.name, "Auth");
    assert!(folder.description.is_none());
    assert!(folder.requests.is_empty());
    assert!(folder.folders.is_empty());
}

#[test]
fn test_folder_with_description() {
    let folder = Folder::new("Auth").with_description("Authentication endpoints");

    assert_eq!(folder.name, "Auth");
    assert_eq!(
        folder.description,
        Some("Authentication endpoints".to_string())
    );
}

#[test]
fn test_folder_add_request() {
    let mut folder = Folder::new("Users");
    let request = HttpRequest::get("https://api.example.com/users");
    let saved = SavedRequest::new("get-users", "Get Users", &request);

    folder.add_request(saved);
    assert_eq!(folder.requests.len(), 1);
}

#[test]
fn test_folder_add_subfolder() {
    let mut parent = Folder::new("API");
    let child = Folder::new("Users");

    parent.add_folder(child);
    assert_eq!(parent.folders.len(), 1);
    assert_eq!(parent.folders[0].name, "Users");
}

#[test]
fn test_folder_find_request() {
    let mut folder = Folder::new("Test");
    let request = HttpRequest::post("https://example.com");
    let saved = SavedRequest::new("post1", "Post Test", &request);

    folder.add_request(saved);

    let found = folder.find_request("post1");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "Post Test");
}

#[test]
fn test_folder_find_request_in_subfolder() {
    let mut parent = Folder::new("Parent");
    let mut child = Folder::new("Child");
    let request = HttpRequest::get("https://example.com");
    let saved = SavedRequest::new("nested", "Nested Request", &request);

    child.add_request(saved);
    parent.add_folder(child);

    let found = parent.find_request("nested");
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, "nested");
}

#[test]
fn test_saved_request_new() {
    let request = HttpRequest::get("https://api.example.com/users")
        .with_header("Authorization", "Bearer token");

    let saved = SavedRequest::new("get-users", "Get All Users", &request);

    assert_eq!(saved.id, "get-users");
    assert_eq!(saved.name, "Get All Users");
    assert!(saved.description.is_none());
    assert_eq!(saved.request.method, "GET");
    assert_eq!(saved.request.url, "https://api.example.com/users");
}

#[test]
fn test_saved_request_with_description() {
    let request = HttpRequest::get("https://example.com");
    let saved =
        SavedRequest::new("req1", "Test", &request).with_description("This is a test request");

    assert_eq!(
        saved.description,
        Some("This is a test request".to_string())
    );
}

#[test]
fn test_saved_request_to_http_request() {
    let original = HttpRequest::post("https://api.example.com/users")
        .with_header("Content-Type", "application/json")
        .with_json_body(r#"{"name": "Alice"}"#)
        .with_timeout(45);

    let saved = SavedRequest::new("create-user", "Create User", &original);
    let restored = saved.to_http_request();

    assert_eq!(restored.url, "https://api.example.com/users");
    assert_eq!(restored.headers.len(), 1);
    assert_eq!(restored.timeout_seconds, Some(45));
}

#[test]
fn test_serializable_request_from_get() {
    let request = HttpRequest::get("https://example.com/api");
    let saved = SavedRequest::new("test", "Test", &request);

    assert_eq!(saved.request.method, "GET");
    assert_eq!(saved.request.url, "https://example.com/api");
    assert!(saved.request.body.is_none());
}

#[test]
fn test_serializable_request_from_post_with_json() {
    let request =
        HttpRequest::post("https://example.com/api").with_json_body(r#"{"key": "value"}"#);

    let saved = SavedRequest::new("test", "Test", &request);

    assert_eq!(saved.request.method, "POST");
    assert_eq!(saved.request.body, Some(r#"{"key": "value"}"#.to_string()));
}

#[test]
fn test_serializable_request_from_post_with_text() {
    let request = HttpRequest::post("https://example.com/api").with_text_body("plain text");

    let saved = SavedRequest::new("test", "Test", &request);

    assert_eq!(saved.request.body, Some("plain text".to_string()));
}

#[test]
fn test_serializable_request_with_headers() {
    let request = HttpRequest::get("https://example.com")
        .with_header("Authorization", "Bearer token")
        .with_header("Accept", "application/json");

    let saved = SavedRequest::new("test", "Test", &request);

    assert_eq!(saved.request.headers.len(), 2);
    assert!(
        saved
            .request
            .headers
            .contains(&("Authorization".to_string(), "Bearer token".to_string()))
    );
    assert!(
        saved
            .request
            .headers
            .contains(&("Accept".to_string(), "application/json".to_string()))
    );
}

#[test]
fn test_nested_folder_structure() {
    let mut root = Collection::new("API");
    let mut v1 = Folder::new("v1");
    let mut users = Folder::new("users");

    let get_request = HttpRequest::get("https://api.example.com/v1/users");
    let post_request = HttpRequest::post("https://api.example.com/v1/users");

    users.add_request(SavedRequest::new("get", "Get Users", &get_request));
    users.add_request(SavedRequest::new("post", "Create User", &post_request));

    v1.add_folder(users);
    root.add_folder(v1);

    assert_eq!(root.folders.len(), 1);
    assert_eq!(root.folders[0].folders.len(), 1);
    assert_eq!(root.folders[0].folders[0].requests.len(), 2);

    let found = root.find_request("get");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "Get Users");
}
