// Generated by https://github.com/foxglove/schemas

import { MarkerDeletionType } from "./MarkerDeletionType";
import { Time } from "./Time";

/** Command to remove previously published markers */
export type MarkerDeletion = {
  /** Timestamp of the marker. Only matching markers earlier than this timestamp will be deleted. */
  timestamp: Time;

  /** Type of deletion action to perform */
  type: MarkerDeletionType;

  /** Numeric identifier which must match if `kind` is `MATCHING_ID`. */
  id: string;
};
