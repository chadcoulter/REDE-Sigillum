require "json"

TRAVERSAL_FIXTURE_PATH = File.expand_path(
  "../../fixtures/traversal.name-resolution.json",
  __dir__
)

TRAVERSAL_VOWELS = %w[a e i o u].freeze

def load_traversal_fixture
  JSON.parse(File.read(TRAVERSAL_FIXTURE_PATH))
end

def traversal_direction(placement)
  return 1 if placement == "above"
  return -1 if placement == "below"

  0
end

def next_traversal_index(current_index, number, placement, circumference_size)
  (
    current_index +
    traversal_direction(placement) * number
  ) % circumference_size
end

def normalize_traversal_name(symbols)
  normalized = []

  symbols.each_with_index do |symbol, index|
    if index > 1 &&
       !normalized.empty? &&
       TRAVERSAL_VOWELS.include?(symbol.downcase) &&
       normalized.last.downcase == symbol.downcase
      next
    end

    normalized << symbol
  end

  normalized.join
end

def format_traversal_trace(steps)
  steps.map do |step|
    symbol = step.fetch("symbol")
    number = step["number"]
    placement = step["placement"]

    next symbol if number.nil?

    signed_number = placement == "above" ? number : -number
    "#{symbol}[#{signed_number}]"
  end.join(" -> ")
end

Given("the traversal begins at {string}") do |start|
  @traversal_fixture = load_traversal_fixture
  @traversal_steps = @traversal_fixture
    .fetch("documentedTraversals")
    .fetch(start)
end

When("the circumference traversal is resolved") do
  @traversal_trace = format_traversal_trace(@traversal_steps)
  @resolved_name = normalize_traversal_name(
    @traversal_steps.map { |step| step.fetch("symbol") }
  )
end

Then("the traversal is {string}") do |expected|
  raise "expected #{expected}, got #{@traversal_trace}" unless @traversal_trace == expected
end

Then("the resolved name is {string}") do |expected|
  raise "expected #{expected}, got #{@resolved_name}" unless @resolved_name == expected
end

Given("a traversal step with a number above the current letter") do
  @traversal_fixture = load_traversal_fixture
  @traversal_step = @traversal_fixture.fetch("stepExamples").fetch("above")
end

Given("a traversal step with a number below the current letter") do
  @traversal_fixture = load_traversal_fixture
  @traversal_step = @traversal_fixture.fetch("stepExamples").fetch("below")
end

When("the traversal step is resolved") do
  if @traversal_step["number"].nil?
    @traversal_terminated = true
    @terminal_symbol = @traversal_step.fetch("symbol")
    next
  end

  @traversal_terminated = false
  @selected_direction =
    @traversal_step.fetch("placement") == "above" ? "clockwise" : "counterclockwise"
  @selected_number = @traversal_step.fetch("number")
  @selected_index = next_traversal_index(
    @traversal_step.fetch("currentIndex"),
    @selected_number,
    @traversal_step.fetch("placement"),
    @traversal_fixture.fetch("circumferenceSize")
  )
end

Then("the next chamber is selected clockwise by that number") do
  raise "step did not resolve clockwise" unless @selected_direction == "clockwise"
  raise "step number changed" unless @selected_number == @traversal_step.fetch("number")
  raise "wrong target chamber" unless @selected_index == @traversal_step.fetch("expectedIndex")
end

Then("the next chamber is selected counterclockwise by that number") do
  raise "step did not resolve counterclockwise" unless @selected_direction == "counterclockwise"
  raise "step number changed" unless @selected_number == @traversal_step.fetch("number")
  raise "wrong target chamber" unless @selected_index == @traversal_step.fetch("expectedIndex")
end

Given("a traversal reaches a letter without a number") do
  @traversal_fixture = load_traversal_fixture
  @traversal_step = @traversal_fixture.fetch("stepExamples").fetch("termination")
end

Then("the name terminates at that letter") do
  raise "traversal did not terminate" unless @traversal_terminated
  raise "wrong terminal letter" unless @terminal_symbol == @traversal_step.fetch("symbol")
end

Given("a resolved traversal contains consecutive duplicate vowels after the first letter") do
  fixture = load_traversal_fixture
  example = fixture
    .fetch("normalizationExamples")
    .fetch("duplicateVowelsAfterBeginning")

  @normalization_symbols = example.fetch("symbols")
  @expected_normalized_name = example.fetch("expected")
end

When("the name is normalized") do
  @normalized_name = normalize_traversal_name(@normalization_symbols)
end

Then("the consecutive duplicate vowels are represented once") do
  unless @normalized_name == @expected_normalized_name
    raise "expected #{@expected_normalized_name}, got #{@normalized_name}"
  end
end
