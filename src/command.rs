use crate::{Config, CursedErrorHandle};

pub struct Command {
    pub method: fn(&Config) -> Result<(), CursedErrorHandle>,
    pub description: &'static str,
    pub name: &'static str,
}
