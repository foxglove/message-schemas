// Generated by https://github.com/foxglove/schemas
// Options: {}

/** An enumeration indicating how input points should be interpreted to create lines */
export enum LineType {
  /** Connected line segments: 0-1, 1-2, ..., (n-1)-n */
  LINE_STRIP = 0,

  /** Closed polygon: 0-1, 1-2, ..., (n-1)-n, n-0 */
  LINE_LOOP = 1,

  /** Individual line segments: 0-1, 2-3, 4-5, ... */
  LINE_LIST = 2,
}
