Feature: Winner Announcement System
  As a competition organizer
  I want to announce winners through different media channels
  So that the public can learn about competition results

  Background:
    Given the announcement system is initialized
    And competition results are available

  Scenario Outline: Announce triathlon winner through different channels
    Given a triathlon competition with winner "Daffy the Duck"
    And the winner scored <score> points
    When I announce the winner via <media_type>
    Then the announcement should be published
    And the message should contain "Daffy the Duck"
    And the message should be formatted for <media_type>

    Examples:
      | media_type | score |
      | Television |   450 |
      | Newspaper  |   450 |
      | Radio      |   450 |

  Scenario: Multiple announcements for relay race
    Given a relay race with winning team "Team Aquatic"
    When I announce via multiple channels:
      | channel    |
      | Television |
      | Newspaper  |
      | Radio      |
    Then all announcements should be published
    And each channel should receive the appropriate format

  Scenario: Announcement includes competition details
    Given a vehicle race winner "Sky Master" with score 580
    When I create an announcement for "Television"
    Then the announcement should include:
      | detail            | value        |
      | winner_name       | Sky Master   |
      | score             |          580 |
      | competition       | Vehicle Race |
      | announcement_time | present      |
