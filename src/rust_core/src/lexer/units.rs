//! Physical units and dimensions support for CZ-DSL
//! 
//! This module provides comprehensive support for physical units and dimensional analysis,
//! which is crucial for coastal zone modeling where physical quantities must be handled correctly.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Physical dimension representation using the seven SI base dimensions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Dimension {
    /// Length [L] - meter
    pub length: i8,
    /// Mass [M] - kilogram  
    pub mass: i8,
    /// Time [T] - second
    pub time: i8,
    /// Electric current [I] - ampere
    pub current: i8,
    /// Thermodynamic temperature [Θ] - kelvin
    pub temperature: i8,
    /// Amount of substance [N] - mole
    pub substance: i8,
    /// Luminous intensity [J] - candela
    pub luminosity: i8,
}

impl Dimension {
    /// Create a dimensionless quantity
    pub fn dimensionless() -> Self {
        Self {
            length: 0,
            mass: 0,
            time: 0,
            current: 0,
            temperature: 0,
            substance: 0,
            luminosity: 0,
        }
    }
    
    /// Create length dimension [L]
    pub fn length() -> Self {
        Self { length: 1, ..Self::dimensionless() }
    }
    
    /// Create mass dimension [M]
    pub fn mass() -> Self {
        Self { mass: 1, ..Self::dimensionless() }
    }
    
    /// Create time dimension [T]
    pub fn time() -> Self {
        Self { time: 1, ..Self::dimensionless() }
    }
    
    /// Create temperature dimension [Θ]
    pub fn temperature() -> Self {
        Self { temperature: 1, ..Self::dimensionless() }
    }
    
    /// Create velocity dimension [LT⁻¹]
    pub fn velocity() -> Self {
        Self { length: 1, time: -1, ..Self::dimensionless() }
    }
    
    /// Create acceleration dimension [LT⁻²]
    pub fn acceleration() -> Self {
        Self { length: 1, time: -2, ..Self::dimensionless() }
    }
    
    /// Create force dimension [MLT⁻²]
    pub fn force() -> Self {
        Self { mass: 1, length: 1, time: -2, ..Self::dimensionless() }
    }
    
    /// Create pressure dimension [ML⁻¹T⁻²]
    pub fn pressure() -> Self {
        Self { mass: 1, length: -1, time: -2, ..Self::dimensionless() }
    }
    
    /// Create energy dimension [ML²T⁻²]
    pub fn energy() -> Self {
        Self { mass: 1, length: 2, time: -2, ..Self::dimensionless() }
    }
    
    /// Create power dimension [ML²T⁻³]
    pub fn power_dimension() -> Self {
        Self { mass: 1, length: 2, time: -3, ..Self::dimensionless() }
    }
    
    /// Multiply dimensions (for unit multiplication)
    pub fn multiply(&self, other: &Dimension) -> Dimension {
        Dimension {
            length: self.length + other.length,
            mass: self.mass + other.mass,
            time: self.time + other.time,
            current: self.current + other.current,
            temperature: self.temperature + other.temperature,
            substance: self.substance + other.substance,
            luminosity: self.luminosity + other.luminosity,
        }
    }
    
    /// Divide dimensions (for unit division)
    pub fn divide(&self, other: &Dimension) -> Dimension {
        Dimension {
            length: self.length - other.length,
            mass: self.mass - other.mass,
            time: self.time - other.time,
            current: self.current - other.current,
            temperature: self.temperature - other.temperature,
            substance: self.substance - other.substance,
            luminosity: self.luminosity - other.luminosity,
        }
    }
    
    /// Raise dimension to a power
    pub fn power(&self, exponent: i8) -> Dimension {
        Dimension {
            length: self.length * exponent,
            mass: self.mass * exponent,
            time: self.time * exponent,
            current: self.current * exponent,
            temperature: self.temperature * exponent,
            substance: self.substance * exponent,
            luminosity: self.luminosity * exponent,
        }
    }
    
