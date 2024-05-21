class UserAgent {
    constructor() {
        this.browser_name = "",
        this.browser_version = "",
    
        this.os_name = "",
        this.os_version = "",
    
        this.device_model = "",
        this.device_type = "",
        this.device_vendor = "",
        this.cpu_architecture = ""
    }
}


class UserData {
    constructor() {
        this.user_agent = new UserAgent,

        this.ip = "",

        this.screen_width = 0,
        this.screen_height = 0,
        this.color_depth = 0,

        this.serif_width = 0,
        this.sans_serif_width = 0,
        this.monospace_width = 0,
        this.cursive_width = 0,
        this.fantasy_width = 0
    }
}

let user;

function generate_user_data() {
    user = new UserData

    user.user_agent = get_user_agent()    

    user.screen_width = window.screen.width;
    user.screen_height = window.screen.height;
    user.color_depth = window.screen.colorDepth;

    let font_sizes = get_font_sizes()

    user.serif_width = font_sizes[0]
    user.sans_serif_width = font_sizes[1]
    user.monospace_width = font_sizes[2]
    user.cursive_width = font_sizes[3]
    user.fantasy_width = font_sizes[4]

    document.getElementById("message").innerText = JSON.stringify(user, null, 2);
    document.getElementsByTagName("form")[0].action = "/user_id/" + JSON.stringify(user)

    return user
}

function get_user_agent() {
    let parsed_user_agent = UAParser()
    console.log(parsed_user_agent)
    let user_agent = new UserAgent;

    if (parsed_user_agent.browser != undefined) {
        user_agent.browser_name = parsed_user_agent.browser.name
    }

    if (parsed_user_agent.browser != undefined) {
        user_agent.browser_version = parsed_user_agent.browser.version
    }

    if (parsed_user_agent.os != undefined) {
        user_agent.os_name = parsed_user_agent.os.name
    }
    
    if(parsed_user_agent.os != undefined) {
        user_agent.os_version = parsed_user_agent.os.version
    }

    if(parsed_user_agent.device != undefined) {
        user_agent.device_model = parsed_user_agent.device.model
    }

    if(parsed_user_agent.device != undefined) {
        user_agent.device_type = parsed_user_agent.device.type
    }

    if(parsed_user_agent.device != undefined) {
        user_agent.device_vendor = parsed_user_agent.device.vendor
    }

    if(parsed_user_agent.cpu.architecture != undefined) {
        console.log("fs")
        user_agent.cpu_architecture = parsed_user_agent.cpu.architecture
    }

    return user_agent
}

function get_font_sizes() {
    let tester_text = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
    let fingerprinter = document.getElementById("fingerprinter")
    let font_sizes = []

    fingerprinter.innerText = tester_text

    for(font_name of ["serif", "sans-serif", "monospace", "cursive", "fantasy"]) {
        fingerprinter.style.fontFamily = font_name
        font_sizes.push(fingerprinter.offsetWidth)
    }

    fingerprinter.style.display = "none"
    return font_sizes

}