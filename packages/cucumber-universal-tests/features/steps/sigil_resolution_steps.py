import json
from pathlib import Path

from behave import given, then, when


SIGIL_FIXTURE_PATH = (
    Path(__file__).resolve().parents[2]
    / "fixtures"
    / "internal-sigil.galethog.json"
)


def load_sigil_fixture():
    with SIGIL_FIXTURE_PATH.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def resolve_internal_element(fixture, element):
    reference = element.get("circumferenceReference")

    if reference is None:
        return element["literal"]

    return fixture["upperCircleLookup"][str(reference)]


@given("the historical Galethog sigil data is loaded")
def load_galethog_sigil(context):
    context.sigil_fixture = load_sigil_fixture()
    context.galethog_elements = context.sigil_fixture["galethog"]["internalElements"]


@when("its internal references are resolved against the circumference")
def resolve_galethog_references(context):
    context.resolved_name = "".join(
        resolve_internal_element(context.sigil_fixture, element)
        for element in context.galethog_elements
    )


@given("an internal Sigillum element contains a circumference reference")
def internal_element_with_reference(context):
    context.sigil_fixture = load_sigil_fixture()
    context.internal_sigil_element = context.sigil_fixture["lookupExample"]


@when("the element is resolved")
def resolve_internal_sigil_element(context):
    context.resolved_internal_value = resolve_internal_element(
        context.sigil_fixture,
        context.internal_sigil_element,
    )


@then("its value is obtained from the referenced circumference position")
def assert_internal_lookup(context):
    assert (
        context.resolved_internal_value
        == context.internal_sigil_element["expectedValue"]
    )
