use lixiv_backend::prelude::*;
use petgraph::prelude::*;
use serde_json::{Value, json};

fn main() {
    // Build a tiny example graph:
    //
    // Kinds:
    //   Food
    //     └─ Ingredient
    //          └─ Nutrition
    //
    // Nodes:
    //   lasagne (Food)
    //   pizza-margherita (Food)
    //   tomatoes (Ingredient)
    //   lasagne-kcal (Nutrition)
    //
    // Edges:
    //   lasagne -> tomatoes (ingredient-of)
    //   pizza-margherita -> tomatoes (ingredient-of)
    //   tomatoes -> lasagne-kcal (nutrition-of)

    let mut registry = SchemaRegistry::default();

    let payload = json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://example.com/food.schema.json",
        "title": "Food",
        "description": "Food",
        "type": "object",
        "properties": {
            "name": {
                "type": "string"
            },
        },
        "required": [ "name" ]
    });
    let _ = registry.insert(&payload);

    let payload = json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://example.com/ingredient.schema.json",
        "title": "Ingredient",
        "description": "Ingredient",
        "type": "object",
        "properties": {
            "name": {
                "type": "string"
            },
        },
        "required": [ "name" ]
    });
    let _ = registry.insert(&payload);

    let payload = json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://example.com/nutrition.schema.json",
        "title": "Nutrition",
        "description": "Nutrition",
        "type": "object",
        "properties": {
            "name": {
                "type": "string",
            },
            "kcal": {
                "type": "integer"
            }
        },
        "required": [ "name" ]
    });
    let _ = registry.insert(&payload);
    // Nodes
    let lasagne = NodeInstance::new("Food", "Lasagne");
    let pizza = NodeInstance::new("Food", "Pizza Margherita");
    let tomatoes = NodeInstance::new("Ingredient", "Tomatoes");
    let mut tomato_kcal = NodeInstance::new("Nutrition", "tomatoes-kcal");
    tomato_kcal.insert("kcal", Value::Number(22.into()));

    // Validate nodes against their kinds
    for node in [&lasagne, &pizza, &tomatoes, &tomato_kcal] {
        registry
            .validate_instance(node)
            .expect("node should be valid");
    }

    let mut graph = StableDiGraph::<NodeInstance, String>::new();

    let i_lasagne = graph.add_node(lasagne);
    let i_pizza = graph.add_node(pizza);
    let i_tomatoes = graph.add_node(tomatoes);
    let i_tomatoes_kcal = graph.add_node(tomato_kcal.clone());
    let i_tomatoes_kcal2 = graph.add_node_s(tomato_kcal);

    graph.add_edge(i_lasagne, i_tomatoes, "has-ingredient".to_string());
    graph.add_edge(i_pizza, i_tomatoes, "has-ingredient".to_string());
    graph.add_edge_s(i_tomatoes, i_tomatoes_kcal, "has-nutrition".to_string());
    graph.add_edge_s(i_tomatoes, i_tomatoes_kcal2, "has-nutrition".to_string());

    // Print a simple overview
    println!("Nodes:");
    for node_index in graph.node_indices() {
        println!(
            "- {} ({})",
            &graph[node_index].name(),
            &graph[node_index].schema()
        );
    }

    println!("\nEdges:");
    for edge_index in graph.edge_indices() {
        let edge_endpoints = graph.edge_endpoints(edge_index).unwrap();
        println!(
            "- {} -[{}]-> {}",
            &graph[edge_endpoints.0].name(),
            &graph[edge_index],
            &graph[edge_endpoints.1].name()
        );
    }
}
