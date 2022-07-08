import { VCPFeatureCode, VCPValue, DisplayData, Query } from './types'

export function displayGetVcpFeature(index: number, featureCode: VCPFeatureCode): VCPValue
export function displaySetVcpFeature(index: number, featureCode: VCPFeatureCode, value: number)
export function displaySetTableVcpFeature(index: number, featureCode: VCPFeatureCode, data: ArrayBuffer, offset: number)
export function displayManagerGetByIndex(index: number): DisplayData
export function displayManagerList(queries: Query[]): DisplayData[]
