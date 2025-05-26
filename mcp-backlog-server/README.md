# MCP Server for Backlog

## Sample Configuration

```
  "mcpServers": {
    "backlog_mcp_server": {
      "autoApprove": [
        "get_document_details",
        "get_issue_details"
      ],
      "disabled": false,
      "timeout": 60,
      "command": "/path/to/mcp-backlog-server",
      "args": [],
      "env": {
        "BACKLOG_BASE_URL": "https://example.backlog.com",
        "BACKLOG_API_KEY": "YOUR_API_KEY"
      },
      "transportType": "stdio"
    }
  }
```
