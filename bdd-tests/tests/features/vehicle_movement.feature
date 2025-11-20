Feature: Vehicle Movement Capabilities

  Scenario: Car can drive and land move
    Given a car named "Tesla" with manufacturer "Tesla Motors" and year 2023
    Then the car should be able to drive
    And the car should be able to land move
    But the car should not be able to fly
    And the car should not be able to swim

  Scenario: Airplane can fly and drive (taxi)
    Given an airplane named "Boeing 747" with manufacturer "Boeing" and type "Commercial"
    Then the airplane should be able to fly
    And the airplane should be able to drive
    And the airplane should be able to land move
    But the airplane should not be able to swim

  Scenario: Ship can only swim
    Given a ship named "Titanic" with manufacturer "Harland" and type "CruiseShip"
    Then the ship should be able to swim
    But the ship should not be able to drive
    And the ship should not be able to fly
    And the ship should not be able to land move

  Scenario: Helicopter can only fly
    Given a helicopter named "Apache" with manufacturer "Boeing" and type "Military"
    Then the helicopter should be able to fly
    But the helicopter should not be able to drive
    And the helicopter should not be able to swim
    And the helicopter should not be able to land move

  Scenario: Motorcycle can drive and land move
    Given a motorcycle named "Harley" with manufacturer "Harley-Davidson" and engine size 1200
    Then the motorcycle should be able to drive
    And the motorcycle should be able to land move
    But the motorcycle should not be able to fly
    And the motorcycle should not be able to swim
