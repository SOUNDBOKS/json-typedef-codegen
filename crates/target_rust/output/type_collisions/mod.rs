// Code generated by jtd-codegen for Rust v0.1.0

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RootFooBar {
    #[serde(rename = "x")]
    pub x: bool,
}

#[derive(Serialize, Deserialize)]
pub struct RootFoo {
    #[serde(rename = "bar")]
    pub bar: RootFooBar,
}

#[derive(Serialize, Deserialize)]
pub struct RootFooBar0 {
    #[serde(rename = "x")]
    pub x: String,
}

#[derive(Serialize, Deserialize)]
pub struct Root {
    #[serde(rename = "foo")]
    pub foo: RootFoo,

    #[serde(rename = "foo_bar")]
    pub fooBar: RootFooBar0,
}