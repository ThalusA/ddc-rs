export interface DisplayValue {
    value: number;
    maximum: number;
}

export interface DisplayInfo {
    display_id: string;
    id: number;
    serial_number?: string;
    model_name?: string;
    model_id?: number;
    manufacturer_id?: string;
}

export default class Display {
    constructor(id: number);

    get_brightness(): DisplayValue;
    set_brightness(value: number);
    does_support_ddc(): boolean;

    static info(): Array<DisplayInfo>;
}
