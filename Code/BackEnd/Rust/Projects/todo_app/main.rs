// Todo Application Project
// A command-line todo list manager with file persistence

use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    fn from_str(s: &str) -> Option<Priority> {
        match s.to_lowercase().as_str() {
            "low" | "l" => Some(Priority::Low),
            "medium" | "med" | "m" => Some(Priority::Medium),
            "high" | "h" => Some(Priority::High),
            _ => None,
        }
    }
    
    fn to_string(&self) -> &str {
        match self {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
        }
    }
    
    fn emoji(&self) -> &str {
        match self {
            Priority::Low => "🟢",
            Priority::Medium => "🟡",
            Priority::High => "🔴",
        }
    }
}

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    description: Option<String>,
    completed: bool,
    priority: Priority,
    created_at: String,
}

impl Task {
    fn new(id: u32, title: String, priority: Priority) -> Self {
        Task {
            id,
            title,
            description: None,
            completed: false,
            priority,
            created_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
    
    fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }
    
    fn toggle_completed(&mut self) {
        self.completed = !self.completed;
    }
    
    fn to_file_format(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}",
            self.id,
            self.title,
            self.description.as_deref().unwrap_or(""),
            self.completed,
            self.priority.to_string(),
            self.created_at
        )
    }
    
    fn from_file_format(line: &str) -> Option<Task> {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != 6 {
            return None;
        }
        
        let id = parts[0].parse().ok()?;
        let title = parts[1].to_string();
        let description = if parts[2].is_empty() {
            None
        } else {
            Some(parts[2].to_string())
        };
        let completed = parts[3].parse().ok()?;
        let priority = Priority::from_str(parts[4])?;
        let created_at = parts[5].to_string();
        
        Some(Task {
            id,
            title,
            description,
            completed,
            priority,
            created_at,
        })
    }
}

struct TodoApp {
    tasks: HashMap<u32, Task>,
    next_id: u32,
    file_path: String,
}

impl TodoApp {
    fn new(file_path: String) -> Self {
        let mut app = TodoApp {
            tasks: HashMap::new(),
            next_id: 1,
            file_path,
        };
        app.load_from_file();
        app
    }
    
    fn add_task(&mut self, title: String, priority: Priority) -> u32 {
        let task = Task::new(self.next_id, title, priority);
        let id = task.id;
        self.tasks.insert(id, task);
        self.next_id += 1;
        self.save_to_file();
        id
    }
    
    fn remove_task(&mut self, id: u32) -> bool {
        if self.tasks.remove(&id).is_some() {
            self.save_to_file();
            true
        } else {
            false
        }
    }
    
