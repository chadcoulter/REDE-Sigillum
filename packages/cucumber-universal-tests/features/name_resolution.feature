@name-resolution
Feature: Sigillum name resolution

  Scenario: Resolve the documented Galas traversal
    Given the historical Sigillum data set is loaded
    And the traversal begins at "G"
    When the circumference traversal is resolved
    Then the traversal is "G[9] -> a[6] -> l[-8] -> a[20] -> a[8] -> s"
    And the resolved name is "Galas"

  Scenario: A number above a letter moves clockwise
    Given a traversal step with a number above the current letter
    When the traversal step is resolved
    Then the next chamber is selected clockwise by that number

  Scenario: A number below a letter moves counterclockwise
    Given a traversal step with a number below the current letter
    When the traversal step is resolved
    Then the next chamber is selected counterclockwise by that number

  Scenario: A letter without a number terminates a name
    Given a traversal reaches a letter without a number
    When the traversal step is resolved
    Then the name terminates at that letter

  Scenario: Consecutive duplicate vowels are collapsed after the beginning
    Given a resolved traversal contains consecutive duplicate vowels after the first letter
    When the name is normalized
    Then the consecutive duplicate vowels are represented once
