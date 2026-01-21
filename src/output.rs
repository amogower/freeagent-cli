//! Output formatting utilities.

use colored::*;
use comfy_table::{presets::UTF8_FULL, ColumnConstraint, ContentArrangement, Table, Width};
use serde_json::Value;
use std::collections::HashSet;
use std::env;

const DEFAULT_MAX_TABLE_COLUMNS: usize = 8;
const DEFAULT_MAX_CELL_WIDTH: usize = 40;
const TABLE_COLUMNS_ENV: &str = "FREEAGENT_TABLE_COLUMNS";
const TABLE_MAX_COLUMNS_ENV: &str = "FREEAGENT_TABLE_MAX_COLUMNS";
const TABLE_MAX_CELL_WIDTH_ENV: &str = "FREEAGENT_TABLE_MAX_CELL_WIDTH";
const TABLE_WIDTH_ENV: &str = "FREEAGENT_TABLE_WIDTH";

/// Output format options
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum OutputFormat {
    /// JSON output (default)
    Json,
    /// Table output for lists
    Table,
    /// Compact JSON (single line)
    Compact,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Json
    }
}

/// Format and print output
pub fn print_output(data: &Value, format: OutputFormat) {
    match format {
        OutputFormat::Json => print_json(data),
        OutputFormat::Table => print_table(data),
        OutputFormat::Compact => print_compact(data),
    }
}

/// Print formatted JSON
fn print_json(data: &Value) {
    match serde_json::to_string_pretty(data) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("{}: {}", "Error formatting JSON".red(), e),
    }
}

/// Print compact JSON
fn print_compact(data: &Value) {
    match serde_json::to_string(data) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("{}: {}", "Error formatting JSON".red(), e),
    }
}

/// Print as table (for list responses)
fn print_table(data: &Value) {
    // Try to find an array in the response
    let array = find_array(data);
    
    match array {
        Some(arr) if !arr.is_empty() => {
            let mut table = Table::new();
            table.load_preset(UTF8_FULL);
            table.set_content_arrangement(ContentArrangement::DynamicFullWidth);
            if let Some(width) = resolve_table_width() {
                table.set_width(width);
            }
            
            // Get headers from first object
            if let Some(first) = arr.first() {
                if let Some(obj) = first.as_object() {
                    let headers = select_headers(obj);
                    table.set_header(&headers);
                    let max_cell_width = max_cell_width();
                    let column_constraint = ColumnConstraint::UpperBoundary(Width::Fixed(
                        max_cell_width.min(u16::MAX as usize) as u16,
                    ));
                    table.set_constraints(std::iter::repeat(column_constraint).take(headers.len()));
                    
                    // Add rows
                    for item in arr {
                        if let Some(item_obj) = item.as_object() {
                            let row: Vec<String> = headers
                                .iter()
                                .map(|h| format_value_with_width(item_obj.get(h), max_cell_width))
                                .collect();
                            table.add_row(row);
                        }
                    }
                    
                    println!("{}", table);
                    return;
                }
            }
            
            // Fallback to JSON
            print_json(data);
        }
        _ => {
            // Not a list, print as JSON
            print_json(data);
        }
    }
}

/// Find an array in the JSON response
fn find_array(data: &Value) -> Option<&Vec<Value>> {
    // If it's already an array
    if let Some(arr) = data.as_array() {
        return Some(arr);
    }
    
    // If it's an object, look for an array value
    if let Some(obj) = data.as_object() {
        for value in obj.values() {
            if let Some(arr) = value.as_array() {
                return Some(arr);
            }
        }
    }
    
    None
}

fn select_headers(obj: &serde_json::Map<String, Value>) -> Vec<String> {
    if let Ok(columns) = env::var(TABLE_COLUMNS_ENV) {
        let selected: Vec<String> = columns
            .split(',')
            .map(|col| col.trim())
            .filter(|col| !col.is_empty())
            .filter(|col| obj.contains_key(*col))
            .map(|col| col.to_string())
            .collect();
        if !selected.is_empty() {
            return selected;
        }
    }

    let max_columns = env::var(TABLE_MAX_COLUMNS_ENV)
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_MAX_TABLE_COLUMNS);

    if obj.len() <= max_columns {
        return obj.keys().cloned().collect();
    }

    let priority = [
        "id",
        "reference",
        "name",
        "contact_name",
        "full_name",
        "first_name",
        "last_name",
        "email",
        "billing_email",
        "status",
        "state",
        "date",
        "dated_on",
        "due_on",
        "created_at",
        "updated_at",
        "total",
        "gross_value",
        "net_value",
        "amount",
        "currency",
        "project",
        "contact",
        "url",
    ];

    let mut selected = Vec::new();
    let mut seen = HashSet::new();

    for key in priority {
        if obj.contains_key(key) {
            selected.push(key.to_string());
            seen.insert(key.to_string());
            if selected.len() >= max_columns {
                return selected;
            }
        }
    }

    let mut remaining: Vec<&String> = obj
        .keys()
        .filter(|key| !seen.contains(key.as_str()))
        .collect();
    remaining.sort();

    for key in remaining {
        selected.push(key.clone());
        if selected.len() >= max_columns {
            break;
        }
    }

    selected
}