    fn toggle_task(&mut self, id: u32) -> bool {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.toggle_completed();
            self.save_to_file();
            true
        } else {
            false
        }
    }
    
    fn update_description(&mut self, id: u32, description: String) -> bool {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.set_description(description);
            self.save_to_file();
            true
        } else {
            false
        }
    }
    
    fn list_tasks(&self, filter: Option<&str>) {
        let mut tasks: Vec<&Task> = self.tasks.values().collect();
        
        // Apply filter
        match filter {
            Some("completed") => tasks.retain(|t| t.completed),
            Some("pending") => tasks.retain(|t| !t.completed),
            Some("high") => tasks.retain(|t| matches!(t.priority, Priority::High)),
            Some("medium") => tasks.retain(|t| matches!(t.priority, Priority::Medium)),
            Some("low") => tasks.retain(|t| matches!(t.priority, Priority::Low)),
            _ => {}
        }
        
        // Sort by priority (High -> Medium -> Low) then by ID
        tasks.sort_by(|a, b| {
            let priority_order = |p: &Priority| match p {
                Priority::High => 0,
                Priority::Medium => 1,
                Priority::Low => 2,
            };
            
            priority_order(&a.priority)
                .cmp(&priority_order(&b.priority))
                .then(a.id.cmp(&b.id))
        });
        
        if tasks.is_empty() {
            println!("No tasks found.");
            return;
        }
        
        println!("\n📋 Todo List:");
        println!("{:-<80}", "");
        
        for task in tasks {
            let status = if task.completed { "✅" } else { "⏳" };
            let priority_emoji = task.priority.emoji();
            
            println!(
                "{} {} [{}] {} - {}",
                status, priority_emoji, task.id, task.title, task.priority.to_string()
            );
            
            if let Some(desc) = &task.description {
                println!("    📝 {}", desc);
            }
            
            println!("    🕒 Created: {}", task.created_at);
            println!();
        }
    }
    
    fn get_stats(&self) -> (usize, usize, usize) {
        let total = self.tasks.len();
        let completed = self.tasks.values().filter(|t| t.completed).count();
        let pending = total - completed;
        (total, completed, pending)
    }
    
    fn save_to_file(&self) {
        let content: String = self
            .tasks
            .values()
            .map(|task| task.to_file_format())
            .collect::<Vec<String>>()
            .join("\n");
        
        if let Err(e) = fs::write(&self.file_path, content) {
            eprintln!("Error saving to file: {}", e);
        }
    }
    
    fn load_from_file(&mut self) {
        if !Path::new(&self.file_path).exists() {
            return;
        }
        
        match fs::read_to_string(&self.file_path) {
            Ok(content) => {
                for line in content.lines() {
                    if let Some(task) = Task::from_file_format(line) {
                        if task.id >= self.next_id {
                            self.next_id = task.id + 1;
                        }
                        self.tasks.insert(task.id, task);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error loading from file: {}", e);
            }
        }
    }
}

fn print_help() {
    println!("\n📚 Todo App Commands:");
    println!("{:-<50}", "");
    println!("  add <title> [priority]     - Add a new task");
    println!("  remove <id>               - Remove a task");
    println!("  toggle <id>               - Toggle task completion");
    println!("  desc <id> <description>   - Add description to task");
    println!("  list [filter]             - List tasks");
    println!("    Filters: all, completed, pending, high, medium, low");
    println!("  stats                     - Show statistics");
    println!("  help                      - Show this help");
    println!("  quit                      - Exit application");
    println!("\nPriorities: high, medium, low (default: medium)");
    println!();
}

fn main() {
    let file_path = "todos.txt".to_string();
    let mut app = TodoApp::new(file_path);
    
    println!("📝 Welcome to Rust Todo App!");
    println!("Type 'help' for commands or 'quit' to exit.");
    
    loop {
        print!("todo> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                if input.is_empty() {
                    continue;
                }
                
                let parts: Vec<&str> = input.split_whitespace().collect();
                if parts.is_empty() {
                    continue;
                }
                
                match parts[0].to_lowercase().as_str() {
                    "quit" | "exit" | "q" => {
                        println!("Goodbye! 👋");
                        break;
                    }
                    "help" | "h" => {
                        print_help();
                    }
                    "add" => {
                        if parts.len() < 2 {
                            println!("Usage: add <title> [priority]");
                            continue;
                        }
                        
                        let title = parts[1..].join(" ");
                        let (title, priority) = if let Some(last_word) = parts.last() {
                            if let Some(p) = Priority::from_str(last_word) {
                                let title_parts = &parts[1..parts.len()-1];
                                if title_parts.is_empty() {
                                    (title, Priority::Medium)
                                } else {
                                    (title_parts.join(" "), p)
                                }
                            } else {
                                (title, Priority::Medium)
                            }
                        } else {
                            (title, Priority::Medium)
                        };
                        
                        let id = app.add_task(title.clone(), priority);
                        println!("✅ Added task #{}: {}", id, title);
                    }
                    "remove" | "rm" => {
                        if parts.len() != 2 {
                            println!("Usage: remove <id>");
                            continue;
                        }
                        
                        match parts[1].parse::<u32>() {
                            Ok(id) => {
                                if app.remove_task(id) {
                                    println!("🗑️ Removed task #{}", id);
                                } else {
                                    println!("❌ Task #{} not found", id);
                                }
                            }
                            Err(_) => println!("❌ Invalid task ID"),
                        }
                    }
                    "toggle" | "t" => {
                        if parts.len() != 2 {
                            println!("Usage: toggle <id>");
                            continue;
                        }
                        
                        match parts[1].parse::<u32>() {
                            Ok(id) => {
                                if app.toggle_task(id) {
                                    println!("🔄 Toggled task #{}", id);
                                } else {
                                    println!("❌ Task #{} not found", id);
                                }
                            }
                            Err(_) => println!("❌ Invalid task ID"),
                        }
                    }
                    "desc" | "description" => {
                        if parts.len() < 3 {
                            println!("Usage: desc <id> <description>");
                            continue;
                        }
                        
                        match parts[1].parse::<u32>() {
                            Ok(id) => {
                                let description = parts[2..].join(" ");
                                if app.update_description(id, description.clone()) {
                                    println!("📝 Updated description for task #{}", id);
                                } else {
                                    println!("❌ Task #{} not found", id);
                                }
                            }
                            Err(_) => println!("❌ Invalid task ID"),
                        }
                    }
                    "list" | "ls" => {
                        let filter = parts.get(1).copied();
                        app.list_tasks(filter);
                    }
                    "stats" | "statistics" => {
                        let (total, completed, pending) = app.get_stats();
                        println!("\n📊 Statistics:");
                        println!("  Total tasks: {}", total);
                        println!("  Completed: {} ✅", completed);
                        println!("  Pending: {} ⏳", pending);
                        if total > 0 {
                            let completion_rate = (completed as f64 / total as f64) * 100.0;
                            println!("  Completion rate: {:.1}%", completion_rate);
                        }
                        println!();
                    }
                    _ => {
                        println!("❌ Unknown command. Type 'help' for available commands.");
                    }
                }
            }
            Err(error) => {
                println!("Error reading input: {}", error);
                break;
            }
        }
    }
}

// Note: This example uses a placeholder for chrono
// In a real project, you would add chrono to Cargo.toml
mod chrono {
    pub struct Utc;
    
    impl Utc {
        pub fn now() -> DateTime {
            DateTime
        }
    }
    
    pub struct DateTime;
    
    impl DateTime {
        pub fn format(&self, _fmt: &str) -> FormattedDateTime {
            FormattedDateTime
        }
    }
    
    pub struct FormattedDateTime;
    
    impl FormattedDateTime {
        pub fn to_string(&self) -> String {
            "2024-01-01 12:00:00".to_string()
        }
    }
}