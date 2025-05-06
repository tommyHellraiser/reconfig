use serde::Deserialize;
use crate::manager::ConfigType;

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
    String::from(r#"server:
    host: 127.0.0.1
    ports:
        - 8080
        - 8443
        - 9000
    admin_email: admin@example.com
    max_connections: 1000
    tls_enabled: true
    certificate_path: /etc/ssl/server.crt
    private_key_path: /etc/ssl/server.key
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
        - /var/log/app.log
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

fn get_full_toml_input() -> String {
    String::from(r#"server = {
    host = "127.0.0.1",
    ports = [8080, 8443, 9000],
    admin_email = "admin@example.com",
    max_connections = 1000,
    tls_enabled = true,
    certificate_path = "/etc/ssl/server.crt",
    private_key_path = "/etc/ssl/server.key",
    timeouts = {
        read = "30s",
        write = "60s"
    }
    }

    database = {
    type = "postgresql",
    url = "postgres://user:password@localhost:5432/mydb",
    connection_pool_size = 20,
    migrations_enabled = true,
    retry_attempts = 5
    }

    logging = {
    level = "info",
    output_paths = ["/var/log/app.log", "stdout"],
    format = "json"
    }

    features = {
    new_dashboard = true,
    analytics_enabled = false,
    experimental_widgets = ["widget_a", "widget_b"]
    }

    users = [
    { id = 1, username = "alice", roles = ["admin", "editor"] },
    { id = 2, username = "bob", roles = ["viewer"] },
    { id = 3, username = "charlie", roles = [] }
    ]

    optional_setting = "this is present"
    nullable_value = null"#)
}

fn get_full_input_deserialized() -> Config {
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
fn test_json_input() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct Test {
        configuration: String,
        testing: String,
        number_of_test: u8
    }
    let expected = Test {
        configuration: "Config".to_string(),
        testing: "testing1".to_string(),
        number_of_test: 1
    };

    let input = r#"{
        "configuration": "Config",
        "testing": "testing1",
        "number_of_test": 1
    }"#;
    let output = crate::ConfigManager::new_from_str::<Test>(ConfigType::Json, input).unwrap();

    assert_eq!(output, expected)
}

#[test]
fn full_json_test() {
    let expected = get_full_input_deserialized();

    let input = get_full_input_json();
    let output = crate::ConfigManager::new_from_str::<Config>(ConfigType::Json, &input).unwrap();

    assert_eq!(output, expected)
}
