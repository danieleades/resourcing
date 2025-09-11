Feature: Project assignments

  Scenario: View resource assignments for projects

    Given I am on the "Project Assignments" page
    And I have added projects
    And I have added resources
    And I have assigned resources to projects
    When I view the project assignments
    Then I can see the resource assignments for each project
