# Generated by https://github.com/foxglove/foxglove-sdk
from enum import Enum
from typing import List, Optional

#
# Enums
#

class LinePrimitiveLineType(Enum):
    """
    An enumeration indicating how input points should be interpreted to create lines
    """

    LineStrip = 0
    LineLoop = 1
    LineList = 2

class LocationFixPositionCovarianceType(Enum):
    """
    Type of position covariance
    """

    Unknown = 0
    Approximated = 1
    DiagonalKnown = 2
    Known = 3

class LogLevel(Enum):
    """
    Log level
    """

    Unknown = 0
    Debug = 1
    Info = 2
    Warning = 3
    Error = 4
    Fatal = 5

class PackedElementFieldNumericType(Enum):
    """
    Numeric type
    """

    Unknown = 0
    Uint8 = 1
    Int8 = 2
    Uint16 = 3
    Int16 = 4
    Uint32 = 5
    Int32 = 6
    Float32 = 7
    Float64 = 8

class PointsAnnotationType(Enum):
    """
    Type of points annotation
    """

    Unknown = 0
    Points = 1
    LineLoop = 2
    LineStrip = 3
    LineList = 4

class SceneEntityDeletionType(Enum):
    """
    An enumeration indicating which entities should match a SceneEntityDeletion command
    """

    MatchingId = 0
    All = 1

#
# Classes
#

class ArrowPrimitive:
    """
    A primitive representing an arrow
    """

    def __new__(
        cls,
        *,
        pose: "Optional[Pose]" = None,
        shaft_length: "Optional[float]" = 0.0,
        shaft_diameter: "Optional[float]" = 0.0,
        head_length: "Optional[float]" = 0.0,
        head_diameter: "Optional[float]" = 0.0,
        color: "Optional[Color]" = None,
    ) -> "ArrowPrimitive": ...

class CameraCalibration:
    """
    Camera calibration parameters
    """

    def __new__(
        cls,
        *,
        timestamp: "Optional[Timestamp]" = None,
        frame_id: "Optional[str]" = "",
        width: "Optional[int]" = 0,
        height: "Optional[int]" = 0,
        distortion_model: "Optional[str]" = "",
        D: "Optional[List[float]]" = [],
        K: "Optional[List[float]]" = [],
        R: "Optional[List[float]]" = [],
        P: "Optional[List[float]]" = [],
    ) -> "CameraCalibration": ...

class CircleAnnotation:
    """
    A circle annotation on a 2D image
    """

    def __new__(
        cls,
        *,
        timestamp: "Optional[Timestamp]" = None,
        position: "Optional[Point2]" = None,
        diameter: "Optional[float]" = 0.0,
        thickness: "Optional[float]" = 0.0,
        fill_color: "Optional[Color]" = None,
        outline_color: "Optional[Color]" = None,
    ) -> "CircleAnnotation": ...

class Color:
    """
    A color in RGBA format
    """

    def __new__(
        cls,
        *,
        r: "Optional[float]" = 0.0,
        g: "Optional[float]" = 0.0,
        b: "Optional[float]" = 0.0,
        a: "Optional[float]" = 0.0,
    ) -> "Color": ...

class CompressedImage:
    """
    A compressed image
    """

    def __new__(
        cls,
        *,
        timestamp: "Optional[Timestamp]" = None,
        frame_id: "Optional[str]" = "",
        data: "Optional[bytes]" = b"",
        format: "Optional[str]" = "",
    ) -> "CompressedImage": ...

class CompressedVideo:
    """
    A single frame of a compressed video bitstream
    """

    def __new__(
        cls,
        *,
        timestamp: "Optional[Timestamp]" = None,
        frame_id: "Optional[str]" = "",
        data: "Optional[bytes]" = b"",
        format: "Optional[str]" = "",
    ) -> "CompressedVideo": ...

class CubePrimitive:
    """
    A primitive representing a cube or rectangular prism
    """

    def __new__(
        cls,
        *,
        pose: "Optional[Pose]" = None,
        size: "Optional[Vector3]" = None,
        color: "Optional[Color]" = None,
    ) -> "CubePrimitive": ...

