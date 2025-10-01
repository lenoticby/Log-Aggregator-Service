// main.rs - A comprehensive Rust starter script for GitHub's Linguist
// Created by Abie Haryatmo, enhanced by Gemini

struct Project {
    name: String,
    version: String,
}

impl Project {
    fn new(name: &str) -> Self {
        Project {
            name: String::from(name),
            version: String::from("1.0.0"),
        }
    }

    fn display_info(&self) {
        println!("Project Name: {}", self.name);
        println!("Version: {}", self.version);
    }
}

fn main() {
    let my_project = Project::new("GitHub Auto-Repo Project");
    my_project.display_info();

    println!("\nThis script is now comprehensive enough for GitHub's language detection.");

    let features = vec!["Structs", "Impl Blocks", "Methods", "Vectors"];
    println!("Features demonstrated:");
    for feature in features {
        println!("  - {}", feature);
    }
}
