fn main() {
    use std::process::Command;
    println!("cargo:rerun-if-changed=build.tmp");
    fn log(msg: String) {
        if let Ok(_) = std::env::var("DEBUG") {
            println!("{msg}");
        }
    }
    if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") {
        // Additional check for Arch Linux
        if std::fs::read_to_string("/etc/os-release")
            .map(|content| content.contains("Arch Linux"))
            .unwrap_or(false)
        {
            log("Running on Arch Linux".to_string());
            check_install!("cargo make --version", "sudo pacman -S cargo-make");
        } else {
            log("Running on Linux (x86_64)".to_string());
            check_install!(
                "cargo make --version",
                "cargo install --no-default-features --force cargo-make"
            );
        }
    } else {
        log("Not running on Linux (x86_64)".to_string());
        check_install!(
            "cargo make --version",
            "cargo install --no-default-features --force cargo-make"
        );
    }
}
#[macro_export]
macro_rules! check_install {
    ($install: expr) => {{
        let install: &'static str = $install;
        let installed = if cfg!(target_os = "windows") {
            Command::new("cmd").args(["/C", install]).output()
        } else {
            Command::new("sh").arg("-c").arg(install).output()
        };
        match installed {
            Ok(output) if output.stdout.len() > 0 => log(format!(
                "[{}] Executed with resonse {:?}",
                install,
                std::str::from_utf8(&output.stdout)
            )),
            Ok(output) if !output.status.success() && output.stderr.len() > 0 => log(format!(
                "[{}] Executed with resonse code {:?} and message {:?} ",
                install,
                output.status,
                std::str::from_utf8(&output.stderr)
            )),
            Ok(output) => log(format!(
                "Install command [{}] execution response {:?}",
                install, output
            )),
            Err(err) => log(format!("Error {:?} executing [{}]", err, install)),
        }
    }};

    ($check:expr, $install: expr) => {{
        let check: &'static str = $check;
        let install: &'static str = $install;
        let present = if cfg!(target_os = "windows") {
            Command::new("cmd").args(["/C", check]).output()
        } else {
            Command::new("sh").arg("-c").arg(check).output()
        };

        let present = match present {
            Ok(output) if output.stdout.len() > 0 => {
                log(format!(
                    "[{}] Executed with resonse {:?}",
                    install,
                    std::str::from_utf8(&output.stdout)
                ));
                true
            }

            Ok(output) if !output.status.success() && output.stderr.len() > 0 => {
                log(format!(
                    "[{}] Executed with resonse code {:?} and message {:?} ",
                    install,
                    output.status,
                    std::str::from_utf8(&output.stderr)
                ));
                false
            }

            Ok(output) => {
                log(format!(
                    "Install command [{}] execution response {:?}",
                    install, output
                ));
                true
            }
            Err(err) => {
                log(format!("Error {:?} executing [{}]", err, install));
                false
            }
        };

        match present {
            true => log(format!("[{}] is present", check)),
            false => {
                let installed = if cfg!(target_os = "windows") {
                    Command::new("cmd").args(["/C", install]).output()
                } else {
                    Command::new("sh").arg("-c").arg(install).output()
                };
                match installed {
                    Ok(output) if output.stdout.len() > 0 => log(format!(
                        "[{}] Executed with resonse {:?}",
                        install,
                        std::str::from_utf8(&output.stdout)
                    )),

                    Ok(output) if !output.status.success() && output.stderr.len() > 0 => {
                        log(format!(
                            "[{}] Executed with resonse code {:?} and message {:?} ",
                            install,
                            output.status,
                            std::str::from_utf8(&output.stderr)
                        ))
                    }
                    Ok(output) => log(format!(
                        "Install command [{}] execution response {:?}",
                        install, output
                    )),
                    Err(err) => log(format!("Error {:?} executing [{}]", err, install)),
                }
            }
        }
    }};
}
