pub struct User {
    ua: UserAgent,

    screen_width: usize,
    screen_height: usize,
    color_depth: usize,

    serif_width: usize,
    sans_serif_width: usize,
    monospace_width: usize,
    cursive_width: usize,
    fantasy_width: usize    
}

pub struct UserAgent {
    browser_name: String,
    browser_version: String,
    os_name: String,
    os_version: String,
    device_model: String,
    device_type: String,
    cpu_architecture: String,
}