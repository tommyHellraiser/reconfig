use serde::Deserialize;
use crate::{manager::ConfigType, ConfigManager};

#[derive(Deserialize, Debug, PartialEq)]
struct SmallTest {
    configuration: String,
    testing: String,
    number_of_test: u8,
    sub: SubSmallTest
}

#[derive(Deserialize, Debug, PartialEq)]
struct SubSmallTest {
    this: String,
    the: i8
}

#[derive(Deserialize, Debug, PartialEq)]
struct Config {
    server: ServerConfig,
    database: DatabaseConfig,
    logging: LoggingConfig,
    features: FeaturesConfig,
    users: Vec<User>,
    optional_setting: Option<String>,
    nullable_value: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ServerConfig {
    host: String,
    ports: Vec<u16>,
    admin_email: String,
    max_connections: u32,
    #[serde(default = "default_tls_enabled")]
    tls_enabled: bool,
    certificate_path: String,
    private_key_path: String,
    timeouts: TimeoutsConfig,
}

fn default_tls_enabled() -> bool {
    false
}

#[derive(Deserialize, Debug, PartialEq)]
struct TimeoutsConfig {
    read: String, // You might want to parse this into a Duration later
    write: String, // Same here
}

#[derive(Deserialize, Debug, PartialEq)]
struct DatabaseConfig {
    #[serde(rename = "type")]
    db_type: String,
    url: String,
    connection_pool_size: u32,
    migrations_enabled: bool,
    retry_attempts: u32,
}

#[derive(Deserialize, Debug, PartialEq)]
struct LoggingConfig {
    level: String,
    output_paths: Vec<String>,
    format: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct FeaturesConfig {
    new_dashboard: bool,
    analytics_enabled: bool,
    experimental_widgets: Vec<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct User {
    id: u32,
    username: String,
    roles: Vec<String>,
}

fn get_full_input_json() -> String {
    String::from(r#"{
    "server": {
        "host": "127.0.0.1",
        "ports": [8080, 8443, 9000],
        "admin_email": "admin@example.com",
        "max_connections": 1000,
        "tls_enabled": true,
        "certificate_path": "/etc/ssl/server.crt",
        "private_key_path": "/etc/ssl/server.key",
        "timeouts": {
        "read": "30s",
        "write": "60s"
        }
    },
    "database": {
        "type": "postgresql",
        "url": "postgres://user:password@localhost:5432/mydb",
        "connection_pool_size": 20,
        "migrations_enabled": true,
        "retry_attempts": 5
    },
    "logging": {
        "level": "info",
        "output_paths": ["/var/log/app.log", "stdout"],
        "format": "json"
    },
    "features": {
        "new_dashboard": true,
        "analytics_enabled": false,
        "experimental_widgets": ["widget_a", "widget_b"]
    },
    "users": [
        { "id": 1, "username": "alice", "roles": ["admin", "editor"] },
        { "id": 2, "username": "bob", "roles": ["viewer"] },
        { "id": 3, "username": "charlie", "roles": [] }
    ],
    "optional_setting": "this is present",
    "nullable_value": null
    }"#)
}

fn get_full_input_yaml() -> String {
    String::from(r#"
server:
  host: 127.0.0.1
  ports:
  - 8080
  - 8443
  - 9000
  admin_email: admin@example.com
  max_connections: 1000
  tls_enabled: true
  certificate_path: "/etc/ssl/server.crt"
  private_key_path: "/etc/ssl/server.key"
  timeouts:
    read: 30s
    write: 60s
database:
  type: postgresql
  url: postgres://user:password@localhost:5432/mydb
  connection_pool_size: 20
  migrations_enabled: true
  retry_attempts: 5
logging:
  level: info
  output_paths:
  - "/var/log/app.log"
  - stdout
  format: json
features:
  new_dashboard: true
  analytics_enabled: false
  experimental_widgets:
  - widget_a
  - widget_b
users:
- id: 1
  username: alice
  roles:
  - admin
  - editor
- id: 2
  username: bob
  roles:
  - viewer
- id: 3
  username: charlie
  roles: []
optional_setting: this is present
nullable_value: null"#)
}

fn get_full_input_toml() -> String {
    String::from(r#"optional_setting = "this is present"

[server]
host = "127.0.0.1"
ports = [ 8_080, 8_443, 9_000 ]
admin_email = "admin@example.com"
max_connections = 1_000
tls_enabled = true
certificate_path = "/etc/ssl/server.crt"
private_key_path = "/etc/ssl/server.key"

  [server.timeouts]
  read = "30s"
  write = "60s"

[database]
type = "postgresql"
url = "postgres://user:password@localhost:5432/mydb"
connection_pool_size = 20
migrations_enabled = true
retry_attempts = 5

[logging]
level = "info"
output_paths = [ "/var/log/app.log", "stdout" ]
format = "json"

[features]
new_dashboard = true
analytics_enabled = false
experimental_widgets = [ "widget_a", "widget_b" ]

[[users]]
id = 1
username = "alice"
roles = [ "admin", "editor" ]

[[users]]
id = 2
username = "bob"
roles = [ "viewer" ]

[[users]]
id = 3
username = "charlie"
roles = [ ]"#)
}

fn get_small_expected_output() -> SmallTest {
    let sub = SubSmallTest {
        this: String::from("is"),
        the: 5
    };
    SmallTest {
        configuration: "Config".to_string(),
        testing: "testing1".to_string(),
        number_of_test: 1,
        sub: sub
    }
}

fn get_full_expected_output() -> Config {
    Config {
        server: ServerConfig {
            host: "127.0.0.1".to_string(),
            ports: vec![8080, 8443, 9000],
            admin_email: "admin@example.com".to_string(),
            max_connections: 1000,
            tls_enabled: true,
            certificate_path: "/etc/ssl/server.crt".to_string(),
            private_key_path: "/etc/ssl/server.key".to_string(),
            timeouts: TimeoutsConfig {
                read: "30s".to_string(),
                write: "60s".to_string(),
            },
        },
        database: DatabaseConfig {
            db_type: "postgresql".to_string(),
            url: "postgres://user:password@localhost:5432/mydb".to_string(),
            connection_pool_size: 20,
            migrations_enabled: true,
            retry_attempts: 5,
        },
        logging: LoggingConfig {
            level: "info".to_string(),
            output_paths: vec!["/var/log/app.log".to_string(), "stdout".to_string()],
            format: "json".to_string(),
        },
        features: FeaturesConfig {
            new_dashboard: true,
            analytics_enabled: false,
            experimental_widgets: vec!["widget_a".to_string(), "widget_b".to_string()],
        },
        users: vec![
            User {
                id: 1,
                username: "alice".to_string(),
                roles: vec!["admin".to_string(), "editor".to_string()],
            },
            User {
                id: 2,
                username: "bob".to_string(),
                roles: vec!["viewer".to_string()],
            },
            User {
                id: 3,
                username: "charlie".to_string(),
                roles: vec![],
            },
        ],
        optional_setting: Some("this is present".to_string()),
        nullable_value: None,
    }
}

#[test]
fn small_json_test_from_str() {
    let expected = get_small_expected_output();

    let input = r#"{
        "configuration": "Config",
        "testing": "testing1",
        "number_of_test": 1,
        "sub": {
            "this": "is",
            "the": 5
        }
    }"#;
    let output = crate::ConfigManager::new_from_str::<SmallTest>(ConfigType::Json, input).unwrap();

    assert_eq!(output, expected)
}

#[test]
fn small_yaml_test_from_str() {
    let expected = get_small_expected_output();

    let input = r#"
configuration: Config
testing: testing1
number_of_test: 1
sub:
    this: is
    the: 5"#;
    let output = serde_yaml::from_str::<SmallTest>(input).unwrap();

    assert_eq!(output, expected)
}

#[test]
fn small_toml_test_from_str() {
    let expected = get_small_expected_output();

    let input = r#"configuration = "Config"
testing = "testing1"
number_of_test = 1

[sub]
this = "is"
the = 5"#;
    let output = ConfigManager::new_from_str::<SmallTest>(ConfigType::Toml, input).unwrap();

    assert_eq!(output, expected)
}

#[test]
fn full_json_test_from_str() {
    let expected = get_full_expected_output();

    let input = get_full_input_json();
    let output = crate::ConfigManager::new_from_str::<Config>(ConfigType::Json, &input).unwrap();

    assert_eq!(output, expected)
}

#[test]
fn full_yaml_from_str() {
    let expected = get_full_expected_output();

    let input = get_full_input_yaml();
    let output = crate::ConfigManager::new_from_str::<Config>(ConfigType::Yaml, &input).unwrap();

    assert_eq!(output, expected)
}

#[test]
fn full_toml_from_str() {
    let expected = get_full_expected_output();

    let input = get_full_input_toml();
    let output = crate::ConfigManager::new_from_str::<Config>(ConfigType::Toml, &input).unwrap();

    assert_eq!(output, expected)
}

#[test]
fn full_json_from_file() {
    let expected = get_full_expected_output();

    let dir = std::env::current_dir().unwrap();
    let dir_path = dir.to_str().unwrap();
    let path = format!("{dir_path}/test_files/input.json");

    let output = ConfigManager::new_from_path::<Config>(ConfigType::Json, path.as_str()).unwrap();

    assert_eq!(output, expected)
}

#[test]
fn full_yaml_from_file() {
    let expected = get_full_expected_output();

    let dir = std::env::current_dir().unwrap();
    let dir_path = dir.to_str().unwrap();
    let path = format!("{dir_path}/test_files/input.yaml");

    let output = ConfigManager::new_from_path::<Config>(ConfigType::Yaml, path.as_str()).unwrap();

    assert_eq!(output, expected)
}

#[test]
fn full_toml_from_file() {
    let expected = get_full_expected_output();

    let dir = std::env::current_dir().unwrap();
    let dir_path = dir.to_str().unwrap();
    let path = format!("{dir_path}/test_files/input.toml");

    let output = ConfigManager::new_from_path::<Config>(ConfigType::Toml, path.as_str()).unwrap();

    assert_eq!(output, expected)
}
