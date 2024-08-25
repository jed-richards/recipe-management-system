// @generated automatically by Diesel CLI.

diesel::table! {
    ingredients (id) {
        id -> Uuid,
        recipe_id -> Nullable<Uuid>,
        name -> Text,
        quantity -> Int4,
        units -> Nullable<Text>,
    }
}

diesel::table! {
    recipes (id) {
        id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
        preparation_time -> Nullable<Int4>,
        cooking_time -> Nullable<Int4>,
        servings -> Nullable<Int4>,
        tags -> Nullable<Array<Nullable<Text>>>,
        categories -> Nullable<Array<Nullable<Uuid>>>,
    }
}

diesel::joinable!(ingredients -> recipes (recipe_id));

diesel::allow_tables_to_appear_in_same_query!(ingredients, recipes,);
