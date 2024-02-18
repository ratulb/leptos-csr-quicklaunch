fn main() {
    use std::process::Command;
    if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") {
        // Additional check for Arch Linux
        if std::fs::read_to_string("/etc/os-release")
            .map(|content| content.contains("Arch Linux"))
            .unwrap_or(false)
        {
            println!("Running on Arch Linux");
            check_install!("cargo make --version", "sudo pacman -S cargo-make");
        } else {
            println!("Running on Linux (x86_64)");
            check_install!(
                "cargo make --version",
                "cargo install --no-default-features --force cargo-make"
            );
        }
    } else {
        println!("Not running on Linux (x86_64)");
        check_install!(
            "cargo make --version",
            "cargo install --no-default-features --force cargo-make"
        );
    }
    println!("Waiting here");
    //check_install!("cargo make wasm-bindgen");
    println!("Done .....");
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
            Ok(output) if output.stdout.len() > 0 => println!(
                "[{}] Executed with resonse {:?}",
                install,
                std::str::from_utf8(&output.stdout)
            ),
            Ok(output) if !output.status.success() && output.stderr.len() > 0 => println!(
                "[{}] Executed with resonse code {:?} and message {:?} ",
                install,
                output.status,
                std::str::from_utf8(&output.stderr)
            ),
            Ok(output) => eprintln!(
                "Install command [{}] execution response {:?}",
                install, output
            ),
            Err(err) => eprintln!("Error {:?} executing [{}]", err, install),
        }
        // }
        // }
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
                println!(
                    "[{}] Executed with resonse {:?}",
                    install,
                    std::str::from_utf8(&output.stdout)
                );
                true
            }

            Ok(output) if !output.status.success() && output.stderr.len() > 0 => {
                println!(
                    "[{}] Executed with resonse code {:?} and message {:?} ",
                    install,
                    output.status,
                    std::str::from_utf8(&output.stderr)
                );
                false
            }

            Ok(output) => {
                println!(
                    "Install command [{}] execution response {:?}",
                    install, output
                );
                true
            }
            Err(err) => {
                eprintln!("Error {:?} executing [{}]", err, install);
                false
            }
        };

        match present {
            true => println!("[{}] is present", check),
            false => {
                let installed = if cfg!(target_os = "windows") {
                    Command::new("cmd").args(["/C", install]).output()
                } else {
                    Command::new("sh").arg("-c").arg(install).output()
                };
                match installed {
                    Ok(output) if output.stdout.len() > 0 => println!(
                        "[{}] Executed with resonse {:?}",
                        install,
                        std::str::from_utf8(&output.stdout)
                    ),

                    Ok(output) if !output.status.success() && output.stderr.len() > 0 => println!(
                        "[{}] Executed with resonse code {:?} and message {:?} ",
                        install,
                        output.status,
                        std::str::from_utf8(&output.stderr)
                    ),
                    Ok(output) => eprintln!(
                        "Install command [{}] execution response {:?}",
                        install, output
                    ),
                    Err(err) => eprintln!("Error {:?} executing [{}]", err, install),
                }
            }
        }
    }};
}
