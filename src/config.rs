use std::collections::HashMap;

#[derive(Clone)]
pub struct ConfigData {
    pub email: String,
    pub billing_rate: String,
    pub tk_number: String,
    pub multi_bill_rate: bool,
}

/// Loads and parses config.ini — equivalent of the Python `load_config()`,
/// which used ConfigParser. Only the [user] section is read.
pub fn load_config() -> Result<ConfigData, String> {
    let text = std::fs::read_to_string("config.ini").map_err(|_| {
        "Configuration file not found. Please make sure 'config.ini' is \
         located in the same directory as the executable and try again."
            .to_string()
    })?;

    let mut values: HashMap<String, String> = HashMap::new();
    let mut in_user_section = false;

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            in_user_section = line[1..line.len() - 1].trim().eq_ignore_ascii_case("user");
            continue;
        }
        if !in_user_section {
            continue;
        }
        if let Some(eq) = line.find('=') {
            let key = line[..eq].trim().to_lowercase();
            let value = line[eq + 1..].trim().to_string();
            values.insert(key, value);
        }
    }

    let get = |key: &str| {
        values
            .get(key)
            .cloned()
            .ok_or_else(|| format!("Missing setting '{}' in the [user] section of config.ini.", key))
    };

    Ok(ConfigData {
        email: get("email")?,
        billing_rate: get("billing_rate")?,
        tk_number: get("tk_number")?,
        multi_bill_rate: get("multi_bill_rate")?.eq_ignore_ascii_case("yes"),
    })
}