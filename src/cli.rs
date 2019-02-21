use crate::action::Action::*;
use crate::action::Flag;
use crate::action::Action;
use std::collections::HashMap;

use std::env;
use colored::*;

pub struct Settings {
    pub actions: Vec<Action>,
    pub flags: HashMap<Flag, bool>,
}

pub fn parse() -> ((String, String), Settings, Vec<String>) {

    let mut settings = Settings {
        actions: Vec::new(),
        flags: HashMap::new(),
    };
    let args: Vec<String> = env::args().collect();

    if &args[1] == "--help" || &args[1] == "-h" || &args[1] == "/h" || &args[1] == "help" {
        println!(
            "Syntax:
    {imagene} {o}infile{c} ...{o}action{c}:{o}value{c}... ...{o}flag{c}... {o}outfile{c}

Available Actions:
    contrast:{o}float{c}  {comment} Increase contrast
    resize:{o}int,int{c}  {comment} Resize an image, leave one of the ints empty to auto scale it
    append:{o}string{c}   {comment} Add another image next to source image

Available Flags:
    shrink            {comment} Appended images will inherit the height of the shortest

Examples:
     {comment} Increases the contrast of the original image and adds an extra image next to it
     {imagene} {infile} contrast:2 append:extra_image.png {outfile}

     {comment} Adds an extra image next to in_file.png and new image inherit height of the smallest
     {imagene} {infile} {shrink} append:extra_image.png {outfile}

     {comment} Sets width to 2000 and automatically scales height to keep aspect ratio
     {imagene} {infile} resize:2000,0 {outfile}

     {comment} Overwrites an image with increased contrast
     {imagene} {infile} contrast:2 {infile} ",
            imagene = "imagene".green(),
            shrink = "shrink".purple(),
            infile = "in_file.png".blue(),
            outfile = "out_file.png".blue(),
            comment = "//".blue(),
            o = "<".green(),
            c = ">".green()
        );
        std::process::exit(0);
    }

    let infile = &args[1];
    let outfile = &args[args.len() - 1];
    println!("Using infile {} and outfile {}", infile, outfile);

    let mut images: Vec<String> = Vec::new();
    images.push(infile.to_owned());

    for arg in &args[2..args.len() - 1] {
        let step = split_kv(arg);

        match step {

            Ok((k, v)) => {
                // Key:Value based argument
                settings.actions.push(match k {
                    "contrast" => Contrast(v.to_owned().parse::<f32>().expect(&format!(
                        "{}: Invalid value for {}",
                        k,
                        v
                    ))),
                    "brightness" => Brightness(v.to_owned().parse::<i32>().expect(&format!(
                        "{}: Invalid value for {}",
                        k,
                        v
                    ))),
                    "resize" => {
                        let resize_arguments: Vec<&str> = v.split(",").collect();
                        Scale(
                            resize_arguments[0].to_owned().parse::<u32>().expect(
                                &format!(
                                "{}: Invalid value for {}",
                                resize_arguments[0],
                                v,
                        ),
                            ),
                            resize_arguments[1].to_owned().parse::<u32>().expect(
                                &format!(
                                "{}: Invalid value for {}",
                                resize_arguments[1],
                                v,
                        ),
                            ),
                        )
                    }
                    "append" => {
                        images.push(v.to_owned());
                        Append(v.to_owned())
                    }
                    &_ => {
                        println!("{}: action not found", k);
                        std::process::exit(1)
                    }
                });

            }
            Err(err) => {
                // Flag based argument
                match arg.as_ref() {
                    "shrink" => {
                        settings.flags.insert(Flag::Shrink, true);
                    }
                    "vertical" => {
                        settings.flags.insert(Flag::Vertical, true);
                    }
                    &_ => {
                        println!("Unrecognized argument \"{}\"\n{}", arg, err);
                    }
                };
            }
        }
    }

    ((infile.to_owned(), outfile.to_owned()), settings, images)
}

fn split_kv(s: &str) -> Result<(&str, &str), String> {
    let split: Vec<&str> = s.split(":").collect();
    if split.len() != 2 {
        let mut r = String::from("Parse error on argument: ");
        r.push_str(s);
        Err(r)
    } else {
        Ok((&split[0], &split[1]))
    }
}
