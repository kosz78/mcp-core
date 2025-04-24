# MCP Core Macros

This crate provides macros for the MCP (Model Communication Protocol) Core library.

## Tool Macro

The `#[tool]` macro simplifies creating tools for the MCP protocol by generating the necessary boilerplate code.

### Basic Usage

```rust
use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;

#[tool(
    name = "hello_world",
    description = "A simple hello world tool"
)]
async fn hello_world_tool(name: String) -> Result<ToolResponseContent> {
    Ok(tool_text_content!(format!("Hello, {}!", name)))
}

// This generates a struct named HelloWorldTool with tool() and call() methods
let tool = HelloWorldTool::tool();
```

### Parameter Descriptions

You can add descriptions for parameters using the `params` attribute:

```rust
#[tool(
    name = "search_database",
    description = "Search a database for records",
    params(
        db_name = "Name of the database to search",
        query = "Search query string",
        limit = "Maximum number of results to return"
    )
)]
async fn search_database_tool(
    db_name: String,
    query: String,
    limit: Option<i32>,
) -> Result<ToolResponseContent> {
    // Implementation...
}
```

### Tool Annotations

Tool annotations provide additional metadata about a tool's behavior:

```rust
#[tool(
    name = "delete_file",
    description = "Delete a file from the filesystem",
    title = "Delete File",                  // Human-readable title
    read_only_hint = false,                 // Whether the tool modifies its environment
    destructive_hint = true,                // Whether the tool performs destructive updates
    idempotent_hint = true,                 // Whether calling repeatedly has no additional effect
    open_world_hint = false                 // Whether the tool interacts with external entities
)]
async fn delete_file_tool(path: String) -> Result<ToolResponseContent> {
    // Implementation...
}
```

Alternatively, you can group annotations:

```rust
#[tool(
    name = "create_record",
    description = "Create a database record",
    annotations(
        title = "Create Database Record",
        readOnlyHint = false,
        destructiveHint = false,
        idempotentHint = false,
        openWorldHint = false
    )
)]
async fn create_record_tool(table: String, data: String) -> Result<ToolResponseContent> {
    // Implementation...
}
```

### Generated Schema

The `#[tool]` macro automatically generates a JSON Schema for the tool parameters with the following features:

- Required parameters (non-Option types) are listed in the "required" field
- Optional parameters (Option<T> types) are properly marked as optional
- All numeric types (i32, f64, etc.) are normalized to "number" type
- Enums that derive schemars::JsonSchema are properly handled
- Parameter descriptions are included in the schema

### Default Values

- `title`: defaults to the tool name if not specified
- `readOnlyHint`: defaults to `false`
- `destructiveHint`: defaults to `true`
- `idempotentHint`: defaults to `false`
- `openWorldHint`: defaults to `true` 