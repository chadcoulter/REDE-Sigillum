@dataset-variants
Feature: Historical data set variants

  Scenario: A historical variant identifies its source
    Given multiple historical Sigillum data variants are available
    When a variant is selected
    Then the selected variant identifies its source

  Scenario Outline: Known circumference discrepancies remain variant-specific
    Given multiple historical Sigillum data variants are available
    When I inspect circumference division <division>
    Then each differing value remains associated with its historical variant
    And the variants remain distinguishable

    Examples:
      | division |
      | 22       |
      | 32       |
