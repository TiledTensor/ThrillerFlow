use std::{fmt::Display, str::FromStr};

use crate::ThrillerError;

/// Data Type Define for NVIDIA GPU.
pub enum DataType {
    /// 32-bit floating point.
    Float32,
    /// 64-bit floating point.
    Float64,
    /// 16-bit floating point.
    Half,
    /// Cutlass 16-bit floating point.
    Cutlasshalf,
    /// Brain floating point.
    BF16,
}

impl FromStr for DataType {
    type Err = ThrillerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "float" => Ok(DataType::Float32),
            "double" => Ok(DataType::Float64),
            "half" => Ok(DataType::Half),
            "cutlass::half_t" => Ok(DataType::Cutlasshalf),
            _ => Err(ThrillerError::ParseError),
        }
    }
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Float32 => write!(f, "float"),
            DataType::Float64 => write!(f, "double"),
            DataType::Half => write!(f, "half"),
            DataType::Cutlasshalf => write!(f, "cutlass::half_t"),
            DataType::BF16 => write!(f, "bfloat16"),
        }
    }
}
