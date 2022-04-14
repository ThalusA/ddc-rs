import {
    display_new, display_list, display_get_brightness,
    display_get_contrast, display_set_brightness, display_set_contrast
} from ".";

export default class Display {
    constructor(id) {
        this.display = display_new(id)
    }

    get brightness() {
        return display_get_brightness(this.display);
    }

    get contrast() {
        return display_get_contrast(this.display);
    }

    set_brightness(value) {
        display_set_brightness(this.display, value);
    }

    set_contrast(value) {
        display_set_contrast(this.display, value);
    }

    static list() {
        return display_list();
    }
}