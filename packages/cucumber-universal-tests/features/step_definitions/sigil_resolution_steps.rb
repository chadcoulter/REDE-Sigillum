require "json"

SIGIL_FIXTURE_PATH = File.expand_path(
  "../../fixtures/internal-sigil.galethog.json",
  __dir__
)

def load_sigil_fixture
  JSON.parse(File.read(SIGIL_FIXTURE_PATH))
end

def resolve_internal_sigil_element(fixture, element)
  reference = element["circumferenceReference"]

  return element.fetch("literal") if reference.nil?

  fixture
    .fetch("upperCircleLookup")
    .fetch(reference.to_s)
end

Given("the historical Galethog sigil data is loaded") do
  @sigil_fixture = load_sigil_fixture
  @galethog_elements = @sigil_fixture
    .fetch("galethog")
    .fetch("internalElements")
end

When("its internal references are resolved against the circumference") do
  @resolved_name = @galethog_elements
    .map { |element| resolve_internal_sigil_element(@sigil_fixture, element) }
    .join
end

Given("an internal Sigillum element contains a circumference reference") do
  @sigil_fixture = load_sigil_fixture
  @internal_sigil_element = @sigil_fixture.fetch("lookupExample")
end

When("the element is resolved") do
  @resolved_internal_value =
    resolve_internal_sigil_element(@sigil_fixture, @internal_sigil_element)
end

Then("its value is obtained from the referenced circumference position") do
  expected = @internal_sigil_element.fetch("expectedValue")

  unless @resolved_internal_value == expected
    raise "expected #{expected}, got #{@resolved_internal_value}"
  end
end
