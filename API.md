# Backlog API Implementation Status

This document tracks the implementation status of Backlog API endpoints. Items marked with ✅ are implemented, ❌ are not implemented.

## Implementation Summary by Domain

### backlog-space (SpaceApi)
- **Implemented**: 3/9 endpoints (33%)
- **Read operations**: get_space(), get_space_logo()
- **Write operations**: upload_attachment()

### backlog-project (ProjectApi)
- **Implemented**: 23/36 endpoints (64%)
- **Read operations**: get_project_list(), get_project(), get_project_icon(), get_status_list(), get_issue_type_list(), get_version_milestone_list(), get_category_list(), get_priority_list(), get_resolution_list(), get_project_user_list(), get_custom_field_list()
- **Write operations** (requires `writable` feature): add_category(), update_category(), delete_category(), add_issue_type(), delete_issue_type(), update_issue_type(), add_version(), update_version(), delete_version(), add_status(), update_status(), delete_status(), update_status_order()

### backlog-issue (IssueApi)
- **Implemented**: 18/26 endpoints (69%)
- **Read operations**: get_issue(), get_issue_list(), count_issue(), get_comment_list(), get_comment(), count_comment(), get_attachment_list(), get_attachment_file(), get_shared_file_list(), get_participant_list(), get_comment_notifications()
- **Write operations** (requires `writable` feature): add_issue(), delete_issue(), update_issue(), add_comment(), delete_comment(), link_shared_files_to_issue(), add_comment_notification()

### backlog-file (FileApi)
- **Implemented**: 2/2 endpoints (100%)
- **Read operations**: get_shared_files_list(), get_file()

### backlog-git (GitApi)
- **Implemented**: 15/15 endpoints (100%) ✨ COMPLETE
- **Read operations**: get_repository_list(), get_repository(), get_pull_request_list(), get_pull_request_list_with_params(), get_pull_request(), get_pull_request_attachment_list(), download_pull_request_attachment(), get_pull_request_comment_list(), get_pull_request_comment_count(), get_pull_request_count(), get_pull_request_count_with_params()
- **Write operations** (requires `writable` feature): add_pull_request(), add_pull_request_comment(), update_pull_request(), update_pull_request_comment(), delete_pull_request_attachment()
- **Note**: Includes all official Backlog Git/Pull Request API endpoints plus enhanced parameter variants for improved usability

### backlog-user (UserApi)
- **Implemented**: 4/7 endpoints (57%)
- **Read operations**: get_own_user(), get_user_list(), get_user(), get_user_icon()

### backlog-document (DocumentApi)
- **Implemented**: 4/4 endpoints (100%) ✨ COMPLETE
- **Read operations**: list_documents(), get_document_tree(), get_document(), download_attachment()

### backlog-wiki (WikiApi)
- **Implemented**: 12/15 endpoints (80%)
- **Read operations**: get_wiki_list(), get_wiki_detail(), get_wiki_count(), get_wiki_attachment_list(), download_wiki_attachment(), get_wiki_tag_list(), get_wiki_history()
- **Write operations** (requires `writable` feature): add_wiki(), update_wiki(), delete_wiki(), attach_files_to_wiki(), delete_wiki_attachment()

### Not Implemented
- **Activities**: 0/4 endpoints
- **Notifications**: 0/4 endpoints
- **Watchings**: 0/7 endpoints
- **Stars**: 0/4 endpoints
- **Teams**: 0/6 endpoints
- **Recent**: 0/5 endpoints
- **Webhooks**: 0/5 endpoints
- **Rate Limit**: 0/1 endpoints
- **OAuth 2.0**: 0/1 endpoints

---

## Detailed API Status

### Spaces
- ✅ GetSpace: Returns information about your space: GET /api/v2/space
- ✅ GetSpaceLogo: Returns logo image of your space: GET /api/v2/space/image
- ❌ GetSpaceNotification: Returns space notification: GET /api/v2/space/notification
- ❌ UpdateSpaceNotification: Updates space notification: PUT /api/v2/space/notification
- ❌ GetLicence: Returns licence: GET /api/v2/space/licence
- ❌ GetSpaceDiskUsage: Returns information about space disk usage: GET /api/v2/space/diskUsage
- ✅ PostAttachmentFile: Posts an attachment file for issue or wiki. Returns id of the attachment file: POST /api/v2/space/attachment

