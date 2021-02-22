// Code generated by jtd-codegen for Rust v0.1.0

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct RootOverrideTypeDiscriminatorBaz {}

#[derive(Serialize, Deserialize)]
pub struct Root {
    #[serde(rename = "override_elements_container")]
    pub overrideElementsContainer: Vec<String>,

    #[serde(rename = "override_type_discriminator")]
    pub overrideTypeDiscriminator: serde_json::Value,

    #[serde(rename = "override_type_enum")]
    pub overrideTypeEnum: serde_json::Value,

    #[serde(rename = "override_type_expr")]
    pub overrideTypeExpr: serde_json::Value,

    #[serde(rename = "override_type_properties")]
    pub overrideTypeProperties: serde_json::Value,

    #[serde(rename = "override_values_container")]
    pub overrideValuesContainer: HashMap<String, String>,
}