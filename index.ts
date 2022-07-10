/* eslint-disable no-unused-vars */

import {
  displayGetVcpFeature, displaySetVcpFeature, displaySetTableVcpFeature, displayManagerGetByIndex, displayManagerList
} from './ddc_rs.node'

import {
  DisplayData, VCPValue, Query
} from './types'

export {
  VCPValue, Query, DisplayData, Continuous, NonContinuous, Table, VCPFeatures, QueryType, VCPFeatureCode, VCPValueType
} from './types'

export class Display {
  constructor (data: number | DisplayData) {
    if (Number.isInteger(data) && !Number.isNaN(data)) {
      const index = data as number
      data = displayManagerGetByIndex(index)
    }
    const effectiveData = data as DisplayData
    this.index = effectiveData.index
    this.backend = effectiveData.backend
    if (Object.hasOwn(effectiveData, 'edidData')) this.edidData = effectiveData.edidData
    if (Object.hasOwn(effectiveData, 'version')) this.version = effectiveData.version
    if (Object.hasOwn(effectiveData, 'mccsVersion')) this.mccsVersion = effectiveData.mccsVersion
    this.displayId = effectiveData.displayId
    if (Object.hasOwn(effectiveData, 'serial')) this.serial = effectiveData.serial
    if (Object.hasOwn(effectiveData, 'serialNumber')) this.serialNumber = effectiveData.serialNumber
    if (Object.hasOwn(effectiveData, 'modelId')) this.modelId = effectiveData.modelId
    if (Object.hasOwn(effectiveData, 'modelName')) this.modelName = effectiveData.modelName
    if (Object.hasOwn(effectiveData, 'manufacturerId')) this.manufacturerId = effectiveData.manufacturerId
    if (Object.hasOwn(effectiveData, 'manufactureYear')) this.manufactureYear = effectiveData.manufactureYear
    if (Object.hasOwn(effectiveData, 'manufactureWeek')) this.manufactureWeek = effectiveData.manufactureWeek
    if (Object.hasOwn(effectiveData, 'capabilities')) this.capabilities = effectiveData.capabilities
  }

  getVcpFeature (featureCode: number): VCPValue {
    return displayGetVcpFeature(this.index, featureCode)
  }

  setVcpFeature (featureCode: number, value: number) {
    displaySetVcpFeature(this.index, featureCode, value)
  }

  setTableVcpFeature (featureCode: number, data: ArrayBuffer, offset: number) {
    displaySetTableVcpFeature(this.index, featureCode, data, offset)
  }

  readonly index: number
  readonly backend: string
  readonly edidData?: ArrayBuffer
  readonly version?: string
  readonly mccsVersion?: string
  readonly displayId: string
  readonly serial?: number
  readonly serialNumber?: string
  readonly modelId?: number
  readonly modelName?: string
  readonly manufacturerId?: string
  readonly manufactureYear?: number
  readonly manufactureWeek?: number
  readonly capabilities?: string
}

export class DisplayManager {
  constructor (queries?: Query[] | Query) {
    if (Array.isArray(queries)) {
      this._queries = queries
    } else if (queries === undefined || queries === null) {
      this._queries = []
    } else {
      this._queries = [queries]
    }
  }

  get queries (): Query[] {
    return this._queries
  }

  set queries (queries: Query[]) {
    this._queries = queries
  }

  addQueries (queries?: Query[] | Query): DisplayManager {
    let newQueries: Query[]
    if (Array.isArray(queries)) {
      newQueries = queries
    } else if (queries === undefined || queries === null) {
      newQueries = []
    } else {
      newQueries = [queries]
    }
    this.queries = this.queries.concat(newQueries)
    return this
  }

  static getByIndex (index: number): Display {
    const display = displayManagerGetByIndex(index)
    return new Display(display)
  }

  collect (): Display[] {
    return displayManagerList(this.queries).map(display => new Display(display))
  }

  list (): Display[] {
    return displayManagerList(this.queries).map(display => new Display(display))
  }

  private _queries: Query[]
}
