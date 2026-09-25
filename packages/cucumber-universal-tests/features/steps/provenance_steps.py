import json
from pathlib import Path

from behave import given, then, when


PROVENANCE_FIXTURE_PATH = (
    Path(__file__).resolve().parents[2]
    / "fixtures"
    / "provenance.retention.json"
)


def load_provenance_fixture():
    with PROVENANCE_FIXTURE_PATH.open("r", encoding="utf-8") as handle:
        return json.load(handle)


@given("a Sigillum transform is accepted")
def accepted_sigillum_transform(context):
    context.provenance_fixture = load_provenance_fixture()
    context.accepted_transform = context.provenance_fixture["acceptedTransform"]


@when("I inspect its transform trace")
def inspect_transform_trace(context):
    context.transform_trace = context.accepted_transform["steps"]


@then("every step identifies the rule that produced it")
def assert_rule_retention(context):
    for step in context.transform_trace:
        assert isinstance(step.get("rule"), str)
        assert step["rule"].strip()


@then("every step identifies the source data used by that rule")
def assert_source_retention(context):
    source_registry = context.provenance_fixture["sourceRegistry"]

    for step in context.transform_trace:
        sources = step.get("sources")
        assert isinstance(sources, list)
        assert sources

        for source_id in sources:
            assert source_id in source_registry
            assert source_registry[source_id].get("url")


@given("a result is derived through multiple transform steps")
def result_with_multiple_transform_steps(context):
    context.provenance_fixture = load_provenance_fixture()
    context.provenance_steps = context.provenance_fixture["acceptedTransform"]["steps"]
    context.derived_result = context.provenance_fixture["derivedResult"]


@when("I inspect the result provenance")
def inspect_result_provenance(context):
    context.result_lineage = context.derived_result["lineage"]


@then("the complete ordered transform lineage is available")
def assert_complete_ordered_lineage(context):
    expected = [
        step["id"]
        for step in sorted(
            context.provenance_steps,
            key=lambda step: step["order"],
        )
    ]

    assert context.result_lineage == expected


@then("no earlier transform step has been erased")
def assert_no_lineage_erasure(context):
    original_ids = [step["id"] for step in context.provenance_steps]

    assert len(context.result_lineage) == len(original_ids)
    assert context.result_lineage == original_ids
    assert len(set(context.result_lineage)) == len(original_ids)
