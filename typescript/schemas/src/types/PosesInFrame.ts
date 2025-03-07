// Generated by https://github.com/foxglove/foxglove-sdk
// Options: {}

import { Pose } from "./Pose";
import { Time } from "./Time";

/** An array of timestamped poses for an object or reference frame in 3D space */
export type PosesInFrame = {
  /** Timestamp of pose */
  timestamp: Time;

  /** Frame of reference for pose position and orientation */
  frame_id: string;

  /** Poses in 3D space */
  poses: Pose[];
};
