# Zed GraphQL Context Server

This extension provides a Model Context Server for GraphQL, for use with the Zed AI assistant.

It adds a `/graphql-schema` slash command to the Assistant Panel.

## Configuration

To use the extension, you will need to point the context server at a GraphQL API by setting the `api_url` in your Zed `settings.json`:

```json
{
  "context_servers": {
    "graphql-context-server": {
      "settings": {
        "api_url": "http://localhost:4000/graphql"
      }
    }
  }
}
```

## Usage

- `/graphql-schema`: Retrieve the schema for the API.
