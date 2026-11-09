const PER_PROCESS_HEADER: &str = "Listed by owning process:";
const TEAMS_PROCESS_TAG: &str = "(MSTeams):";
const TEAMS_CALL_ASSERTION: &str = "\"Microsoft Teams Call in progress\"";

/// Parses `pmset -g assertions` output; Teams holds this assertion for the whole call.
/// Returns `None` when the output is not a complete listing, so a bad read is not mistaken for "call ended".
pub fn teams_call_in_progress(pmset_output: &str) -> Option<bool> {
    if !pmset_output.contains(PER_PROCESS_HEADER) {
        return None;
    }
    Some(
        pmset_output
            .lines()
            .any(|line| line.contains(TEAMS_PROCESS_TAG) && line.contains(TEAMS_CALL_ASSERTION)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const IN_CALL: &str = "Listed by owning process:
   pid 73363(MSTeams): [0x0003d8a80001a5e3] 00:28:38 NoIdleSleepAssertion named: \"Microsoft Teams Call in progress\"
   pid 73363(MSTeams): [0x0003d8a80005a5e4] 00:28:38 NoDisplaySleepAssertion named: \"Microsoft Teams Call in progress\"
   pid 412(powerd): [0x0003d8a800019999] 01:00:00 PreventUserIdleSystemSleep named: \"Powerd - Prevent sleep while display is on\"
";

    const NOT_IN_CALL: &str = "Listed by owning process:
   pid 412(powerd): [0x0003d8a800019999] 01:00:00 PreventUserIdleSystemSleep named: \"Powerd - Prevent sleep while display is on\"
   pid 901(coreaudiod): [0x0003d8a80001aaaa] 00:00:10 PreventUserIdleSystemSleep named: \"com.apple.audio.context.preventuseridlesleep\"
";

    #[test]
    fn detects_teams_call_assertion() {
        assert_eq!(teams_call_in_progress(IN_CALL), Some(true));
    }

    #[test]
    fn no_call_when_assertion_absent() {
        assert_eq!(teams_call_in_progress(NOT_IN_CALL), Some(false));
    }

    #[test]
    fn unknown_on_empty_output() {
        assert_eq!(teams_call_in_progress(""), None);
    }

    #[test]
    fn unknown_on_truncated_output() {
        assert_eq!(teams_call_in_progress("Assertion status system-wide:\n   BackgroundTask 0\n"), None);
    }

    #[test]
    fn ignores_other_process_using_same_name() {
        let spoofed = "Listed by owning process:\n   pid 555(evil): [0x1] 00:00:01 NoIdleSleepAssertion named: \"Microsoft Teams Call in progress\"\n";
        assert_eq!(teams_call_in_progress(spoofed), Some(false));
    }

    #[test]
    fn ignores_other_teams_assertions() {
        let other = "Listed by owning process:\n   pid 73363(MSTeams): [0x1] 00:00:05 PreventUserIdleSystemSleep named: \"Microsoft Teams playing audio\"\n";
        assert_eq!(teams_call_in_progress(other), Some(false));
    }
}
