use secrecy::{ExposeSecret, SecretBox};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretBox<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

// impl Clone for DatabaseSettings {
//     fn clone(&self) -> Self {
//         DatabaseSettings {
//             database_name: self.database_name.clone(),
//             host: self.host.clone(),
//             port: self.port,
//             username: self.username.clone(),
//             password: SecretBox::new(Box::new(self.password.expose_secret().clone())),
//         }
//     }
// }

impl DatabaseSettings {
    pub fn connection_string(&self) -> SecretBox<String> {
        SecretBox::new(Box::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        )))
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;

    settings.try_deserialize::<Settings>()
}
