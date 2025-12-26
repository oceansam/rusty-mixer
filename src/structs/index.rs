pub trait Device {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn is_default(&self) -> bool;
}

#[derive(Debug, Clone)]
pub struct AudioDevice {
    pub id: String,
    pub name: String,
    pub is_default: bool,
}

impl Device for AudioDevice {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn is_default(&self) -> bool {
        self.is_default
    }
}