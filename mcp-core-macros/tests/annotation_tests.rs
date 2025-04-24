use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use serde_json::json;

#[tokio::test]
async fn test_readonly_tool_annotations() {
    #[tool(
        name = "web_search",
        description = "Search the web for information",
        read_only_hint = true,
        open_world_hint = true
    )]
    async fn web_search_tool(query: String) -> Result<ToolResponseContent> {
        Ok(tool_text_content!(query.to_string()))
    }

    let tool = WebSearchTool::tool();

    assert_eq!(tool.name, "web_search");
    assert_eq!(
        tool.description,
        Some("Search the web for information".to_string())
    );

    let expected_schema = json!({
        "type": "object",
        "properties": {
            "query": {
                "type": "string"
            }
        },
        "required": ["query"]
    });

    assert_eq!(tool.input_schema, expected_schema);

    let annotations = tool.annotations.unwrap();
    assert_eq!(annotations["title"], "web_search");
    assert_eq!(annotations["readOnlyHint"], true);
    assert_eq!(annotations["destructiveHint"], true); // Default value
    assert_eq!(annotations["idempotentHint"], false); // Default value
    assert_eq!(annotations["openWorldHint"], true);
}

#[tokio::test]
async fn test_destructive_tool_annotations() {
    #[tool(
        name = "delete_file",
        description = "Delete a file from the filesystem",
        read_only_hint = false,
        destructive_hint = true,
        idempotent_hint = true,
        open_world_hint = false,
        title = "Delete File"
    )]
    async fn delete_file_tool(path: String) -> Result<ToolResponseContent> {
        Ok(tool_text_content!(path.to_string()))
    }

    let tool = DeleteFileTool::tool();

    assert_eq!(tool.name, "delete_file");
    assert_eq!(
        tool.description,
        Some("Delete a file from the filesystem".to_string())
    );

    let expected_schema = json!({
        "type": "object",
        "properties": {
            "path": {
                "type": "string"
            }
        },
        "required": ["path"]
    });

    assert_eq!(tool.input_schema, expected_schema);

    let annotations = tool.annotations.unwrap();
    assert_eq!(annotations["title"], "Delete File");
    assert_eq!(annotations["readOnlyHint"], false);
    assert_eq!(annotations["destructiveHint"], true);
    assert_eq!(annotations["idempotentHint"], true);
    assert_eq!(annotations["openWorldHint"], false);
}

#[tokio::test]
async fn test_annotation_nested_syntax() {
    #[tool(
        name = "create_record",
        description = "Create a new record in the database",
        annotations(
            title = "Create Database Record",
            readOnlyHint = false,
            destructiveHint = false,
            idempotentHint = false,
            openWorldHint = false
        )
    )]
    async fn create_record_tool(table: String, data: String) -> Result<ToolResponseContent> {
        Ok(tool_text_content!(table.to_string()))
    }

    let tool = CreateRecordTool::tool();

    assert_eq!(tool.name, "create_record");
    assert_eq!(
        tool.description,
        Some("Create a new record in the database".to_string())
    );

    let expected_schema = json!({
        "type": "object",
        "properties": {
            "table": {
                "type": "string"
            },
            "data": {
                "type": "string"
            }
        },
        "required": ["table", "data"]
    });

    assert_eq!(tool.input_schema, expected_schema);

    let annotations = tool.annotations.unwrap();
    assert_eq!(annotations["title"], "Create Database Record");
    assert_eq!(annotations["readOnlyHint"], false);
    assert_eq!(annotations["destructiveHint"], false);
    assert_eq!(annotations["idempotentHint"], false);
    assert_eq!(annotations["openWorldHint"], false);
}

#[tokio::test]
async fn test_numeric_parameters() {
    #[tool(name = "calculate", description = "Perform a calculation")]
    async fn calculate_tool(
        value1: f64,
        value2: i32,
        operation: String,
    ) -> Result<ToolResponseContent> {
        Ok(tool_text_content!("Calculation result".to_string()))
    }

    let tool = CalculateTool::tool();

    assert_eq!(tool.name, "calculate");

    let expected_schema = json!({
        "type": "object",
        "properties": {
            "value1": {
                "type": "number"
            },
            "value2": {
                "type": "number"
            },
            "operation": {
                "type": "string"
            }
        },
        "required": ["value1", "value2", "operation"]
    });

    assert_eq!(tool.input_schema, expected_schema);
}

#[tokio::test]
async fn test_optional_parameters() {
    #[tool(
        name = "optional_params_tool",
        description = "Tool with optional parameters"
    )]
    async fn optional_params_tool(
        required_param: String,
        optional_string: Option<String>,
        optional_number: Option<i32>,
    ) -> Result<ToolResponseContent> {
        Ok(tool_text_content!(
            "Tool with optional params executed".to_string()
        ))
    }

    let tool = OptionalParamsTool::tool();

    let expected_schema = json!({
        "type": "object",
        "properties": {
            "required_param": {
                "type": "string"
            },
            "optional_string": {
                "type": "string"
            },
            "optional_number": {
                "type": "number"
            }
        },
        "required": ["required_param"]
    });

    assert_eq!(tool.input_schema, expected_schema);
}

#[tokio::test]
async fn test_parameter_descriptions() {
    #[tool(
        name = "query_database",
        description = "Query a database with parameters",
        params(
            db_name = "Name of the database to query",
            query = "SQL query to execute",
            timeout_ms = "Query timeout in milliseconds"
        )
    )]
    async fn query_database_tool(
        db_name: String,
        query: String,
        timeout_ms: Option<i32>,
    ) -> Result<ToolResponseContent> {
        Ok(tool_text_content!("Query executed".to_string()))
    }

    let tool = QueryDatabaseTool::tool();

    assert_eq!(tool.name, "query_database");
    assert_eq!(
        tool.description,
        Some("Query a database with parameters".to_string())
    );

    // Check parameter descriptions are included in schema
    let schema = tool.input_schema.as_object().unwrap();
    let properties = schema["properties"].as_object().unwrap();

    assert_eq!(
        properties["db_name"]["description"].as_str().unwrap(),
        "Name of the database to query"
    );
    assert_eq!(
        properties["query"]["description"].as_str().unwrap(),
        "SQL query to execute"
    );
    assert_eq!(
        properties["timeout_ms"]["description"].as_str().unwrap(),
        "Query timeout in milliseconds"
    );

    // Validate schema structure
    let expected_schema = json!({
        "type": "object",
        "properties": {
            "db_name": {
                "type": "string",
                "description": "Name of the database to query"
            },
            "query": {
                "type": "string",
                "description": "SQL query to execute"
            },
            "timeout_ms": {
                "type": "number",
                "description": "Query timeout in milliseconds"
            }
        },
        "required": ["db_name", "query"]
    });

    assert_eq!(tool.input_schema, expected_schema);
}
