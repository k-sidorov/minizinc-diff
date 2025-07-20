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
use rand::prelude::*;
use std::{
    fs,
    path::{Path, PathBuf},
};
fn generate_model(path: &Path) -> std::io::Result<PathBuf> {
    let model_dir = path.join("market_split");
    fs::create_dir_all(&model_dir)?;
    let model_path = model_dir.join("model.mzn");

    let model_content = r#"
int: m;
int: n;

array[1..m, 1..n+1] of int: a;

array[1..n] of var 0..1: x;

constraint
    forall(i in 1..m)(
        sum(j in 1..n)( a[i,j]*x[j] ) = a[i,n+1]
    );

solve :: int_search(x, input_order, indomain_min, complete)
  satisfy;

output [show(x)];
"#;

    fs::write(&model_path, model_content).unwrap();
    Ok(model_dir)
}

fn generate_data(
    path: &Path,
    run_index: usize,
    n_constraints: usize,
    n_variables: usize,
    coef_range: u32,
) -> std::io::Result<()> {
    let data_filename = format!(
        "{}_{}_{}_{}.dzn",
        n_constraints, n_variables, coef_range, run_index
    );
    let data_path = path.join(&data_filename);
    let mut data_content = format!("m = {n_constraints};\nn = {n_variables};\na=[|");
    let mut rng = SmallRng::seed_from_u64(run_index as u64);
    for _ in 0..n_constraints {
        data_content.push('\n');
        let mut total = 0;
        for _ in 0..n_variables {
            let coef = rng.random_range(1..=coef_range);
            data_content.push_str(format!("{coef}, ").as_str());
            total += coef;
        }
        data_content.push_str(format!("{} |", total / 2).as_str());
    }
    data_content.push_str("];\n");

    fs::write(&data_path, data_content)?;
    Ok(())
}

pub(crate) fn run(
    out: &Path,
    n_runs: usize,
    n_constraints: usize,
    n_variables: usize,
    coef_range: u32,
) -> Result<(), ()> {
    let path = generate_model(out).map_err(|e| {
        eprintln!("{e}");
    })?;
    for run_index in 1..=n_runs {
        generate_data(
            path.as_path(),
            run_index,
            n_constraints,
            n_variables,
            coef_range,
        )
        .map_err(|e| {
            eprintln!("{e}");
        })?;
    }
    Ok(())
}
