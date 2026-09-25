@provenance
Feature: Transform provenance

  Scenario: Every accepted transform retains its rule and source
    Given a Sigillum transform is accepted
    When I inspect its transform trace
    Then every step identifies the rule that produced it
    And every step identifies the source data used by that rule

  Scenario: Derived results retain their full transform lineage
    Given a result is derived through multiple transform steps
    When I inspect the result provenance
    Then the complete ordered transform lineage is available
    And no earlier transform step has been erased
