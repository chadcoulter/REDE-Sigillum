import json
from pathlib import Path

from behave import given, then, when


FIXTURE_PATH = (
    Path(__file__).resolve().parents[2]
    / "fixtures"
    / "circumference.structure.json"
)


@given("the historical Sigillum data set is loaded")
def load_historical_sigillum(context):
    with FIXTURE_PATH.open("r", encoding="utf-8") as handle:
        context.sigillum_dataset = json.load(handle)


@when("I inspect the circumference")
def inspect_circumference(context):
    context.circumference = context.sigillum_dataset["circumference"]


@then("it contains {count:d} chambers")
def assert_chamber_count(context, count):
    assert len(context.circumference["chambers"]) == count


@then("each chamber spans {degrees:d} degrees")
def assert_chamber_span(context, degrees):
    for chamber in context.circumference["chambers"]:
        span = chamber["endDegree"] - chamber["startDegree"]
        assert span == degrees


@then("the circumference spans {degrees:d} degrees")
def assert_total_span(context, degrees):
    total = sum(
        chamber["endDegree"] - chamber["startDegree"]
        for chamber in context.circumference["chambers"]
    )
    assert total == degrees


@when("I group the circumference into companies")
def group_circumference(context):
    groups = {}
    for chamber in context.sigillum_dataset["circumference"]["chambers"]:
        groups.setdefault(chamber["company"], []).append(chamber)
    context.companies = groups


@then("there are {count:d} companies")
def assert_company_count(context, count):
    assert len(context.companies) == count


@then("each company contains {count:d} chambers")
def assert_company_size(context, count):
    assert all(len(chambers) == count for chambers in context.companies.values())
