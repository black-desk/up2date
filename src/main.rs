// SPDX-FileCopyrightText: 2025 Chen Linxuan <me@black-desk.cn>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(name = "up2date")]
#[command(
    about = "Check if all dependencies in the current repository have been configured for automatic updates via dependabot"
)]
struct Args {
    /// Output in JSON format
    #[arg(long)]
    json: bool,

    /// Output in YAML format
    #[arg(long)]
    yaml: bool,

    /// Output in TOML format
    #[arg(long)]
    toml: bool,
}

impl Args {
    fn output_format(&self) -> OutputFormat {
        if self.json {
            OutputFormat::Json
        } else if self.yaml {
            OutputFormat::Yaml
        } else if self.toml {
            OutputFormat::Toml
        } else {
            OutputFormat::Markdown
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct DependabotConfig {
    version: u8,
    updates: Vec<UpdateConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateConfig {
    #[serde(rename = "package-ecosystem")]
    package_ecosystem: String,
    directory: String,
    schedule: ScheduleConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct ScheduleConfig {
    interval: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DependencyReport {
    project_dependencies: Vec<ProjectDependency>,
    dependabot_ecosystems: Vec<String>,
    missing_from_dependabot: Vec<String>,
    summary: ReportSummary,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProjectDependency {
    ecosystem: String,
    directory: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ReportSummary {
    total_ecosystems: usize,
    configured_ecosystems: usize,
    missing_ecosystems: usize,
}

#[derive(Debug)]
enum OutputFormat {
    Markdown,
    Json,
    Yaml,
    Toml,
}

fn main() {
    let args = Args::parse();
    let output_format = args.output_format();

    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let report = analyze_dependencies(&current_dir);

    match output_format {
        OutputFormat::Markdown => print_markdown_report(&report),
        OutputFormat::Json => print_json_report(&report),
        OutputFormat::Yaml => print_yaml_report(&report),
        OutputFormat::Toml => print_toml_report(&report),
    }

    // Exit with code 1 if there are missing ecosystems
    if !report.missing_from_dependabot.is_empty() {
        std::process::exit(1);
    }
}

fn analyze_dependencies(project_root: &Path) -> DependencyReport {
    let project_dependencies = find_project_dependencies(project_root);
    let dependabot_ecosystems = find_dependabot_ecosystems(project_root);

    let project_ecosystem_set: HashSet<String> = project_dependencies
        .iter()
        .map(|dep| dep.ecosystem.clone())
        .collect();

    let dependabot_ecosystem_set: HashSet<String> = dependabot_ecosystems.iter().cloned().collect();

    let missing_from_dependabot: Vec<String> = project_ecosystem_set
        .difference(&dependabot_ecosystem_set)
        .cloned()
        .collect();

    let total_ecosystems = project_ecosystem_set.len();
    let configured_ecosystems = total_ecosystems - missing_from_dependabot.len();

    let missing_ecosystems_count = missing_from_dependabot.len();

    DependencyReport {
        project_dependencies,
        dependabot_ecosystems,
        missing_from_dependabot,
        summary: ReportSummary {
            total_ecosystems,
            configured_ecosystems,
            missing_ecosystems: missing_ecosystems_count,
        },
    }
}

fn find_project_dependencies(project_root: &Path) -> Vec<ProjectDependency> {
    let mut dependencies = Vec::new();
    let mut ecosystem_dirs: HashMap<String, String> = HashMap::new();
    let mut has_github_workflows = false;

    // Check for GitHub Actions workflows in .github/workflows (root only)
    let workflows_dir = project_root.join(".github/workflows");
    if workflows_dir.exists() && workflows_dir.is_dir() {
        if let Ok(entries) = std::fs::read_dir(&workflows_dir) {
            for entry in entries.flatten() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(".yml") || file_name.ends_with(".yaml") {
                        has_github_workflows = true;
                        break;
                    }
                }
            }
        }
    }

    // Recursively check for other dependency files
    for entry in WalkDir::new(project_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_string_lossy();

        let ecosystem = match file_name.as_ref() {
            "Cargo.toml" => Some("cargo"),
            "package.json" => Some("npm"),
            "requirements.txt" | "pyproject.toml" | "setup.py" | "Pipfile" => Some("pip"),
            "go.mod" => Some("gomod"),
            ".gitmodules" => Some("gitsubmodule"),
            "Dockerfile" | "Containerfile" => Some("docker"),
            "action.yaml" | "action.yml" => Some("github-action"),
            _ => None,
        };

        if let Some(ecosystem) = ecosystem {
            let relative_dir = path
                .parent()
                .unwrap()
                .strip_prefix(project_root)
                .unwrap_or(Path::new("."))
                .to_string_lossy()
                .to_string();

            let dir_key = format!("{}:{}", ecosystem, relative_dir);

            if !ecosystem_dirs.contains_key(&dir_key) {
                ecosystem_dirs.insert(dir_key, relative_dir);
            }
        }
    }

    // Add GitHub Actions workflows if found
    if has_github_workflows {
        dependencies.push(ProjectDependency {
            ecosystem: "github-actions".to_string(),
            directory: ".".to_string(),
        });
    }

    for (ecosystem_dir, directory) in ecosystem_dirs {
        let ecosystem = ecosystem_dir.split(':').next().unwrap().to_string();
        dependencies.push(ProjectDependency {
            ecosystem,
            directory: if directory.is_empty() {
                ".".to_string()
            } else {
                directory
            },
        });
    }

    dependencies
}

fn find_dependabot_ecosystems(project_root: &Path) -> Vec<String> {
    let dependabot_paths = [".github/dependabot.yml", ".github/dependabot.yaml"];

    for path in &dependabot_paths {
        let full_path = project_root.join(path);
        if !full_path.exists() {
            continue;
        }

        let Ok(content) = fs::read_to_string(&full_path) else {
            continue;
        };

        let Ok(config) = serde_yaml::from_str::<DependabotConfig>(&content) else {
            continue;
        };

        return config
            .updates
            .into_iter()
            .map(|update| update.package_ecosystem)
            .collect();
    }

    Vec::new()
}

fn print_markdown_report(report: &DependencyReport) {
    println!("# Dependabot Coverage Report\n");

    println!("## Summary\n");
    println!(
        "- **Total ecosystems found**: {}",
        report.summary.total_ecosystems
    );
    println!(
        "- **Configured in dependabot**: {}",
        report.summary.configured_ecosystems
    );
    println!(
        "- **Missing from dependabot**: {}\n",
        report.summary.missing_ecosystems
    );

    println!("## Project Dependencies\n");
    for dep in &report.project_dependencies {
        println!("- **{}** in `{}`", dep.ecosystem, dep.directory);
    }
    println!();

    if !report.missing_from_dependabot.is_empty() {
        println!("## Missing from Dependabot\n");
        for ecosystem in &report.missing_from_dependabot {
            println!("- {}", ecosystem);
        }
        println!();
    }

    if !report.dependabot_ecosystems.is_empty() {
        println!("## Configured in Dependabot\n");
        for ecosystem in &report.dependabot_ecosystems {
            println!("- {}", ecosystem);
        }
    }
}

fn print_json_report(report: &DependencyReport) {
    let json = serde_json::to_string_pretty(report).unwrap();
    println!("{}", json);
}

fn print_yaml_report(report: &DependencyReport) {
    let yaml = serde_yaml::to_string(report).unwrap();
    println!("{}", yaml);
}

fn print_toml_report(report: &DependencyReport) {
    let toml_value = toml::to_string(report).unwrap();
    println!("{}", toml_value);
}
