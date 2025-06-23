// SPDX-FileCopyrightText: 2025 Chen Linxuan <me@black-desk.cn>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

// Helper function to compare JSON values with order-independent arrays
fn assert_json_eq_unordered(actual: &serde_json::Value, expected: &serde_json::Value) {
    match (actual, expected) {
        (serde_json::Value::Array(actual_arr), serde_json::Value::Array(expected_arr)) => {
            assert_eq!(
                actual_arr.len(),
                expected_arr.len(),
                "Array length mismatch"
            );
            for expected_item in expected_arr {
                assert!(
                    actual_arr.contains(expected_item),
                    "Missing expected item: {}",
                    expected_item
                );
            }
        }
        (serde_json::Value::Object(actual_obj), serde_json::Value::Object(expected_obj)) => {
            assert_eq!(
                actual_obj.len(),
                expected_obj.len(),
                "Object field count mismatch"
            );
            for (key, expected_value) in expected_obj {
                let actual_value = actual_obj
                    .get(key)
                    .unwrap_or_else(|| panic!("Missing key: {}", key));
                assert_json_eq_unordered(actual_value, expected_value);
            }
        }
        _ => assert_eq!(actual, expected),
    }
}

fn create_test_project() -> TempDir {
    let temp_dir = TempDir::new().unwrap();

    // Create a basic Rust project
    fs::write(
        temp_dir.path().join("Cargo.toml"),
        r#"[package]
name = "test-project"
version = "0.1.0"
edition = "2021"
"#,
    )
    .unwrap();

    // Create .gitmodules
    fs::write(
        temp_dir.path().join(".gitmodules"),
        r#"[submodule "external"]
	path = external
	url = https://github.com/example/external.git
"#,
    )
    .unwrap();

    // Create .github directory and dependabot config
    fs::create_dir_all(temp_dir.path().join(".github")).unwrap();
    fs::write(
        temp_dir.path().join(".github/dependabot.yaml"),
        r#"version: 2
updates:
  - directory: /
    package-ecosystem: cargo
    schedule:
      interval: daily
  - directory: /
    package-ecosystem: gitsubmodule
    schedule:
      interval: daily
  - directory: /
    package-ecosystem: github-actions
    schedule:
      interval: daily
"#,
    )
    .unwrap();

    // Create .github/workflows directory with a workflow
    fs::create_dir_all(temp_dir.path().join(".github/workflows")).unwrap();
    fs::write(
        temp_dir.path().join(".github/workflows/ci.yml"),
        r#"name: CI
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run tests
        run: cargo test
"#,
    )
    .unwrap();

    temp_dir
}

fn create_incomplete_project() -> TempDir {
    let temp_dir = TempDir::new().unwrap();

    // Create project with dependencies but no dependabot config
    fs::write(
        temp_dir.path().join("Cargo.toml"),
        r#"[package]
name = "test-project"
version = "0.1.0"
edition = "2021"
"#,
    )
    .unwrap();

    fs::write(
        temp_dir.path().join("package.json"),
        r#"{
  "name": "test-project",
  "version": "1.0.0",
  "dependencies": {
    "lodash": "^4.17.21"
  }
}
"#,
    )
    .unwrap();

    temp_dir
}

