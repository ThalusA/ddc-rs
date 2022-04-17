const {
    display_info, display_get_brightness, display_set_brightness
} = require("./ddc_enhanced_rs.node");

class Display {
    constructor(uuid) {
        this.display_uuid = uuid
    }

    get_brightness() {
        return display_get_brightness(this.display_uuid);
    }

    set_brightness(value) {
        return display_set_brightness(this.display_uuid, value);
    }

    static info() {
        return display_info();
    }
}

module.exports = Display;