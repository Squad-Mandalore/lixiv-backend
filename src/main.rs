mod model;

use crate::model::{Edge, JsonType, KindDefinition, KindRegistry, NodeInstance};
use serde_json::Value;
use std::collections::HashMap;
use petgraph::prelude::*;

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
    nutrition_fields.insert("kcal".to_string(), JsonType::Number);
    registry
        .register_kind(KindDefinition {
            name: "Nutrition".to_string(),
            parent: Some("Ingredient".to_string()),
            fields: nutrition_fields,
        })
        .expect("register Nutrition kind");

    // Nodes
    let lasagne = NodeInstance {
        id: "lasagne".to_string(),
        kind: "Food".to_string(),
        data: {
            let mut d = HashMap::new();
            d.insert("name".to_string(), Value::String("Lasagne".to_string()));
            d
        },
    };

    let pizza = NodeInstance {
        id: "pizza-margherita".to_string(),
        kind: "Food".to_string(),
        data: {
            let mut d = HashMap::new();
            d.insert(
                "name".to_string(),
                Value::String("Pizza Margherita".to_string()),
            );
            d
        },
    };

    let tomatoes = NodeInstance {
        id: "tomatoes".to_string(),
        kind: "Ingredient".to_string(),
        data: {
            let mut d = HashMap::new();
            d.insert("name".to_string(), Value::String("Tomatoes".to_string()));
            d
        },
    };

    let tomato_kcal = NodeInstance {
        id: "tomato-kcal".to_string(),
        kind: "Nutrition".to_string(),
        data: {
            let mut d = HashMap::new();
            d.insert(
                "kcal".to_string(),
                Value::Number(serde_json::Number::from(22)),
            );
            d
        },
    };

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
    let i_tomatoes_kcal = graph.add_node(tomato_kcal);

    graph.add_edge(i_lasagne, i_tomatoes, "has-ingredient".to_string());
    graph.add_edge(i_pizza, i_tomatoes, "has-ingredient".to_string());
    graph.add_edge(i_tomatoes, i_tomatoes_kcal, "has-nutrition".to_string());

    // Print a simple overview
    println!("Nodes:");
    for node in graph.clone().into_nodes_edges_iters().0 {
        println!("- {} ({})", &graph[node.index].id, &graph[node.index].kind);
    }

    println!("\nEdges:");
    for edge in graph.clone().into_nodes_edges_iters().1 {
        println!("- {} -[{}]-> {}", &graph[edge.source].id, edge.weight, &graph[edge.target].id);
    }
}
