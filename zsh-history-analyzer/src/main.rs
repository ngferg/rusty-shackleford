use std::collections::HashMap;

struct ZshHist {
    full_command_hist: HashMap<String, u64>,
    program_hist: HashMap<String, u64>,
}


#[derive(Debug)]
enum ReportType {
    COMMAND, PROGRAM
}

impl ZshHist {
    fn new() -> ZshHist {
        let mut zsh_history = ZshHist { full_command_hist: HashMap::new(), program_hist: HashMap::new() };
        let home_dir = std::env::var("HOME").expect("HOME environment variable not set");
        let history_file = format!("{home_dir}/.zsh_history");
        let history = std::fs::read(history_file).expect("Failed to read zsh history file");
        let history = String::from_utf8_lossy(&history);
        let lines = history.lines();

        lines.for_each(|cmd| {
            let cmd_count = zsh_history.full_command_hist.get(cmd).unwrap_or(&0).clone();
            zsh_history.full_command_hist.insert(cmd.to_string(), cmd_count + 1);
            let program = cmd.split(" ").next().unwrap_or("");
            let prgm_count = zsh_history.program_hist.get(program).unwrap_or(&0).clone();
            zsh_history.program_hist.insert(program.to_string(), prgm_count + 1);
        });
        zsh_history
    }
    
    fn print_report(&self, report_type: ReportType) {
        
        
        println!("------ {:?} report ----------", report_type);
        
        let mut cmds: Vec<_> = match report_type { 
            ReportType::COMMAND => self.full_command_hist.iter().collect(),
            ReportType::PROGRAM => self.program_hist.iter().collect(),
        };
        
        cmds.sort_by(|a, b| a.1.cmp(b.1));

        cmds.iter()
            .filter(|(_, count)| *count > &10)
            .for_each(|(cmd, count)| println!("times {}: command: {}", count, cmd));
    }
}

fn main() {
    let hist = ZshHist::new();
    
    hist.print_report(ReportType::COMMAND);
    hist.print_report(ReportType::PROGRAM);
}