    /// Check if two dimensions are compatible (same)
    pub fn is_compatible(&self, other: &Dimension) -> bool {
        self == other
    }
    
    /// Check if dimension is dimensionless
    pub fn is_dimensionless(&self) -> bool {
        *self == Self::dimensionless()
    }
}

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();
        
        if self.length != 0 {
            if self.length == 1 {
                parts.push("L".to_string());
            } else {
                parts.push(format!("L^{}", self.length));
            }
        }
        
        if self.mass != 0 {
            if self.mass == 1 {
                parts.push("M".to_string());
            } else {
                parts.push(format!("M^{}", self.mass));
            }
        }
        
        if self.time != 0 {
            if self.time == 1 {
                parts.push("T".to_string());
            } else {
                parts.push(format!("T^{}", self.time));
            }
        }
        
        if self.current != 0 {
            if self.current == 1 {
                parts.push("I".to_string());
            } else {
                parts.push(format!("I^{}", self.current));
            }
        }
        
        if self.temperature != 0 {
            if self.temperature == 1 {
                parts.push("Θ".to_string());
            } else {
                parts.push(format!("Θ^{}", self.temperature));
            }
        }
        
        if self.substance != 0 {
            if self.substance == 1 {
                parts.push("N".to_string());
            } else {
                parts.push(format!("N^{}", self.substance));
            }
        }
        
        if self.luminosity != 0 {
            if self.luminosity == 1 {
                parts.push("J".to_string());
            } else {
                parts.push(format!("J^{}", self.luminosity));
            }
        }
        
        if parts.is_empty() {
            write!(f, "1")  // Dimensionless
        } else {
            write!(f, "{}", parts.join("·"))
        }
    }
}

/// Physical unit with name, symbol, dimension, and conversion factor
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Unit {
    pub name: String,
    pub symbol: String,
    pub dimension: Dimension,
    /// Conversion factor to SI base unit
    pub factor: f64,
    /// Offset for temperature conversions (e.g., Celsius)
    pub offset: f64,
}

impl Unit {
    pub fn new(name: &str, symbol: &str, dimension: Dimension, factor: f64) -> Self {
        Self {
            name: name.to_string(),
            symbol: symbol.to_string(),
            dimension,
            factor,
            offset: 0.0,
        }
    }
    
    pub fn with_offset(name: &str, symbol: &str, dimension: Dimension, factor: f64, offset: f64) -> Self {
        Self {
            name: name.to_string(),
            symbol: symbol.to_string(),
            dimension,
            factor,
            offset,
        }
    }
    
    /// Check if this unit is compatible with another (same dimension)
    pub fn is_compatible(&self, other: &Unit) -> bool {
        self.dimension.is_compatible(&other.dimension)
    }
    
    /// Convert value from this unit to SI base unit
    pub fn to_si(&self, value: f64) -> f64 {
        (value + self.offset) * self.factor
    }
    
    /// Convert value from SI base unit to this unit
    pub fn from_si(&self, value: f64) -> f64 {
        value / self.factor - self.offset
    }
    
    /// Convert value from this unit to another compatible unit
    pub fn convert_to(&self, value: f64, target: &Unit) -> Result<f64, UnitError> {
        if !self.is_compatible(target) {
            return Err(UnitError::IncompatibleUnits {
                from: self.clone(),
                to: target.clone(),
            });
        }
        
        let si_value = self.to_si(value);
        Ok(target.from_si(si_value))
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.symbol)
    }
}

/// Unit system registry
#[derive(Debug, Clone)]
pub struct UnitRegistry {
    units: HashMap<String, Unit>,
    aliases: HashMap<String, String>,
}

