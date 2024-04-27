use std::fmt::{self, Display, Formatter};
use std::panic;
use std::time::Duration;
use std::{fs, u16};

use serde::{Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
struct Node {
    protocol: String,
    host: String,
    user: String,
    password: String,
}

#[derive(Deserialize, Debug)]
struct AppConf {
    name: String,
    #[serde(deserialize_with = "deserialize_duration")]
    request_timeout: DurationDef,
    #[serde(deserialize_with = "deserialize_duration")]
    slow_threshold: DurationDef,
    max_threads: u16,
    node: Node,
}

// 反序列化函数，用于将字符串反序列化为'DurationDef'
fn deserialize_duration<'de, D>(deserializer: D) -> Result<DurationDef, D::Error>
where
    D: Deserializer<'de>,
{
    DurationDef::deserialize(deserializer)
}

// 加载配置文件
fn load_config(path: &str) -> AppConf {
    let config_str = fs::read_to_string(path).unwrap();
    let config: Result<AppConf, _> = toml::from_str(&config_str);
    match config {
        Ok(app) => app,
        Err(e) => {
            eprintln!("Error deserializing config: {}", e);
            panic!("Failed to load config");
        }
    }
}

#[derive(Debug, PartialEq)]
struct DurationParseError(String);
impl Display for DurationParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid duration string: {}", self.0)
    }
}

// 辅助函数，用于将字符串解析为'Duration'
fn parse_duration(s: &str) -> Result<Duration, DurationParseError> {
    let mut chars = s.chars();
    let mut duration = 0;

    while let Some(c) = chars.next() {
        let x = match c.to_digit(10) {
            Some(x) => x,
            None => {
                return Err(DurationParseError(format!(
                    "Invalid character in duration string: {}",
                    c
                )))
            }
        };
        duration = duration * 10 + x as u64;

        if chars.as_str().starts_with('s') {
            return Ok(Duration::from_secs(duration));
        } else if chars.as_str().starts_with("ms") {
            return Ok(Duration::from_millis(duration));
        }
    }
    Err(DurationParseError(format!(
        "Invalid character in duration string: {}",
        s
    )))
}

// 定义一个新的类型实现'Deserializer' trait
#[derive(Debug, PartialEq)]
struct DurationDef(Duration);

// 为我们新定义的类型实现 `Deserialize` trait
impl<'de> Deserialize<'de> for DurationDef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DurationVisitor;

        impl<'de> serde::de::Visitor<'de> for DurationVisitor {
            type Value = DurationDef;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a duration string like '30s' or '200ms'")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                parse_duration(s)
                    .map(DurationDef)
                    .map_err(|e| E::custom(format!("Invalid duration string: {}", e)))
            }
        }

        deserializer.deserialize_str(DurationVisitor)
    }
}

#[cfg(test)]
mod tests {
    use std::{env, path::Path, time::Duration};

    use super::*;

    #[test]
    fn parse_duration_test() {
        assert_eq!(parse_duration("200ms"), Ok(Duration::from_millis(200)));
        assert_eq!(parse_duration("2s"), Ok(Duration::from_secs(2)));
        assert_eq!(
            parse_duration("ff2"),
            Err(DurationParseError(
                "Invalid character in duration string: f".to_string()
            ))
        );
        assert_eq!(
            parse_duration("2f"),
            Err(DurationParseError(
                "Invalid character in duration string: f".to_string()
            ))
        );
    }

    #[test]
    fn load_config_test() {
        let execute_path = env::current_exe().unwrap();
        let config_toml_path = Path::new(&execute_path)
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap();

        // error config toml
        // let error_config_file = "src/conf/error.toml";
        // load_config(config_toml_path.join(error_config_file).to_str().unwrap());

        // correct config toml
        let correct_config_file = "src/conf/examples.toml";
        load_config(config_toml_path.join(correct_config_file).to_str().unwrap());
    }
}
