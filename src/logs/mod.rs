use crate::{
    config::Config,
    utils::{ServerEvent, ServerEventBus},
};
use async_process::Command;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LogLine {
    pub line: String,
    pub level: Option<&'static str>,
}

fn resolve_log_level(line: &str) -> Option<&'static str> {
    line.split(": ").find_map(|piece| {
        Some(match piece {
            "Verbose" => "info",
            "Error" => "error",
            "Warning" => "warn",
            _ => return None,
        })
    })
}

pub fn start(event_bus: &ServerEventBus, config: &Config) {
    let refresh_interval = config.refresh_interval();
    let tx = event_bus.sender();

    tokio::spawn(async move {
        let mut timeout = tokio::time::interval(tokio::time::Duration::from_secs(refresh_interval));
        loop {
            timeout.tick().await;
            let out = Command::new("journalctl")
                .args(["-u", "vein-server.service", "-n", "800", "--no-pager"])
                .output()
                .await
                .map(|out| out.stdout);
            if let Err(e) = &out {
                tracing::error!("Unable to read server logs: {e}");
                #[cfg(not(debug_assertions))]
                {
                    continue;
                }
            }

            let out = if cfg!(debug_assertions) {
                LOGS.trim().bytes().collect::<Vec<u8>>()
            } else {
                out.unwrap_or_default()
            };

            let Ok(lines) = String::from_utf8(out) else {
                continue;
            };
            let lines = lines
                .lines()
                .map(|line| LogLine {
                    level: resolve_log_level(&line),
                    line: line.to_owned(),
                })
                .collect();

            let Ok(_) = tx
                .send(ServerEvent::Logs(lines))
                .inspect_err(|e| tracing::warn!("Event hub closed: {e}"))
            else {
                break;
            };
        }
    });
}

const LOGS: &str = r#"
lis 12 16:50:34 exp VeinServer.sh[199373]: [2025.11.12-15.45.34:876][187]LogVeinSaveGame: Swapped save in 0.006907 seconds.
lis 12 16:50:34 exp VeinServer.sh[199373]: [2025.11.12-15.50.32:933][846]LogRamjetSteamSocketsAPI: Verbose: Certificate expires in 43h50m at 1763120432 (current time 1762962632), will renew in 41h50m
lis 12 16:50:34 exp VeinServer.sh[199373]: [2025.11.12-15.50.33:596][885]LogVeinSaveGame: Saving all objects.
lis 12 16:50:34 exp VeinServer.sh[199373]: [2025.11.12-15.50.33:642][885]LogVeinSaveGame: Finished saving 46685 objects (0.045361 seconds).
lis 12 16:50:34 exp VeinServer.sh[199373]: [2025.11.12-15.50.33:776][885]LogVeinSaveGame: Cloned save game in 0.134054 seconds. Created 1 objects by duplication.
lis 12 16:50:34 exp VeinServer.sh[199373]: [2025.11.12-15.50.33:776][885]LogVeinSaveGame: Saved autosave game to disk (0.134081 seconds).
lis 12 16:50:34 exp VeinServer.sh[199373]: [2025.11.12-15.50.33:776][885]LogVeinSaveGame: Saved to slot Server.
lis 12 15:55:33 exp VeinServer.sh[199373]: [2025.11.12-14.50.17:186][736]LogRamjetNetworking: Warning: Failed to heartbeat (code 504).
lis 12 12:57:47 exp VeinServer.sh[199373]: [2025.11.12-11.57.37:467][541]LogRamjetOptimization: Error: Failed to register tick (invalid object) (StaggerComponent)
lis 12 13:00:22 exp VeinServer-Linux-DebugGame[199373]: LogNetPackageMap: Error: GetObjectFromNetGUID: Failed to find outer. FullNetGUIDPath: [5155]/Game/Vein/BuildObjects/Workbenches/BO_Workbench.[5157]BO_Workbench
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_lines() {
        let lines = r#"
lis 12 16:50:34 exp VeinServer.sh[199373]: [2025.11.12-15.45.34:876][187]LogVeinSaveGame: Swapped save in 0.006907 seconds.
lis 12 16:50:34 exp VeinServer.sh[199373]: [2025.11.12-15.50.32:933][846]LogRamjetSteamSocketsAPI: Verbose: Certificate expires in 43h50m at 1763120432 (current time 1762962632), will renew in 41h50m
lis 12 16:50:34 exp VeinServer.sh[199373]: [2025.11.12-15.50.33:596][885]LogVeinSaveGame: Saving all objects.
lis 12 16:50:34 exp VeinServer.sh[199373]: [2025.11.12-15.50.33:642][885]LogVeinSaveGame: Finished saving 46685 objects (0.045361 seconds).
lis 12 16:50:34 exp VeinServer.sh[199373]: [2025.11.12-15.50.33:776][885]LogVeinSaveGame: Cloned save game in 0.134054 seconds. Created 1 objects by duplication.
lis 12 16:50:34 exp VeinServer.sh[199373]: [2025.11.12-15.50.33:776][885]LogVeinSaveGame: Saved autosave game to disk (0.134081 seconds).
lis 12 16:50:34 exp VeinServer.sh[199373]: [2025.11.12-15.50.33:776][885]LogVeinSaveGame: Saved to slot Server.
lis 12 15:55:33 exp VeinServer.sh[199373]: [2025.11.12-14.50.17:186][736]LogRamjetNetworking: Warning: Failed to heartbeat (code 504).
lis 12 12:57:47 exp VeinServer.sh[199373]: [2025.11.12-11.57.37:467][541]LogRamjetOptimization: Error: Failed to register tick (invalid object) (StaggerComponent)
lis 12 13:00:22 exp VeinServer-Linux-DebugGame[199373]: LogNetPackageMap: Error: GetObjectFromNetGUID: Failed to find outer. FullNetGUIDPath: [5155]/Game/Vein/BuildObjects/Workbenches/BO_Workbench.[5157]BO_Workbench
"#.trim();
        let levels = lines.lines().map(resolve_log_level).collect::<Vec<_>>();
        let expected = vec![
            None,
            Some("info"),
            None,
            None,
            None,
            None,
            None,
            Some("warn"),
            Some("error"),
            Some("error"),
        ];

        assert_eq!(levels, expected);
    }
}
