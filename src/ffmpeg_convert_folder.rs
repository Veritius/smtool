use std::{collections::VecDeque, ffi::{OsStr, OsString}, iter::repeat_with, path::{Path, PathBuf}, process::{Command, ExitStatus}, time::Instant};
use clap::Parser;
use crate::{ExitCode, EXIT_CODE_SUCCESS};

#[derive(Parser, Debug)]
#[command(version)]
pub(super) struct CommandArgs {
    /// The directory to search for files in.
    pub input_directory: PathBuf,

    /// The directory where processed files will be placed
    pub output_directory: PathBuf,

    /// The format to output files in
    pub output_extension: OsString,

    /// Traverses symbolic links while recursively finding files.
    #[arg(long = "access_symlinks", default_value_t=false)]
    pub access_symlinks: bool,

    /// Includes hidden files and reads hidden folders.
    #[arg(long = "access_hidden", default_value_t=false)]
    pub access_hidden: bool,
}

pub(super) fn run(args: CommandArgs) -> ExitCode {
    // Check FFmpeg exists
    let mut command = Command::new("ffmpeg");
    command.arg("-version");
    if command.output().is_err() {
        println!("FFmpeg not detected, check your PATH");
        return -1; // Terminate program
    }

    // Check the path is valid
    if !args.input_directory.exists() || !args.input_directory.is_dir() {
        println!("The search path did not exist or wasn't a directory");
    }

    // Create WalkDir iterator to find all files recursively
    let walker = walkdir::WalkDir::new(&args.input_directory)
        .follow_links(args.access_symlinks);

    // List of target files
    let mut unprocessed_files = VecDeque::new();

    struct TargetFile {
        path: PathBuf,
        scramble: bool,
    }

    // Data from traversing the filesystem
    let mut file_access_successes: u64 = 0;
    let mut file_access_failures: u64 = 0;

    // Access all files
    for file in walker {
        // Check we can access each file
        match file {
            Ok(file) => {
                // Check metadata to make sure it's a file
                if !file.metadata().unwrap().is_file() { continue }

                // Create the path of the output file
                // We use this to check if we need to rename it to prevent collisions
                let theoretical_file = build_output_path(
                    &args.output_directory,
                    file.path().file_name().unwrap(),
                    false,
                    &args.output_extension,
                );

                // Add the path of the file to a queue
                unprocessed_files.push_back(TargetFile {
                    path: file.path().to_path_buf(),
                    scramble: theoretical_file.exists(),
                });

                file_access_successes += 1;
            },
            Err(_) => {
                file_access_failures += 1;
            },
        }
    }

    // Early return if there's no files
    match (file_access_successes > 0, file_access_failures > 0) {
        (true, false) => {
            println!("Processing {file_access_successes} files");
        },
        (false, true) => {
            println!("All {file_access_failures} discovered files were inaccessible");
            return -2
        },
        (true, true) => {
            println!("Found {file_access_successes} usable files and {file_access_failures} inaccessible files");
        },
        (false, false) => {
            println!("Didn't discover any files in the input directory");
            return -3
        },
    }

    // Record the moment we start processing
    let started = Instant::now();

    // Process all the files
    let ffmpeg_path = OsString::from("ffmpeg");
    for file in unprocessed_files.drain(..) {
        // Build the path for the output file
        let output_path = build_output_path(
            &args.output_directory,
            file.path.file_name().unwrap(),
            file.scramble,
            &args.output_extension,
        );

        // Run ffmpeg :)
        let outcome = ffmpeg_convert(
            &ffmpeg_path,
            &file.path,
            &output_path
        );

        // Write the outcome to stdout
        println!("{}", display_conversion_outcome(
            &file.path,
            &output_path,
            &outcome
        ));
    }

    // Log how long we've spent on this to the console
    let time_spent = Instant::now().saturating_duration_since(started);
    println!("Finished in {} minutes, {} seconds, and {} milliseconds",
        time_spent.as_secs() / 60,
        time_spent.as_secs() % 60,
        time_spent.as_millis() % 1000
    );

    return EXIT_CODE_SUCCESS
}

fn build_output_path(
    directory: &Path,
    filename: &OsStr,
    with_rand: bool,
    extension: &OsStr,
) -> PathBuf {
    // Create the individual file's name
    let mut file_id = OsString::new();
    file_id.push(filename);

    // Append a random value to the file name
    // Used for when there's name collisions
    if with_rand {
        file_id.push(" (");
        file_id.push(
            repeat_with(fastrand::alphanumeric)
            .take(16)
            .collect::<String>()
        );
        file_id.push(")");
    }

    // Adds the extension
    file_id.push(".");
    file_id.push(extension);

    // Builds the full ptah
    let mut path = PathBuf::new();
    path.push(directory);
    path.push(file_id);

    return path;
}

fn ffmpeg_convert(
    ffmpeg_path: &OsStr,
    input_file: &Path,
    output_file: &Path
) -> ConversionOutcome {
    // Run FFmpeg
    let output = Command::new(ffmpeg_path)
        .arg("-i")
        .arg(input_file)
        .arg(output_file)
        .output()
        .unwrap();

    // Return
    match output.status.success() {
        true => ConversionOutcome::Success,
        false => ConversionOutcome::Failure {
            status: output.status
        },
    }
}

enum ConversionOutcome {
    Success,
    Failure {
        status: ExitStatus
    },
}

fn display_conversion_outcome(
    input_path: &Path,
    output_path: &Path,
    outcome: &ConversionOutcome,
) -> String {
    match outcome {
        ConversionOutcome::Success => {
            format!("Successfully converted {input_path:?} to {output_path:?}")
        },
        ConversionOutcome::Failure { status } => {
            format!("Failed to convert {input_path:?} to {output_path:?}: {status}")
        },
    }
}