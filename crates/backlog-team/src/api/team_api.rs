use crate::api::{GetTeamParams, GetTeamResponse, ListTeamsParams, ListTeamsResponse};
use backlog_api_core::Result;
use client::Client;

/// Team API client for interacting with Backlog team endpoints.
pub struct TeamApi(Client);

impl TeamApi {
    /// Creates a new `TeamApi` instance.
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    /// Gets a team by its ID.
    ///
    /// This API requires administrator permission.
    ///
    /// # Arguments
    ///
    /// * `params` - Parameters for getting a team
    ///
    /// # Returns
    ///
    /// Returns the team information if successful.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * The user doesn't have administrator permission (403)
    /// * The team is not found (404)
    /// * The API request fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// use backlog_team::api::{TeamApi, GetTeamParams};
    /// use backlog_core::id::TeamId;
    ///
    /// # async fn example(api: TeamApi) -> Result<(), Box<dyn std::error::Error>> {
    /// let params = GetTeamParams {
    ///     team_id: TeamId::new(123),
    /// };
    /// let team = api.get_team(params).await?;
    /// println!("Team name: {}", team.name);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Corresponds to `GET /api/v2/teams/:teamId`.
    pub async fn get_team(&self, params: GetTeamParams) -> Result<GetTeamResponse> {
        self.0.execute(params).await
    }

    /// Lists teams in the space.
    ///
    /// This API requires administrator or project administrator permission.
    ///
    /// # Arguments
    ///
    /// * `params` - Parameters for listing teams
    ///
    /// # Returns
    ///
    /// Returns a list of teams if successful.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * The user doesn't have administrator or project administrator permission (403)
    /// * The API request fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// use backlog_team::api::{TeamApi, ListTeamsParams, ListTeamsOrder};
    ///
    /// # async fn example(api: TeamApi) -> Result<(), Box<dyn std::error::Error>> {
    /// let params = ListTeamsParams {
    ///     order: Some(ListTeamsOrder::Asc),
    ///     count: Some(50),
    ///     ..Default::default()
    /// };
    /// let teams = api.list_teams(params).await?;
    /// for team in teams {
    ///     println!("Team: {} (ID: {})", team.team.name, team.team.id);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Corresponds to `GET /api/v2/teams`.
    pub async fn list_teams(&self, params: ListTeamsParams) -> Result<ListTeamsResponse> {
        self.0.execute(params).await
    }
}
