use backlog_api_client::TeamApi;
use backlog_core::{id::TeamId, identifier::Identifier};
use backlog_team::api::GetTeamParams;
use clap::Parser;

#[derive(Parser)]
pub struct TeamArgs {
    #[clap(subcommand)]
    pub command: TeamCommands,
}

#[derive(Parser)]
pub enum TeamCommands {
    /// Get team information by ID (requires administrator permission)
    Show {
        /// Team ID
        #[clap(name = "TEAM_ID")]
        team_id: u32,
    },
}

pub async fn handle_team_command(api: TeamApi, args: TeamArgs) {
    match args.command {
        TeamCommands::Show { team_id } => {
            let params = GetTeamParams {
                team_id: TeamId::new(team_id),
            };

            match api.get_team(params).await {
                Ok(team) => {
                    println!("Team Information:");
                    println!("================");
                    println!("ID: {}", team.id.value());
                    println!("Name: {}", team.name);
                    println!();

                    println!("Members ({}):", team.members.len());
                    for member in &team.members {
                        println!(
                            "  - {} ({}) - {}",
                            member.name,
                            member.user_id.as_deref().unwrap_or("N/A"),
                            member.mail_address
                        );
                    }
                    println!();

                    println!("Created by: {} at {}", team.created_user.name, team.created);
                    println!("Updated by: {} at {}", team.updated_user.name, team.updated);
                }
                Err(e) => {
                    eprintln!("❌ Failed to get team: {e}");
                    std::process::exit(1);
                }
            }
        }
    }
}
