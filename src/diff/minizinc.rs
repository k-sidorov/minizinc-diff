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
use std::{
    collections::HashSet,
    path::PathBuf,
    process::Command,
    time::{Duration, Instant},
};

use crate::diff::{SolverOutput, SolverSpec};

pub(crate) fn run_solver(
    model: PathBuf,
    instance: PathBuf,
    solver: SolverSpec,
    timeout: Option<Duration>,
) -> Result<SolverOutput, String> {
    let start_time = Instant::now();
    let mut flags: Vec<String> = solver.flags;
    if let Some(timeout) = timeout {
        flags.push("-t".into());
        flags.push(timeout.as_millis().to_string());
    }

    let mut cmd = Command::new("minizinc");
    cmd.args(["-a", "--solver", &solver.tag]);
    cmd.args(flags);
    cmd.arg(model);
    cmd.arg(instance);

    let output = match cmd.output() {
        Ok(output) => output,
        Err(e) => return Err(e.to_string()),
    };
    let mzn_duration = start_time.elapsed();

    if !output.status.success() {
        Err(String::from_utf8_lossy(&output.stderr).into())
    } else {
        Ok(parse_solutions(
            &String::from_utf8_lossy(&output.stdout),
            mzn_duration,
        ))
    }
}

fn parse_solutions(output: &str, mzn_duration: Duration) -> SolverOutput {
    let mut solutions = HashSet::new();
    let mut current_sol = String::new();
    let mut is_complete = false;
    for line_raw in output.lines() {
        let line = line_raw.trim();
        if line.trim().is_empty() || line.starts_with('%') {
            continue;
        } else if line == "----------" {
            solutions.insert(current_sol.to_string());
            current_sol.clear();
        } else if line == "==========" {
            is_complete = true;
        } else {
            if !current_sol.is_empty() {
                current_sol.push('\n');
            }
            current_sol.push_str(line);
        }
    }
    if is_complete {
        SolverOutput::Complete(solutions, mzn_duration)
    } else {
        SolverOutput::Timeout
    }
}
