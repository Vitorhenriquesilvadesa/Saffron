# Examples

Real-world examples showing how to use Saffron effectively.

## Table of Contents

- [Basic Requests](#basic-requests)
- [Authentication](#authentication)
- [Working with REST APIs](#working-with-rest-apis)
- [Environment Management](#environment-management)
- [Collections Workflow](#collections-workflow)
- [Importing Collections](#importing-collections)
- [Advanced Scenarios](#advanced-scenarios)

## Basic Requests

### Simple GET Request

```bash
# Fetch data from an API
saffron send https://jsonplaceholder.typicode.com/posts/1
```

### GET with Query Parameters

```bash
# Query parameters in URL
saffron send "https://api.github.com/search/repositories?q=rust&sort=stars"
```

### POST with JSON

```bash
# Create a new resource
saffron send https://jsonplaceholder.typicode.com/posts \
  -m POST \
  -j '{
    "title": "My Post",
    "body": "This is the content",
    "userId": 1
  }' \
  -H "Content-Type:application/json"
```

### PUT to Update Resource

```bash
# Update existing resource
saffron send https://jsonplaceholder.typicode.com/posts/1 \
  -m PUT \
  -j '{"title":"Updated Title","body":"Updated content","userId":1}'
```

### DELETE Request

```bash
# Delete a resource
saffron send https://jsonplaceholder.typicode.com/posts/1 \
  -m DELETE
```

## Authentication

### Bearer Token

```bash
# API with Bearer token authentication
saffron send https://api.github.com/user \
  -H "Authorization:Bearer ghp_yourtoken123"
```

### Basic Authentication

```bash
# Basic auth (encode credentials separately)
saffron send https://api.example.com/protected \
  -H "Authorization:Basic dXNlcjpwYXNz"
```

### API Key in Header

```bash
# API key authentication
saffron send https://api.openweathermap.org/data/2.5/weather \
  -H "X-API-Key:your_api_key_here"
```

### API Key in Query

```bash
# API key in URL
saffron send "https://api.example.com/data?api_key=your_key"
```

## Working with REST APIs

### CRUD Operations Example (Blog API)

```bash
# 1. Create a post
saffron send https://api.blog.com/posts \
  -m POST \
  -j '{"title":"Hello World","content":"My first post"}' \
  -H "Content-Type:application/json"

# 2. List all posts
saffron send https://api.blog.com/posts

# 3. Get specific post
saffron send https://api.blog.com/posts/123

# 4. Update post
saffron send https://api.blog.com/posts/123 \
  -m PUT \
  -j '{"title":"Updated Title","content":"Updated content"}'

# 5. Delete post
saffron send https://api.blog.com/posts/123 -m DELETE
```

### Pagination

```bash
# Page 1
saffron send "https://api.example.com/items?page=1&limit=10"

# Page 2
saffron send "https://api.example.com/items?page=2&limit=10"
```

### Filtering and Sorting

```bash
# Filter by status and sort by date
saffron send "https://api.example.com/orders?status=pending&sort=-created_at"
```

## Environment Management

### Development Environment

```bash
# Create dev environment
saffron env set dev \
  base_url=http://localhost:3000 \
  api_key=dev_key_123 \
  db_name=myapp_dev

# Use in requests
saffron send "{{base_url}}/api/users" -e dev
```

### Multiple Environments

```bash
# Development
saffron env set dev \
  base_url=http://localhost:3000 \
  api_key=dev_key

# Staging
saffron env set staging \
  base_url=https://staging.example.com \
  api_key=staging_key

# Production
saffron env set production \
  base_url=https://api.example.com \
  api_key=prod_key

# Switch between environments
saffron send "{{base_url}}/health" -e dev
saffron send "{{base_url}}/health" -e staging
saffron send "{{base_url}}/health" -e production
```

### Dynamic Variables

```bash
# Set user-specific variables
saffron env set test \
  user_id=12345 \
  username=johndoe \
  email=john@example.com

# Use in request
saffron send "https://api.example.com/users/{{user_id}}" \
  -e test \
  -H "X-Username:{{username}}"
```

## Collections Workflow

### Building an API Test Suite

```bash
# 1. Create collection
saffron collection new "User API" \
  -d "Complete user management API"

# 2. Add endpoints
saffron collection add "User API" "List Users" \
  https://api.example.com/users \
  -m GET \
  -d "Get all users"

saffron collection add "User API" "Create User" \
  https://api.example.com/users \
  -m POST \
  -b '{"name":"John","email":"john@example.com"}' \
  -d "Create new user"

saffron collection add "User API" "Get User" \
  https://api.example.com/users/{{user_id}} \
  -m GET \
  -d "Get user by ID"

saffron collection add "User API" "Update User" \
  https://api.example.com/users/{{user_id}} \
  -m PUT \
  -b '{"name":"Updated Name"}' \
  -d "Update user details"

saffron collection add "User API" "Delete User" \
  https://api.example.com/users/{{user_id}} \
  -m DELETE \
  -d "Delete user"

# 3. View collection
saffron collection show "User API"

# 4. Run requests from collection
saffron send --from-collection "User API/List Users"
saffron send --from-collection "User API/Get User" -e development
```

### Running Saved Requests

```bash
# Run request as saved
saffron send --from-collection "User API/List Users"

# Run with environment variables
saffron send --from-collection "User API/Get User" -e production

# Override saved URL
saffron send https://api-v2.example.com/users \
  --from-collection "User API/List Users"

# Add verbose output
saffron send --from-collection "Auth API/Login" -v
```

---

## Importing Collections

### Import from Insomnia

Migrate your existing Insomnia collections to Saffron:

```bash
# 1. Export from Insomnia
# In Insomnia: Application Menu → Import/Export → Export Data
# Select "Export" and choose your workspace(s)
# Save as insomnia-export.json

# 2. Import to Saffron
saffron collection import insomnia-export.json

# 3. Verify import
saffron collection list
saffron collection show "Your Workspace Name"

# 4. Run imported requests
saffron send --from-collection "Your Workspace Name/Request Name"
```

### Real-World Insomnia Import Example

```bash
# Example: Importing a REST API collection from Insomnia
# The export contains a workspace with multiple requests

# Import file
saffron collection import my-api-export.json

# Output:
# ✓ Imported collection 'My REST API'
# 1 collection(s) imported successfully

# View what was imported
saffron collection show "My REST API"

# Output:
# Collection: My REST API
# Description: Production API endpoints
#
# Requests:
#   • Get Users - https://api.example.com/users
#   • Create User - https://api.example.com/users
#   • Update User - https://api.example.com/users/{{id}}
#   • Delete User - https://api.example.com/users/{{id}}

# Run imported requests
saffron send --from-collection "My REST API/Get Users"
saffron send --from-collection "My REST API/Create User" -e production
```

### Batch Import Multiple Collections

```bash
# Import multiple Insomnia exports at once
saffron collection import team-apis-export.json

# The tool will import all workspaces found in the file
# Each workspace becomes a separate collection

# Output:
# ✓ Imported collection 'Auth API'
# ✓ Imported collection 'User API'
# ✓ Imported collection 'Payment API'
# 3 collection(s) imported successfully
```

### Import Workflow Best Practices

```bash
# 1. Backup existing collections first
saffron collection list > my-collections.txt
saffron collection export "Important API" backup.json

# 2. Import new collections
saffron collection import insomnia-export.json

# 3. Verify imported data
saffron collection list
saffron collection show "Imported Collection"

# 4. Test imported requests
saffron send --from-collection "Imported Collection/Health Check" -v

# 5. Set up environments for imported collections
saffron env set imported-dev base_url https://api-dev.example.com
saffron send --from-collection "Imported Collection/Get Data" -e imported-dev
```

### Supported Import Formats

**Currently Supported:**
- ✅ Insomnia v4 format
- ✅ Saffron native JSON format

**Coming Soon:**
- 🔜 Postman Collection v2.1
- 🔜 Thunder Client collections
- 🔜 OpenAPI/Swagger specs

### Import Limitations

When importing from Insomnia:

1. **Request Groups**: Nested folders are flattened (all requests appear at collection level)
2. **Environments**: Insomnia environments are not imported (create manually with `saffron env set`)
3. **Authentication**: Pre-configured auth (Bearer, Basic, etc.) is not preserved
4. **Variables**: Insomnia variables need to be recreated in Saffron environments

```bash
# Example: Setting up environments after import
# If your Insomnia collection used {{base_url}} variable:

saffron env set development base_url https://api-dev.example.com
saffron env set production base_url https://api.example.com

# Now use with imported requests
saffron send --from-collection "Imported API/Endpoint" -e development
```

### Organizing Collections

```bash
# Create multiple collections
saffron collection new "Auth API" -d "Authentication endpoints"
saffron collection new "Products API" -d "Product management"
saffron collection new "Orders API" -d "Order processing"

# List all
saffron collection list
```

### Sharing Collections

```bash
# Export collection
saffron collection export "User API" user-api.json

# Share file with team (via Git, email, etc.)

# Import on another machine
saffron collection import user-api.json
```

## Advanced Scenarios

### GitHub API Integration

```bash
# Setup
saffron env set github \
  api_url=https://api.github.com \
  token=ghp_your_token \
  username=octocat

# Get user info
saffron send "{{api_url}}/users/{{username}}" \
  -e github \
  -H "Authorization:Bearer {{token}}" \
  -H "Accept:application/vnd.github+json"

# List repositories
saffron send "{{api_url}}/users/{{username}}/repos" \
  -e github \
  -H "Authorization:Bearer {{token}}"

# Create a repository
saffron send "{{api_url}}/user/repos" \
  -m POST \
  -e github \
  -j '{
    "name": "new-repo",
    "description": "Created with Saffron",
    "private": false
  }' \
  -H "Authorization:Bearer {{token}}"
```

### Testing Webhooks

```bash
# Simulate webhook payload
saffron send http://localhost:3000/webhook \
  -m POST \
  -j '{
    "event": "user.created",
    "data": {
      "id": 123,
      "email": "user@example.com"
    },
    "timestamp": "2025-12-08T12:00:00Z"
  }' \
  -H "Content-Type:application/json" \
  -H "X-Webhook-Secret:your_secret"
```

### Load Testing Preparation

```bash
# Test endpoint performance
for i in {1..10}; do
  saffron send https://api.example.com/health
done

# Use history to analyze
saffron history list -l 10
```

### Debugging API Issues

```bash
# Verbose mode to see all headers
saffron send https://api.example.com/problematic-endpoint \
  -v \
  -H "Debug:true"

# View in history
saffron history list
saffron history show 1

# Rerun with modifications
saffron history rerun 1 -v
```

### Working with Large Responses

```bash
# Pipe to file
saffron send https://api.example.com/large-dataset > data.json

# Pipe to jq for filtering
saffron send https://api.github.com/users/octocat | jq '.login'
```

### Form Data

```bash
# Simple form submission
saffron send https://api.example.com/login \
  -m POST \
  -d username=john \
  -d password=secret123 \
  -d remember=true
```

### Custom Timeouts

```bash
# For slow APIs
saffron send https://slow-api.example.com/process \
  -t 120 \
  -m POST \
  -j '{"data":"large payload"}'
```

### Following Redirects

```bash
# Follow up to 10 redirects
saffron send https://example.com/redirect \
  -L \
  -v
```

### Chaining Requests with History

```bash
# 1. Login and get token
saffron send https://api.example.com/login \
  -m POST \
  -j '{"username":"john","password":"secret"}'

# 2. Copy token from response, add to environment
saffron env set current token=eyJhbGc...

# 3. Use token in subsequent requests
saffron send "{{base_url}}/protected" \
  -e current \
  -H "Authorization:Bearer {{token}}"

# 4. Or rerun previous requests
saffron history rerun 1
```

## Tips and Tricks

### Aliasing Common Commands

```bash
# Add to .bashrc or .zshrc
alias sget='saffron send'
alias spost='saffron send -m POST'
alias sput='saffron send -m PUT'
alias sdelete='saffron send -m DELETE'

# Usage
sget https://api.github.com
spost https://api.example.com/users -j '{"name":"John"}'
```

### JSON Formatting

```bash
# Saffron already formats JSON, but you can pipe to jq
saffron send https://api.github.com | jq '.'
```

### Saving Responses

```bash
# Save response to file
saffron send https://api.example.com/data > response.json

# Save with metadata
saffron send https://api.example.com/data -v > response.txt
```
