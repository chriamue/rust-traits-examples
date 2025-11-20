Feature: Vehicle Capability Matrix
  As a fleet manager
  I want to understand vehicle capabilities across different terrains
  So that I can deploy the right vehicle for each mission

  Background:
    Given the vehicle fleet management system is active
    And all vehicle types are available

  Scenario: Vehicle capability comparison
    Given the following vehicles are available:
      | name        | type       | manufacturer | year |
      | Road Runner | Car        | Tesla        | 2023 |
      | Sky Master  | Airplane   | Boeing       | 2023 |
      | Wave Rider  | Ship       | Maritime     | 2023 |
      | Air Patrol  | Helicopter | AgustaW      | 2023 |
      | Speed Demon | Motorcycle | Ducati       | 2023 |
    When I query their movement capabilities
    Then I should get the following capability matrix:
      | vehicle     | can_drive | can_swim | can_fly | can_land_move |
      | Road Runner | true      | false    | false   | true          |
      | Sky Master  | true      | false    | true    | true          |
      | Wave Rider  | false     | true     | false   | false         |
      | Air Patrol  | false     | false    | true    | false         |
      | Speed Demon | true      | false    | false   | true          |

  Scenario Outline: Vehicle performance on different road types
    Given a car named "TestCar" with manufacturer "Generic" and year 2023
    And the car has energy level "Hyperactive"
    When the car drives on "<road_type>" road
    Then the action should <result>
    And the energy consumption should be approximately <energy_cost> levels

    Examples:
      | road_type  | result  | energy_cost |
      | Highway    | succeed |           1 |
      | City       | succeed |           1 |
      | Country    | succeed |           1 |
      | Mountain   | succeed |           2 |
      | OffRoad    | fail    |           0 |
      | ExtremeOff | fail    |           0 |
