use crate::app::{GraphicsMode, Rtd3Level};
use anyhow::{anyhow, Result};
use std::process::Command;

pub fn query_mode() -> Result<Option<GraphicsMode>> {
    let output = Command::new("envycontrol").arg("--query").output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("not found") || stderr.contains("No such file") {
            return Err(anyhow!("envycontrol not installed"));
        }
        return Ok(None);
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();

    if stdout.contains("integrated") {
        Ok(Some(GraphicsMode::Integrated))
    } else if stdout.contains("hybrid") {
        Ok(Some(GraphicsMode::Hybrid))
    } else if stdout.contains("nvidia") {
        Ok(Some(GraphicsMode::Nvidia))
    } else {
        Ok(None)
    }
}

pub struct SwitchOptions {
    pub mode: GraphicsMode,
    pub rtd3_enabled: bool,
    pub rtd3_level: Rtd3Level,
    pub force_comp: bool,
    pub coolbits_enabled: bool,
    pub coolbits_value: u8,
}

pub fn switch_mode(options: SwitchOptions) -> Result<String> {
    let mut args = vec!["-s".to_string(), options.mode.to_string()];

    match options.mode {
        GraphicsMode::Hybrid if options.rtd3_enabled => {
            args.push("--rtd3".to_string());
            args.push(options.rtd3_level.value().to_string());
        }
        GraphicsMode::Nvidia => {
            if options.force_comp {
                args.push("--force-comp".to_string());
            }
            if options.coolbits_enabled {
                args.push("--coolbits".to_string());
                args.push(options.coolbits_value.to_string());
            }
        }
        _ => {}
    }

    args.push("--verbose".to_string());

    let args_str = args.join(" ");
    let output = Command::new("pkexec")
        .arg("sh")
        .arg("-c")
        .arg(format!("yes | envycontrol {}", args_str))
        .output()?;

    if output.status.success() {
        Ok(format!(
            "Switched to {} mode. Please reboot for changes to take effect.",
            options.mode
        ))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow!("Failed to switch mode: {}", stderr))
    }
}

pub fn reset() -> Result<String> {
    let output = Command::new("pkexec")
        .arg("sh")
        .arg("-c")
        .arg("yes | envycontrol --reset --verbose")
        .output()?;

    if output.status.success() {
        Ok("Reset successful. Please reboot for changes to take effect.".to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(anyhow!("Failed to reset: {}", stderr))
    }
}

pub fn is_envycontrol_installed() -> bool {
    Command::new("which")
        .arg("envycontrol")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub fn reboot() -> Result<()> {
    Command::new("systemctl").arg("reboot").spawn()?;
    Ok(())
}

#[derive(Debug, Clone)]
pub struct GpuInfo {
    pub name: String,
    pub temperature: String,
    pub memory_used: String,
    pub memory_total: String,
}

impl GpuInfo {
    pub fn memory_display(&self) -> String {
        format!("{} / {} MiB", self.memory_used, self.memory_total)
    }
}

pub fn query_gpu_info() -> Option<GpuInfo> {
    let output = Command::new("nvidia-smi")
        .args([
            "--query-gpu=name,temperature.gpu,memory.used,memory.total",
            "--format=csv,noheader,nounits",
        ])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let parts: Vec<&str> = stdout.trim().split(',').map(|s| s.trim()).collect();

    if parts.len() >= 4 {
        Some(GpuInfo {
            name: parts[0].to_string(),
            temperature: format!("{}°C", parts[1]),
            memory_used: parts[2].to_string(),
            memory_total: parts[3].to_string(),
        })
    } else {
        None
    }
}
