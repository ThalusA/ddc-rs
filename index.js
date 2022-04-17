const {
    display_info, display_get_brightness, display_set_brightness
} = require("./ddc_enhanced_rs.node");

class Display {
    constructor(id) {
        this.display_id = id
    }

    get_brightness() {
        return display_get_brightness(this.display_id);
    }

    set_brightness(value) {
        return display_set_brightness(this.display_id, value);
    }

    static info() {
        return display_info();
    }
}

module.exports = Display;