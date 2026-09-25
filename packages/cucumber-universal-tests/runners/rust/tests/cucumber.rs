use std::{fs, path::PathBuf};

use cucumber::{given, then, when, World as _};
use serde_json::Value;

#[derive(Debug, Default, cucumber::World)]
struct World {
    sigillum_dataset: Option<Value>,
    circumference: Option<Value>,
    companies: Option<Vec<Vec<Value>>>,

    traversal_fixture: Option<Value>,
    traversal_steps: Option<Vec<Value>>,
    traversal_trace: Option<String>,
    resolved_name: Option<String>,

    traversal_step: Option<Value>,
    selected_direction: Option<String>,
    selected_number: Option<i64>,
    selected_index: Option<i64>,
    traversal_terminated: bool,
    terminal_symbol: Option<String>,

    normalization_symbols: Option<Vec<String>>,
    expected_normalized_name: Option<String>,
    normalized_name: Option<String>,

    sigil_fixture: Option<Value>,
    galethog_elements: Option<Vec<Value>>,
    internal_sigil_element: Option<Value>,
    resolved_internal_value: Option<String>,
}

fn circumference_fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/circumference.structure.json")
}

fn traversal_fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/traversal.name-resolution.json")
}

fn sigil_fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/internal-sigil.galethog.json")
}

fn load_json(path: PathBuf, description: &str) -> Value {
    let content = fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("failed to read {description} fixture"));
    serde_json::from_str(&content)
        .unwrap_or_else(|_| panic!("invalid {description} fixture JSON"))
}

fn resolve_internal_sigil_element(fixture: &Value, element: &Value) -> String {
    let reference = element
        .get("circumferenceReference")
        .and_then(Value::as_i64);

    if let Some(reference) = reference {
        return fixture
            .get("upperCircleLookup")
            .and_then(|lookup| lookup.get(reference.to_string().as_str()))
            .and_then(Value::as_str)
            .unwrap_or_else(|| panic!("missing upper-circle lookup for reference {reference}"))
            .to_owned();
    }

    element
        .get("literal")
        .and_then(Value::as_str)
        .expect("unnumbered internal sigil element must contain a literal")
        .to_owned()
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

fn traversal_direction(placement: &str) -> i64 {
    match placement {
        "above" => 1,
        "below" => -1,
        _ => 0,
    }
}

fn next_traversal_index(
    current_index: i64,
    number: i64,
    placement: &str,
    circumference_size: i64,
) -> i64 {
    (current_index + traversal_direction(placement) * number)
        .rem_euclid(circumference_size)
}

fn normalize_traversal_name(symbols: &[String]) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut normalized: Vec<String> = Vec::new();

    for (index, symbol) in symbols.iter().enumerate() {
        let current = symbol.to_lowercase();
        let duplicate_vowel = index > 1
            && normalized
                .last()
                .map(|previous| previous.to_lowercase() == current)
                .unwrap_or(false)
            && current
                .chars()
                .next()
                .map(|character| vowels.contains(&character))
                .unwrap_or(false);

        if !duplicate_vowel {
            normalized.push(symbol.clone());
        }
    }

    normalized.concat()
}

fn format_traversal_trace(steps: &[Value]) -> String {
    steps
        .iter()
        .map(|step| {
            let symbol = step
                .get("symbol")
                .and_then(Value::as_str)
                .expect("traversal symbol must be a string");

            let Some(number) = step.get("number").and_then(Value::as_i64) else {
                return symbol.to_owned();
            };

            let placement = step
                .get("placement")
                .and_then(Value::as_str)
                .expect("numbered traversal step must have a placement");

            let signed_number = if placement == "above" {
                number
            } else {
                -number
            };

            format!("{symbol}[{signed_number}]")
        })
        .collect::<Vec<_>>()
        .join(" -> ")
}

