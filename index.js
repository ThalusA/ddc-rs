const {
    displayGetVcpFeature, displaySetVcpFeature, displaySetTableVcpFeature, displayManagerGetByIndex, displayManagerList
} = require("./ddc_node_rs.node");

class Display {
    constructor(data) {
        if (Number.isInteger(data)) {
            const index = data;
            data = displayManagerGetByIndex(index);
        }
        this.index = data.index;
        this.backend = data.backend;
        if (data.hasOwnProperty('edidData')) this.edidData = data.edidData;
        if (data.hasOwnProperty('version')) this.version = data.version;
        if (data.hasOwnProperty('mccsVersion')) this.mccsVersion = data.mccsVersion;
        this.displayId = data.displayId;
        if (data.hasOwnProperty('serial')) this.serial = data.serial;
        if (data.hasOwnProperty('serialNumber')) this.serialNumber = data.serialNumber;
        if (data.hasOwnProperty('modelId')) this.modelId = data.modelId;
        if (data.hasOwnProperty('modelName')) this.modelName = data.modelName;
        if (data.hasOwnProperty('manufacturerId')) this.manufacturerId = data.manufacturerId;
        if (data.hasOwnProperty('manufactureYear')) this.manufactureYear = data.manufactureYear;
        if (data.hasOwnProperty('manufactureWeek')) this.manufactureWeek = data.manufactureWeek;
    }

    getVcpFeature(featureCode) {
        return displayGetVcpFeature(this.index, featureCode);
    }

    setVcpFeature(featureCode, value) {
        displaySetVcpFeature(this.index, featureCode, value);
    }

    setTableVcpFeature(featureCode, data, offset) {
        displaySetTableVcpFeature(this.index, featureCode, data, offset);
    }
}

class DisplayManager {
    constructor(queries) {
        if (Array.isArray(queries)) {
            this._queries = queries;
        } else if (queries === undefined || queries === null) {
            this._queries = []
        } else {
            this._queries = [queries]
        }
    }

    get queries() {
        return this._queries;
    }

    set queries(queries) {
        this._queries = queries;
    }

    addQueries(queries) {
        let new_queries;
        if (Array.isArray(queries)) {
            new_queries = queries;
        } else if (queries === undefined || queries === null) {
            new_queries = []
        } else {
            new_queries = [queries]
        }
        this.queries = this.queries + new_queries;
        return this;
    }

    static getByIndex(index) {
        const display = displayManagerGetByIndex(index);
        return new Display(display);
    }

    collect() {
        return displayManagerList.map(display => new Display(display));
    }

    list = this.collect;
}

module.exports = {
    Display,
    DisplayManager
};