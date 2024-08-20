use common::{Ingredient, Recipe};

use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};

#[get("/{id}")]
async fn get_recipe(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();

    let recipe = Recipe {
        id,
        name: "name".to_string(),
        description: "description".to_string(),
        ingredients: vec![
            Ingredient {
                name: "ingredient 1".to_string(),
                quantity: 3,
                units: Some("Cups".to_string()),
            },
            Ingredient {
                name: "ingredient 2".to_string(),
                quantity: 5,
                units: None,
            },
        ],
        steps: vec![
            "Step 1".to_string(),
            "Step 2".to_string(),
            "Step 3".to_string(),
        ],
        preparation_time: None,
        cook_time: None,
        servings: None,
    };

    HttpResponse::Ok().json(recipe)
}

#[get("")]
async fn list_recipes() -> impl Responder {
    let recipes = vec![
        Recipe {
            id: 1,
            name: "name".to_string(),
            description: "description".to_string(),
            ingredients: vec![
                Ingredient {
                    name: "ingredient 1".to_string(),
                    quantity: 3,
                    units: Some("Cups".to_string()),
                },
                Ingredient {
                    name: "ingredient 2".to_string(),
                    quantity: 5,
                    units: None,
                },
            ],
            steps: vec![
                "Step 1".to_string(),
                "Step 2".to_string(),
                "Step 3".to_string(),
            ],
            preparation_time: None,
            cook_time: None,
            servings: None,
        },
        Recipe {
            id: 2,
            name: "name 2".to_string(),
            description: "description".to_string(),
            ingredients: vec![
                Ingredient {
                    name: "ingredient 1".to_string(),
                    quantity: 3,
                    units: Some("Cups".to_string()),
                },
                Ingredient {
                    name: "ingredient 2".to_string(),
                    quantity: 5,
                    units: None,
                },
            ],
            steps: vec![
                "Step 1".to_string(),
                "Step 2".to_string(),
                "Step 3".to_string(),
            ],
            preparation_time: None,
            cook_time: None,
            servings: None,
        },
    ];

    HttpResponse::Ok().json(recipes)
}

#[post("")]
async fn create_recipe() -> impl Responder {
    let recipe = Recipe {
        id: 1,
        name: "name".to_string(),
        description: "description".to_string(),
        ingredients: vec![
            Ingredient {
                name: "ingredient 1".to_string(),
                quantity: 3,
                units: Some("Cups".to_string()),
            },
            Ingredient {
                name: "ingredient 2".to_string(),
                quantity: 5,
                units: None,
            },
        ],
        steps: vec![
            "Step 1".to_string(),
            "Step 2".to_string(),
            "Step 3".to_string(),
        ],
        preparation_time: None,
        cook_time: None,
        servings: None,
    };

    HttpResponse::Ok().json(recipe)
}

#[delete("/{id}")]
async fn delete_recipe(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();

    let recipe = Recipe {
        id,
        name: "name".to_string(),
        description: "description".to_string(),
        ingredients: vec![
            Ingredient {
                name: "ingredient 1".to_string(),
                quantity: 3,
                units: Some("Cups".to_string()),
            },
            Ingredient {
                name: "ingredient 2".to_string(),
                quantity: 5,
                units: None,
            },
        ],
        steps: vec![
            "Step 1".to_string(),
            "Step 2".to_string(),
            "Step 3".to_string(),
        ],
        preparation_time: None,
        cook_time: None,
        servings: None,
    };

    HttpResponse::Ok().json(recipe)
}

#[patch("/{id}")]
async fn update_recipe(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();

    let recipe = Recipe {
        id,
        name: "name".to_string(),
        description: "description".to_string(),
        ingredients: vec![
            Ingredient {
                name: "ingredient 1".to_string(),
                quantity: 3,
                units: Some("Cups".to_string()),
            },
            Ingredient {
                name: "ingredient 2".to_string(),
                quantity: 5,
                units: None,
            },
        ],
        steps: vec![
            "Step 1".to_string(),
            "Step 2".to_string(),
            "Step 3".to_string(),
        ],
        preparation_time: None,
        cook_time: None,
        servings: None,
    };

    HttpResponse::Ok().json(recipe)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_recipe)
        .service(list_recipes)
        .service(create_recipe)
        .service(delete_recipe)
        .service(update_recipe);
}
