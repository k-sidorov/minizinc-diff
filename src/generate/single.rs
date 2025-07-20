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
    fs,
    path::{Path, PathBuf},
};

fn generate_model(path: &Path, constraint: &str) -> std::io::Result<PathBuf> {
    let model_dir = path.join(constraint);
    fs::create_dir_all(&model_dir)?;
    let model_path = model_dir.join("model.mzn");

    let constraint_code = match constraint {
        "eq" => "constraint sum(i in 1..n_bin)(x[i]) + sum(i in 1..n_int)(y[i]) = 5;",
        "le" => "constraint sum(i in 1..n_bin)(x[i]) + sum(i in 1..n_int)(y[i]) <= 5;",
        "alldiff" => "constraint alldifferent(y);",
        _ => panic!("Unsupported constraint type"),
    };

    let model_content = format!(
        r#"include "alldifferent.mzn";
int: n_bin;
array[1..n_bin] of var 0..1: x;

int: n_int;
array[1..n_int] of var 1..5: y;

{constraint}

solve satisfy;
output ["x=" ++ show(x) ++ " y=" ++ show(y)];
"#,
        constraint = constraint_code
    );

    fs::write(&model_path, model_content)?;
    Ok(model_dir)
}

fn generate_data(
    path: &Path,
    _constraint: &str,
    n_bin: usize,
    n_int: usize,
) -> std::io::Result<()> {
    let data_filename = format!("data_{}_{}.dzn", n_bin, n_int);
    let data_path = path.join(&data_filename);

    let data_content = format!("n_bin = {n_bin};\nn_int = {n_int};\n");

    fs::write(&data_path, data_content)?;
    Ok(())
}

pub(crate) fn run(out: &Path) -> Result<(), ()> {
    for &constraint in &["eq", "le", "alldiff"] {
        // TODO Better error handling
        let model_dir = generate_model(out, constraint).map_err(|e| {
            eprintln!("{e}");
        })?;
        for &n_bin in &[2, 3] {
            for &n_int in &[1, 2] {
                generate_data(model_dir.as_path(), constraint, n_bin, n_int).map_err(|e| {
                    eprintln!("{e}");
                })?;
            }
        }
    }
    Ok(())
}
