use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CmecsComponent {
    WaterColumn,
    Geoform,
    Substrate,
    Biotic,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CmecsUnit {
    pub id: String,
    pub name: String,
    pub component: CmecsComponent,
    pub description: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CmecsCatalog {
    pub units: HashMap<String, CmecsUnit>,
    pub version: Option<String>,
    pub source: Option<String>,
}

impl CmecsUnit {
    pub fn new(id: String, name: String, component: CmecsComponent, description: String) -> Self {
        Self {
            id,
            name,
            component,
            description,
        }
    }
}

impl CmecsCatalog {
    pub fn new() -> Self {
        Self {
            units: HashMap::new(),
            version: None,
            source: None,
        }
    }

    pub fn register_unit(&mut self, unit: CmecsUnit) {
        self.units.insert(unit.id.clone(), unit);
    }

    pub fn get(&self, id: &str) -> Option<&CmecsUnit> {
        self.units.get(id)
    }
}