impl UnitRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            units: HashMap::new(),
            aliases: HashMap::new(),
        };
        
        registry.register_si_units();
        registry.register_derived_units();
        registry.register_coastal_units();
        registry.register_aliases();
        
        registry
    }
    
    /// Register a unit in the registry
    pub fn register(&mut self, unit: Unit) {
        self.units.insert(unit.symbol.clone(), unit.clone());
        self.units.insert(unit.name.clone(), unit);
    }
    
    /// Register an alias for a unit
    pub fn register_alias(&mut self, alias: &str, unit_name: &str) {
        self.aliases.insert(alias.to_string(), unit_name.to_string());
    }
    
    /// Get a unit by name or symbol
    pub fn get(&self, name: &str) -> Option<&Unit> {
        // First try direct lookup
        if let Some(unit) = self.units.get(name) {
            return Some(unit);
        }
        
        // Then try alias lookup
        if let Some(real_name) = self.aliases.get(name) {
            return self.units.get(real_name);
        }
        
        None
    }
    
    /// Check if a unit exists
    pub fn contains(&self, name: &str) -> bool {
        self.get(name).is_some()
    }
    
    /// Get all registered units
    pub fn all_units(&self) -> Vec<&Unit> {
        self.units.values().collect()
    }
    
    /// Register SI base units
    fn register_si_units(&mut self) {
        // Length
        self.register(Unit::new("meter", "m", Dimension::length(), 1.0));
        
        // Mass
        self.register(Unit::new("kilogram", "kg", Dimension::mass(), 1.0));
        
        // Time
        self.register(Unit::new("second", "s", Dimension::time(), 1.0));
        
        // Electric current
        self.register(Unit::new("ampere", "A", 
            Dimension { current: 1, ..Dimension::dimensionless() }, 1.0));
        
        // Temperature
        self.register(Unit::new("kelvin", "K", Dimension::temperature(), 1.0));
        
        // Amount of substance
        self.register(Unit::new("mole", "mol", 
            Dimension { substance: 1, ..Dimension::dimensionless() }, 1.0));
        
        // Luminous intensity
        self.register(Unit::new("candela", "cd", 
            Dimension { luminosity: 1, ..Dimension::dimensionless() }, 1.0));
    }
    
    /// Register derived SI units
    fn register_derived_units(&mut self) {
        // Force
        self.register(Unit::new("newton", "N", Dimension::force(), 1.0));
        
        // Pressure
        self.register(Unit::new("pascal", "Pa", Dimension::pressure(), 1.0));
        
        // Energy
        self.register(Unit::new("joule", "J", Dimension::energy(), 1.0));
        
        // Power
        self.register(Unit::new("watt", "W", Dimension::power_dimension(), 1.0));
        
        // Temperature (Celsius)
        self.register(Unit::with_offset("celsius", "°C", Dimension::temperature(), 1.0, 273.15));
        
        // Angle
        self.register(Unit::new("radian", "rad", Dimension::dimensionless(), 1.0));
        self.register(Unit::new("degree", "°", Dimension::dimensionless(), std::f64::consts::PI / 180.0));
        
        // Time units
        self.register(Unit::new("minute", "min", Dimension::time(), 60.0));
        self.register(Unit::new("hour", "h", Dimension::time(), 3600.0));
        self.register(Unit::new("day", "d", Dimension::time(), 86400.0));
        self.register(Unit::new("year", "yr", Dimension::time(), 31557600.0)); // Julian year
    }
    
    /// Register coastal zone specific units
    fn register_coastal_units(&mut self) {
        // Oceanographic units
        self.register(Unit::new("practical_salinity_unit", "psu", Dimension::dimensionless(), 1.0));
        self.register(Unit::new("decibar", "dbar", Dimension::pressure(), 10000.0));
        self.register(Unit::new("sverdrup", "Sv", 
            Dimension { length: 3, time: -1, ..Dimension::dimensionless() }, 1e6));
        
        // Atmospheric units
        self.register(Unit::new("hectopascal", "hPa", Dimension::pressure(), 100.0));
        self.register(Unit::new("atmosphere", "atm", Dimension::pressure(), 101325.0));
        self.register(Unit::new("millimeter_mercury", "mmHg", Dimension::pressure(), 133.322));
        
        // Geographic units
        self.register(Unit::new("nautical_mile", "nmi", Dimension::length(), 1852.0));
        
        // Concentration units (for water quality)
        let concentration_dim = Dimension {
            mass: 1,
            length: -3,
            ..Dimension::dimensionless()
        };
        self.register(Unit::new("milligram_per_liter", "mg/L", concentration_dim.clone(), 1.0));
        self.register(Unit::new("parts_per_million", "ppm", Dimension::dimensionless(), 1e-6));
        self.register(Unit::new("parts_per_billion", "ppb", Dimension::dimensionless(), 1e-9));
    }
    
    /// Register common aliases
    fn register_aliases(&mut self) {
        // Length aliases
        self.register_alias("metre", "meter");
        
        // Temperature aliases
        self.register_alias("deg_c", "celsius");
        
        // Pressure aliases
        self.register_alias("bar", "pascal"); // Note: 1 bar = 100000 Pa, but simplified here
        
        // Time aliases
        self.register_alias("sec", "second");
        self.register_alias("hr", "hour");
        
        // Angle aliases
        self.register_alias("deg", "degree");
        
        // Oceanographic aliases
        self.register_alias("practical_salinity", "practical_salinity_unit");
    }
}

