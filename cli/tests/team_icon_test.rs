use clap::Parser;

#[test]
fn test_team_icon_command_parsing() {
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
        Icon {
            team_id: u32,
            #[clap(short, long)]
            output: String,
        },
    }

    // Test basic icon command
    let args =
        Cli::try_parse_from(["prog", "team", "icon", "123", "--output", "team_icon.png"]).unwrap();
    match args.command {
        Commands::Team { command } => match command {
            TeamCommands::Icon { team_id, output } => {
                assert_eq!(team_id, 123);
                assert_eq!(output, "team_icon.png");
            }
        },
    }

    // Test short form
    let args = Cli::try_parse_from(["prog", "team", "icon", "456", "-o", "icon.gif"]).unwrap();
    match args.command {
        Commands::Team { command } => match command {
            TeamCommands::Icon { team_id, output } => {
                assert_eq!(team_id, 456);
                assert_eq!(output, "icon.gif");
            }
        },
    }
}

#[test]
fn test_team_icon_missing_args() {
    #[derive(Parser, Debug)]
    struct Cli {
        #[clap(subcommand)]
        command: TeamCommands,
    }

    #[derive(Parser, Debug)]
    enum TeamCommands {
        Icon {
            team_id: u32,
            #[clap(short, long)]
            output: String,
        },
    }

    // Missing output should fail
    let result = Cli::try_parse_from(["prog", "icon", "123"]);
    assert!(result.is_err());

    // Missing team_id should fail
    let result = Cli::try_parse_from(["prog", "icon", "--output", "icon.png"]);
    assert!(result.is_err());

    // No args should fail
    let result = Cli::try_parse_from(["prog", "icon"]);
    assert!(result.is_err());
}
