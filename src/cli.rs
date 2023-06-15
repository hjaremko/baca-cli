use clap::{Parser, Subcommand};

// todo: AppSettings::ArgRequiredElseHel
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    /// Sets the level of log verbosity
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Disable update check
    #[arg(short = 'u', long)]
    pub no_update: bool,

    /// Force update check
    #[arg(short = 'U', long)]
    pub force_update: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Initialise the current directory as a BaCa workspace
    Init {
        /// BaCa hostname, ex. mn2020
        #[arg(long)]
        host: Option<String>,

        /// BaCa login
        #[arg(long, short)]
        login: Option<String>,

        /// BaCa password
        #[arg(long, short)]
        password: Option<String>,
    },

    /// Get submit details
    Details { submit_id: u32 },

    /// Refresh session, use in case of a cookie expiration
    Refresh {},

    /// Print the last N (default 3) submits
    Log {
        #[arg(default_value_t = 3)]
        amount: u16,

        /// Print only the specified task's logs, use 'baca tasks' to see what ids are available
        #[arg(long, short, value_name = "TASK_ID")]
        task: Option<u32>,
    },

    /// Print available tasks
    Tasks {},

    /// Make a submit
    Submit {
        /// Task id, use 'baca tasks' to see what ids are available, overrides saved task id
        #[arg(long, short, value_name = "TASK_ID")]
        task: Option<u32>,

        /// A file to submit, overrides saved path
        #[arg(short, long, value_name = "FILE", num_args(0..))]
        file: Option<Vec<String>>,

        /// Task language. Please provide it exactly as is displayed on BaCa
        #[arg(short, long)]
        language: Option<String>,

        /// Submit input file under different name
        #[arg(short, long, value_name = "NEW_NAME")]
        rename: Option<String>,

        /// Save task config. If provided, future 'submit' calls won't require providing task config
        #[arg(short, long)]
        save: bool,

        /// Zip files to 'source.zip' before submitting, overrides saved config
        #[arg(short, long)]
        zip: bool,

        /// Do not ask for save
        #[arg(long)]
        no_save: bool,

        /// Remove main function before submitting. Takes effect only on C/C++ files
        #[arg(long)]
        no_main: bool,

        /// Transliterate Unicode strings in the input file into pure ASCII, effectively removing Polish diacritics
        #[arg(long)]
        no_polish: bool,

        /// Skip header verification
        #[arg(long)]
        skip_header: bool,

        #[command(subcommand)]
        command: Option<SubmitCommands>,
    },

    /// Print details of the last submit
    Last {
        /// Print only the specified task's logs, use 'baca tasks' to see what ids are available
        #[arg(long, short, value_name = "TASK_ID")]
        task: Option<u32>,
    },

    /// Open a editor to edit BaCa configuration
    Config {},

    /// Remove the whole `.baca` directory
    Clear {},
}

#[derive(Subcommand)]
pub(crate) enum SubmitCommands {
    /// Open a editor to edit submit config
    Config {},

    /// Clear saved submit config
    Clear {},
}
