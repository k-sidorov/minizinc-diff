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
mod display;
mod minizinc;

use std::{
    collections::HashSet, io::IsTerminal, path::PathBuf, sync::mpsc, thread, time::Duration,
};

use clap::Args;

use crate::diff::{
    display::{OutputMode, SolverErrorType, print_diff, report_crash},
    minizinc::run_solver,
};

#[derive(Args, Debug)]
pub(crate) struct DiffArgs {
    /// MiniZinc model file
    model: PathBuf,
    /// MiniZinc data file
    instance: PathBuf,
    /// Solver tag used for the first solver run (displayed on the left), followed by --flags if needed
    #[arg(value_parser = parse_solver_spec)]
    solver_left: SolverSpec,
    /// Solver tag used for the second solver run (displayed on the right), followed by --flags if needed
    #[arg(value_parser = parse_solver_spec)]
    solver_right: SolverSpec,
    /// Timeout for both runs
    #[arg(short, long)]
    timeout_secs: Option<u64>,
    /// Disables all output
    #[arg(short, long)]
    quiet: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct SolverSpec {
    pub(crate) tag: String,
    pub(crate) flags: Vec<String>,
}

fn parse_solver_spec(s: &str) -> Result<SolverSpec, String> {
    let parts: Vec<&str> = s.splitn(2, ":").collect();
    let tag = parts[0].to_string();
    let flags = if parts.len() > 1 {
        shell_words::split(parts[1]).map_err(|e| e.to_string())?
    } else {
        vec![]
    };
    Ok(SolverSpec { tag, flags })
}

pub(crate) enum SolverOutput {
    Complete(HashSet<String>, Duration),
    Timeout,
}

pub(crate) fn run(args: DiffArgs) -> std::process::ExitCode {
    let rich_output = std::io::stdout().is_terminal();
    let output_mode = if args.quiet {
        OutputMode::None
    } else if rich_output {
        OutputMode::Rich
    } else {
        OutputMode::Ascii
    };
    let timeout = args.timeout_secs.map(Duration::from_secs);
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    let model_left = args.model.clone();
    let model_right = args.model.clone();
    let instance_left = args.instance.clone();
    let instance_right = args.instance.clone();

    thread::spawn(move || {
        let res = run_solver(model_left, instance_left, args.solver_left, timeout);
        tx1.send(res).unwrap();
    });

    thread::spawn(move || {
        let res = run_solver(model_right, instance_right, args.solver_right, timeout);
        tx2.send(res).unwrap();
    });

    let res1 = rx1.recv().unwrap();
    let res2 = rx2.recv().unwrap();

    let status = match (res1, res2) {
        (Ok(set1), Ok(set2)) => print_diff(&set1, &set2, output_mode),
        (Err(e), Ok(_)) => report_crash(e, SolverErrorType::Left, output_mode),
        (Ok(_), Err(e)) => report_crash(e, SolverErrorType::Right, output_mode),
        (Err(e_left), Err(e_right)) => report_crash(
            format!("\nLeft: {e_left}\nRight: {e_right}").to_string(),
            SolverErrorType::Both,
            output_mode,
        ),
    };
    status.into()
}
