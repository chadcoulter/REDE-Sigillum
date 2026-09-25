@transform-resolution
Feature: Sigillum transform resolution

  Scenario Outline: Resolve a known Sigillum transform
    Given the Sigillum data set is loaded
    When I resolve "<source>"
    Then the result is "<result>"

    Examples:
      | source | result |
      | G      | Galas  |