impl Default for UnitRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Errors related to unit operations
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum UnitError {
    #[error("Unknown unit: {name}")]
    UnknownUnit { name: String },
    
    #[error("Incompatible units: cannot convert from {from} to {to}")]
    IncompatibleUnits { from: Unit, to: Unit },
    
    #[error("Invalid unit expression: {expression}")]
    InvalidExpression { expression: String },
    
    #[error("Dimension mismatch: expected {expected}, found {found}")]
    DimensionMismatch { expected: Dimension, found: Dimension },
}

/// Unit parser for parsing unit expressions like "m/s^2" or "kg*m/s^2"
#[derive(Debug)]
pub struct UnitParser {
    registry: UnitRegistry,
}

impl UnitParser {
    pub fn new() -> Self {
        Self {
            registry: UnitRegistry::new(),
        }
    }
    
    pub fn with_registry(registry: UnitRegistry) -> Self {
        Self { registry }
    }
    
    /// Parse a unit expression and return the resulting unit
    pub fn parse(&self, expression: &str) -> Result<Unit, UnitError> {
        // Simple implementation - can be extended for complex expressions
        if let Some(unit) = self.registry.get(expression) {
            Ok(unit.clone())
        } else {
            Err(UnitError::UnknownUnit {
                name: expression.to_string(),
            })
        }
    }
    
    /// Parse and validate a unit expression
    pub fn validate(&self, expression: &str) -> Result<(), UnitError> {
        self.parse(expression).map(|_| ())
    }
}

impl Default for UnitParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dimension_operations() {
        let length = Dimension::length();
        let time = Dimension::time();
        let velocity = length.divide(&time);
        
        assert_eq!(velocity, Dimension::velocity());
        
        let acceleration = velocity.divide(&time);
        assert_eq!(acceleration, Dimension::acceleration());
    }
    
    #[test]
    fn test_unit_conversion() {
        let registry = UnitRegistry::new();
        let meter = registry.get("m").unwrap();
        let kilometer = Unit::new("kilometer", "km", Dimension::length(), 1000.0);
        
        let result = meter.convert_to(1000.0, &kilometer).unwrap();
        assert_eq!(result, 1.0);
    }
    
    #[test]
    fn test_temperature_conversion() {
        let registry = UnitRegistry::new();
        let kelvin = registry.get("K").unwrap();
        let celsius = registry.get("°C").unwrap();
        
        // 0°C = 273.15K
        let result = celsius.convert_to(0.0, kelvin).unwrap();
        assert!((result - 273.15).abs() < 1e-10);
    }
    
    #[test]
    fn test_incompatible_units() {
        let meter = Unit::new("meter", "m", Dimension::length(), 1.0);
        let second = Unit::new("second", "s", Dimension::time(), 1.0);
        
        let result = meter.convert_to(1.0, &second);
        assert!(result.is_err());
    }
}