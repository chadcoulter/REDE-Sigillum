import json
from collections import deque
from pathlib import Path

from behave import given, then, when


CORRESPONDENCE_FIXTURE_PATH = (
    Path(__file__).resolve().parents[2]
    / "fixtures"
    / "correspondence.aaoth-aaron.json"
)


def load_correspondence_fixture():
    with CORRESPONDENCE_FIXTURE_PATH.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def resolve_correspondence_path(fixture, source, target):
    adjacency = {}

    for edge in fixture["edges"]:
        adjacency.setdefault(edge["from"], []).append(edge["to"])

    queue = deque([[source]])
    visited = {source}

    while queue:
        path = queue.popleft()
        current = path[-1]

        if current == target:
            return path

        for next_node in adjacency.get(current, []):
            if next_node in visited:
                continue
            visited.add(next_node)
            queue.append(path + [next_node])

    raise AssertionError(f"no correspondence path from {source} to {target}")


@given("the documented correspondence data is loaded")
def load_documented_correspondence_data(context):
    context.correspondence_fixture = load_correspondence_fixture()


@when('I resolve the correspondence path from "{source}" to "{target}"')
def resolve_documented_correspondence_path(context, source, target):
    context.correspondence_path = resolve_correspondence_path(
        context.correspondence_fixture,
        source,
        target,
    )

    context.correspondence_result_type = "correspondence_path"
    context.correspondence_is_direct_name_transform = False


@then("the path is:")
def assert_correspondence_path(context):
    expected = [row["node"] for row in context.table]
    assert context.correspondence_path == expected


@then("the result is identified as a correspondence path")
def assert_correspondence_result_type(context):
    assert context.correspondence_result_type == "correspondence_path"


@then("the result is not identified as a direct name transform")
def assert_not_direct_name_transform(context):
    assert context.correspondence_is_direct_name_transform is False
