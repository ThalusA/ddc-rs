export interface DisplayValue {
    value: number;
    maximum: number;
}

export interface DisplayInfo {
    id: string;
    serial_number?: string;
    model_name?: string;
    model_id?: number;
    manufacturer_id?: string;
}

export default class Display {
    constructor(id: string);

    get_brightness(): DisplayValue;
    set_brightness(value: number);

    static info(): Array<DisplayInfo>;
}