class CylinderPrimitive:
    """
    A primitive representing a cylinder, elliptic cylinder, or truncated cone
    """

    def __new__(
        cls,
        *,
        pose: "Optional[Pose]" = None,
        size: "Optional[Vector3]" = None,
        bottom_scale: "Optional[float]" = 0.0,
        top_scale: "Optional[float]" = 0.0,
        color: "Optional[Color]" = None,
    ) -> "CylinderPrimitive": ...

class Duration:
    """
    A duration in seconds and nanoseconds
    """

    def __new__(
        cls,
        sec: int,
        nsec: Optional[int] = None,
    ) -> "Duration": ...

class FrameTransform:
    """
    A transform between two reference frames in 3D space
    """

    def __new__(
        cls,
        *,
        timestamp: "Optional[Timestamp]" = None,
        parent_frame_id: "Optional[str]" = "",
        child_frame_id: "Optional[str]" = "",
        translation: "Optional[Vector3]" = None,
        rotation: "Optional[Quaternion]" = None,
    ) -> "FrameTransform": ...

class FrameTransforms:
    """
    An array of FrameTransform messages
    """

    def __new__(
        cls, *, transforms: "Optional[List[FrameTransform]]" = []
    ) -> "FrameTransforms": ...

class GeoJson:
    """
    GeoJSON data for annotating maps
    """

    def __new__(cls, *, geojson: "Optional[str]" = "") -> "GeoJson": ...

class Grid:
    """
    A 2D grid of data
    """

    def __new__(
        cls,
        *,
        timestamp: "Optional[Timestamp]" = None,
        frame_id: "Optional[str]" = "",
        pose: "Optional[Pose]" = None,
        column_count: "Optional[int]" = 0,
        cell_size: "Optional[Vector2]" = None,
        row_stride: "Optional[int]" = 0,
        cell_stride: "Optional[int]" = 0,
        fields: "Optional[List[PackedElementField]]" = [],
        data: "Optional[bytes]" = b"",
    ) -> "Grid": ...

class ImageAnnotations:
    """
    Array of annotations for a 2D image
    """

    def __new__(
        cls,
        *,
        circles: "Optional[List[CircleAnnotation]]" = [],
        points: "Optional[List[PointsAnnotation]]" = [],
        texts: "Optional[List[TextAnnotation]]" = [],
    ) -> "ImageAnnotations": ...

class KeyValuePair:
    """
    A key with its associated value
    """

    def __new__(
        cls, *, key: "Optional[str]" = "", value: "Optional[str]" = ""
    ) -> "KeyValuePair": ...

class LaserScan:
    """
    A single scan from a planar laser range-finder
    """

    def __new__(
        cls,
        *,
        timestamp: "Optional[Timestamp]" = None,
        frame_id: "Optional[str]" = "",
        pose: "Optional[Pose]" = None,
        start_angle: "Optional[float]" = 0.0,
        end_angle: "Optional[float]" = 0.0,
        ranges: "Optional[List[float]]" = [],
        intensities: "Optional[List[float]]" = [],
    ) -> "LaserScan": ...

class LinePrimitive:
    """
    A primitive representing a series of points connected by lines
    """

    def __new__(
        cls,
        *,
        type: "Optional[LinePrimitiveLineType]" = LinePrimitiveLineType.LineStrip,
        pose: "Optional[Pose]" = None,
        thickness: "Optional[float]" = 0.0,
        scale_invariant: "Optional[bool]" = False,
        points: "Optional[List[Point3]]" = [],
        color: "Optional[Color]" = None,
        colors: "Optional[List[Color]]" = [],
        indices: "Optional[List[int]]" = [],
    ) -> "LinePrimitive": ...

class LocationFix:
    """
    A navigation satellite fix for any Global Navigation Satellite System
    """

    def __new__(
        cls,
        *,
        timestamp: "Optional[Timestamp]" = None,
        frame_id: "Optional[str]" = "",
        latitude: "Optional[float]" = 0.0,
        longitude: "Optional[float]" = 0.0,
        altitude: "Optional[float]" = 0.0,
        position_covariance: "Optional[List[float]]" = [],
        position_covariance_type: "Optional[LocationFixPositionCovarianceType]" = LocationFixPositionCovarianceType.Unknown,
    ) -> "LocationFix": ...

