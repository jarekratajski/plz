use crate::contains_any;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExecutionMode {
    Default,
    Safe,
    Force,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RiskLevel {
    Safe,
    Moderate,
    Dangerous,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PolicyAction {
    Execute,
    Confirm,
    Reject,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RiskAssessment {
    pub level: RiskLevel,
    pub reason: &'static str,
}

pub fn decide_policy(mode: ExecutionMode, risk_level: RiskLevel) -> PolicyAction {
    match mode {
        ExecutionMode::Default => match risk_level {
            RiskLevel::Safe => PolicyAction::Execute,
            RiskLevel::Moderate => PolicyAction::Confirm,
            RiskLevel::Dangerous => PolicyAction::Reject,
        },
        ExecutionMode::Safe => match risk_level {
            RiskLevel::Safe => PolicyAction::Execute,
            RiskLevel::Moderate | RiskLevel::Dangerous => PolicyAction::Reject,
        },
        ExecutionMode::Force => match risk_level {
            RiskLevel::Safe | RiskLevel::Moderate => PolicyAction::Execute,
            RiskLevel::Dangerous => PolicyAction::Confirm,
        },
    }
}

pub fn classify_risk(command: &str) -> RiskAssessment {
    let command_lower = command.to_lowercase();

    if contains_any(&command_lower, &["sudo ", "sudo\n", "sudo\t"]) {
        return RiskAssessment {
            level: RiskLevel::Dangerous,
            reason: "uses sudo",
        };
    }

    if touches_system_or_config(&command_lower) {
        return RiskAssessment {
            level: RiskLevel::Dangerous,
            reason: "touches system or user config files",
        };
    }

    if has_high_impact_delete(&command_lower) {
        return RiskAssessment {
            level: RiskLevel::Dangerous,
            reason: "high-impact or broad delete operation",
        };
    }

    if has_non_standard_install(&command_lower) {
        return RiskAssessment {
            level: RiskLevel::Dangerous,
            reason: "installs non-standard tools",
        };
    }

    if has_external_api_call(&command_lower) {
        return RiskAssessment {
            level: RiskLevel::Moderate,
            reason: "calls external network/api",
        };
    }

    if has_known_popular_install(&command_lower) {
        return RiskAssessment {
            level: RiskLevel::Moderate,
            reason: "installs popular tooling",
        };
    }

    if has_low_impact_delete(&command_lower) {
        return RiskAssessment {
            level: RiskLevel::Moderate,
            reason: "deletes files",
        };
    }

    if contains_any(
        &command_lower,
        &[
            "docker start",
            "docker stop",
            "docker rm",
            "docker container",
            "docker compose up",
            "docker compose down",
            "docker run",
        ],
    ) {
        return RiskAssessment {
            level: RiskLevel::Safe,
            reason: "docker lifecycle command on developer machine",
        };
    }

    if is_safe_read_command(&command_lower) {
        return RiskAssessment {
            level: RiskLevel::Safe,
            reason: "read or common local file operation",
        };
    }

    RiskAssessment {
        level: RiskLevel::Moderate,
        reason: "unknown command pattern (conservative default)",
    }
}

const SAFE_COMMANDS: &[&str] = &[
    "cat", "ls", "dir", "tree", "find", "grep", "head", "tail",
    "wc", "echo", "touch", "mkdir", "cp", "mv", "stat", "file", "git",
];

fn is_safe_read_command(command: &str) -> bool {
    SAFE_COMMANDS.iter().any(|cmd| {
        command == *cmd
            || command.starts_with(&format!("{cmd} "))
            || command.starts_with(&format!("{cmd}\n"))
    })
}

fn touches_system_or_config(command: &str) -> bool {
    contains_any(
        command,
        &[
            "/etc/",
            "/usr/",
            "/bin/",
            "/sbin/",
            "/var/",
            "/boot/",
            "/root/",
            "~/.config/",
            "$home/.config/",
            ".config/",
        ],
    )
}

fn has_high_impact_delete(command: &str) -> bool {
    contains_any(
        command,
        &[
            "rm -rf /",
            "rm -rf ~",
            "rm -rf *",
            "find / -delete",
            "find . -delete",
            "shred ",
            "wipefs ",
            "mkfs",
        ],
    )
}

fn has_low_impact_delete(command: &str) -> bool {
    contains_any(command, &["rm ", "unlink ", "rmdir ", "trash "])
}

fn has_known_popular_install(command: &str) -> bool {
    contains_any(
        command,
        &[
            "npm install",
            "pnpm add",
            "yarn add",
            "pip install",
            "cargo add",
            "apt install",
            "dnf install",
            "brew install",
        ],
    )
}

fn has_non_standard_install(command: &str) -> bool {
    contains_any(command, &["curl ", "wget "])
        && contains_any(command, &["| sh", "| bash", "bash -c", "sh -c"])
}

fn has_external_api_call(command: &str) -> bool {
    contains_any(command, &["curl ", "wget ", "http://", "https://", "nc ", "ncat ", "telnet "])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_matrix_default_mode() {
        assert_eq!(
            decide_policy(ExecutionMode::Default, RiskLevel::Safe),
            PolicyAction::Execute
        );
        assert_eq!(
            decide_policy(ExecutionMode::Default, RiskLevel::Moderate),
            PolicyAction::Confirm
        );
        assert_eq!(
            decide_policy(ExecutionMode::Default, RiskLevel::Dangerous),
            PolicyAction::Reject
        );
    }

    #[test]
    fn test_policy_matrix_safe_mode() {
        assert_eq!(
            decide_policy(ExecutionMode::Safe, RiskLevel::Safe),
            PolicyAction::Execute
        );
        assert_eq!(
            decide_policy(ExecutionMode::Safe, RiskLevel::Moderate),
            PolicyAction::Reject
        );
        assert_eq!(
            decide_policy(ExecutionMode::Safe, RiskLevel::Dangerous),
            PolicyAction::Reject
        );
    }

    #[test]
    fn test_policy_matrix_force_mode() {
        assert_eq!(
            decide_policy(ExecutionMode::Force, RiskLevel::Safe),
            PolicyAction::Execute
        );
        assert_eq!(
            decide_policy(ExecutionMode::Force, RiskLevel::Moderate),
            PolicyAction::Execute
        );
        assert_eq!(
            decide_policy(ExecutionMode::Force, RiskLevel::Dangerous),
            PolicyAction::Confirm
        );
    }

    #[test]
    fn test_classifier_examples() {
        let safe = classify_risk("docker stop $(docker ps -q)");
        assert_eq!(safe.level, RiskLevel::Safe);

        let moderate = classify_risk("curl https://api.github.com/repos/rust-lang/rust");
        assert_eq!(moderate.level, RiskLevel::Moderate);

        let dangerous = classify_risk("sudo rm -rf /tmp/some_dir");
        assert_eq!(dangerous.level, RiskLevel::Dangerous);
    }

    #[test]
    fn test_folder_listing_commands_are_safe() {
        assert_eq!(classify_risk("ls").level, RiskLevel::Safe);
        assert_eq!(classify_risk("dir").level, RiskLevel::Safe);
        assert_eq!(classify_risk("tree").level, RiskLevel::Safe);
        assert_eq!(classify_risk("ls -la /tmp").level, RiskLevel::Safe);
        assert_eq!(classify_risk("dir /home").level, RiskLevel::Safe);
        assert_eq!(classify_risk("tree -L 2").level, RiskLevel::Safe);
        assert_eq!(classify_risk("stat somefile.txt").level, RiskLevel::Safe);
        assert_eq!(classify_risk("file image.png").level, RiskLevel::Safe);
    }
}
