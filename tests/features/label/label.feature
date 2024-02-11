Feature: Labels

  Scenario: Valid Label parse_string
    Given "this_label"="is_good"
    When parser is parse_string
    Then label is valid

  Scenario: Valid Label parse_str
    Given "this_label"="is_good"
    When parser is parse_str
    Then label is valid