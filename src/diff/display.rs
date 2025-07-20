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
use std::{process::ExitCode, time::Duration};

use crate::{diff::SolverOutput, hhmmss::Hhmmss};

pub(crate) enum OutputMode {
    Ascii,
    Rich,
    None,
}

fn print_diff_ascii(
    added: &[&String],
    removed: &[&String],
    n_shared: usize,
    dur_left: Duration,
    dur_right: Duration,
) {
    if added.is_empty() && removed.is_empty() {
        println!("[OK] All {n_shared} solutions match.");
        println!("Left time: {} \x1b[0m", dur_left.as_millis());
        println!("Right time: {} \x1b[0m", dur_right.as_millis());
        return;
    }

    println!("[FAIL] Mismatch found:");
    for sol in removed {
        println!("- {}", sol);
    }
    for sol in added {
        println!("+ {}", sol);
    }
}

fn print_diff_rich(
    added: &[&String],
    removed: &[&String],
    n_shared: usize,
    dur_left: Duration,
    dur_right: Duration,
) {
    if added.is_empty() && removed.is_empty() {
        println!("\x1b[32m✅ All {n_shared} solutions \x1b[1mmatch\x1b[0m.\x1b[0m");
        println!(
            "\x1b[32m⌛ Duration of left run: {} \x1b[0m",
            dur_left.hhmmssxxx()
        );
        println!(
            "\x1b[32m⌛ Duration of right run: {} \x1b[0m",
            dur_right.hhmmssxxx()
        );
        return;
    }

    println!("\x1b[31m❌ Mismatch found:\x1b[0m");

    for sol in removed {
        println!("\x1b[31m- {}\x1b[0m", sol);
    }
    for sol in added {
        println!("\x1b[32m+ {}\x1b[0m", sol);
    }
}

pub(crate) enum SolverErrorType {
    Left,
    Right,
    Both,
}

fn report_timeout_ascii(result: SolverErrorType) {
    let result_str = match result {
        SolverErrorType::Left => "Left solver",
        SolverErrorType::Right => "Right solver",
        SolverErrorType::Both => "Both solvers",
    };
    println!("[TIMEOUT] {} timed out", result_str);
}

fn report_timeout_rich(result: SolverErrorType) {
    let result_str = match result {
        SolverErrorType::Left => "Left solver",
        SolverErrorType::Right => "Right solver",
        SolverErrorType::Both => "Both solvers",
    };
    println!("\x1b[33m⏳ \x1b[1m{}\x1b[0m timed out\x1b[0m", result_str);
}

fn report_timeout(result: SolverErrorType, output: OutputMode) {
    match output {
        OutputMode::Ascii => report_timeout_ascii(result),
        OutputMode::Rich => report_timeout_rich(result),
        OutputMode::None => {}
    };
}

pub(crate) enum CheckStatus {
    Success,
    Diff,
    CrashLeft,
    CrashRight,
    CrashBoth,
    TimeoutLeft,
    TimeoutRight,
    TimeoutBoth,
}

impl From<CheckStatus> for ExitCode {
    fn from(value: CheckStatus) -> Self {
        let code = match value {
            // Success correspond to the zero exit code, as per normal.
            CheckStatus::Success => 0,
            // Diff corresponds to setting all bits in the exit code.
            CheckStatus::Diff => u8::MAX,
            // If status is neither success nor difference, then:
            // - the third least significant bit is set to 1 for a timeout and to 0 for a crash,
            // - the second least significant bits is set to 1 if the event (timeout/crash)
            //   has happened to the right solver,
            // - and the least significant bit is set similarly for the left solver.
            CheckStatus::CrashLeft => 1,
            CheckStatus::CrashRight => 2,
            CheckStatus::CrashBoth => 3,
            CheckStatus::TimeoutLeft => 5,
            CheckStatus::TimeoutRight => 6,
            CheckStatus::TimeoutBoth => 7,
        };
        ExitCode::from(code)
    }
}

pub(crate) fn print_diff(
    set_left: &SolverOutput,
    set_right: &SolverOutput,
    output: OutputMode,
) -> CheckStatus {
    match (set_left, set_right) {
        (SolverOutput::Timeout, SolverOutput::Timeout) => {
            report_timeout(SolverErrorType::Both, output);
            CheckStatus::TimeoutBoth
        }
        (SolverOutput::Timeout, SolverOutput::Complete(_, _)) => {
            report_timeout(SolverErrorType::Left, output);
            CheckStatus::TimeoutLeft
        }
        (SolverOutput::Complete(_, _), SolverOutput::Timeout) => {
            report_timeout(SolverErrorType::Right, output);
            CheckStatus::TimeoutRight
        }
        (
            SolverOutput::Complete(set_left, dur_left),
            SolverOutput::Complete(set_right, dur_right),
        ) => {
            let added: Vec<_> = set_right.difference(set_left).collect();
            let removed: Vec<_> = set_left.difference(set_right).collect();
            let num_shared = set_left.intersection(set_right).count();

            match output {
                OutputMode::Ascii => print_diff_ascii(
                    added.as_slice(),
                    removed.as_slice(),
                    num_shared,
                    *dur_left,
                    *dur_right,
                ),
                OutputMode::Rich => print_diff_rich(
                    added.as_slice(),
                    removed.as_slice(),
                    num_shared,
                    *dur_left,
                    *dur_right,
                ),
                OutputMode::None => {}
            };

            if added.is_empty() && removed.is_empty() {
                CheckStatus::Success
            } else {
                CheckStatus::Diff
            }
        }
    }
}

pub(crate) fn report_crash(e: String, solvers: SolverErrorType, output: OutputMode) -> CheckStatus {
    let solvers_str = match solvers {
        SolverErrorType::Left => "left solver",
        SolverErrorType::Right => "right solver",
        SolverErrorType::Both => "both solvers",
    };
    match output {
        OutputMode::Ascii => {
            println!("[FAIL] Error running {solvers_str}: {e}");
        }
        OutputMode::Rich => {
            println!("\x1b[31m⛔ Error running \x1b[1m{solvers_str}\x1b[0m:\x1b[0m",);
            println!("{}", e);
        }
        OutputMode::None => {}
    };

    match solvers {
        SolverErrorType::Left => CheckStatus::CrashLeft,
        SolverErrorType::Right => CheckStatus::CrashRight,
        SolverErrorType::Both => CheckStatus::CrashBoth,
    }
}
