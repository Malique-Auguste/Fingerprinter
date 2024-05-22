use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Hash, Clone)]
pub struct User {
    pub id: u64,
    user_agent: UserAgent,
    pub ip: String,

    screen_width: u64,
    screen_height: u64,
    color_depth: u64,

    serif_width: u64,
    sans_serif_width: u64,
    monospace_width: u64,
    cursive_width: u64,
    fantasy_width: u64    
}

impl User {
    pub fn new(id: u64, ua: UserAgent, ip: String, 
        screen_width: u64, screen_height: u64, color_depth: u64,
        serif_width: u64, sans_serif_width: u64, monospace_width: u64, cursive_width: u64, fantasy_width: u64) -> User {

        User {
            id,
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

    pub fn compare(&self, other: &User) -> f64 {
        let mut result: f64 = 0.0;

        /*
        There are 17 unique properties.
        UA is likely to be least important as they are slightly randomised by the browsers for various reasons (security and compatability)
        Therefor the remaiing properties are to be prioritized. (10%)
        
        IP also changes if the router is turned off an dthen on, but this is less likely so it shouldn't be as priotized a smost propertis, but more so than UA. (10%)

        Scree dimensions and fonts will be treated as 2 "chunks" of importance (40% each)
        */

        result = self.user_agent.compare(&other.user_agent) / 10.0;

        if self.ip == other.ip {
            result += 10.0
        }

        if (self.screen_height == other.screen_height) {
            result += 10.0
        }

        if (self.screen_width == other.screen_width) {
            result += 10.0
        }

        if (self.color_depth == other.color_depth) {
            result += 10.0
        }

        if (self.serif_width == other.serif_width) {
            result += 10.0
        }

        if (self.sans_serif_width == other.sans_serif_width) {
            result += 10.0
        }

        if  (self.monospace_width == other.monospace_width) {
            result += 10.0
        }

        if (self.cursive_width == other.cursive_width) {
            result += 10.0
        }
        
        if self.fantasy_width == other.fantasy_width {
            result += 10.0
        }

        result
    }
}

#[derive(Debug, Deserialize, Serialize, Hash, Clone)]
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

    pub fn compare(&self, other: &UserAgent) -> f64 {
        let mut result: f64 = 0.0;

        if self.browser_name == other.browser_name {
            result += 1.0;
            if self.browser_version == other.browser_version {
                result += 1.0
            }
        } 
        
        if self.os_name == other.os_name {
            result += 1.0;
            if self.os_version == other.os_version {
                result += 1.0;
            }
        }

        if self.device_model == other.device_model {
            result += 1.0;
            if self.device_type == other.device_type {
                result += 1.0;
            }
        }

        if self.device_vendor == other.device_vendor {
            result += 1.0
        }

        if self.cpu_architecture == other.cpu_architecture {
            result += 1.0
        }

        result = (result / 8.0) * 100.0;

        result
    }
}