class Log:
    """
    A log message
    """

    def __new__(
        cls,
        *,
        timestamp: "Optional[Timestamp]" = None,
        level: "Optional[LogLevel]" = LogLevel.Unknown,
        message: "Optional[str]" = "",
        name: "Optional[str]" = "",
        file: "Optional[str]" = "",
        line: "Optional[int]" = 0,
    ) -> "Log": ...

class ModelPrimitive:
    """
    A primitive representing a 3D model file loaded from an external URL or embedded data
    """

    def __new__(
        cls,
        *,
        pose: "Optional[Pose]" = None,
        scale: "Optional[Vector3]" = None,
        color: "Optional[Color]" = None,
        override_color: "Optional[bool]" = False,
        url: "Optional[str]" = "",
        media_type: "Optional[str]" = "",
        data: "Optional[bytes]" = b"",
    ) -> "ModelPrimitive": ...

class PackedElementField:
    """
    A field present within each element in a byte array of packed elements.
    """

    def __new__(
        cls,
        *,
        name: "Optional[str]" = "",
        offset: "Optional[int]" = 0,
        type: "Optional[PackedElementFieldNumericType]" = PackedElementFieldNumericType.Unknown,
    ) -> "PackedElementField": ...

class Point2:
    """
    A point representing a position in 2D space
    """

    def __new__(
        cls, *, x: "Optional[float]" = 0.0, y: "Optional[float]" = 0.0
    ) -> "Point2": ...

class Point3:
    """
    A point representing a position in 3D space
    """

    def __new__(
        cls,
        *,
        x: "Optional[float]" = 0.0,
        y: "Optional[float]" = 0.0,
        z: "Optional[float]" = 0.0,
    ) -> "Point3": ...

class PointCloud:
    """
    A collection of N-dimensional points, which may contain additional fields with information like normals, intensity, etc.
    """

    def __new__(
        cls,
        *,
        timestamp: "Optional[Timestamp]" = None,
        frame_id: "Optional[str]" = "",
        pose: "Optional[Pose]" = None,
        point_stride: "Optional[int]" = 0,
        fields: "Optional[List[PackedElementField]]" = [],
        data: "Optional[bytes]" = b"",
    ) -> "PointCloud": ...

class PointsAnnotation:
    """
    An array of points on a 2D image
    """

    def __new__(
        cls,
        *,
        timestamp: "Optional[Timestamp]" = None,
        type: "Optional[PointsAnnotationType]" = PointsAnnotationType.Unknown,
        points: "Optional[List[Point2]]" = [],
        outline_color: "Optional[Color]" = None,
        outline_colors: "Optional[List[Color]]" = [],
        fill_color: "Optional[Color]" = None,
        thickness: "Optional[float]" = 0.0,
    ) -> "PointsAnnotation": ...

class Pose:
    """
    A position and orientation for an object or reference frame in 3D space
    """

    def __new__(
        cls,
        *,
        position: "Optional[Vector3]" = None,
        orientation: "Optional[Quaternion]" = None,
    ) -> "Pose": ...

class PoseInFrame:
    """
    A timestamped pose for an object or reference frame in 3D space
    """

    def __new__(
        cls,
        *,
        timestamp: "Optional[Timestamp]" = None,
        frame_id: "Optional[str]" = "",
        pose: "Optional[Pose]" = None,
    ) -> "PoseInFrame": ...

class PosesInFrame:
    """
    An array of timestamped poses for an object or reference frame in 3D space
    """

    def __new__(
        cls,
        *,
        timestamp: "Optional[Timestamp]" = None,
        frame_id: "Optional[str]" = "",
        poses: "Optional[List[Pose]]" = [],
    ) -> "PosesInFrame": ...

class Quaternion:
    """
    A [quaternion](https://eater.net/quaternions) representing a rotation in 3D space
    """

    def __new__(
        cls,
        *,
        x: "Optional[float]" = 0.0,
        y: "Optional[float]" = 0.0,
        z: "Optional[float]" = 0.0,
        w: "Optional[float]" = 0.0,
    ) -> "Quaternion": ...

class RawImage:
    """
    A raw image
    """

    def __new__(
        cls,
        *,
        timestamp: "Optional[Timestamp]" = None,
        frame_id: "Optional[str]" = "",
        width: "Optional[int]" = 0,
        height: "Optional[int]" = 0,
        encoding: "Optional[str]" = "",
        step: "Optional[int]" = 0,
        data: "Optional[bytes]" = b"",
    ) -> "RawImage": ...

