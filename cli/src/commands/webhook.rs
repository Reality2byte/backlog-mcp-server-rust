use backlog_api_client::{Webhook, client::BacklogApiClient};
use backlog_core::ProjectIdOrKey;
use clap::{Parser, Subcommand, ValueEnum};
use prettytable::{Cell, Row, Table, row};
use std::error::Error;

#[derive(Parser)]
pub struct WebhookArgs {
    #[clap(subcommand)]
    pub command: WebhookCommands,
}

#[derive(Subcommand)]
pub enum WebhookCommands {
    /// List webhooks for a project
    #[clap(alias = "ls")]
    List {
        /// Project ID or key
        #[arg(short, long)]
        project: String,

        /// Output format
        #[arg(short, long, value_enum, default_value = "table")]
        format: OutputFormat,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
    Csv,
}

pub async fn execute(client: &BacklogApiClient, args: WebhookArgs) -> Result<(), Box<dyn Error>> {
    match args.command {
        WebhookCommands::List { project, format } => list_webhooks(client, &project, format).await,
    }
}

async fn list_webhooks(
    client: &BacklogApiClient,
    project: &str,
    format: OutputFormat,
) -> Result<(), Box<dyn Error>> {
    let project_id_or_key = parse_project_id_or_key(project)?;
    let webhooks = client.webhook().get_webhook_list(project_id_or_key).await?;

    match format {
        OutputFormat::Table => display_webhooks_table(&webhooks),
        OutputFormat::Json => display_webhooks_json(&webhooks)?,
        OutputFormat::Csv => display_webhooks_csv(&webhooks),
    }

    Ok(())
}

fn parse_project_id_or_key(project: &str) -> Result<ProjectIdOrKey, Box<dyn Error>> {
    // Try to parse as numeric ID first
    if let Ok(id) = project.parse::<u32>() {
        Ok(ProjectIdOrKey::from(backlog_core::id::ProjectId::new(id)))
    } else {
        // Otherwise treat as project key
        let key = project
            .parse::<backlog_core::ProjectKey>()
            .map_err(|e| format!("Invalid project key '{project}': {e}"))?;
        Ok(ProjectIdOrKey::from(key))
    }
}

fn display_webhooks_table(webhooks: &[Webhook]) {
    if webhooks.is_empty() {
        println!("No webhooks found.");
        return;
    }

    let mut table = Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(row![
        "ID",
        "Name",
        "Hook URL",
        "All Events",
        "Activity Types"
    ]);

    for webhook in webhooks {
        let activity_types = if webhook.all_event {
            "All".to_string()
        } else if webhook.activity_type_ids.is_empty() {
            "None".to_string()
        } else {
            webhook
                .activity_type_ids
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        };

        table.add_row(Row::new(vec![
            Cell::new(&webhook.id.to_string()),
            Cell::new(&webhook.name),
            Cell::new(&webhook.hook_url),
            Cell::new(if webhook.all_event { "Yes" } else { "No" }),
            Cell::new(&activity_types),
        ]));
    }

    table.printstd();
    println!("\nTotal: {} webhook(s)", webhooks.len());
}

fn display_webhooks_json(webhooks: &[Webhook]) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(webhooks)?;
    println!("{json}");
    Ok(())
}

fn display_webhooks_csv(webhooks: &[Webhook]) {
    println!(
        "id,name,description,hook_url,all_event,activity_type_ids,created_user,created,updated_user,updated"
    );

    for webhook in webhooks {
        println!(
            "{},{},{},{},{},{},{},{},{},{}",
            webhook.id,
            escape_csv(&webhook.name),
            escape_csv(&webhook.description),
            escape_csv(&webhook.hook_url),
            webhook.all_event,
            webhook
                .activity_type_ids
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(";"),
            webhook.created_user.name,
            webhook.created.format("%Y-%m-%d %H:%M:%S"),
            webhook.updated_user.name,
            webhook.updated.format("%Y-%m-%d %H:%M:%S"),
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