#[test]
fn test_help_output() {
    let mut cmd = Command::cargo_bin("up2date").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn test_complete_project_markdown_output() {
    let temp_dir = create_test_project();

    let mut cmd = Command::cargo_bin("up2date").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn test_complete_project_json_output() {
    let temp_dir = create_test_project();

    let mut cmd = Command::cargo_bin("up2date").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("--json");

    let output = cmd.assert().success().get_output().stdout.clone();
    let json_str = String::from_utf8(output).unwrap();

    // Parse actual JSON
    let actual: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    // Expected JSON for complete project
    let expected = serde_json::json!({
        "summary": {
            "total_ecosystems": 3,
            "configured_ecosystems": 3,
            "missing_ecosystems": 0
        },
        "project_dependencies": [
            {
                "ecosystem": "cargo",
                "directory": "."
            },
            {
                "ecosystem": "gitsubmodule",
                "directory": "."
            },
            {
                "ecosystem": "github-actions",
                "directory": "."
            }
        ],
        "dependabot_ecosystems": ["cargo", "gitsubmodule", "github-actions"],
        "missing_from_dependabot": []
    });

    // Compare with order-independent comparison
    assert_json_eq_unordered(&actual, &expected);
}

#[test]
fn test_incomplete_project() {
    let temp_dir = create_incomplete_project();

    let mut cmd = Command::cargo_bin("up2date").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("--json");

    let output = cmd.assert().code(1).get_output().stdout.clone();
    let json_str = String::from_utf8(output).unwrap();

    // Parse actual JSON
    let actual: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    // Expected JSON for incomplete project
    let expected = serde_json::json!({
        "summary": {
            "total_ecosystems": 2,
            "configured_ecosystems": 0,
            "missing_ecosystems": 2
        },
        "project_dependencies": [
            {
                "ecosystem": "cargo",
                "directory": "."
            },
            {
                "ecosystem": "npm",
                "directory": "."
            }
        ],
        "dependabot_ecosystems": [],
        "missing_from_dependabot": ["cargo", "npm"]
    });

    // Compare with order-independent comparison
    assert_json_eq_unordered(&actual, &expected);
}

#[test]
fn test_yaml_output_format() {
    let temp_dir = create_test_project();

    let mut cmd = Command::cargo_bin("up2date").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("--yaml");

    let output = cmd.assert().success().get_output().stdout.clone();
    let yaml_str = String::from_utf8(output).unwrap();

    // Parse actual YAML and convert to JSON Value for comparison
    let actual: serde_yaml::Value = serde_yaml::from_str(&yaml_str).unwrap();
    let actual_json: serde_json::Value = serde_json::to_value(actual).unwrap();

    // Expected data for complete project
    let expected = serde_json::json!({
        "summary": {
            "total_ecosystems": 3,
            "configured_ecosystems": 3,
            "missing_ecosystems": 0
        },
        "project_dependencies": [
            {
                "ecosystem": "cargo",
                "directory": "."
            },
            {
                "ecosystem": "gitsubmodule",
                "directory": "."
            },
            {
                "ecosystem": "github-actions",
                "directory": "."
            }
        ],
        "dependabot_ecosystems": ["cargo", "gitsubmodule", "github-actions"],
        "missing_from_dependabot": []
    });

    // Compare with order-independent comparison
    assert_json_eq_unordered(&actual_json, &expected);
}

#[test]
fn test_toml_output_format() {
    let temp_dir = create_test_project();

    let mut cmd = Command::cargo_bin("up2date").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("--toml");

    let output = cmd.assert().success().get_output().stdout.clone();
    let toml_str = String::from_utf8(output).unwrap();

    // Parse actual TOML and convert to JSON Value for comparison
    let actual: toml::Value = toml::from_str(&toml_str).unwrap();
    let actual_json: serde_json::Value = serde_json::to_value(actual).unwrap();

    // Expected data for complete project
    let expected = serde_json::json!({
        "summary": {
            "total_ecosystems": 3,
            "configured_ecosystems": 3,
            "missing_ecosystems": 0
        },
        "project_dependencies": [
            {
                "ecosystem": "cargo",
                "directory": "."
            },
            {
                "ecosystem": "gitsubmodule",
                "directory": "."
            },
            {
                "ecosystem": "github-actions",
                "directory": "."
            }
        ],
        "dependabot_ecosystems": ["cargo", "gitsubmodule", "github-actions"],
        "missing_from_dependabot": []
    });

    // Compare with order-independent comparison
    assert_json_eq_unordered(&actual_json, &expected);
}

#[test]
fn test_project_with_docker() {
    let temp_dir = TempDir::new().unwrap();

    // Create project with Dockerfile
    fs::write(
        temp_dir.path().join("Dockerfile"),
        r#"FROM rust:1.70
WORKDIR /app
COPY . .
RUN cargo build --release
"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("up2date").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("--json");

    let output = cmd.assert().code(1).get_output().stdout.clone();
    let json_str = String::from_utf8(output).unwrap();

    // Parse actual JSON
    let actual: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    // Expected JSON for Docker project
    let expected = serde_json::json!({
        "summary": {
            "total_ecosystems": 1,
            "configured_ecosystems": 0,
            "missing_ecosystems": 1
        },
        "project_dependencies": [
            {
                "ecosystem": "docker",
                "directory": "."
            }
        ],
        "dependabot_ecosystems": [],
        "missing_from_dependabot": ["docker"]
    });

    // Compare with order-independent comparison
    assert_json_eq_unordered(&actual, &expected);
}

#[test]
fn test_project_with_containerfile() {
    let temp_dir = TempDir::new().unwrap();

    // Create project with Containerfile
    fs::write(
        temp_dir.path().join("Containerfile"),
        r#"FROM ubuntu:20.04
RUN apt-get update
"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("up2date").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("--json");

    let output = cmd.assert().code(1).get_output().stdout.clone();
    let json_str = String::from_utf8(output).unwrap();

    // Parse actual JSON
    let actual: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    // Expected JSON for Containerfile project
    let expected = serde_json::json!({
        "summary": {
            "total_ecosystems": 1,
            "configured_ecosystems": 0,
            "missing_ecosystems": 1
        },
        "project_dependencies": [
            {
                "ecosystem": "docker",
                "directory": "."
            }
        ],
        "dependabot_ecosystems": [],
        "missing_from_dependabot": ["docker"]
    });

    // Compare with order-independent comparison
    assert_json_eq_unordered(&actual, &expected);
}

#[test]
fn test_empty_project() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("up2date").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("--json");

    let output = cmd.assert().success().get_output().stdout.clone();
    let json_str = String::from_utf8(output).unwrap();

    // Parse actual JSON
    let actual: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    // Expected JSON for empty project
    let expected = serde_json::json!({
        "summary": {
            "total_ecosystems": 0,
            "configured_ecosystems": 0,
            "missing_ecosystems": 0
        },
        "project_dependencies": [],
        "dependabot_ecosystems": [],
        "missing_from_dependabot": []
    });

    // Compare with order-independent comparison
    assert_json_eq_unordered(&actual, &expected);
}

#[test]
fn test_subdirectory_dependencies() {
    let temp_dir = TempDir::new().unwrap();

    // Create subdirectory with package.json
    fs::create_dir_all(temp_dir.path().join("frontend")).unwrap();
    fs::write(
        temp_dir.path().join("frontend/package.json"),
        r#"{
  "name": "frontend",
  "version": "1.0.0"
}
"#,
    )
    .unwrap();

    // Create another subdirectory with go.mod
    fs::create_dir_all(temp_dir.path().join("backend")).unwrap();
    fs::write(
        temp_dir.path().join("backend/go.mod"),
        r#"module backend

go 1.19
"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("up2date").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("--json");

    let output = cmd.assert().code(1).get_output().stdout.clone();
    let json_str = String::from_utf8(output).unwrap();

    // Parse actual JSON
    let actual: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    // Expected JSON for subdirectory dependencies
    let expected = serde_json::json!({
        "summary": {
            "total_ecosystems": 2,
            "configured_ecosystems": 0,
            "missing_ecosystems": 2
        },
        "project_dependencies": [
            {
                "ecosystem": "npm",
                "directory": "frontend"
            },
            {
                "ecosystem": "gomod",
                "directory": "backend"
            }
        ],
        "dependabot_ecosystems": [],
        "missing_from_dependabot": ["npm", "gomod"]
    });

    // Compare with order-independent comparison
    assert_json_eq_unordered(&actual, &expected);
}

#[test]
fn test_exit_code_success_when_all_covered() {
    let temp_dir = create_test_project();

    let mut cmd = Command::cargo_bin("up2date").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("--json");

    // Should exit with code 0 when all ecosystems are covered
    cmd.assert().success();
}

#[test]
fn test_exit_code_failure_when_missing_coverage() {
    let temp_dir = create_incomplete_project();

    let mut cmd = Command::cargo_bin("up2date").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("--json");

    // Should exit with code 1 when some ecosystems are missing
    cmd.assert().code(1);
}

#[test]
fn test_exit_code_success_when_empty() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("up2date").unwrap();
    cmd.current_dir(temp_dir.path());
    cmd.arg("--json");

    // Should exit with code 0 when no ecosystems are found
    cmd.assert().success();
}
