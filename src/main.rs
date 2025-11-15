mod model;

use crate::model::{AddDedup, JsonType, KindDefinition, KindRegistry, NodeInstance};
use petgraph::prelude::*;
use serde_json::Value;
use std::collections::HashMap;

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

    let mut registry = KindRegistry::default();

    // Define kinds
    let mut food_fields = HashMap::new();
    food_fields.insert("name".to_string(), JsonType::String);
    registry
        .register_kind(KindDefinition {
            name: "Food".to_string(),
            parent: None,
            fields: food_fields,
        })
        .expect("register Food kind");

    let mut ingredient_fields = HashMap::new();
    ingredient_fields.insert("name".to_string(), JsonType::String);
    registry
        .register_kind(KindDefinition {
            name: "Ingredient".to_string(),
            parent: Some("Food".to_string()),
            fields: ingredient_fields,
        })
        .expect("register Ingredient kind");

    let mut nutrition_fields = HashMap::new();
    nutrition_fields.insert("name".to_string(), JsonType::String);
    nutrition_fields.insert("kcal".to_string(), JsonType::Number);
    registry
        .register_kind(KindDefinition {
            name: "Nutrition".to_string(),
            parent: Some("Ingredient".to_string()),
            fields: nutrition_fields,
        })
        .expect("register Nutrition kind");

    // Nodes
    let lasagne = NodeInstance::new("Food".into(), "Lasagne".into());
    let pizza = NodeInstance::new("Food".into(), "Pizza Margherita".into());
    let tomatoes = NodeInstance::new("Ingredient".into(), "Tomatoes".into());
    let mut tomato_kcal = NodeInstance::new("Nutrition".into(), "tomatoes-kcal".into());
    tomato_kcal
        .data
        .insert("kcal".into(), Value::Number(22.into()));

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
            &graph[node_index].get_name(),
            &graph[node_index].kind
        );
    }

    println!("\nEdges:");
    for edge_index in graph.edge_indices() {
        let edge_endpoints = graph.edge_endpoints(edge_index).unwrap();
        println!(
            "- {} -[{}]-> {}",
            &graph[edge_endpoints.0].get_name(),
            &graph[edge_index],
            &graph[edge_endpoints.1].get_name()
        );
    }
}
