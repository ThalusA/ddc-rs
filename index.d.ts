export interface DisplayValue {
    value: number;
    maximum: number;
}

export interface DisplayInfo {
    backend: string;
    edid_data?: ArrayBuffer;
    version?: string;
    mccs_version?: string;
    id: number;
    display_id: string;
    serial?: number
    serial_number?: string;
    model_id?: number;
    model_name?: string;
    manufacturer_id?: string;
    manufacture_year?: number;
    manufacture_week?: number;
}

export default class Display {
    constructor(id: number);

    get_brightness(): DisplayValue;
    set_brightness(value: number);

    static info(): Array<DisplayInfo>;
}
