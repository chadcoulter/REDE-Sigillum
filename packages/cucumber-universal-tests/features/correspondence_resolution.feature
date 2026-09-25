@correspondence-resolution
Feature: Cross-system correspondence resolution

  Scenario: Resolve AAOTH to Aaron through the documented correspondence chain
    Given the documented correspondence data is loaded
    When I resolve the correspondence path from "AAOTH" to "Aaron"
    Then the path is:
      | node    |
      | AAOTH   |
      | Mercury |
      | Hod     |
      | Aaron   |

  Scenario: Correspondence resolution does not become a spelling transform
    Given the documented correspondence data is loaded
    When I resolve the correspondence path from "AAOTH" to "Aaron"
    Then the result is identified as a correspondence path
    And the result is not identified as a direct name transform
