use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};

#[get("/{id}")]
async fn get_recipe(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("Get Recipe {}", id))
}

#[get("")]
async fn list_recipes() -> impl Responder {
    HttpResponse::Ok().body("List Recipes")
}

#[post("")]
async fn create_recipe() -> impl Responder {
    HttpResponse::Ok().body("Create Recipe")
}

#[delete("/{id}")]
async fn delete_recipe(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("Delete Recipe {}", id))
}

#[patch("/{id}")]
async fn update_recipe(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("Update Recipe {}", id))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_recipe)
        .service(list_recipes)
        .service(create_recipe)
        .service(delete_recipe)
        .service(update_recipe);
}
