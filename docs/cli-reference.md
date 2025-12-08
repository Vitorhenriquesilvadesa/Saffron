# CLI Reference

Complete reference for all Saffron commands and options.

## Table of Contents

- [Global Options](#global-options)
- [send](#send) - Send HTTP requests
- [collection](#collection) - Manage collections
- [env](#env) - Manage environments
- [history](#history) - View request history

## Global Options

```bash
saffron --help              # Show help
saffron --version           # Show version
```

---

## send

Send an HTTP request.

### Usage

```bash
saffron send <URL> [OPTIONS]
```

### Arguments

- `<URL>` - The URL to send the request to (supports `{{variables}}`, optional if using `--from-collection`)

### Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--method` | `-m` | HTTP method | `GET` |
| `--header` | `-H` | Add header (key:value) | - |
| `--body` | `-b` | Request body (text) | - |
| `--json` | `-j` | JSON request body | - |
| `--data` | `-d` | Form data (key=value) | - |
| `--timeout` | `-t` | Timeout in seconds | `30` |
| `--follow-redirects` | `-L` | Follow redirects | `false` |
| `--env` | `-e` | Environment name | - |
| `--verbose` | `-v` | Show headers | `false` |
| `--from-collection` | `-f` | Load request from collection (format: collection_name/request_name) | - |

### HTTP Methods

Supported methods: `GET`, `POST`, `PUT`, `PATCH`, `DELETE`, `HEAD`, `OPTIONS`

### Examples

**Simple GET:**
```bash
saffron send https://api.github.com
```

**POST with JSON:**
```bash
saffron send https://api.example.com/users \
  -m POST \
  -j '{"name":"John","email":"john@example.com"}'
```

**Custom headers:**
```bash
saffron send https://api.example.com/protected \
  -H "Authorization:Bearer token123" \
  -H "Accept:application/json"
```

**Form data:**
```bash
saffron send https://api.example.com/login \
  -m POST \
  -d username=john \
  -d password=secret
```

**With environment variables:**
```bash
saffron send "{{base_url}}/users/{{user_id}}" \
  -e production \
  -H "Authorization:Bearer {{api_token}}"
```

**Follow redirects with timeout:**
```bash
saffron send https://example.com/redirect \
  -L \
  -t 60
```

**Load and run request from collection:**
```bash
saffron send --from-collection "My API/Get Users"
```

**Load from collection and override URL:**
```bash
saffron send https://api.example.com/v2/users \
  --from-collection "My API/Get Users"
```

**Load from collection with environment:**
```bash
saffron send --from-collection "My API/Get Users" \
  -e production
```

---

## collection

Manage request collections.

### Subcommands

- `new` - Create a new collection
- `list` - List all collections
- `show` - Show collection details
- `add` - Add a request to collection
- `delete` - Delete a collection
- `export` - Export collection to file
- `import` - Import collection from file

### collection new

Create a new collection.

```bash
saffron collection new <NAME> [OPTIONS]
```

**Options:**
- `-d, --description <TEXT>` - Collection description

**Example:**
```bash
saffron collection new "My API" -d "Production API endpoints"
```

### collection list

List all collections.

```bash
saffron collection list
```

### collection show

Show collection details.

```bash
saffron collection show <NAME>
```

**Example:**
```bash
saffron collection show "My API"
```

### collection add

Add a request to a collection.

```bash
saffron collection add <COLLECTION> <NAME> <URL> [OPTIONS]
```

**Arguments:**
- `<COLLECTION>` - Collection name
- `<NAME>` - Request name
- `<URL>` - Request URL

**Options:**
- `-m, --method <METHOD>` - HTTP method (default: GET)
- `-H, --header <KEY:VALUE>` - Add header
- `-b, --body <TEXT>` - Request body
- `-d, --description <TEXT>` - Request description

**Example:**
```bash
saffron collection add "My API" "Get Users" \
  https://api.example.com/users \
  -m GET \
  -H "Accept:application/json" \
  -d "Retrieves all users"
```

### collection delete

Delete a collection.

```bash
saffron collection delete <NAME>
```

### collection export

Export a collection to a JSON file.

```bash
saffron collection export <NAME> <OUTPUT_FILE>
```

**Example:**
```bash
saffron collection export "My API" my-api.json
```

### collection import

Import a collection from a JSON file.

```bash
saffron collection import <INPUT_FILE>
```

**Example:**
```bash
saffron collection import my-api.json
```

---

## env

Manage environments and variables.

### Subcommands

- `list` - List all environments
- `set` - Create or update environment
- `show` - Show environment details
- `delete` - Delete environment
- `use` - Set active environment

### env list

List all environments.

```bash
saffron env list
```

The active environment is marked with `*`.

### env set

Create or update an environment with variables.

```bash
saffron env set <NAME> <KEY=VALUE>...
```

**Arguments:**
- `<NAME>` - Environment name
- `<KEY=VALUE>` - Variable key-value pairs

**Example:**
```bash
saffron env set production \
  base_url=https://api.prod.com \
  api_key=prod_key_123 \
  db_host=db.prod.com
```

### env show

Show environment variables.

```bash
saffron env show <NAME>
```

**Example:**
```bash
saffron env show production
```

### env delete

Delete an environment.

```bash
saffron env delete <NAME>
```

### env use

Set an environment as active.

```bash
saffron env use <NAME>
```

**Example:**
```bash
saffron env use production
```

---

## history

View and manage request history.

### Subcommands

- `list` - List request history
- `show` - Show request details
- `rerun` - Rerun a request
- `clear` - Clear all history

### history list

List recent requests.

```bash
saffron history list [OPTIONS]
```

**Options:**
- `-l, --limit <N>` - Number of entries to show (default: 10)

**Example:**
```bash
saffron history list -l 20
```

### history show

Show detailed information about a request.

```bash
saffron history show <ID>
```

**Arguments:**
- `<ID>` - Entry index (1-based) or UUID

**Example:**
```bash
saffron history show 1
```

### history rerun

Rerun a previous request.

```bash
saffron history rerun <ID> [OPTIONS]
```

**Arguments:**
- `<ID>` - Entry index (1-based) or UUID

**Options:**
- `-v, --verbose` - Show response headers

**Example:**
```bash
saffron history rerun 1 -v
```

### history clear

Clear all request history.

```bash
saffron history clear
```

---

## Variable Substitution

Saffron supports `{{variable}}` syntax in:
- URLs
- Headers (both keys and values)
- Request bodies (JSON and text)

Variables are resolved from the specified environment (`-e` flag).

**Example:**

```bash
# Set variables
saffron env set dev \
  host=localhost:3000 \
  token=dev_token_123

# Use in request
saffron send "http://{{host}}/api/users" \
  -e dev \
  -H "Authorization:Bearer {{token}}" \
  -j '{"api_key":"{{token}}"}'
```

The actual request will be sent to:
```
http://localhost:3000/api/users
Headers: Authorization: Bearer dev_token_123
Body: {"api_key":"dev_token_123"}
```
