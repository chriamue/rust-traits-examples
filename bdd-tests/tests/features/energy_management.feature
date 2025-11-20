Feature: Energy Management

  Scenario: Walking consumes energy
    Given a dog named "Max" with breed "Labrador"
    And the dog has energy level "Energetic"
    When the dog walks
    Then the dog's energy should decrease
    And the dog should have energy level "Normal"

  Scenario: Running consumes more energy than walking
    Given a dog named "Bolt" with breed "Greyhound"
    And the dog has energy level "Hyperactive"
    When the dog runs
    Then the dog's energy should decrease by 2 levels
    And the dog should have energy level "Energetic"

  Scenario: Swimming consumes energy
    Given a duck named "Donald"
    And the duck has energy level "Energetic"
    When the duck swims
    Then the duck's energy should decrease
    And the duck should have energy level "Normal"

  Scenario: Flying consumes energy
    Given an eagle named "Liberty"
    And the eagle has energy level "Hyperactive"
    When the eagle flies
    Then the eagle's energy should decrease
    And the eagle should have energy level "Energetic"

  Scenario: Cannot move when collapsed
    Given a dog named "Tired" with breed "Bulldog"
    And the dog has energy level "Collapsed"
    When the dog attempts to walk
    Then the action should fail with "Collapsed" error

  Scenario: Insufficient energy prevents running
    Given a dog named "Weary" with breed "Husky"
    And the dog has energy level "Tired"
    When the dog attempts to run
    Then the action should fail with "InsufficientEnergy" error

  Scenario: Resting restores energy
    Given a duck named "Sleepy"
    And the duck has energy level "Tired"
    When the duck rests
    Then the duck's energy should increase
    And the duck should have energy level "Normal"
