export interface Continuous {
  currentValue: number;
  maximumValue: number;
}

export interface NonContinuous {
  /** The first element is the number representation and the second one is the string representation if existing. */
  currentValue: [number, string | undefined];
  /** This map all possibles values number representation into their string representation if existing. */
  possibleValues: Record<number, string | undefined>;
}

export interface Table {
  currentData: ArrayBuffer;
}

export type VCPValue = Continuous | NonContinuous | Table;

export enum QueryType {
  // eslint-disable-next-line no-unused-vars
  Backend,
  // eslint-disable-next-line no-unused-vars
  Id,
  // eslint-disable-next-line no-unused-vars
  ManufacturerId,
  // eslint-disable-next-line no-unused-vars
  ModelName,
  // eslint-disable-next-line no-unused-vars
  SerialNumber,
}

export interface Query {
  queryType: QueryType;
  queryValue: string;
}

export class Display {
  getVcpFeature(featureCode: number): VCPValue;

  setVcpFeature(featureCode: number, value: number);

  setTableVcpFeature(featureCode: number, data: ArrayBuffer, offset: number);

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
}

export default class DisplayManager {
  constructor(queries?: Query[] | Query);

  get queries(): Query[];
  set queries(queries: Query[]);

  addQueries(queries?: Query[] | Query): DisplayManager;

  static getByIndex(index: number): Display;

  collect(): Display[];

  list(): Display[];

  private _queries: Query[]
}
