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


@when("I inspect the source metadata for every historical variant")
def inspect_all_variant_source_metadata(context):
    context.variant_source_metadata = {
        variant_id: variant["source"]
        for variant_id, variant in context.historical_variants.items()
    }


@then("every historical variant retains its source URL")
def assert_all_variant_source_urls(context):
    assert context.variant_source_metadata

    for variant_id, source in context.variant_source_metadata.items():
        assert variant_id
        assert isinstance(source.get("url"), str)
        assert source["url"].strip()


@then("every historical variant retains citation metadata")
def assert_all_variant_citation_metadata(context):
    required_fields = ("author", "title", "container")

    for variant_id, source in context.variant_source_metadata.items():
        citation = source.get("citation")
        assert isinstance(citation, dict), (
            f"{variant_id} is missing citation metadata"
        )

        for field in required_fields:
            value = citation.get(field)
            assert isinstance(value, str), (
                f"{variant_id} citation is missing {field}"
            )
            assert value.strip(), (
                f"{variant_id} citation field {field} is empty"
            )


def assert_variant_citation_field(context, field):
    for variant_id, source in context.variant_source_metadata.items():
        citation = source.get("citation")
        assert isinstance(citation, dict), (
            f"{variant_id} is missing citation metadata"
        )

        value = citation.get(field)
        assert isinstance(value, str), (
            f"{variant_id} citation is missing {field}"
        )
        assert value.strip(), (
            f"{variant_id} citation field {field} is empty"
        )


@then("every historical variant retains citation author")
def assert_all_variant_citation_authors(context):
    assert_variant_citation_field(context, "author")


@then("every historical variant retains citation title")
def assert_all_variant_citation_titles(context):
    assert_variant_citation_field(context, "title")


@then("every historical variant retains citation container")
def assert_all_variant_citation_containers(context):
    assert_variant_citation_field(context, "container")
