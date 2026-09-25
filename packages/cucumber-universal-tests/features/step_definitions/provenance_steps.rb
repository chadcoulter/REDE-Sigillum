require "json"

PROVENANCE_FIXTURE_PATH = File.expand_path(
  "../../fixtures/provenance.retention.json",
  __dir__
)

def load_provenance_fixture
  JSON.parse(File.read(PROVENANCE_FIXTURE_PATH))
end

Given("a Sigillum transform is accepted") do
  @provenance_fixture = load_provenance_fixture
  @accepted_transform = @provenance_fixture.fetch("acceptedTransform")
end

When("I inspect its transform trace") do
  @transform_trace = @accepted_transform.fetch("steps")
end

Then("every step identifies the rule that produced it") do
  @transform_trace.each do |step|
    rule = step["rule"]
    raise "transform step is missing its rule" unless rule.is_a?(String) && !rule.strip.empty?
  end
end

Then("every step identifies the source data used by that rule") do
  source_registry = @provenance_fixture.fetch("sourceRegistry")

  @transform_trace.each do |step|
    sources = step["sources"]

    unless sources.is_a?(Array) && !sources.empty?
      raise "transform step is missing source references"
    end

    sources.each do |source_id|
      source = source_registry[source_id]
      raise "unknown source #{source_id}" if source.nil?
      raise "source #{source_id} has no URL" if source["url"].to_s.empty?
    end
  end
end

Given("a result is derived through multiple transform steps") do
  @provenance_fixture = load_provenance_fixture
  @provenance_steps = @provenance_fixture
    .fetch("acceptedTransform")
    .fetch("steps")
  @derived_result = @provenance_fixture.fetch("derivedResult")
end

When("I inspect the result provenance") do
  @result_lineage = @derived_result.fetch("lineage")
end

Then("the complete ordered transform lineage is available") do
  expected = @provenance_steps
    .sort_by { |step| step.fetch("order") }
    .map { |step| step.fetch("id") }

  unless @result_lineage == expected
    raise "expected ordered lineage #{expected.inspect}, got #{@result_lineage.inspect}"
  end
end

Then("no earlier transform step has been erased") do
  original_ids = @provenance_steps.map { |step| step.fetch("id") }

  unless @result_lineage.length == original_ids.length &&
         @result_lineage == original_ids &&
         @result_lineage.uniq.length == original_ids.length
    raise "transform lineage was erased, reordered, or duplicated"
  end
end