### Projects
- ✅ GetProject: Returns information about project: GET /api/v2/projects/:projectIdOrKey
- ✅ GetProjectList: Returns list of projects: GET /api/v2/projects
- ✅ GetProjectIcon: Downloads project icon: GET /api/v2/projects/:projectIdOrKey/image
- ❌ AddProject: Adds new project: POST /api/v2/projects
- ❌ DeleteProject: Deletes project: DELETE /api/v2/projects/:projectIdOrKey
- ❌ GetProjectDiskUsage: Returns information about project disk usage: GET /api/v2/projects/:projectIdOrKey/diskUsage
- ❌ UpdateProject: Updates information about project: PATCH /api/v2/projects/:projectIdOrKey

#### Users
- ✅ GetProjectUserList: Returns list of project members: GET /api/v2/projects/:projectIdOrKey/users
- ❌ AddProjectUser: Adds user to list of project members: POST /api/v2/projects/:projectIdOrKey/users
- ❌ DeleteProjectUser: Removes user from list project members: DELETE /api/v2/projects/:projectIdOrKey/users

#### Project Admin
- ❌ GetListOfProjectAdministrators: Returns list of users who has Project Administrator role: GET /api/v2/projects/:projectIdOrKey/administrators
- ❌ AddProjectAdministrator: Adds “Project Administrator” role to user: POST /api/v2/projects/:projectIdOrKey/administrators
- ❌ DeleteProjectAdministrator: Removes Project Administrator role from user: DELETE /api/v2/projects/:projectIdOrKey/administrators

#### Teams
- ❌ AddProjectTeam: Add team to project: POST /api/v2/projects/:projectIdOrKey/teams
- ❌ DeleteProjectTeam: Removes a team from the project: DELETE /api/v2/projects/:projectIdOrKey/teams
- ❌ GetProjectTeamList: Returns list of project teams: GET /api/v2/projects/:projectIdOrKey/teams


### Issues
- ✅ GetIssueList: Returns list of issues: GET /api/v2/issues
- ✅ GetIssue: Returns information about issue: GET /api/v2/issues/:issueIdOrKey
- ✅ AddIssue: Adds new issue: POST /api/v2/issues
- ✅ DeleteIssue: Deletes issue: DELETE /api/v2/issues/:issueIdOrKey
- ✅ CountIssue: Returns number of issues: GET /api/v2/issues/count
- ✅ UpdateIssue: Updates information about issue: PATCH /api/v2/issues/:issueIdOrKey

#### Attachements
- ✅ GetListOfIssueAttachments: Returns the list of issue attachments: GET /api/v2/issues/:issueIdOrKey/attachments
- ✅ GetIssueAttachment: Downloads issue’s attachment file: GET /api/v2/issues/:issueIdOrKey/attachments/:attachmentId
- ✅ DeleteIssueAttachment: Deletes an attachment of issue: DELETE /api/v2/issues/:issueIdOrKey/attachments/:attachmentId

#### Shared files for issue
- ✅ GetListOfLinkedSharedFiles: Returns the list of linked Shared Files to issues: GET /api/v2/issues/:issueIdOrKey/sharedFiles
- ✅ LinkSharedFilesToIssue: Links shared files to issue: POST /api/v2/issues/:issueIdOrKey/sharedFiles
- ✅ RemoveLinkToSharedFileFromIssue: Removes link to shared file from issue: DELETE /api/v2/issues/:issueIdOrKey/sharedFiles/:id

### Comment
- ✅ UpdateComment: Updates content of comment: PATCH /api/v2/issues/:issueIdOrKey/comments/:commentId
- ✅ CountComment: Returns number of comments in issue: GET /api/v2/issues/:issueIdOrKey/comments/count
- ✅ AddComment: Adds a comment to the issue: POST /api/v2/issues/:issueIdOrKey/comments
- ✅ DeleteComment: Delete comment: DELETE /api/v2/issues/:issueIdOrKey/comments/:commentId
- ✅ GetCommentList: Returns list of comments in issue: GET /api/v2/issues/:issueIdOrKey/comments
- ✅ GetComment: Returns information about comment: GET /api/v2/issues/:issueIdOrKey/comments/:commentId
- ✅ GetIssueParticipantList: Returns list of issue participants: GET /api/v2/issues/:issueIdOrKey/participants
- ✅ GetListOfCommentNotifications: Returns the list of comment notifications: GET /api/v2/issues/:issueIdOrKey/comments/:commentId/notifications
- ✅ AddCommentNotification: Adds notifications to the comment. Only the user who added the comment can add notifications: POST /api/v2/issues/:issueIdOrKey/comments/:commentId/notifications

