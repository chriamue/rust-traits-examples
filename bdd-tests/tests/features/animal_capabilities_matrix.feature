Feature: Animal Capabilities Matrix
  As a wildlife researcher
  I want to document animal movement capabilities
  So that I can classify animals by their mobility traits

  Background:
    Given I am researching animal locomotion
    And I have access to various animal species
    And the energy tracking system is operational

  Scenario: Create comprehensive animal capability profile
    Given the following animals:
      | name    | species | breed     |
      | Buddy   | Dog     | Labrador  |
      | Daffy   | Duck    | N/A       |
      | Eddie   | Eagle   | N/A       |
      | Waddles | Penguin | N/A       |
      | Moby    | Whale   | BlueWhale |
      | Sly     | Snake   | Anaconda  |
    When I test all movement capabilities
    Then I should have the following capability matrix:
      | animal  | walk  | swim  | fly   | land_move |
      | Buddy   | true  | true  | false | true      |
      | Daffy   | true  | true  | true  | true      |
      | Eddie   | true  | false | true  | true      |
      | Waddles | true  | true  | false | true      |
      | Moby    | false | true  | false | false     |
      | Sly     | false | true  | false | false     |

  Scenario: Energy consumption patterns across species
    Given the following animals with initial energy:
      | name | species | energy      |
      | Max  | Dog     | Hyperactive |
      | Don  | Duck    | Hyperactive |
      | Fred | Eagle   | Hyperactive |
    When each animal performs their primary movement:
      | animal | movement |
      | Max    | walk     |
      | Don    | swim     |
      | Fred   | fly      |
    Then the energy consumption should be:
      | animal | starting    | ending    | levels_consumed |
      | Max    | Hyperactive | Energetic |               1 |
      | Don    | Hyperactive | Normal    |               2 |
      | Fred   | Hyperactive | Tired     |               2 |
