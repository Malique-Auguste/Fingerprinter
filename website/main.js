class UserData {
    constructor() {
        this.user_agent = ""

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

    user.user_agent = UAParser()

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
    return user
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