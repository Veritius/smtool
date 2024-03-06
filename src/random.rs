use std::io::Write;

use clap::{Parser, Subcommand};
use crate::{ExitCode, EXIT_CODE_SUCCESS};

const EXIT_CODE_BAD_BASE: ExitCode = -2;

pub(super) fn run(args: CommandArgs) -> ExitCode {
    let mut rng = fastrand::Rng::default();

    match args.target {
        RandomTarget::Integer { min, max } => {
            let min = min.unwrap_or(i128::MIN);
            let max = max.unwrap_or(i128::MAX);
            let val = rng.i128(0..=max);
            println!("{val}");
        },

        RandomTarget::Digits { length, base } => {
            if length == 0 { return EXIT_CODE_SUCCESS }

            // Verify the base is acceptable
            let base = match base {
                None => 10,
                Some(1..=36) => {
                    base.unwrap()
                },
                Some(0) => {
                    println!("Numerical base was zero");
                    return EXIT_CODE_BAD_BASE;
                },
                Some(_) => {
                    println!("Numerical base was above 36");
                    return EXIT_CODE_BAD_BASE;
                }
            };

            // Fill a string with random digits
            let string = (0..length)
                .into_iter()
                .map(|_| rng.digit(base))
                .collect::<String>();

            // Print the string to stdout
            println!("{string}");
        }
    }

    EXIT_CODE_SUCCESS
}

#[derive(Parser, Debug)]
#[command(version)]
pub(super) struct CommandArgs {
    #[command(subcommand)]
    target: RandomTarget,
}

#[derive(Subcommand, Debug)]
enum RandomTarget {
    /// Generates a random signed integer.
    Integer {
        /// The maximum value that can be returned.
        #[arg(long)]
        min: Option<i128>,

        /// The maximum value that can be returned.
        #[arg(long)]
        max: Option<i128>,
    },
    
    /// Generates random digits with a given length.
    Digits {
        /// The amount of digits to generate.
        length: usize,

        /// The numerical base of the digits.
        /// Cannot be zero or greater than 36.
        /// Defaults to base 10.
        #[arg(long)]
        base: Option<u32>,
    }
}