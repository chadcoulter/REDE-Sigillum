import json
from pathlib import Path

from behave import given, then, when


VARIANT_FIXTURE_PATH = (
    Path(__file__).resolve().parents[2]
    / "fixtures"
    / "historical.dataset-variants.json"
)


def load_variant_fixture():
    with VARIANT_FIXTURE_PATH.open("r", encoding="utf-8") as handle:
        return json.load(handle)


@given("multiple historical Sigillum data variants are available")
def load_historical_variants(context):
    context.variant_fixture = load_variant_fixture()
    context.historical_variants = context.variant_fixture["variants"]


@when("a variant is selected")
def select_historical_variant(context):
    variant_id = context.variant_fixture["defaultVariant"]
    context.selected_variant_id = variant_id
    context.selected_variant = context.historical_variants[variant_id]


@then("the selected variant identifies its source")
def assert_selected_variant_source(context):
    source = context.selected_variant.get("source")

    assert isinstance(source, dict)
    assert source.get("id")
    assert source.get("url")


@when("I inspect circumference division {division:d}")
def inspect_circumference_division(context, division):
    division_key = str(division)

    context.inspected_division = division
    context.variant_division_values = {
        variant_id: {
            "value": variant["circumferenceDivisions"][division_key],
            "source": variant["source"],
        }
        for variant_id, variant in context.historical_variants.items()
    }


@then("each differing value remains associated with its historical variant")
def assert_variant_value_associations(context):
    assert len(context.variant_division_values) >= 2

    for variant_id, reading in context.variant_division_values.items():
        assert variant_id
        assert reading["value"]
        assert reading["source"].get("id")
        assert reading["source"].get("url")


@then("the variants remain distinguishable")
def assert_variants_distinguishable(context):
    readings = context.variant_division_values

    assert len(set(readings.keys())) == len(readings)
    assert len({entry["value"] for entry in readings.values()}) > 1
