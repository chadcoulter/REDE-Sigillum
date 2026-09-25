use std::{fs, path::PathBuf};

use cucumber::{given, then, when, World as _};
use serde_json::Value;

#[derive(Debug, Default, cucumber::World)]
struct World {
    sigillum_dataset: Option<Value>,
    circumference: Option<Value>,
    companies: Option<Vec<Vec<Value>>>,
}

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/circumference.structure.json")
}

fn chambers(world: &World) -> &Vec<Value> {
    world
        .circumference
        .as_ref()
        .expect("circumference has not been inspected")
        .get("chambers")
        .and_then(Value::as_array)
        .expect("circumference chambers must be an array")
}

#[given("the historical Sigillum data set is loaded")]
async fn load_historical_sigillum(world: &mut World) {
    let content =
        fs::read_to_string(fixture_path()).expect("failed to read circumference fixture");
    world.sigillum_dataset =
        Some(serde_json::from_str(&content).expect("invalid circumference fixture JSON"));
}

#[when("I inspect the circumference")]
async fn inspect_circumference(world: &mut World) {
    let circumference = world
        .sigillum_dataset
        .as_ref()
        .expect("historical Sigillum data set is not loaded")
        .get("circumference")
        .expect("fixture does not contain circumference")
        .clone();

    world.circumference = Some(circumference);
}

#[then(expr = "it contains {int} chambers")]
async fn assert_chamber_count(world: &mut World, expected: usize) {
    assert_eq!(chambers(world).len(), expected);
}

#[then(expr = "each chamber spans {int} degrees")]
async fn assert_chamber_span(world: &mut World, expected: i64) {
    for chamber in chambers(world) {
        let start = chamber
            .get("startDegree")
            .and_then(Value::as_i64)
            .expect("startDegree must be an integer");
        let end = chamber
            .get("endDegree")
            .and_then(Value::as_i64)
            .expect("endDegree must be an integer");
        assert_eq!(end - start, expected);
    }
}

#[then(expr = "the circumference spans {int} degrees")]
async fn assert_total_span(world: &mut World, expected: i64) {
    let actual: i64 = chambers(world)
        .iter()
        .map(|chamber| {
            let start = chamber
                .get("startDegree")
                .and_then(Value::as_i64)
                .expect("startDegree must be an integer");
            let end = chamber
                .get("endDegree")
                .and_then(Value::as_i64)
                .expect("endDegree must be an integer");
            end - start
        })
        .sum();

    assert_eq!(actual, expected);
}

#[when("I group the circumference into companies")]
async fn group_circumference(world: &mut World) {
    let chamber_values = world
        .sigillum_dataset
        .as_ref()
        .expect("historical Sigillum data set is not loaded")
        .get("circumference")
        .and_then(|value| value.get("chambers"))
        .and_then(Value::as_array)
        .expect("fixture does not contain circumference chambers");

    let mut companies = vec![Vec::new(); 8];

    for chamber in chamber_values {
        let company = chamber
            .get("company")
            .and_then(Value::as_u64)
            .expect("company must be an integer") as usize;

        assert!((1..=8).contains(&company), "company must be between 1 and 8");
        companies[company - 1].push(chamber.clone());
    }

    world.companies = Some(companies);
}

#[then(expr = "there are {int} companies")]
async fn assert_company_count(world: &mut World, expected: usize) {
    let companies = world
        .companies
        .as_ref()
        .expect("circumference has not been grouped into companies");
    assert_eq!(companies.len(), expected);
}

#[then(expr = "each company contains {int} chambers")]
async fn assert_company_size(world: &mut World, expected: usize) {
    let companies = world
        .companies
        .as_ref()
        .expect("circumference has not been grouped into companies");

    assert!(
        companies.iter().all(|company| company.len() == expected),
        "not every company contains {expected} chambers"
    );
}

#[tokio::main]
async fn main() {
    World::run("../../features").await;
}
