use std::{collections::VecDeque, fs, path::PathBuf};

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

    correspondence_fixture: Option<Value>,
    correspondence_path: Option<Vec<String>>,
    correspondence_result_type: Option<String>,
    correspondence_is_direct_name_transform: bool,

    provenance_fixture: Option<Value>,
    transform_trace: Option<Vec<Value>>,
    provenance_steps: Option<Vec<Value>>,
    result_lineage: Option<Vec<String>>,

    variant_fixture: Option<Value>,
    selected_variant_id: Option<String>,
    selected_variant: Option<Value>,
    variant_division_values: Option<Vec<(String, String, String, String)>>,
    variant_source_metadata: Option<Vec<(String, Value)>>,
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

fn correspondence_fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/correspondence.aaoth-aaron.json")
}

fn provenance_fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/provenance.retention.json")
}

fn variant_fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/historical.dataset-variants.json")
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

fn resolve_correspondence_path(
    fixture: &Value,
    source: &str,
    target: &str,
) -> Vec<String> {
    let edges = fixture
        .get("edges")
        .and_then(Value::as_array)
        .expect("correspondence fixture must contain edges");

    let mut queue = VecDeque::from([vec![source.to_owned()]]);
    let mut visited = vec![source.to_owned()];

    while let Some(path) = queue.pop_front() {
        let current = path
            .last()
            .expect("correspondence path cannot be empty");

        if current == target {
            return path;
        }

        for edge in edges {
            let from = edge
                .get("from")
                .and_then(Value::as_str)
                .expect("correspondence edge must contain from");
            let to = edge
                .get("to")
                .and_then(Value::as_str)
                .expect("correspondence edge must contain to");

            if from != current || visited.iter().any(|node| node == to) {
                continue;
            }

            visited.push(to.to_owned());

            let mut next_path = path.clone();
            next_path.push(to.to_owned());
            queue.push_back(next_path);
        }
    }

    panic!("no correspondence path from {source} to {target}");
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

#[given("the documented correspondence data is loaded")]
async fn load_documented_correspondence_data(world: &mut World) {
    world.correspondence_fixture = Some(load_json(
        correspondence_fixture_path(),
        "correspondence",
    ));
}

#[when(expr = "I resolve the correspondence path from {string} to {string}")]
async fn resolve_documented_correspondence_path(
    world: &mut World,
    source: String,
    target: String,
) {
    let fixture = world
        .correspondence_fixture
        .as_ref()
        .expect("documented correspondence data is not loaded");

    world.correspondence_path = Some(resolve_correspondence_path(
        fixture,
        source.as_str(),
        target.as_str(),
    ));

    let expectation = fixture
        .get("expectedPaths")
        .and_then(Value::as_array)
        .and_then(|paths| {
            paths.iter().find(|path| {
                path.get("from").and_then(Value::as_str) == Some(source.as_str())
                    && path.get("to").and_then(Value::as_str) == Some(target.as_str())
            })
        })
        .expect("correspondence fixture is missing the expected path metadata");

    world.correspondence_result_type = Some(
        expectation
            .get("resultType")
            .and_then(Value::as_str)
            .expect("expected path must contain resultType")
            .to_owned(),
    );

    world.correspondence_is_direct_name_transform = expectation
        .get("directNameTransform")
        .and_then(Value::as_bool)
        .expect("expected path must contain directNameTransform");
}

#[then("the path is:")]
async fn assert_correspondence_path(
    world: &mut World,
    step: &cucumber::gherkin::Step,
) {
    let table = step
        .table
        .as_ref()
        .expect("correspondence path assertion requires a data table");

    let expected = table
        .rows
        .iter()
        .skip(1)
        .map(|row| {
            row.first()
                .expect("correspondence path table row must contain a node")
                .clone()
        })
        .collect::<Vec<_>>();

    assert_eq!(
        world
            .correspondence_path
            .as_ref()
            .expect("correspondence path has not been resolved"),
        &expected
    );
}

#[then("the result is identified as a correspondence path")]
async fn assert_correspondence_result_type(world: &mut World) {
    assert_eq!(
        world.correspondence_result_type.as_deref(),
        Some("correspondence_path")
    );
}

#[then("the result is not identified as a direct name transform")]
async fn assert_not_direct_name_transform(world: &mut World) {
    assert!(
        !world.correspondence_is_direct_name_transform,
        "correspondence path was incorrectly identified as a direct name transform"
    );
}

#[given("a Sigillum transform is accepted")]
async fn accepted_sigillum_transform(world: &mut World) {
    let fixture = load_json(provenance_fixture_path(), "provenance");

    let steps = fixture
        .get("acceptedTransform")
        .and_then(|transform| transform.get("steps"))
        .and_then(Value::as_array)
        .expect("acceptedTransform must contain steps")
        .clone();

    world.provenance_fixture = Some(fixture);
    world.transform_trace = Some(steps);
}

#[when("I inspect its transform trace")]
async fn inspect_transform_trace(world: &mut World) {
    assert!(
        world.transform_trace.is_some(),
        "accepted transform trace has not been loaded"
    );
}

#[then("every step identifies the rule that produced it")]
async fn assert_rule_retention(world: &mut World) {
    let trace = world
        .transform_trace
        .as_ref()
        .expect("accepted transform trace has not been loaded");

    for step in trace {
        let rule = step
            .get("rule")
            .and_then(Value::as_str)
            .expect("transform step is missing its rule");

        assert!(
            !rule.trim().is_empty(),
            "transform step rule must not be empty"
        );
    }
}

#[then("every step identifies the source data used by that rule")]
async fn assert_source_retention(world: &mut World) {
    let fixture = world
        .provenance_fixture
        .as_ref()
        .expect("provenance fixture has not been loaded");

    let source_registry = fixture
        .get("sourceRegistry")
        .and_then(Value::as_object)
        .expect("provenance fixture must contain sourceRegistry");

    let trace = world
        .transform_trace
        .as_ref()
        .expect("accepted transform trace has not been loaded");

    for step in trace {
        let sources = step
            .get("sources")
            .and_then(Value::as_array)
            .expect("transform step is missing source references");

        assert!(
            !sources.is_empty(),
            "transform step must retain at least one source"
        );

        for source_id in sources {
            let source_id = source_id
                .as_str()
                .expect("source reference must be a string");

            let source = source_registry
                .get(source_id)
                .unwrap_or_else(|| panic!("unknown source {source_id}"));

            let url = source
                .get("url")
                .and_then(Value::as_str)
                .expect("source entry must contain a URL");

            assert!(
                !url.trim().is_empty(),
                "source URL must not be empty"
            );
        }
    }
}

#[given("a result is derived through multiple transform steps")]
async fn result_with_multiple_transform_steps(world: &mut World) {
    let fixture = load_json(provenance_fixture_path(), "provenance");

    let steps = fixture
        .get("acceptedTransform")
        .and_then(|transform| transform.get("steps"))
        .and_then(Value::as_array)
        .expect("acceptedTransform must contain steps")
        .clone();

    let lineage = fixture
        .get("derivedResult")
        .and_then(|result| result.get("lineage"))
        .and_then(Value::as_array)
        .expect("derivedResult must contain lineage")
        .iter()
        .map(|value| {
            value
                .as_str()
                .expect("lineage entry must be a string")
                .to_owned()
        })
        .collect::<Vec<_>>();

    world.provenance_fixture = Some(fixture);
    world.provenance_steps = Some(steps);
    world.result_lineage = Some(lineage);
}

#[when("I inspect the result provenance")]
async fn inspect_result_provenance(world: &mut World) {
    assert!(
        world.result_lineage.is_some(),
        "derived result provenance has not been loaded"
    );
}

#[then("the complete ordered transform lineage is available")]
async fn assert_complete_ordered_lineage(world: &mut World) {
    let mut steps = world
        .provenance_steps
        .as_ref()
        .expect("provenance steps have not been loaded")
        .clone();

    steps.sort_by_key(|step| {
        step.get("order")
            .and_then(Value::as_i64)
            .expect("provenance step order must be an integer")
    });

    let expected = steps
        .iter()
        .map(|step| {
            step.get("id")
                .and_then(Value::as_str)
                .expect("provenance step must contain an id")
                .to_owned()
        })
        .collect::<Vec<_>>();

    assert_eq!(
        world
            .result_lineage
            .as_ref()
            .expect("result lineage has not been loaded"),
        &expected,
        "result lineage does not preserve transform order"
    );
}

#[then("no earlier transform step has been erased")]
async fn assert_no_lineage_erasure(world: &mut World) {
    let steps = world
        .provenance_steps
        .as_ref()
        .expect("provenance steps have not been loaded");

    let original_ids = steps
        .iter()
        .map(|step| {
            step.get("id")
                .and_then(Value::as_str)
                .expect("provenance step must contain an id")
                .to_owned()
        })
        .collect::<Vec<_>>();

    let lineage = world
        .result_lineage
        .as_ref()
        .expect("result lineage has not been loaded");

    assert_eq!(
        lineage.len(),
        original_ids.len(),
        "one or more transform steps were erased from lineage"
    );
    assert_eq!(
        lineage,
        &original_ids,
        "transform lineage was reordered or altered"
    );

    let mut unique = lineage.clone();
    unique.sort();
    unique.dedup();

    assert_eq!(
        unique.len(),
        original_ids.len(),
        "transform lineage contains duplicated step identifiers"
    );
}

#[given("multiple historical Sigillum data variants are available")]
async fn load_historical_variants(world: &mut World) {
    world.variant_fixture = Some(load_json(
        variant_fixture_path(),
        "historical dataset variants",
    ));
}

#[when("a variant is selected")]
async fn select_historical_variant(world: &mut World) {
    let fixture = world
        .variant_fixture
        .as_ref()
        .expect("historical dataset variants are not loaded");

    let variant_id = fixture
        .get("defaultVariant")
        .and_then(Value::as_str)
        .expect("variant fixture must contain defaultVariant");

    let variant = fixture
        .get("variants")
        .and_then(|variants| variants.get(variant_id))
        .unwrap_or_else(|| panic!("default variant {variant_id} is not defined"))
        .clone();

    world.selected_variant_id = Some(variant_id.to_owned());
    world.selected_variant = Some(variant);
}

#[then("the selected variant identifies its source")]
async fn assert_selected_variant_source(world: &mut World) {
    let source = world
        .selected_variant
        .as_ref()
        .expect("no historical variant has been selected")
        .get("source")
        .expect("selected variant must contain source");

    let source_id = source
        .get("id")
        .and_then(Value::as_str)
        .expect("variant source must contain id");
    let source_url = source
        .get("url")
        .and_then(Value::as_str)
        .expect("variant source must contain url");

    assert!(!source_id.trim().is_empty(), "variant source id must not be empty");
    assert!(!source_url.trim().is_empty(), "variant source url must not be empty");
}

#[when("I inspect the source metadata for every historical variant")]
async fn inspect_all_variant_source_metadata(world: &mut World) {
    let fixture = world
        .variant_fixture
        .as_ref()
        .expect("historical dataset variants are not loaded");

    let variants = fixture
        .get("variants")
        .and_then(Value::as_object)
        .expect("variant fixture must contain variants");

    world.variant_source_metadata = Some(
        variants
            .iter()
            .map(|(variant_id, variant)| {
                let source = variant
                    .get("source")
                    .expect("historical variant must contain source")
                    .clone();

                (variant_id.clone(), source)
            })
            .collect(),
    );
}

#[then("every historical variant retains its source URL")]
async fn assert_all_variant_source_urls(world: &mut World) {
    let metadata = world
        .variant_source_metadata
        .as_ref()
        .expect("historical variant source metadata has not been inspected");

    assert!(
        !metadata.is_empty(),
        "no historical variant source metadata was loaded"
    );

    for (variant_id, source) in metadata {
        assert!(
            !variant_id.trim().is_empty(),
            "historical variant id must not be empty"
        );

        let url = source
            .get("url")
            .and_then(Value::as_str)
            .unwrap_or_else(|| panic!("{variant_id} is missing its source URL"));

        assert!(
            !url.trim().is_empty(),
            "{variant_id} source URL must not be empty"
        );
    }
}

#[then("every historical variant retains citation metadata")]
async fn assert_all_variant_citation_metadata(world: &mut World) {
    let metadata = world
        .variant_source_metadata
        .as_ref()
        .expect("historical variant source metadata has not been inspected");

    for (variant_id, source) in metadata {
        let citation = source
            .get("citation")
            .and_then(Value::as_object)
            .unwrap_or_else(|| panic!("{variant_id} is missing citation metadata"));

        for field in ["author", "title", "container"] {
            let value = citation
                .get(field)
                .and_then(Value::as_str)
                .unwrap_or_else(|| panic!("{variant_id} citation is missing {field}"));

            assert!(
                !value.trim().is_empty(),
                "{variant_id} citation field {field} must not be empty"
            );
        }
    }
}

#[when(expr = "I inspect circumference division {int}")]
async fn inspect_variant_division(world: &mut World, division: i64) {
    let fixture = world
        .variant_fixture
        .as_ref()
        .expect("historical dataset variants are not loaded");

    let variants = fixture
        .get("variants")
        .and_then(Value::as_object)
        .expect("variant fixture must contain variants");

    let division_key = division.to_string();
    let mut readings = Vec::new();

    for (variant_id, variant) in variants {
        let value = variant
            .get("circumferenceDivisions")
            .and_then(|divisions| divisions.get(division_key.as_str()))
            .and_then(Value::as_str)
            .unwrap_or_else(|| {
                panic!(
                    "variant {variant_id} does not define circumference division {division}"
                )
            });

        let source = variant
            .get("source")
            .expect("historical variant must contain source");

        let source_id = source
            .get("id")
            .and_then(Value::as_str)
            .expect("variant source must contain id");

        let source_url = source
            .get("url")
            .and_then(Value::as_str)
            .expect("variant source must contain url");

        readings.push((
            variant_id.clone(),
            value.to_owned(),
            source_id.to_owned(),
            source_url.to_owned(),
        ));
    }

    world.variant_division_values = Some(readings);
}

#[then("each differing value remains associated with its historical variant")]
async fn assert_variant_value_associations(world: &mut World) {
    let readings = world
        .variant_division_values
        .as_ref()
        .expect("no circumference variant division has been inspected");

    assert!(
        readings.len() >= 2,
        "at least two historical variants are required"
    );

    for (variant_id, value, source_id, source_url) in readings {
        assert!(!variant_id.trim().is_empty(), "variant id must not be empty");
        assert!(!value.trim().is_empty(), "variant value must not be empty");
        assert!(!source_id.trim().is_empty(), "source id must not be empty");
        assert!(!source_url.trim().is_empty(), "source url must not be empty");
    }
}

#[then("the variants remain distinguishable")]
async fn assert_variants_distinguishable(world: &mut World) {
    let readings = world
        .variant_division_values
        .as_ref()
        .expect("no circumference variant division has been inspected");

    let mut variant_ids = readings
        .iter()
        .map(|reading| reading.0.clone())
        .collect::<Vec<_>>();
    let original_variant_count = variant_ids.len();
    variant_ids.sort();
    variant_ids.dedup();

    assert_eq!(
        variant_ids.len(),
        original_variant_count,
        "historical variant identifiers are not unique"
    );

    let mut values = readings
        .iter()
        .map(|reading| reading.1.clone())
        .collect::<Vec<_>>();
    values.sort();
    values.dedup();

    assert!(
        values.len() > 1,
        "historical variant readings are not distinguishable"
    );
}

fn assert_variant_citation_field(world: &World, field: &str) {
    let metadata = world
        .variant_source_metadata
        .as_ref()
        .expect("historical variant source metadata has not been inspected");

    for (variant_id, source) in metadata {
        let citation = source
            .get("citation")
            .and_then(Value::as_object)
            .unwrap_or_else(|| panic!("{variant_id} is missing citation metadata"));

        let value = citation
            .get(field)
            .and_then(Value::as_str)
            .unwrap_or_else(|| panic!("{variant_id} citation is missing {field}"));

        assert!(
            !value.trim().is_empty(),
            "{variant_id} citation field {field} must not be empty"
        );
    }
}

#[then("every historical variant retains citation author")]
async fn assert_all_variant_citation_authors(world: &mut World) {
    assert_variant_citation_field(world, "author");
}

#[then("every historical variant retains citation title")]
async fn assert_all_variant_citation_titles(world: &mut World) {
    assert_variant_citation_field(world, "title");
}

#[then("every historical variant retains citation container")]
async fn assert_all_variant_citation_containers(world: &mut World) {
    assert_variant_citation_field(world, "container");
}

#[tokio::main]
async fn main() {
    World::run("../../features").await;
}
