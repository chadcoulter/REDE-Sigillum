@circumference
Feature: Sigillum circumference structure

  Scenario: The historical circumference is complete
    Given the historical Sigillum data set is loaded
    When I inspect the circumference
    Then it contains 40 chambers
    And each chamber spans 9 degrees
    And the circumference spans 360 degrees

  Scenario: The circumference is organized into eight companies
    Given the historical Sigillum data set is loaded
    When I group the circumference into companies
    Then there are 8 companies
    And each company contains 5 chambers
