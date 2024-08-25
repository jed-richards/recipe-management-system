use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Recipe {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub ingredients: Vec<Ingredient>,
    pub steps: Vec<String>,
    pub preparation_time: Option<u32>,
    pub cook_time: Option<u32>,
    pub servings: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ingredient {
    pub name: String,
    pub quantity: u32,
    pub units: Option<String>,
}