/// Format a single value for table display
fn format_value_with_width(value: Option<&Value>, max_width: usize) -> String {
    match value {
        None => String::new(),
        Some(Value::Null) => String::new(),
        Some(Value::Bool(b)) => b.to_string(),
        Some(Value::Number(n)) => n.to_string(),
        Some(Value::String(s)) => {
            truncate_string(s, max_width)
        }
        Some(Value::Array(arr)) => format!("[{} items]", arr.len()),
        Some(Value::Object(_)) => "[object]".to_string(),
    }
}

fn max_cell_width() -> usize {
    env::var(TABLE_MAX_CELL_WIDTH_ENV)
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 3)
        .unwrap_or(DEFAULT_MAX_CELL_WIDTH)
}

fn resolve_table_width() -> Option<u16> {
    if let Ok(width) = env::var(TABLE_WIDTH_ENV) {
        if let Ok(value) = width.parse::<u16>() {
            if value > 0 {
                return Some(value);
            }
        }
    }

    if let Ok(columns) = env::var("COLUMNS") {
        if let Ok(value) = columns.parse::<u16>() {
            if value > 0 {
                return Some(value);
            }
        }
    }

    None
}

fn truncate_string(value: &str, max_width: usize) -> String {
    if max_width <= 3 {
        return value.to_string();
    }
    let count = value.chars().count();
    if count > max_width {
        let truncated: String = value.chars().take(max_width - 3).collect();
        format!("{}...", truncated)
    } else {
        value.to_string()
    }
}

/// Print success message
pub fn print_success(message: &str) {
    println!("{} {}", "✓".green(), message);
}

/// Print info message
pub fn print_info(message: &str) {
    println!("{} {}", "ℹ".blue(), message);
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    #[test]
    fn output_format_default_is_json() {
        assert_eq!(OutputFormat::default(), OutputFormat::Json);
    }

    #[test]
    fn find_array_returns_top_level_array() {
        let value = json!([{"id": 1}, {"id": 2}]);
        let array = find_array(&value).expect("array should be found");
        assert_eq!(array.len(), 2);
    }

    #[test]
    fn find_array_searches_object_values() {
        let value = json!({
            "meta": {"count": 2},
            "items": [{"id": 1}]
        });
        let array = find_array(&value).expect("array should be found");
        assert_eq!(array.len(), 1);
    }

    #[test]
    fn format_value_truncates_long_strings() {
        std::env::remove_var(TABLE_MAX_CELL_WIDTH_ENV);
        let long_string = "a".repeat(60);
        let value_json = json!(long_string);
        let value = Some(&value_json);
        let formatted = format_value_with_width(value, DEFAULT_MAX_CELL_WIDTH);
        assert!(formatted.ends_with("..."));
        assert_eq!(formatted.len(), DEFAULT_MAX_CELL_WIDTH);
    }

    #[test]
    fn format_value_handles_null_and_missing() {
        assert_eq!(format_value_with_width(None, DEFAULT_MAX_CELL_WIDTH), "");
        assert_eq!(
            format_value_with_width(Some(&Value::Null), DEFAULT_MAX_CELL_WIDTH),
            ""
        );
    }

    #[test]
    fn format_value_handles_scalars_and_containers() {
        assert_eq!(
            format_value_with_width(Some(&json!(true)), DEFAULT_MAX_CELL_WIDTH),
            "true"
        );
        assert_eq!(
            format_value_with_width(Some(&json!(42)), DEFAULT_MAX_CELL_WIDTH),
            "42"
        );
        assert_eq!(
            format_value_with_width(Some(&json!("hi")), DEFAULT_MAX_CELL_WIDTH),
            "hi"
        );
        assert_eq!(
            format_value_with_width(Some(&json!([1, 2, 3])), DEFAULT_MAX_CELL_WIDTH),
            "[3 items]"
        );
        assert_eq!(
            format_value_with_width(Some(&json!({"a": 1})), DEFAULT_MAX_CELL_WIDTH),
            "[object]"
        );
    }
}
