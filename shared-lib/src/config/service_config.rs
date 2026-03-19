use crate::config::env::Environment;

pub trait ServiceConfig {
    fn get_environment(&self) -> Environment;
}
