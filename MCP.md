# Top 20 Useful Backlog APIs for an AI MCP Server

These APIs provide a strong foundation for an AI MCP server to
interact meaningfully with Backlog, covering essential data retrieval
(issues, comments, wikis, project context, users) and manipulation
(creating/updating issues, comments, wikis).

- [x] GET /api/v2/issues/{issueIdOrKey}
    - **Reason:** To retrieve detailed information about a specific issue. This is fundamental for an AI to understand the context, description, status, assignee, etc., of a task it needs to process or analyze.
- [ ] GET /api/v2/issues
    - **Reason:** To fetch a list of issues based on various criteria. Useful for AI to get an overview of tasks in a project, tasks assigned to it, or tasks matching specific conditions for analysis or reporting.
- [ ] PATCH /api/v2/issues/{issueIdOrKey}
    - **Reason:** To update existing issues. Essential for AI to perform actions like changing the status, assigning users, setting due dates, or adding details based on its analysis or external triggers.
- [ ] POST /api/v2/issues
    - **Reason:** To create new issues. Allows AI to automatically create tasks based on incoming requests, detected anomalies, or workflow triggers.
- [ ] GET /api/v2/issues/{issueIdOrKey}/comments
    - **Reason:** To retrieve comments on an issue. Crucial for AI to understand the discussion history, specific instructions, or progress updates related to a task.
- [ ] POST /api/v2/issues/{issueIdOrKey}/comments
    - **Reason:** To add comments to an issue. Enables AI to report findings, ask questions, provide updates, or interact with users directly within the Backlog issue.
- [ ] GET /api/v2/wikis/{wikiId}
    - **Reason:** To retrieve the content of a specific wiki page. Important for AI to access knowledge bases, documentation, or project specifications stored in Backlog Wiki for context or analysis.
- [ ] GET /api/v2/wikis
    - **Reason:** To list wiki pages within a project. Allows AI to discover and access relevant documentation or knowledge resources.
- [ ] GET /api/v2/projects/{projectIdOrKey}
    - **Reason:** To get detailed information about a specific project, including members, issue types, categories, etc. Provides essential context for AI operating within that project.
- [ ] GET /api/v2/users
    - **Reason:** To retrieve a list of users in the space. Necessary for AI when suggesting assignees, mentioning users, or understanding roles.
- [x] GET /api/v2/users/myself
    - **Reason:** For the AI agent (if represented as a user) to get its own user details, potentially checking permissions or identifying itself.
- [ ] GET /api/v2/projects/{projectIdOrKey}/users
    - **Reason:** To get the list of members specifically associated with a project. More targeted than getting all space users when dealing with project-specific assignments or mentions.
- [ ] GET /api/v2/projects/{projectIdOrKey}/statuses
    - **Reason:** To get the available issue statuses for a project. Required when AI needs to update an issue's status correctly within project-specific workflows.
- [ ] GET /api/v2/projects/{projectIdOrKey}/issueTypes
    - **Reason:** To get the available issue types for a project. Needed when AI creates or classifies issues within a project.
- [ ] GET /api/v2/projects/{projectIdOrKey}/categories
    - **Reason:** To get the available categories for a project. Useful for AI when creating or classifying issues.
- [ ] GET /api/v2/projects/{projectIdOrKey}/versions
    - **Reason:** To get the available versions/milestones for a project. Relevant for AI when linking issues to specific releases or milestones.
- [ ] POST /api/v2/wikis
    - **Reason:** To create new wiki pages. Enables AI to generate documentation, meeting notes, or reports directly within Backlog.
- [ ] PATCH /api/v2/wikis/{wikiId}
    - **Reason:** To update existing wiki pages. Allows AI to refine documentation, correct information, or add summaries.
- [ ] GET /api/v2/issues/count
    - **Reason:** To get the number of issues matching specific criteria without fetching the full list. Useful for dashboards, quick summaries, or monitoring by the AI.
- [ ] GET /api/v2/space/activities
    - **Reason:** To get a stream of recent activities across the space or specific projects/users. Can provide the AI with a high-level overview of recent changes and events for situational awareness.
