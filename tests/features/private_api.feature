Feature: API testing in the Private API

  Scenario Outline: Request Open Orders of an account
    Given POST request to retrieve account open orders
    Then the response status should be 200
    Then the account open orders are <orders>

  Examples:
    | orders  |
    | 0 |

