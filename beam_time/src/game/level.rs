use std::path::PathBuf;

use anyhow::Result;
use engine::exports::nalgebra::Vector2;
use serde::Deserialize;

pub fn load_levels() -> Vec<Level> {
    vec![]
}

#[derive(Deserialize)]
pub struct Level {
    name: String,
    description: String,

    tests: Tests,
}

#[derive(Deserialize)]
pub struct Tests {
    cases: Vec<TestCase>,

    lasers: Vec<ElementLocation>,
    detectors: Vec<ElementLocation>,
}

#[derive(Deserialize)]
pub struct TestCase {
    lasers: Vec<bool>,
    detectors: Vec<bool>,
}

#[derive(Deserialize)]
pub enum ElementLocation {
    Static(Vector2<i32>),
    Dynamic(usize),
}

impl Level {
    pub fn load(path: PathBuf) -> Result<Self> {
        todo!()
    }
}
