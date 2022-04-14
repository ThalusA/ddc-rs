export interface DisplayValue {
    value: number;
    maximum: number;
}

export interface DisplayInfo {
    id: string;
    serial_number?: string;
    model_name?: string;
    manufacturer_id?: string;
}

export default class Display {
    constructor(id: string);

    get brightness(): DisplayValue;
    get contrast(): DisplayValue;

    set_brightness(value: number);
    set_contrast(value: number);

    static list(): Array<DisplayInfo>;
}
