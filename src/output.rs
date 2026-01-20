//! Output formatting utilities.

use colored::*;
use comfy_table::{presets::UTF8_FULL, Table};
use serde_json::Value;

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
            
            // Get headers from first object
            if let Some(first) = arr.first() {
                if let Some(obj) = first.as_object() {
                    let headers: Vec<String> = obj.keys().cloned().collect();
                    table.set_header(&headers);
                    
                    // Add rows
                    for item in arr {
                        if let Some(item_obj) = item.as_object() {
                            let row: Vec<String> = headers
                                .iter()
                                .map(|h| format_value(item_obj.get(h)))
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

/// Format a single value for table display
fn format_value(value: Option<&Value>) -> String {
    match value {
        None => String::new(),
        Some(Value::Null) => String::new(),
        Some(Value::Bool(b)) => b.to_string(),
        Some(Value::Number(n)) => n.to_string(),
        Some(Value::String(s)) => {
            // Truncate long strings
            if s.len() > 50 {
                format!("{}...", &s[..47])
            } else {
                s.clone()
            }
        }
        Some(Value::Array(arr)) => format!("[{} items]", arr.len()),
        Some(Value::Object(_)) => "[object]".to_string(),
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