### Priority
- ✅ GetPriorityList: Returns list of priorities: GET /api/v2/priorities

### Resolution
- ✅ GetResolutionList: Returns list of resolutions: GET /api/v2/resolutions

### Category
- ✅ AddCategory: Adds new Category to the project: POST /api/v2/projects/:projectIdOrKey/categories
- ✅ UpdateCategory: Updates information about Category: PATCH /api/v2/projects/:projectIdOrKey/categories/:id
- ✅ DeleteCategory: Deletes Category: DELETE /api/v2/projects/:projectIdOrKey/categories/:id
- ✅ GetCategoryList: Returns list of Categories in the project: GET /api/v2/projects/:projectIdOrKey/categories

### Custom Fields
- ❌ UpdateCustomField: Updates Custom Field: PATCH /api/v2/projects/:projectIdOrKey/customFields/:id
- ✅ GetCustomFieldList: Returns list of Custom Fields in the project: GET /api/v2/projects/:projectIdOrKey/customFields
- ❌ AddCustomField: Adds new Custom Field to the project: POST /api/v2/projects/:projectIdOrKey/customFields
- ❌ AddListItemForListTypeCustomField: Adds new list item for list type custom field. Only administrator can call this API if the option “Add items in adding or editing issues” is disabled in settings. Calling API fails if specified custom field’s type is not a list: POST /api/v2/projects/:projectIdOrKey/customFields/:id/items
- ❌ DeleteCustomField: Deletes Custom Field: DELETE /api/v2/projects/:projectIdOrKey/customFields/:id
- ❌ DeleteListItemForListTypeCustomField: Deletes list item for list type custom field. Calling API fails if specified custom field’s type is not a list: DELETE /api/v2/projects/:projectIdOrKey/customFields/:id/items/:itemId
- ❌ UpdateListItemForListTypeCustomField: Updates list item for list type custom field. Calling API fails if specified custom field’s type is not a list: PATCH /api/v2/projects/:projectIdOrKey/customFields/:id/items/:itemId

### Issue Type
- ✅ UpdateIssueType: Updates information about Issue Type: PATCH /api/v2/projects/:projectIdOrKey/issueTypes/:id
- ✅ GetIssueTypeList: Returns list of Issue Types in the project: GET /api/v2/projects/:projectIdOrKey/issueTypes
- ✅ AddIssueType: Adds new Issue Type to the project: POST /api/v2/projects/:projectIdOrKey/issueTypes
- ✅ DeleteIssueType: Deletes Issue Type: DELETE /api/v2/projects/:projectIdOrKey/issueTypes/:id

### Status
- ✅ GetStatusListOfProject: Returns list of status in the project: GET /api/v2/projects/:projectIdOrKey/statuses
- ✅ AddStatus: Adds new Status to the project. You can create up to 8 custom statuses within a Project aside from the 4 default: POST /api/v2/projects/:projectIdOrKey/statuses
- ✅ UpdateOrderOfStatus: Updates order about Status: PATCH /api/v2/projects/:projectIdOrKey/statuses/updateDisplayOrder
- ✅ UpdateStatus: Updates information about Status: PATCH /api/v2/projects/:projectIdOrKey/statuses/:id
- ✅ DeleteStatus: Deletes Status: DELETE /api/v2/projects/:projectIdOrKey/statuses/:id

### Version/Milestone
- ✅ GetVersionMilestoneList: Returns list of Versions/Milestones in the project: GET /api/v2/projects/:projectIdOrKey/versions
- ✅ AddVersionMilestone: Adds new Version/Milestone to the project: POST /api/v2/projects/:projectIdOrKey/versions
- ✅ DeleteVersion: Deletes Version: DELETE /api/v2/projects/:projectIdOrKey/versions/:id
- ✅ UpdateVersionMilestone: Updates information about Version/Milestone: PATCH /api/v2/projects/:projectIdOrKey/versions/:id

### Webhook
- ❌ GetListOfWebhooks: Returns list of webhooks: GET /api/v2/projects/:projectIdOrKey/webhooks
- ❌ AddWebhook: Adds new webhook: POST /api/v2/projects/:projectIdOrKey/webhooks
- ❌ DeleteWebhook: Deletes webhook: DELETE /api/v2/projects/:projectIdOrKey/webhooks/:webhookId
- ❌ UpdateWebhook: Updates information about webhook: PATCH /api/v2/projects/:projectIdOrKey/webhooks/:webhookId
- ❌ GetWebhook: Returns information about webhook: GET /api/v2/projects/:projectIdOrKey/webhooks/:webhookId

