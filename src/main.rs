#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;

// ====================================================== Webdev ====================================================== //

fn webdev(project_name: &str) {
    if !Path::new(project_name).exists() {
        let _dire = fs::create_dir(project_name).expect(format!("Error encountered while creating project: {}!", project_name).as_str());
    } else {
        println!("{} already exists", project_name);
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

    <script src="app.js"></script>
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

// ====================================================== Neuraldev ====================================================== //

fn neuraldev(project_name: &str) {
    if !Path::new(project_name).exists() {
        let _dire = fs::create_dir(project_name).expect(format!("Error encountered while creating project: {}!", project_name).as_str());
    } else {
        println!("{} already exists", project_name);
        process::exit(0);
    }

    let mut py = File::create(format!("{}/{}", project_name, "main.py")).expect("Failed to create main.py");
    py.write_all(r##"

# Tensorlflow imports
import tensorflow as tf
from tensorflow import keras

# Utitliy imports
import json
import numpy as np
import pandas as pd

model = tf.keras.Sequential([
    # Your layers here...
    tf.keras.layers.Dense(24, activation='relu'),
    tf.keras.layers.Dense(1, activation='sigmoid')
])

model.compile(loss='binary_crossentropy',optimizer='adam',metrics=['accuracy'])

"##.as_bytes()).expect("Error while writing to main.py");

    let json = File::create(format!("{}/{}", project_name, "data.json")).expect("Failed to create data.json");

}

// ====================================================== cv-dev ====================================================== //

fn cvdev(project_name: &str) {
    if !Path::new(project_name).exists() {
        let _dire = fs::create_dir(project_name).expect(format!("Error encountered while creating project: {}!", project_name).as_str());
    } else {
        println!("{} already exists", project_name);
        process::exit(0);
    }

    let mut py = File::create(format!("{}/{}", project_name, "main.py")).expect("Failed to create main.py");
    py.write_all(r##"

# Required libraries
import cv2
from PIL import Image, ImageOps
import numpy as np

video = cv2.VideoCapture(0)

# Video loop
while True:
    ret, frame = video.read()
    img = cv2.cvtColor(frame, cv2.COLOR_BGR2RGB)
    im_pil = Image.fromarray(img)
    im_np = np.asarray(im_pil)
    image = ImageOps.fit(im_pil, size)
    image_array = np.asarray(image)
    normalized_image_array = (image_array.astype(np.float32) / 127.0) - 1

    if cv2.waitKey(1) & 0xFF == ord('q'):
        break

video.release()
cv2.destroyAllWindows()
"##.as_bytes()).expect("Error while writing to main.py");
}

fn main() {

    match std::env::args().nth(1) {
        Some(project_name) => {
            match std::env::args().nth(2) {
                Some(project_mode) => {
                    match project_mode.as_str() {
                        "-web" => webdev(&project_name),
                        "-nn" => neuraldev(&project_name),
                        "-cv" => cvdev(&project_name),
                        _ => {
                            println!("Provide a proper flag: [-web] [-nn] [-cv]");
                            process::exit(0)
                        }
                    }
                },
                None => {
                    println!("No project type given: Include [-web] [-nn] [-cv]");
                    process::exit(0);
                }
            }
        },

        None => {
            println!("No project name given");
            process::exit(0);
        }
    }
}
