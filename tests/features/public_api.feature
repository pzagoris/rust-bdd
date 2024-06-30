Feature: API testing in the Public API

  Scenario: Request Server Time
    Given "GET" request to "server_time_path" path of public api
    Then the response status should be 200
    Then validate the response of server time

  Scenario Outline: Request Information for trading pair
    Given "GET" request to "assetpairs_path" path of public api with param <key>=<value>
    Then the response status should be 200
    Then validate the Asset Pairs response contains <value> info

  Examples:
    | key  | value    |
    | pair | XXBTZUSD |

