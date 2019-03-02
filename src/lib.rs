#[macro_use]
extern crate log;

use std::fs;

pub fn init(project_name: &str) {
    let folders = vec!["models", "tests", "data", "macros", "target", "logs"];

    let template = format!(
        r#"

# Name your package! Package names should contain only lowercase characters
# and underscores. A good package name should reflect your organization's
# name or the intended use of these models
name: '{}'
version: '1.0'

# These configurations specify where dt should look for different types
# of files. The `source-paths` config, for example, states that source
# models can be found  in the "models/" directory.
# You probably won't need to change these!
source-paths: ["models"]
analysis-paths: ["analysis"]
test-paths: ["tests"]
data-paths: ["data"]
macro-paths: ["macros"]

target-path: "target"  # directory which will store compiled SQL files
clean-targets:         # directories to be removed by `dt clean`
    - "target"

# You can define configurations for models in the `source-paths` directory here.
# Using these configurations, you can enable or disable models, change how they
# are materialized, and more!

# In this example config, we tell dt to build all models in the example/ directory
# as views (the default). These settings can be overridden in the individual model files
# using the `{{ config(...) }}` macro.
models:
  {}:
      # Applies to all files under models/example/
      example:
          materialized: view

"#,
        project_name, project_name
    );

    let paths = folders
        .into_iter()
        .map(|f: &str| format!("./{}/{}", project_name, f));

    info!("Creating project folders");

    // https://doc.rust-lang.org/rust-by-example/error/iter_result.html
    let (_, failures): (Vec<_>, Vec<_>) = paths.map(fs::create_dir_all).partition(Result::is_ok);
    if failures.is_empty() {
        error!("Unable to create folders {:#?}", failures);
    }

    fs::write(format!("{}/project.yml", project_name), template).expect("Unable to write file");
}

//Tests, ahoy!
#[cfg(test)]
mod tests {
    use super::*;
}
