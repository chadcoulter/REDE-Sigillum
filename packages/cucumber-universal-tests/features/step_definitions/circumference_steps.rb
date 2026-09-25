require "json"

FIXTURE_PATH = File.expand_path(
  "../../fixtures/circumference.structure.json",
  __dir__
)

Given("the historical Sigillum data set is loaded") do
  @sigillum_dataset = JSON.parse(File.read(FIXTURE_PATH))
end

When("I inspect the circumference") do
  @circumference = @sigillum_dataset.fetch("circumference")
end

Then("it contains {int} chambers") do |expected|
  actual = @circumference.fetch("chambers").length
  raise "expected #{expected} chambers, got #{actual}" unless actual == expected
end

Then("each chamber spans {int} degrees") do |expected|
  @circumference.fetch("chambers").each do |chamber|
    actual = chamber.fetch("endDegree") - chamber.fetch("startDegree")
    raise "expected chamber span #{expected}, got #{actual}" unless actual == expected
  end
end

Then("the circumference spans {int} degrees") do |expected|
  actual = @circumference.fetch("chambers").sum do |chamber|
    chamber.fetch("endDegree") - chamber.fetch("startDegree")
  end
  raise "expected circumference span #{expected}, got #{actual}" unless actual == expected
end

When("I group the circumference into companies") do
  @companies = @sigillum_dataset
    .fetch("circumference")
    .fetch("chambers")
    .group_by { |chamber| chamber.fetch("company") }
end

Then("there are {int} companies") do |expected|
  actual = @companies.length
  raise "expected #{expected} companies, got #{actual}" unless actual == expected
end

Then("each company contains {int} chambers") do |expected|
  invalid = @companies.reject { |_company, chambers| chambers.length == expected }
  raise "company chamber counts did not equal #{expected}: #{invalid.keys}" unless invalid.empty?
end
