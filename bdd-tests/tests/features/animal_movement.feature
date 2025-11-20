Feature: Animal Movement Capabilities

  Scenario: Dog can walk and swim
    Given a dog named "Rex" with breed "Labrador"
    Then the dog should be able to walk
    And the dog should be able to swim
    And the dog should be able to land move
    But the dog should not be able to fly

  Scenario: Duck has all movement capabilities
    Given a duck named "Daffy"
    Then the duck should be able to walk
    And the duck should be able to swim
    And the duck should be able to fly
    And the duck should be able to land move

  Scenario: Eagle can walk and fly but not swim
    Given an eagle named "Freedom"
    Then the eagle should be able to walk
    And the eagle should be able to fly
    And the eagle should be able to land move
    But the eagle should not be able to swim

  Scenario: Whale can only swim
    Given a whale named "Moby" of species "BlueWhale"
    Then the whale should be able to swim
    But the whale should not be able to walk
    And the whale should not be able to fly
    And the whale should not be able to land move

  Scenario: Snake can swim but not walk
    Given a snake named "Sly" of species "Anaconda"
    Then the snake should be able to swim
    But the snake should not be able to walk
    And the snake should not be able to land move
