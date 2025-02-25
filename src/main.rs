use std::fs;

use clap::Parser;
use serde_json::Value;

#[derive(Parser)]
struct Cli {
    files: Vec<String>,
}

#[allow(clippy::too_many_lines)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    for file in args.files {
        let mut result = String::new();

        let content = fs::read_to_string(file)?;
        let v: Value = serde_json::from_str(&content)?;
        let name = v
            .get("name")
            .expect("Conversation name not found")
            .as_str()
            .unwrap();
        let system_prompt = v
            .get("systemPrompt")
            .expect("System prompt not found")
            .as_str()
            .unwrap();

        result.push_str(&format!("# {name}\n"));
        if !system_prompt.is_empty() {
            result.push_str("## System Prompt");
            result.push_str(&format!("{system_prompt}\n"));
        }

        for message in v
            .get("messages")
            .expect("Messages not found")
            .as_array()
            .expect("Messages not an array")
        {
            let last_version = message
                .get("versions")
                .expect("Versions not found")
                .as_array()
                .expect("Versions not an array")
                .last()
                .unwrap();

            let role = last_version
                .get("role")
                .expect("Role not found")
                .as_str()
                .unwrap();

            result.push_str(&format!("## {role}\n"));

            if last_version.get("steps").is_some() {
                let steps = last_version
                    .get("steps")
                    .expect("Steps not found")
                    .as_array()
                    .expect("Steps not an array");
                for step in steps {
                    let mut is_thinking = false;
                    if step.get("style").is_some() {
                        let name = step
                            .get("style")
                            .expect("Style not found")
                            .get("title")
                            .expect("Title not found")
                            .as_str()
                            .unwrap();
                        let step_type = step
                            .get("style")
                            .expect("Style not found")
                            .get("type")
                            .expect("Type not found")
                            .as_str()
                            .unwrap();
                        is_thinking = step_type == "thinking";
                        result.push_str(&format!("### {name}\n"));
                    }
                    if step.get("content").is_some() {
                        let content = step
                            .get("content")
                            .expect("Content not found")
                            .as_array()
                            .unwrap()
                            .last()
                            .unwrap();

                        let mut text = content.get("text").unwrap().as_str().unwrap().to_string();

                        if is_thinking {
                            text = text
                                .split('\n')
                                .map(|l| {
                                    if l.is_empty() {
                                        l.to_string()
                                    } else {
                                        "> ".to_owned() + l
                                    }
                                })
                                .collect::<Vec<String>>()
                                .join("\n");
                        }

                        result.push_str(&format!("{text}\n"));
                    }
                }
            } else {
                let content = last_version
                    .get("content")
                    .expect("Content not found")
                    .as_array()
                    .unwrap()
                    .last()
                    .unwrap();
                let text = content.get("text").unwrap().as_str().unwrap();
                result.push_str(&format!("{text}\n"));
            }
        }
        println!("{}", result);
    }

    Ok(())
}