### Shared Files
- ✅ GetFile: Downloads the file: GET /api/v2/projects/:projectIdOrKey/files/:sharedFileId
- ✅ GetListOfSharedFiles: Gets list of Shared Files: GET /api/v2/projects/:projectIdOrKey/files/metadata/:path

### Wikis
- ✅ UpdateWikiPage: Updates information about Wiki page: PATCH /api/v2/wikis/:wikiId
- ✅ GetWikiPageAttachment: Downloads Wiki page’s attachment file: GET /api/v2/wikis/:wikiId/attachments/:attachmentId
- ✅ GetWikiPageHistory: Returns history of Wiki page: GET /api/v2/wikis/:wikiId/history
- ✅ GetWikiPageList: Returns list of Wiki pages: GET /api/v2/wikis
- ✅ GetWikiPageTagList: Returns list of tags that are used in the project: GET /api/v2/wikis/tags
- ✅ GetWikiPage: Returns information about Wiki page: GET /api/v2/wikis/:wikiId
- ✅ GetListOfWikiAttachments: Gets list of files attached to Wiki: GET /api/v2/wikis/:wikiId/attachments
- ✅ RemoveWikiAttachment: Removes files attached to Wiki: DELETE /api/v2/wikis/:wikiId/attachments/:attachmentId
- ✅ DeleteWikiPage: Deletes Wiki page: DELETE /api/v2/wikis/:wikiId
- ✅ CountWikiPage: Returns number of Wiki pages: GET /api/v2/wikis/count
- ✅ AddWikiPage: Adds new Wiki page: POST /api/v2/wikis
- ✅ AttachFileToWiki: Attaches file to Wiki: POST /api/v2/wikis/:wikiId/attachments

#### Shared files for wiki
- ✅ GetListOfSharedFilesOnWiki: Returns the list of Shared Files on Wiki: GET /api/v2/wikis/:wikiId/sharedFiles
- ❌ LinkSharedFilesToWiki: Links Shared Files to Wiki: POST /api/v2/wikis/:wikiId/sharedFiles
- ❌ RemoveLinkToSharedFileFromWiki: Removes link to shared file from Wiki: DELETE /api/v2/wikis/:wikiId/sharedFiles/:id

### Git
- ✅ GetGitRepository: Returns Git repository: GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName
- ✅ GetListOfGitRepositories: Returns list of Git repositories: GET /api/v2/projects/:projectIdOrKey/git/repositories

### Pull Request
- ✅ UpdatePullRequestCommentInformation: Updates pull request comment information: PATCH /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number/comments/:commentId
- ✅ UpdatePullRequest: Updates pull requests: PATCH /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number
- ✅ GetNumberOfPullRequestComments: Returns number of comments on pull requests: GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number/comments/count
- ✅ GetNumberOfPullRequests: Returns number of pull requests: GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/count
- ✅ GetListOfPullRequestAttachment: Returns list of attached files on pull requests: GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number/attachments
- ✅ GetPullRequestComment: Returns list of pull request comments: GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number/comments
- ✅ GetPullRequestList: Returns list of pull requests: GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests
- ✅ GetPullRequest: Returns pull reuqest: GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number
- ✅ DownloadPullRequestAttachment: Downloads attached files on pull requests: GET /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number/attachments/:attachmentId
- ✅ DeletePullRequestAttachments: Deletes attached files on pull requests: DELETE /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number/attachments/:attachmentId
- ✅ AddPullRequestComment: Adds comments on pull requests: POST /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests/:number/comments
- ✅ AddPullRequest: Adds pull requests: POST /api/v2/projects/:projectIdOrKey/git/repositories/:repoIdOrName/pullRequests


### Activities
- ❌ GetActivity: Returns an activity: GET /api/v2/activities/:activityId
- ❌ GetUserRecentUpdates: Returns user’s recent updates: GET /api/v2/users/:userId/activities
- ❌ GetProjectRecentUpdates: Returns recent update in the project: GET /api/v2/projects/:projectIdOrKey/activities
- ❌ GetRecentUpdates: Returns recent updates in your space: GET /api/v2/space/activities

