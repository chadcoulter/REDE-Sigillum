require "json"

CORRESPONDENCE_FIXTURE_PATH = File.expand_path(
  "../../fixtures/correspondence.aaoth-aaron.json",
  __dir__
)

def load_correspondence_fixture
  JSON.parse(File.read(CORRESPONDENCE_FIXTURE_PATH))
end

def resolve_correspondence_path(fixture, source, target)
  adjacency = Hash.new { |hash, key| hash[key] = [] }

  fixture.fetch("edges").each do |edge|
    adjacency[edge.fetch("from")] << edge.fetch("to")
  end

  queue = [[source]]
  visited = { source => true }

  until queue.empty?
    path = queue.shift
    current = path.last

    return path if current == target

    adjacency[current].each do |next_node|
      next if visited[next_node]

      visited[next_node] = true
      queue << (path + [next_node])
    end
  end

  raise "no correspondence path from #{source} to #{target}"
end

Given("the documented correspondence data is loaded") do
  @correspondence_fixture = load_correspondence_fixture
end

When("I resolve the correspondence path from {string} to {string}") do |source, target|
  @correspondence_path = resolve_correspondence_path(
    @correspondence_fixture,
    source,
    target
  )

  expectation = @correspondence_fixture
    .fetch("expectedPaths")
    .find do |path|
      path.fetch("from") == source && path.fetch("to") == target
    end

  raise "missing expected path metadata" if expectation.nil?

  @correspondence_result_type = expectation.fetch("resultType")
  @correspondence_is_direct_name_transform =
    expectation.fetch("directNameTransform")
end

Then("the path is:") do |table|
  expected = table.hashes.map { |row| row.fetch("node") }

  unless @correspondence_path == expected
    raise "expected #{expected.inspect}, got #{@correspondence_path.inspect}"
  end
end

Then("the result is identified as a correspondence path") do
  unless @correspondence_result_type == "correspondence_path"
    raise "result was not identified as a correspondence path"
  end
end

Then("the result is not identified as a direct name transform") do
  if @correspondence_is_direct_name_transform
    raise "correspondence path was incorrectly identified as a direct name transform"
  end
end