#[given("the historical Sigillum data set is loaded")]
async fn load_historical_sigillum(world: &mut World) {
    world.sigillum_dataset = Some(load_json(
        circumference_fixture_path(),
        "circumference",
    ));
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

#[given(expr = "the traversal begins at {string}")]
async fn traversal_begins(world: &mut World, start: String) {
    let fixture = load_json(traversal_fixture_path(), "traversal");

    let steps = fixture
        .get("documentedTraversals")
        .and_then(|value| value.get(start.as_str()))
        .and_then(Value::as_array)
        .unwrap_or_else(|| panic!("no documented traversal begins at {start}"))
        .clone();

    world.traversal_fixture = Some(fixture);
    world.traversal_steps = Some(steps);
}

#[when("the circumference traversal is resolved")]
async fn resolve_circumference_traversal(world: &mut World) {
    let steps = world
        .traversal_steps
        .as_ref()
        .expect("traversal has not been initialized");

    world.traversal_trace = Some(format_traversal_trace(steps));

    let symbols = steps
        .iter()
        .map(|step| {
            step.get("symbol")
                .and_then(Value::as_str)
                .expect("traversal symbol must be a string")
                .to_owned()
        })
        .collect::<Vec<_>>();

    world.resolved_name = Some(normalize_traversal_name(&symbols));
}

#[then(expr = "the traversal is {string}")]
async fn assert_traversal(world: &mut World, expected: String) {
    assert_eq!(
        world
            .traversal_trace
            .as_deref()
            .expect("traversal has not been resolved"),
        expected
    );
}

#[then(expr = "the resolved name is {string}")]
async fn assert_resolved_name(world: &mut World, expected: String) {
    assert_eq!(
        world
            .resolved_name
            .as_deref()
            .expect("name has not been resolved"),
        expected
    );
}

fn load_step_example(world: &mut World, example_name: &str) {
    let fixture = load_json(traversal_fixture_path(), "traversal");
    let step = fixture
        .get("stepExamples")
        .and_then(|examples| examples.get(example_name))
        .unwrap_or_else(|| panic!("missing traversal step example {example_name}"))
        .clone();

    world.traversal_fixture = Some(fixture);
    world.traversal_step = Some(step);
}

#[given("a traversal step with a number above the current letter")]
async fn traversal_step_above(world: &mut World) {
    load_step_example(world, "above");
}

#[given("a traversal step with a number below the current letter")]
async fn traversal_step_below(world: &mut World) {
    load_step_example(world, "below");
}

#[when("the traversal step is resolved")]
async fn resolve_traversal_step(world: &mut World) {
    let step = world
        .traversal_step
        .as_ref()
        .expect("traversal step has not been initialized");

    let number = step.get("number").and_then(Value::as_i64);

    if number.is_none() {
        world.traversal_terminated = true;
        world.terminal_symbol = Some(
            step.get("symbol")
                .and_then(Value::as_str)
                .expect("terminal step must contain a symbol")
                .to_owned(),
        );
        return;
    }

    let number = number.expect("number was checked above");
    let placement = step
        .get("placement")
        .and_then(Value::as_str)
        .expect("numbered traversal step must have a placement");
    let current_index = step
        .get("currentIndex")
        .and_then(Value::as_i64)
        .expect("step currentIndex must be an integer");
    let circumference_size = world
        .traversal_fixture
        .as_ref()
        .and_then(|fixture| fixture.get("circumferenceSize"))
        .and_then(Value::as_i64)
        .expect("fixture circumferenceSize must be an integer");

    world.traversal_terminated = false;
    world.selected_direction = Some(
        if placement == "above" {
            "clockwise"
        } else {
            "counterclockwise"
        }
        .to_owned(),
    );
    world.selected_number = Some(number);
    world.selected_index = Some(next_traversal_index(
        current_index,
        number,
        placement,
        circumference_size,
    ));
}

#[then("the next chamber is selected clockwise by that number")]
async fn assert_clockwise_step(world: &mut World) {
    let step = world
        .traversal_step
        .as_ref()
        .expect("traversal step has not been initialized");

    assert_eq!(world.selected_direction.as_deref(), Some("clockwise"));
    assert_eq!(
        world.selected_number,
        step.get("number").and_then(Value::as_i64)
    );
    assert_eq!(
        world.selected_index,
        step.get("expectedIndex").and_then(Value::as_i64)
    );
}

#[then("the next chamber is selected counterclockwise by that number")]
async fn assert_counterclockwise_step(world: &mut World) {
    let step = world
        .traversal_step
        .as_ref()
        .expect("traversal step has not been initialized");

    assert_eq!(
        world.selected_direction.as_deref(),
        Some("counterclockwise")
    );
    assert_eq!(
        world.selected_number,
        step.get("number").and_then(Value::as_i64)
    );
    assert_eq!(
        world.selected_index,
        step.get("expectedIndex").and_then(Value::as_i64)
    );
}

#[given("a traversal reaches a letter without a number")]
async fn traversal_reaches_unnumbered_letter(world: &mut World) {
    load_step_example(world, "termination");
}

#[then("the name terminates at that letter")]
async fn assert_name_termination(world: &mut World) {
    let expected_symbol = world
        .traversal_step
        .as_ref()
        .and_then(|step| step.get("symbol"))
        .and_then(Value::as_str)
        .expect("terminal fixture must contain a symbol");

    assert!(world.traversal_terminated);
    assert_eq!(world.terminal_symbol.as_deref(), Some(expected_symbol));
}

#[given("a resolved traversal contains consecutive duplicate vowels after the first letter")]
async fn duplicate_vowel_traversal(world: &mut World) {
    let fixture = load_json(traversal_fixture_path(), "traversal");
    let example = fixture
        .get("normalizationExamples")
        .and_then(|examples| examples.get("duplicateVowelsAfterBeginning"))
        .expect("missing duplicate-vowel normalization example");

    world.normalization_symbols = Some(
        example
            .get("symbols")
            .and_then(Value::as_array)
            .expect("normalization symbols must be an array")
            .iter()
            .map(|symbol| {
                symbol
                    .as_str()
                    .expect("normalization symbol must be a string")
                    .to_owned()
            })
            .collect(),
    );

    world.expected_normalized_name = Some(
        example
            .get("expected")
            .and_then(Value::as_str)
            .expect("normalization expected value must be a string")
            .to_owned(),
    );
}

#[when("the name is normalized")]
async fn normalize_resolved_name(world: &mut World) {
    let symbols = world
        .normalization_symbols
        .as_ref()
        .expect("normalization symbols have not been initialized");

    world.normalized_name = Some(normalize_traversal_name(symbols));
}

#[then("the consecutive duplicate vowels are represented once")]
async fn assert_duplicate_vowels_collapsed(world: &mut World) {
    assert_eq!(
        world.normalized_name,
        world.expected_normalized_name,
        "duplicate vowel normalization did not match the shared fixture"
    );
}

#[given("the historical Galethog sigil data is loaded")]
async fn load_galethog_sigil(world: &mut World) {
    let fixture = load_json(sigil_fixture_path(), "internal sigil");

    let elements = fixture
        .get("galethog")
        .and_then(|galethog| galethog.get("internalElements"))
        .and_then(Value::as_array)
        .expect("Galethog fixture must contain internalElements")
        .clone();

    world.sigil_fixture = Some(fixture);
    world.galethog_elements = Some(elements);
}

#[when("its internal references are resolved against the circumference")]
async fn resolve_galethog_references(world: &mut World) {
    let fixture = world
        .sigil_fixture
        .as_ref()
        .expect("historical Galethog sigil data is not loaded");
    let elements = world
        .galethog_elements
        .as_ref()
        .expect("Galethog internal elements are not loaded");

    let resolved = elements
        .iter()
        .map(|element| resolve_internal_sigil_element(fixture, element))
        .collect::<String>();

    world.resolved_name = Some(resolved);
}

#[given("an internal Sigillum element contains a circumference reference")]
async fn internal_element_with_reference(world: &mut World) {
    let fixture = load_json(sigil_fixture_path(), "internal sigil");

    let example = fixture
        .get("lookupExample")
        .expect("internal sigil fixture must contain lookupExample")
        .clone();

    world.sigil_fixture = Some(fixture);
    world.internal_sigil_element = Some(example);
}

#[when("the element is resolved")]
async fn resolve_internal_element(world: &mut World) {
    let resolved = {
        let fixture = world
            .sigil_fixture
            .as_ref()
            .expect("internal sigil fixture is not loaded");
        let element = world
            .internal_sigil_element
            .as_ref()
            .expect("internal sigil element is not loaded");

        resolve_internal_sigil_element(fixture, element)
    };

    world.resolved_internal_value = Some(resolved);
}

#[then("its value is obtained from the referenced circumference position")]
async fn assert_internal_lookup(world: &mut World) {
    let expected = world
        .internal_sigil_element
        .as_ref()
        .and_then(|element| element.get("expectedValue"))
        .and_then(Value::as_str)
        .expect("lookup example must contain expectedValue");

    assert_eq!(
        world.resolved_internal_value.as_deref(),
        Some(expected),
        "internal sigil value did not match the referenced circumference value"
    );
}

#[tokio::main]
async fn main() {
    World::run("../../features").await;
}
