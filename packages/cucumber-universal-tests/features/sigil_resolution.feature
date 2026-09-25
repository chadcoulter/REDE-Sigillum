@sigil-resolution
Feature: Internal Sigillum resolution

  Scenario: Resolve the documented Galethog derivation
    Given the historical Galethog sigil data is loaded
    When its internal references are resolved against the circumference
    Then the resolved name is "Galethog"

  Scenario: Internal numeric references resolve through the circumference
    Given an internal Sigillum element contains a circumference reference
    When the element is resolved
    Then its value is obtained from the referenced circumference position
