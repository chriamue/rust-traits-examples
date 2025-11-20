Feature: Dog Breed Capabilities
  As a dog trainer
  I want to understand different dog breed capabilities
  So that I can select the right dog for specific tasks

  Background:
    Given the energy management system is initialized
    And all dog breeds are registered in the system

  Scenario Outline: Different dog breeds have varying swimming capabilities
    Given a dog named "<name>" with breed "<breed>"
    When the dog attempts to swim
    Then the dog should be able to swim
    And the dog's maximum swimming depth should be <max_depth> meters

    Examples:
      | name  | breed           | max_depth |
      | Rex   | Labrador        |        10 |
      | Buddy | GoldenRetriever |        10 |
      | Max   | BorderCollie    |        10 |
      | Duke  | SaintBernard    |        10 |
      | Flash | Greyhound       |         3 |
      | Tank  | Bulldog         |         2 |
      | Storm | Husky           |        10 |

  Scenario: Multiple dogs compete in swimming race
    Given the following dogs are entered in the race:
      | name   | breed           |
      | Splash | Labrador        |
      | Paddle | GoldenRetriever |
      | Dive   | BorderCollie    |
      | Float  | Bulldog         |
    When the swimming race begins
    Then all dogs should attempt to swim
    And "Splash" should have better swimming capability than "Float"
    And "Paddle" should have better swimming capability than "Float"

  Scenario: Dog energy levels affect performance
    Given a dog named "Energetic" with breed "Labrador"
    And the dog has the following initial state:
      | property | value       |
      | energy   | Hyperactive |
      | breed    | Labrador    |
    When the dog performs multiple activities:
      | activity | repetitions |
      | walk     |           1 |
      | swim     |           1 |
      | walk     |           1 |
    Then the dog's energy should be significantly depleted
    And the dog should still be able to perform basic movements
