use std::collections::HashMap;

pub fn read_config_file(path: &str) -> HashMap<String, String> {
    let content = std::fs::read_to_string(path).expect("Failed to read config file");
    parse_config(&content)
}

fn parse_config(content: &str) -> HashMap<String, String> {
    let mut config = HashMap::new();
    for line in content.lines() {
        if let Some((key, value)) = line.split_once('=') {
            config.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    config
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config() {
        let content = "key1=value1\nkey2 = value2";
        let config = parse_config(content);
        assert_eq!(config.get("key1"), Some(&"value1".to_string()));
        assert_eq!(config.get("key2"), Some(&"value2".to_string()));
    }
}