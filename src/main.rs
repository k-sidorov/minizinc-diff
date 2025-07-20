// minizinc-diff
// Copyright (C) 2025 Konstantin Sidorov
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
mod diff;
mod generate;
pub(crate) mod hhmmss;

use clap::{Parser, Subcommand};
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "minizinc-diff", version, about = "Compare MiniZinc solver outputs", long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate MiniZinc models + data instances
    Generate(generate::GenArgs),
    /// Test a single model-instance pair with two solvers
    Diff(diff::DiffArgs),
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.cmd {
        Commands::Generate(args) => generate::run(args),
        Commands::Diff(args) => diff::run(args),
    }
}
