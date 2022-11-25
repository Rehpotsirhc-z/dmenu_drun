use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let finalargs: String = format!(
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

    //    println!("{:?}", format!("{}", finalargs));
    //    println!("{:?}", str::replace(&finalargs, " ", "_"));

    let _ = Command::new("j4-dmenu-desktop")
        .arg(finalargs)
        .status()
        .expect("Failed to execute command");
}
