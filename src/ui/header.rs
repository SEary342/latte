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

pub fn print_logo() {
    println!("{}", SPLASH_LOGO.truecolor(200, 150, 100));

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