### Notifications
- ❌ CountNotification: Returns number of Notifications: GET /api/v2/notifications/count
- ❌ GetNotification: Returns own notifications: GET /api/v2/notifications
- ❌ ReadNotification: Changes notifications read: POST /api/v2/notifications/:id/markAsRead
- ❌ ResetUnreadNotificationCount: Resets unread Notification count: POST /api/v2/notifications/markAsRead

### Watchings
- ❌ UpdateWatching: Updates a watching. User can update own note: PATCH /api/v2/watchings/:watchingId
- ❌ AddWatching: Adds a watching. User can add a own watching: POST /api/v2/watchings
- ❌ CountWatching: Returns the number of your watching issues: GET /api/v2/users/:userId/watchings/count
- ❌ DeleteWatching: Deletes a own watching. User can delete a own watching: DELETE /api/v2/watchings/:watchingId
- ❌ GetWatchingList: Returns list of your watching issues: GET /api/v2/users/:userId/watchings
- ❌ GetWatching: Returns the information about a watching: GET /api/v2/watchings/:watchingId
- ❌ MarkWatchingAsRead: Mark a watching as read: POST /api/v2/watchings/:watchingId/markAsRead

### Star
- ❌ AddStar: Adds star: POST /api/v2/stars
- ❌ GetWikiPageStar: Returns list of stars received on the Wiki page: GET /api/v2/wikis/:wikiId/stars
- ❌ CountUserReceivedStars: Returns number of stars that user received: GET /api/v2/users/:userId/stars/count
- ❌ GetReceivedStarList: Returns the list of stars that user received: GET /api/v2/users/:userId/stars

### Document
- ✅ GetDocumentList: Returns list of document pages: GET /api/v2/documents
- ✅ GetDocumentTree: Retrieves the document tree: GET /api/v2/documents/tree
- ✅ GetDocument: Returns information about document page: GET /api/v2/documents/:documentId
- ✅ DownloadDocumentAttachment: Downloads document attachments: GET /api/v2/documents/:documentId/attachments/:attachmentId

### Users
- ✅ GetOwnUser: Returns own information about user: GET /api/v2/users/myself
- ✅ GetUserList: Returns list of users in your space: GET /api/v2/users
- ✅ GetUser: Returns information about user: GET /api/v2/users/:userId
- ✅ GetUserIcon: Downloads user icon: GET /api/v2/users/:userId/icon
- ❌ (Classic) AddUser: Adds new user to the space. “Project Administrator” cannot add “Admin” user. You can’t use this API at new plan space: POST /api/v2/users
- ❌ (Classic) UpdateUser: Updates information about user. You can’t use this API at new plan space: PATCH /api/v2/users/:userId
- ❌ (Classic) DeleteUser: Deletes user from the space. You can’t use this API at new plan space: DELETE /api/v2/users/:userId

### Teams
- ❌ GetTeamIcon: Downloads team icon: GET /api/v2/teams/:teamId/icon
- ❌ GetTeam: Returns information about team: GET /api/v2/teams/:teamId
- ❌ GetListOfTeams: Returns list of teams: GET /api/v2/teams
- ❌ (Classic) AddTeam: Adds new team. You can’t use this API at new plan space: POST /api/v2/teams
- ❌ (Classic) UpdateTeam: Updates information about team. You can’t use this API at new plan space: PATCH /api/v2/teams/:teamId
- ❌ (Classic) DeleteTeam: Deletes team. You can’t use this API at new plan space: DELETE /api/v2/teams/:teamId

### Recent
- ❌ AddRecentlyViewedIssue: Add an issue which the user viewed recently: POST /api/v2/users/myself/recentlyViewedIssues
- ❌ GetListOfRecentlyViewedIssues: Returns list of issues which the user viewed recently: GET /api/v2/users/myself/recentlyViewedIssues
- ❌ GetListOfRecentlyViewedProjects: Returns list of projects which the user viewed recently: GET /api/v2/users/myself/recentlyViewedProjects
- ❌ GetListOfRecentlyViewedWikis: Returns list of Wikis which the user viewed recently: GET /api/v2/users/myself/recentlyViewedWikis
- ❌ AddRecentlyViewedWiki: Add a wiki which the user viewed recently: POST /api/v2/users/myself/recentlyViewedWikis

### Rate Limit
- ❌ GetRateLimit: Return information about the rate limit currently applied to you: GET /api/v2/rateLimit

### OAuth 2.0 (Authentication)
- ❌ POST /api/v2/oauth2/token: Issues an access token.
