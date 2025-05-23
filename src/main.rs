// SPDX-FileCopyrightText: 2025 Rehpotsirhc
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dmenuargs: String = format!(
        "--dmenu=dmenu {}",
        args[1..]
            .join(" ")
            //            .replace(" ", r"\\ ")
            .replace("\\", r"\\\")
            .replace("`", r"\`")
            .replace("~", r"\~")
            .replace("!", r"\!")
            .replace("#", r"\#")
            .replace("$", r"\$")
            .replace("&", r"\&")
            .replace("*", r"\*")
            .replace("(", r"\(")
            .replace(")", r"\)")
            .replace("{", r"\{")
            .replace("}", r"\}")
            .replace("[", r"\[")
            .replace("]", r"\]")
            .replace("|", r"\|")
            .replace(";", r"\;")
            .replace("'", r"\'")
            .replace(r#"""#, r#"\""#)
            .replace("↩", r"\↩")
            .replace("<", r"\<")
            .replace(">", r"\>")
            .replace("?", r"\?")
    );
    let termargs: String = env::var("TERM").unwrap_or("alacritty".to_string());

    let _ = Command::new("j4-dmenu-desktop")
        .arg(dmenuargs)
        .arg("-t")
        .arg(termargs)
        .status()
        .expect("Failed to execute command");
}
