mod ffmpeg_convert_folder;
mod random;

#[cfg(feature="cmds_web")]
mod whoami;

use clap::{Parser, Subcommand}; 

pub type ExitCode = i32;
pub const EXIT_CODE_SUCCESS: ExitCode = 0;

fn main() {
    // Parse command line arguments
    let args = RootArgs::parse();

    // Verbose information
    let config = OutputConfig {
        verbose: args.verbose,
    };

    // Let the subcommand take it from here
    let code = match args.subcommand {
        Subcommands::FfmpegConvertFolder(args) => ffmpeg_convert_folder::run(config, args),
        Subcommands::Random(args) => random::run(config, args),

        #[cfg(feature="cmds_web")]
        Subcommands::WhoAmI => whoami::run(),
    };

    // We're done, exit with a code.
    std::process::exit(code);
}

#[derive(Parser)]
struct RootArgs {
    #[arg(short, default_value_t=false)]
    pub verbose: bool,

    #[command(subcommand)]
    pub subcommand: Subcommands,
}

struct OutputConfig {
    verbose: bool,
}

#[derive(Subcommand)]
enum Subcommands {
    /// Converts all files from folder A to a specified format and places them in folder B.
    FfmpegConvertFolder(ffmpeg_convert_folder::CommandArgs),

    /// Random generation utilities, based on your system's random number generator.
    Random(random::CommandArgs),

    /// Returns the IP address this machine uses to connect to the Internet.
    #[cfg(feature="cmds_web")]
    WhoAmI,
}