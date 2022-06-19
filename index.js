const {
    displays_info, display_get_brightness, display_set_brightness, display_support_ddc
} = require("./ddc_enhanced_rs.node");

class Display {
    constructor(id) {
        this.id = id
    }

    get_brightness() {
        return display_get_brightness(this.id);
    }

    set_brightness(value) {
        return display_set_brightness(this.id, value);
    }

    does_support_ddc() {
        return display_support_ddc(this.id);
    }

    static info() {
        return displays_info();
    }
}

module.exports = Display;