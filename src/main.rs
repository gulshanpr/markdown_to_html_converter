/*
todos:
- read a markdown file
- parse that md file
- detect and convert the files into html
- create a html file

*/

use std::fs::File;
use std::fs;
use std::io::{Write, Error};

fn main() -> Result<(), Error> {
    let contents = fs::read_to_string("test.md")
        .expect("Should have been able to read the file");

    let mut output = File::create("Result.html")?;
    
    let mut buffer = String::new();

        for line in contents.lines() {
            if line.starts_with("##") {
                let sliced_part = &line[3..];
                let ans = format!("<h2>{}</h2>\n", sliced_part);
                buffer.push_str(&ans);
            } else if line.starts_with("#") {
                let sliced_part = &line[2..];
                let ans = format!("<h1>{}</h1>\n", sliced_part);
                buffer.push_str(&ans);
            } else if line.starts_with("-") {
                let sliced_part = &line[2..];
                let ans = format!("<li>{}</li>\n", sliced_part);
                buffer.push_str(&ans);
            } else if line.starts_with("**") {
                let sliced_part_1 = &line[2..];
                let sliced_part_1_len = sliced_part_1.len();
                let sliced_part_2 = &sliced_part_1[0..sliced_part_1_len-2];
                let ans = format!("<b>{}</b>\n", sliced_part_2);
                buffer.push_str(&ans);
            } else if line.trim() == "---" || line.trim() == "***" || line.trim() == "___"  {
                let ans = format!("<hr>\n");
                buffer.push_str(&ans);
            }
                // else if line.starts_with("*") {
                    //     let sliced_part = &line[3..];
                    //     let ans = format!("<h2>{}</h2>", sliced_part);
                    //     let mut _output = std::fs::write("Result.html", ans).expect("not able to write");
                    // }
                }
                //  else if line.starts_with("[") {
                //     let sliced_part = &line[..];
                //     let ans = format!("<h2>{}</h2>", sliced_part);
                //     let mut _output = std::fs::write("Result.html", ans).expect("not able to write");
                // } 
                

        output.write_all(buffer.as_bytes())?;
    
    Ok(())

}


