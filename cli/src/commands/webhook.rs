use backlog_api_client::{Webhook, client::BacklogApiClient};
use backlog_core::{
    ProjectIdOrKey,
    id::{ActivityTypeId, WebhookId},
};
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
    /// Get a specific webhook
    Get {
        /// Project ID or key
        #[arg(short, long)]
        project: String,

        /// Webhook ID
        #[arg(short, long)]
        webhook_id: u32,

        /// Output format
        #[arg(short, long, value_enum, default_value = "table")]
        format: OutputFormat,
    },
    /// Update webhook settings
    #[cfg(feature = "webhook_writable")]
    Update {
        /// Project ID or key
        #[arg(short, long)]
        project: String,

        /// Webhook ID to update
        #[arg(short = 'w', long)]
        webhook_id: u32,

        /// New webhook name
        #[arg(long)]
        name: Option<String>,

        /// New description
        #[arg(long)]
        description: Option<String>,

        /// New hook URL
        #[arg(long)]
        hook_url: Option<String>,

        /// Enable/disable all events (true/false)
        #[arg(long)]
        all_event: Option<bool>,

        /// Activity type IDs (comma-separated)
        #[arg(long, value_delimiter = ',')]
        activity_type_ids: Option<Vec<u32>>,
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
        WebhookCommands::Get {
            project,
            webhook_id,
            format,
        } => get_webhook(client, &project, webhook_id, format).await,
        #[cfg(feature = "webhook_writable")]
        WebhookCommands::Update {
            project,
            webhook_id,
            name,
            description,
            hook_url,
            all_event,
            activity_type_ids,
        } => {
            update_webhook(
                client,
                &project,
                webhook_id,
                name,
                description,
                hook_url,
                all_event,
                activity_type_ids,
            )
            .await
        }
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

async fn get_webhook(
    client: &BacklogApiClient,
    project: &str,
    webhook_id: u32,
    format: OutputFormat,
) -> Result<(), Box<dyn Error>> {
    let project_id_or_key = parse_project_id_or_key(project)?;
    let webhook = client
        .webhook()
        .get_webhook(project_id_or_key, WebhookId::new(webhook_id))
        .await?;

    match format {
        OutputFormat::Table => display_webhook_table(&webhook),
        OutputFormat::Json => display_webhook_json(&webhook)?,
        OutputFormat::Csv => display_webhook_csv(&webhook),
    }

    Ok(())
}

fn display_webhook_table(webhook: &Webhook) {
    let mut table = Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    table.add_row(row!["Field", "Value"]);
    table.add_row(row!["ID", webhook.id]);
    table.add_row(row!["Name", webhook.name]);
    table.add_row(row!["Description", webhook.description]);
    table.add_row(row!["Hook URL", webhook.hook_url]);
    table.add_row(row![
        "All Events",
        if webhook.all_event { "Yes" } else { "No" }
    ]);

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
    table.add_row(row!["Activity Types", activity_types]);

    table.add_row(row!["Created By", webhook.created_user.name]);
    table.add_row(row!["Created", webhook.created.format("%Y-%m-%d %H:%M:%S")]);
    table.add_row(row!["Updated By", webhook.updated_user.name]);
    table.add_row(row!["Updated", webhook.updated.format("%Y-%m-%d %H:%M:%S")]);

    table.printstd();
}

fn display_webhook_json(webhook: &Webhook) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(webhook)?;
    println!("{json}");
    Ok(())
}

fn display_webhook_csv(webhook: &Webhook) {
    println!(
        "id,name,description,hook_url,all_event,activity_type_ids,created_user,created,updated_user,updated"
    );

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

#[cfg(feature = "webhook_writable")]
#[allow(clippy::too_many_arguments)]
async fn update_webhook(
    client: &BacklogApiClient,
    project: &str,
    webhook_id: u32,
    name: Option<String>,
    description: Option<String>,
    hook_url: Option<String>,
    all_event: Option<bool>,
    activity_type_ids: Option<Vec<u32>>,
) -> Result<(), Box<dyn Error>> {
    // Check if at least one parameter is provided
    if name.is_none()
        && description.is_none()
        && hook_url.is_none()
        && all_event.is_none()
        && activity_type_ids.is_none()
    {
        return Err("At least one parameter must be provided to update".into());
    }

    let project_id_or_key = parse_project_id_or_key(project)?;

    let mut builder = client
        .webhook()
        .update_webhook(project_id_or_key, WebhookId::new(webhook_id));

    if let Some(name) = name {
        builder.name(name);
    }
    if let Some(description) = description {
        builder.description(description);
    }
    if let Some(hook_url) = hook_url {
        builder.hook_url(hook_url);
    }
    if let Some(all_event) = all_event {
        builder.all_event(all_event);
    }
    if let Some(ids) = activity_type_ids {
        let activity_ids: Vec<_> = ids.into_iter().map(ActivityTypeId::new).collect();
        builder.activity_type_ids(activity_ids);
    }

    let params = builder.build()?;
    let updated_webhook = client.webhook().execute_update_webhook(params).await?;

    println!("Webhook updated successfully!");
    display_webhook_table(&updated_webhook);

    Ok(())
}
