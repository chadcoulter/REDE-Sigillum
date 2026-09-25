import json
from pathlib import Path

from behave import given, then, when


FIXTURE_PATH = (
    Path(__file__).resolve().parents[2]
    / "fixtures"
    / "traversal.name-resolution.json"
)

VOWELS = {"a", "e", "i", "o", "u"}


def load_fixture():
    with FIXTURE_PATH.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def direction_for(placement):
    if placement == "above":
        return 1
    if placement == "below":
        return -1
    return 0


def next_index(current_index, number, placement, circumference_size):
    return (
        current_index
        + direction_for(placement) * number
    ) % circumference_size


def normalize_name(symbols):
    normalized = []

    for index, symbol in enumerate(symbols):
        if (
            index > 1
            and normalized
            and symbol.lower() in VOWELS
            and normalized[-1].lower() == symbol.lower()
        ):
            continue
        normalized.append(symbol)

    return "".join(normalized)


def format_trace(steps):
    parts = []

    for step in steps:
        symbol = step["symbol"]
        number = step["number"]
        placement = step["placement"]

        if number is None:
            parts.append(symbol)
            continue

        signed_number = number if placement == "above" else -number
        parts.append(f"{symbol}[{signed_number}]")

    return " -> ".join(parts)


@given('the traversal begins at "{start}"')
def traversal_begins(context, start):
    fixture = load_fixture()
    context.traversal_fixture = fixture
    context.traversal_steps = fixture["documentedTraversals"][start]


@when("the circumference traversal is resolved")
def resolve_circumference_traversal(context):
    steps = context.traversal_steps
    context.traversal_trace = format_trace(steps)
    context.resolved_name = normalize_name([step["symbol"] for step in steps])


@then('the traversal is "{expected}"')
def assert_traversal(context, expected):
    assert context.traversal_trace == expected


@then('the resolved name is "{expected}"')
def assert_resolved_name(context, expected):
    assert context.resolved_name == expected


@given("a traversal step with a number above the current letter")
def traversal_step_above(context):
    fixture = load_fixture()
    context.traversal_fixture = fixture
    context.traversal_step = fixture["stepExamples"]["above"]


@given("a traversal step with a number below the current letter")
def traversal_step_below(context):
    fixture = load_fixture()
    context.traversal_fixture = fixture
    context.traversal_step = fixture["stepExamples"]["below"]


@when("the traversal step is resolved")
def resolve_traversal_step(context):
    step = context.traversal_step

    if step["number"] is None:
        context.traversal_terminated = True
        context.terminal_symbol = step["symbol"]
        return

    context.traversal_terminated = False
    context.selected_direction = (
        "clockwise" if step["placement"] == "above" else "counterclockwise"
    )
    context.selected_number = step["number"]
    context.selected_index = next_index(
        step["currentIndex"],
        step["number"],
        step["placement"],
        context.traversal_fixture["circumferenceSize"],
    )


@then("the next chamber is selected clockwise by that number")
def assert_clockwise_step(context):
    step = context.traversal_step
    assert context.selected_direction == "clockwise"
    assert context.selected_number == step["number"]
    assert context.selected_index == step["expectedIndex"]


@then("the next chamber is selected counterclockwise by that number")
def assert_counterclockwise_step(context):
    step = context.traversal_step
    assert context.selected_direction == "counterclockwise"
    assert context.selected_number == step["number"]
    assert context.selected_index == step["expectedIndex"]


@given("a traversal reaches a letter without a number")
def traversal_reaches_unnumbered_letter(context):
    fixture = load_fixture()
    context.traversal_fixture = fixture
    context.traversal_step = fixture["stepExamples"]["termination"]


@then("the name terminates at that letter")
def assert_name_termination(context):
    assert context.traversal_terminated is True
    assert context.terminal_symbol == context.traversal_step["symbol"]


@given("a resolved traversal contains consecutive duplicate vowels after the first letter")
def duplicate_vowel_traversal(context):
    fixture = load_fixture()
    example = fixture["normalizationExamples"]["duplicateVowelsAfterBeginning"]
    context.normalization_symbols = example["symbols"]
    context.expected_normalized_name = example["expected"]


@when("the name is normalized")
def normalize_resolved_name(context):
    context.normalized_name = normalize_name(context.normalization_symbols)


@then("the consecutive duplicate vowels are represented once")
def assert_duplicate_vowels_collapsed(context):
    assert context.normalized_name == context.expected_normalized_name
