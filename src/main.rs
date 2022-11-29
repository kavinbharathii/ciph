#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;

fn webdev(project_name: &str) {
    if !Path::new(project_name).exists() {
        let _dire = fs::create_dir(project_name).expect("Error encountered while creating file!");
    } else {
        println!("This project already exists");
        process::exit(0);
    }

    // ------------------------------------------------ html ------------------------------------------------ // 
    let mut html = File::create(format!("{}/{}", project_name, "index.html")).expect("Failed to create index.html");
    html.write_all(format!(r#"
<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link rel="stylesheet" href="style.css">
    <title>{}</title>
</head>

<body>
    <div id="dev">
        <h1>{} created successfully!</h1>
    </div>
</body>

</html>"#, project_name, project_name).as_bytes()).expect("Error while writing to index.html");

    // ------------------------------------------------ css ------------------------------------------------ // 
    let mut css = File::create(format!("{}/{}", project_name, "style.css")).expect("Failed to create style.css");
    css.write_all(r#"
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
    font-family: monospace, sans-serif;
}

:root {
    --bg-color : #121212;
    --fg-color: #9B37FF;
}

body,
main {
    background-color: var(--bg-color);
    color: var(--fg-color);
}

#dev {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100vw;
    height: 100vh;
}
"#.as_bytes()).expect("Error while writing to style.css");

    // ------------------------------------------------ html ------------------------------------------------ // 
    let mut js = File::create(format!("{}/{}", project_name, "app.js")).expect("Failed to create app.js");
    js.write_all(format!(r#"
console.log('{} created successfully!')
const dev = document.getElementById('dev')
"#, project_name).as_bytes()).expect("Error while writing to app.js");
}

fn main() {
    match std::env::args().nth(1) {
        Some(project_name) => {
            webdev(&project_name);
            println!("{} was created successfully!", project_name);
        },
        None => {
            println!("No project name given");
            process::exit(0);
        }
    }
}
