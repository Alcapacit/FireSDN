use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

#[derive(Deserialize, Serialize, Debug)]
pub struct ProUsers {
    pub users: Vec<String>,
}

impl ProUsers {
    // Load pro.json or create empty if missing
    pub fn load() -> Self {
        let path = "service/pro.json";
        match fs::read_to_string(path) {
            Ok(data) => serde_json::from_str(&data).unwrap_or(ProUsers { users: vec![] }),
            Err(_) => ProUsers { users: vec![] },
        }
    }

    // Save pro.json
    pub fn save(&self) -> io::Result<()> {
        let path = "service/pro.json";
        let data = serde_json::to_string_pretty(&self).unwrap();
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    // Add a new pro user
    pub fn add_user(&mut self, username: &str) -> io::Result<()> {
        if !self.users.contains(&username.to_string()) {
            self.users.push(username.to_string());
            self.save()?;
            println!("Added {} to pro.json", username);
        } else {
            println!("{} is already a Pro user", username);
        }
        Ok(())
    }

    // Remove a pro user
    pub fn remove_user(&mut self, username: &str) -> io::Result<()> {
        if let Some(pos) = self.users.iter().position(|u| u == username) {
            self.users.remove(pos);
            self.save()?;
            println!("Removed {} from pro.json", username);
        } else {
            println!("{} was not found in pro.json", username);
        }
        Ok(())
    }

    // Check if a user is pro locally
    pub fn is_pro(&self, username: &str) -> bool {
        self.users.contains(&username.to_string())
    }
}

// Example usage
fn main() -> io::Result<()> {
    let mut pro = ProUsers::load();

    // Add a new user
    pro.add_user("Abdi123")?;

    // Check if user is pro
    if pro.is_pro("Abdi123") {
        println!("Abdi123 is a Pro user!");
    }

    // Remove a user
    pro.remove_user("Abdi123")?;

    Ok(())
}