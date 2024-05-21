use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    user_agent: UserAgent,
    ip: String,

    screen_width: usize,
    screen_height: usize,
    color_depth: usize,

    serif_width: usize,
    sans_serif_width: usize,
    monospace_width: usize,
    cursive_width: usize,
    fantasy_width: usize    
}

impl User {
    pub fn new(ua: UserAgent, ip: String, 
        screen_width: usize, screen_height: usize, color_depth: usize,
        serif_width: usize, sans_serif_width: usize, monospace_width: usize, cursive_width: usize, fantasy_width: usize) -> User {

            User {
                user_agent: ua,
                ip,

                screen_width,
                screen_height,
                color_depth,

                serif_width,
                sans_serif_width,
                monospace_width,
                cursive_width,
                fantasy_width  
            }
        }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserAgent {
    browser_name: String,
    browser_version: String,

    os_name: String,
    os_version: String,

    device_model: String,
    device_type: String,
    device_vendor: String,
    cpu_architecture: String,
}

impl UserAgent {
    pub fn new( browser_name: String, browser_version: String,
        os_name: String,os_version: String,
        device_model: String, device_type: String, device_vendor: String, cpu_architecture: String,) -> UserAgent {

            UserAgent {
                browser_name,
                browser_version,

                os_name,
                os_version,

                device_model,
                device_type,
                device_vendor,
                cpu_architecture,
            }
        }
}