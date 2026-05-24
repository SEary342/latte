use colored::*;

const SPLASH_LOGO: &str = r#"
 ___            __  ___________  ___________  _______  
|"  |          /""\("     _   ")("     _   ")/"     "| 
||  |         /    \)__/  \\__/  )__/  \\__/(: ______) 
|:  |        /' /\  \  \\_ /        \\_ /    \/    |   
 \  |___    //  __'  \ |.  |        |.  |    // ___)_  
( \_|:  \  /   /  \\  \\:  |        \:  |   (:      "| 
 \_______)(___/    \___)\__|         \__|    \_______)                                                   
"#;

const SMALL_LOGO: &str = r#"
  _         _   _       
 | |   __ _| |_| |_ ___ 
 | |__/ _` |  _|  _/ -_)
 |____\__,_|\__|\__\___|        
"#;

pub fn print_logo(large: bool) {
    if large {
        println!("{}", SPLASH_LOGO.truecolor(200, 150, 100));
    } else {
        println!("{}", SMALL_LOGO.truecolor(200, 150, 100));
    }

    let subtitle = "Logging And Task Tracking Engine";
    let separator = "─".repeat(45);

    println!("{}", subtitle.bold().truecolor(150, 150, 150));
    println!("{}", separator.dimmed());
    println!();
}

pub fn print_header(task_key: &str, task_desc: &str) {
    // Format the text into a string first
    let header = format!("\n{} ({})\n", task_key, task_desc);

    // Then print that string with your styles
    println!("{}", header.bold().purple());
}
