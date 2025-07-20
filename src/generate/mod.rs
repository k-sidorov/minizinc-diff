mod market_split;
mod single;

use std::{
    fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

use clap::Args;

#[derive(Debug, Clone)]
pub(crate) enum InstanceShape {
    MarketSplit {
        n_samples: usize,
        n_constraints: usize,
        n_variables: usize,
        coef_range: u32,
    },
    SingleConstraint,
}

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
#[derive(Args, Debug)]
pub(crate) struct GenArgs {
    /// Output directory for generated models and instances
    output_dir: PathBuf,
    #[arg(value_parser = parse_instance_shape, num_args = 1..)]
    /// Instance generator parameters
    instance_shape: Vec<InstanceShape>,
}

fn parse_instance_shape(s: &str) -> Result<InstanceShape, String> {
    let parts: Vec<&str> = s.split(":").collect();
    assert!(!parts.is_empty());

    match parts[0] {
        "single" => Ok(InstanceShape::SingleConstraint),
        "market-split" => {
            let slice = &parts[1..];
            let mut n_samples = 10;
            let mut n_constraints = 2;
            let mut n_variables = 10;
            let mut coef_range = 100;
            if let Some(n_runs_str) = slice.get(0) {
                if let Ok(n_runs) = n_runs_str.parse() {
                    n_samples = n_runs;
                } else {
                    return Err(format!(
                        "Failed to parse `{}` as part of the market split configuration `{}`",
                        n_runs_str, s
                    ));
                }
            }
            if let Some(n_cons_str) = slice.get(1) {
                if let Ok(n_cons) = n_cons_str.parse() {
                    n_constraints = n_cons;
                    n_variables = if n_constraints <= 1 {
                        10
                    } else {
                        10 * (n_constraints - 1)
                    };
                } else {
                    return Err(format!(
                        "Failed to parse `{}` as part of the market split configuration `{}`",
                        n_cons_str, s
                    ));
                }
            }
            if let Some(n_var_str) = slice.get(2) {
                if let Ok(n_vars) = n_var_str.parse() {
                    n_variables = n_vars;
                } else {
                    return Err(format!(
                        "Failed to parse `{}` as part of the market split configuration `{}`",
                        n_var_str, s
                    ));
                }
            }
            if let Some(coef_str) = slice.get(3) {
                if let Ok(coef) = coef_str.parse() {
                    coef_range = coef;
                } else {
                    return Err(format!(
                        "Failed to parse `{}` as part of the market split configuration `{}`",
                        coef_str, s
                    ));
                }
            }
            Ok(InstanceShape::MarketSplit {
                n_samples,
                n_constraints,
                n_variables,
                coef_range,
            })
        }
        _ => Err(format!("Unknown instance generator `{}`", parts[0])),
    }
}

pub(crate) fn run(args: GenArgs) -> ExitCode {
    let GenArgs {
        output_dir,
        instance_shape,
    } = args;
    let out = Path::new(&output_dir);
    fs::create_dir_all(out).unwrap();
    for generator in instance_shape {
        let res = match generator {
            InstanceShape::MarketSplit {
                n_samples,
                n_constraints,
                n_variables,
                coef_range,
            } => market_split::run(out, n_samples, n_constraints, n_variables, coef_range),
            InstanceShape::SingleConstraint => single::run(out),
        };
        if res.is_err() {
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}
