require "json"

VARIANT_FIXTURE_PATH = File.expand_path(
  "../../fixtures/historical.dataset-variants.json",
  __dir__
)

def load_variant_fixture
  JSON.parse(File.read(VARIANT_FIXTURE_PATH))
end

Given("multiple historical Sigillum data variants are available") do
  @variant_fixture = load_variant_fixture
  @historical_variants = @variant_fixture.fetch("variants")
end

When("a variant is selected") do
  @selected_variant_id = @variant_fixture.fetch("defaultVariant")
  @selected_variant = @historical_variants.fetch(@selected_variant_id)
end

Then("the selected variant identifies its source") do
  source = @selected_variant["source"]

  unless source.is_a?(Hash) &&
         !source["id"].to_s.empty? &&
         !source["url"].to_s.empty?
    raise "selected historical variant does not identify its source"
  end
end

When("I inspect circumference division {int}") do |division|
  division_key = division.to_s
  @inspected_division = division

  @variant_division_values = @historical_variants.to_h do |variant_id, variant|
    [
      variant_id,
      {
        "value" => variant.fetch("circumferenceDivisions").fetch(division_key),
        "source" => variant.fetch("source")
      }
    ]
  end
end

Then("each differing value remains associated with its historical variant") do
  raise "at least two variants are required" unless @variant_division_values.length >= 2

  @variant_division_values.each do |variant_id, reading|
    source = reading.fetch("source")

    if variant_id.to_s.empty? ||
       reading.fetch("value").to_s.empty? ||
       source.fetch("id").to_s.empty? ||
       source.fetch("url").to_s.empty?
      raise "variant reading lost its value or source association"
    end
  end
end

Then("the variants remain distinguishable") do
  variant_ids = @variant_division_values.keys
  values = @variant_division_values.values.map { |reading| reading.fetch("value") }

  raise "variant identifiers are not unique" unless variant_ids.uniq.length == variant_ids.length
  raise "variant values are not distinguishable" unless values.uniq.length > 1
end
