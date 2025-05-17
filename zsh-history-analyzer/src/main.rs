use std::collections::HashMap;

struct ZshHist {
    full_command_hist: HashMap<String, u64>
}

impl ZshHist {
    fn new() -> ZshHist {
        let mut zsh_history = ZshHist { full_command_hist: HashMap::new() };
        let home_dir = std::env::var("HOME").expect("HOME environment variable not set");
        let history_file = format!("{home_dir}/.zsh_history");
        let history = std::fs::read(history_file).expect("Failed to read zsh history file");
        let history = String::from_utf8_lossy(&history);
        let lines = history.lines();

        lines.for_each(|cmd| {
            let cmd_count = zsh_history.full_command_hist.get(cmd).unwrap_or(&1).clone();
            zsh_history.full_command_hist.insert(cmd.to_string(), cmd_count + 1);
        });
        zsh_history
    }
}

fn main() {
    let hist = ZshHist::new();
    let mut commands: Vec<_> = hist.full_command_hist.iter().collect();

    commands.sort_by(|a, b| a.1.cmp(b.1));

    commands.iter()
        .filter(|(_, count)| *count > &10)
        .for_each(|(cmd, count)| println!("times {}: command: {}", count, cmd));
}