class SceneEntity:
    """
    A visual element in a 3D scene. An entity may be composed of multiple primitives which all share the same frame of reference.
    """

    def __new__(
        cls,
        *,
        timestamp: "Optional[Timestamp]" = None,
        frame_id: "Optional[str]" = "",
        id: "Optional[str]" = "",
        lifetime: "Optional[Duration]" = None,
        frame_locked: "Optional[bool]" = False,
        metadata: "Optional[List[KeyValuePair]]" = [],
        arrows: "Optional[List[ArrowPrimitive]]" = [],
        cubes: "Optional[List[CubePrimitive]]" = [],
        spheres: "Optional[List[SpherePrimitive]]" = [],
        cylinders: "Optional[List[CylinderPrimitive]]" = [],
        lines: "Optional[List[LinePrimitive]]" = [],
        triangles: "Optional[List[TriangleListPrimitive]]" = [],
        texts: "Optional[List[TextPrimitive]]" = [],
        models: "Optional[List[ModelPrimitive]]" = [],
    ) -> "SceneEntity": ...

class SceneEntityDeletion:
    """
    Command to remove previously published entities
    """

    def __new__(
        cls,
        *,
        timestamp: "Optional[Timestamp]" = None,
        type: "Optional[SceneEntityDeletionType]" = SceneEntityDeletionType.MatchingId,
        id: "Optional[str]" = "",
    ) -> "SceneEntityDeletion": ...

class SceneUpdate:
    """
    An update to the entities displayed in a 3D scene
    """

    def __new__(
        cls,
        *,
        deletions: "Optional[List[SceneEntityDeletion]]" = [],
        entities: "Optional[List[SceneEntity]]" = [],
    ) -> "SceneUpdate": ...

class SpherePrimitive:
    """
    A primitive representing a sphere or ellipsoid
    """

    def __new__(
        cls,
        *,
        pose: "Optional[Pose]" = None,
        size: "Optional[Vector3]" = None,
        color: "Optional[Color]" = None,
    ) -> "SpherePrimitive": ...

class TextAnnotation:
    """
    A text label on a 2D image
    """

    def __new__(
        cls,
        *,
        timestamp: "Optional[Timestamp]" = None,
        position: "Optional[Point2]" = None,
        text: "Optional[str]" = "",
        font_size: "Optional[float]" = 0.0,
        text_color: "Optional[Color]" = None,
        background_color: "Optional[Color]" = None,
    ) -> "TextAnnotation": ...

class TextPrimitive:
    """
    A primitive representing a text label
    """

    def __new__(
        cls,
        *,
        pose: "Optional[Pose]" = None,
        billboard: "Optional[bool]" = False,
        font_size: "Optional[float]" = 0.0,
        scale_invariant: "Optional[bool]" = False,
        color: "Optional[Color]" = None,
        text: "Optional[str]" = "",
    ) -> "TextPrimitive": ...

class Timestamp:
    """
    A timestamp in seconds and nanoseconds
    """

    def __new__(
        cls,
        sec: int,
        nsec: Optional[int] = None,
    ) -> "Timestamp": ...

class TriangleListPrimitive:
    """
    A primitive representing a set of triangles or a surface tiled by triangles
    """

    def __new__(
        cls,
        *,
        pose: "Optional[Pose]" = None,
        points: "Optional[List[Point3]]" = [],
        color: "Optional[Color]" = None,
        colors: "Optional[List[Color]]" = [],
        indices: "Optional[List[int]]" = [],
    ) -> "TriangleListPrimitive": ...

class Vector2:
    """
    A vector in 2D space that represents a direction only
    """

    def __new__(
        cls, *, x: "Optional[float]" = 0.0, y: "Optional[float]" = 0.0
    ) -> "Vector2": ...

class Vector3:
    """
    A vector in 3D space that represents a direction only
    """

    def __new__(
        cls,
        *,
        x: "Optional[float]" = 0.0,
        y: "Optional[float]" = 0.0,
        z: "Optional[float]" = 0.0,
    ) -> "Vector3": ...
