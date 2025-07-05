use backlog_api_client::{ListTeamsOrder, ListTeamsParams, ListTeamsResponse, TeamApi};
use backlog_core::{id::TeamId, identifier::Identifier};
use backlog_team::api::{GetTeamIconParams, GetTeamParams};
use clap::{Parser, ValueEnum};
use std::path::PathBuf;

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
    /// List all teams (requires administrator or project administrator permission)
    #[clap(alias = "ls")]
    List {
        /// Sort order
        #[clap(short, long, value_enum)]
        order: Option<CliListTeamsOrder>,

        /// Number of items to skip
        #[clap(short = 's', long)]
        offset: Option<u32>,

        /// Number of items to retrieve (1-100)
        #[clap(short, long)]
        count: Option<u32>,

        /// Output format
        #[clap(short, long, value_enum, default_value = "table")]
        format: OutputFormat,
    },
    /// Download team icon image
    Icon {
        /// Team ID
        #[clap(name = "TEAM_ID")]
        team_id: u32,

        /// Output file path
        #[clap(short, long)]
        output: PathBuf,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum CliListTeamsOrder {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
    Csv,
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
        TeamCommands::List {
            order,
            offset,
            count,
            format,
        } => {
            let params = ListTeamsParams {
                order: order.map(|o| match o {
                    CliListTeamsOrder::Asc => ListTeamsOrder::Asc,
                    CliListTeamsOrder::Desc => ListTeamsOrder::Desc,
                }),
                offset,
                count,
            };

            match api.list_teams(params).await {
                Ok(teams) => match format {
                    OutputFormat::Table => display_teams_table(&teams),
                    OutputFormat::Json => display_teams_json(&teams),
                    OutputFormat::Csv => display_teams_csv(&teams),
                },
                Err(e) => {
                    eprintln!("❌ Failed to list teams: {e}");
                    std::process::exit(1);
                }
            }
        }
        TeamCommands::Icon { team_id, output } => {
            let params = GetTeamIconParams {
                team_id: TeamId::new(team_id),
            };

            match api.get_team_icon(params).await {
                Ok(icon) => match std::fs::write(&output, &icon.bytes) {
                    Ok(_) => {
                        println!("✅ Team icon saved to: {}", output.display());
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to save team icon: {e}");
                        std::process::exit(1);
                    }
                },
                Err(e) => {
                    eprintln!("❌ Failed to download team icon: {e}");
                    std::process::exit(1);
                }
            }
        }
    }
}

fn display_teams_table(teams: &ListTeamsResponse) {
    use prettytable::{Cell, Row, Table, format};

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(Row::new(vec![
        Cell::new("ID"),
        Cell::new("Name"),
        Cell::new("Members"),
        Cell::new("Created"),
        Cell::new("Updated"),
    ]));

    for team in teams {
        table.add_row(Row::new(vec![
            Cell::new(&team.team.id.value().to_string()),
            Cell::new(&team.team.name),
            Cell::new(&team.team.members.len().to_string()),
            Cell::new(&team.team.created.format("%Y-%m-%d %H:%M").to_string()),
            Cell::new(&team.team.updated.format("%Y-%m-%d %H:%M").to_string()),
        ]));
    }

    if teams.is_empty() {
        println!("No teams found.");
    } else {
        table.printstd();
        println!("\nTotal: {} teams", teams.len());
    }
}

fn display_teams_json(teams: &ListTeamsResponse) {
    match serde_json::to_string_pretty(teams) {
        Ok(json) => println!("{json}"),
        Err(e) => {
            eprintln!("❌ Failed to serialize teams to JSON: {e}");
            std::process::exit(1);
        }
    }
}

fn display_teams_csv(teams: &ListTeamsResponse) {
    println!("id,name,member_count,created,updated");
    for team in teams {
        println!(
            "{},{},{},{},{}",
            team.team.id.value(),
            escape_csv(&team.team.name),
            team.team.members.len(),
            team.team.created.format("%Y-%m-%d %H:%M:%S"),
            team.team.updated.format("%Y-%m-%d %H:%M:%S")
        );
    }
}

fn escape_csv(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}
