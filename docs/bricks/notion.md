# Notion Brick

The Notion brick integrates with Notion's API to create, read, and update pages and databases.

## Configuration

```json
{
  "brick_type": "notion",
  "config": {
    "api_key": "secret-your-notion-api-key",
    "database_id": "your-database-id",
    "operation": "create_page",
    "properties": {
      "title": "{{page_title}}",
      "content": "{{page_content}}"
    }
  }
}
```

## Configuration Options

- **api_key** (required): Your Notion integration token
- **database_id** (required): Notion database ID
- **operation** (required): Operation to perform (see below)
- **properties** (optional): Page properties

## Supported Operations

### Create Page

```json
{
  "brick_type": "notion",
  "config": {
    "api_key": "secret-...",
    "database_id": "database-id",
    "operation": "create_page",
    "properties": {
      "title": "{{title}}",
      "content": "{{content}}"
    }
  }
}
```

### Get Page

```json
{
  "brick_type": "notion",
  "config": {
    "api_key": "secret-...",
    "operation": "get_page",
    "page_id": "{{page_id}}"
  }
}
```

### Update Page

```json
{
  "brick_type": "notion",
  "config": {
    "api_key": "secret-...",
    "operation": "update_page",
    "page_id": "{{page_id}}",
    "properties": {
      "content": "{{new_content}}"
    }
  }
}
```

## Input Format

For create operations:

```json
{
  "title": "Meeting Notes",
  "content": "Discussion about project..."
}
```

## Output Format

Returns Notion API response:

```json
{
  "id": "page-id",
  "properties": {
    "title": "Meeting Notes",
    "content": "Discussion about project..."
  }
}
```

## Use Cases

- Create meeting notes automatically
- Sync data to Notion databases
- Generate documentation pages
- Track project updates

## Setting Up Notion Integration

1. Go to https://www.notion.so/my-integrations
2. Create a new integration
3. Copy the integration token (starts with `secret_`)
4. Share your database/page with the integration
5. Copy the database/page ID from the URL

