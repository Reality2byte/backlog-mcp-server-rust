use clap::Parser;

#[test]
fn test_team_list_command_parsing() {
    #[derive(Parser, Debug)]
    struct Cli {
        #[clap(subcommand)]
        command: Commands,
    }

    #[derive(Parser, Debug)]
    enum Commands {
        Team {
            #[clap(subcommand)]
            command: TeamCommands,
        },
    }

    #[derive(Parser, Debug)]
    enum TeamCommands {
        List {
            #[clap(short, long)]
            order: Option<String>,
            #[clap(short = 's', long)]
            offset: Option<u32>,
            #[clap(short, long)]
            count: Option<u32>,
            #[clap(short, long)]
            format: Option<String>,
        },
    }

    // Test basic list command
    let args = Cli::try_parse_from(["prog", "team", "list"]).unwrap();
    match args.command {
        Commands::Team { command } => match command {
            TeamCommands::List {
                order,
                offset,
                count,
                format,
            } => {
                assert_eq!(order, None);
                assert_eq!(offset, None);
                assert_eq!(count, None);
                assert_eq!(format, None);
            }
        },
    }

    // Test with all parameters
    let args = Cli::try_parse_from([
        "prog", "team", "list", "--order", "asc", "--offset", "10", "--count", "50", "--format",
        "json",
    ])
    .unwrap();
    match args.command {
        Commands::Team { command } => match command {
            TeamCommands::List {
                order,
                offset,
                count,
                format,
            } => {
                assert_eq!(order, Some("asc".to_string()));
                assert_eq!(offset, Some(10));
                assert_eq!(count, Some(50));
                assert_eq!(format, Some("json".to_string()));
            }
        },
    }
}

#[test]
fn test_team_list_with_pagination() {
    #[derive(Parser, Debug)]
    struct Cli {
        #[clap(subcommand)]
        command: TeamCommands,
    }

    #[derive(Parser, Debug)]
    enum TeamCommands {
        List {
            #[clap(short = 's', long)]
            offset: Option<u32>,
            #[clap(short, long)]
            count: Option<u32>,
        },
    }

    // Test offset and count
    let args = Cli::try_parse_from(["prog", "list", "-s", "20", "-c", "30"]).unwrap();
    match args.command {
        TeamCommands::List { offset, count } => {
            assert_eq!(offset, Some(20));
            assert_eq!(count, Some(30));
        }
    }

    // Test long form
    let args = Cli::try_parse_from(["prog", "list", "--offset", "100", "--count", "10"]).unwrap();
    match args.command {
        TeamCommands::List { offset, count } => {
            assert_eq!(offset, Some(100));
            assert_eq!(count, Some(10));
        }
    }
}

#[test]
fn test_team_list_alias() {
    #[derive(Parser, Debug)]
    struct Cli {
        #[clap(subcommand)]
        command: TeamCommands,
    }

    #[derive(Parser, Debug)]
    enum TeamCommands {
        #[clap(alias = "ls")]
        List,
    }

    // Test alias 'ls'
    let args = Cli::try_parse_from(["prog", "ls"]).unwrap();
    match args.command {
        TeamCommands::List => {
            // Command parsed successfully
        }
    }
}
