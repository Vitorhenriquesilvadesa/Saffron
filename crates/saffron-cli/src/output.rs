use colored::Colorize;
use saffron_core::domain::response::HttpResponse;
use saffron_data::json::{Json, JsonElement};
use saffron_data::parse::Parse;

pub fn print_response(response: &HttpResponse, verbose: bool) {
    println!("\n{} {}", "Status:".bold(), format_status(response.status));

    if verbose {
        println!("\n{}:", "Headers".bold().cyan());
        for (name, value) in &response.headers {
            println!("  {}: {}", name.bright_black(), value);
        }
    }

    println!("\n{}:", "Body".bold().cyan());

    if response.is_json() {
        match std::str::from_utf8(&response.body) {
            Ok(body_str) => {
                if let Ok(json) = Json::parse(body_str) {
                    println!("{}", format_json(&json.root, 0));
                } else {
                    println!("{}", body_str);
                }
            }
            Err(_) => println!("{}", "<binary data>".bright_black()),
        }
    } else if let Ok(body_str) = std::str::from_utf8(&response.body) {
        println!("{}", body_str);
    } else {
        println!(
            "{}",
            format!("<binary data, {} bytes>", response.body.len()).bright_black()
        );
    }

    println!();
}

fn format_status(code: u16) -> String {
    let status_str = code.to_string();
    if (200..300).contains(&code) {
        status_str.green().to_string()
    } else if (300..400).contains(&code) {
        status_str.yellow().to_string()
    } else if (400..500).contains(&code) {
        status_str.red().to_string()
    } else if code >= 500 {
        status_str.bright_red().to_string()
    } else {
        status_str
    }
}

fn format_json(json: &JsonElement, indent: usize) -> String {
    let indent_str = "  ".repeat(indent);
    match json {
        JsonElement::Null => "null".bright_black().to_string(),
        JsonElement::Boolean(b) => b.to_string().yellow().to_string(),
        JsonElement::Number(n) => n.to_string().cyan().to_string(),
        JsonElement::String(s) => format!("\"{}\"", s).green().to_string(),
        JsonElement::Array(arr) => {
            if arr.is_empty() {
                return "[]".to_string();
            }
            let mut result = "[\n".to_string();
            for (i, item) in arr.iter().enumerate() {
                result.push_str(&format!(
                    "{}  {}",
                    indent_str,
                    format_json(item, indent + 1)
                ));
                if i < arr.len() - 1 {
                    result.push(',');
                }
                result.push('\n');
            }
            result.push_str(&format!("{}]", indent_str));
            result
        }
        JsonElement::Object(obj) => {
            if obj.is_empty() {
                return "{}".to_string();
            }
            let mut result = "{\n".to_string();
            let items: Vec<_> = obj.iter().collect();
            for (i, (key, value)) in items.iter().enumerate() {
                result.push_str(&format!(
                    "{}  {}: {}",
                    indent_str,
                    format!("\"{}\"", key).bright_white(),
                    format_json(value, indent + 1)
                ));
                if i < items.len() - 1 {
                    result.push(',');
                }
                result.push('\n');
            }
            result.push_str(&format!("{}}}", indent_str));
            result
        }
    }
}

pub fn print_error(message: &str) {
    eprintln!("{} {}", "Error:".red().bold(), message);
}

pub fn print_success(message: &str) {
    println!("{} {}", "✓".green().bold(), message);
}

pub fn print_info(message: &str) {
    println!("{} {}", "ℹ".cyan().bold(), message);
}
