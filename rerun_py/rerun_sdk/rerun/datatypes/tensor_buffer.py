# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/datatypes/tensor_buffer.fbs".

# You can extend this class by creating a "TensorBufferExt" class in "tensor_buffer_ext.py".

from __future__ import annotations

from typing import TYPE_CHECKING, Any, Literal, Sequence, Union

import numpy as np
import numpy.typing as npt
import pyarrow as pa
from attrs import define, field

from .._baseclasses import BaseBatch, BaseExtensionType
from .tensor_buffer_ext import TensorBufferExt

__all__ = ["TensorBuffer", "TensorBufferArrayLike", "TensorBufferBatch", "TensorBufferLike", "TensorBufferType"]


@define
class TensorBuffer(TensorBufferExt):
    """
    **Datatype**: The underlying storage for a `Tensor`.

    Tensor elements are stored in a contiguous buffer of a single type.
    """

    # You can define your own __init__ function as a member of TensorBufferExt in tensor_buffer_ext.py

    inner: Union[
        npt.NDArray[np.float16],
        npt.NDArray[np.float32],
        npt.NDArray[np.float64],
        npt.NDArray[np.int16],
        npt.NDArray[np.int32],
        npt.NDArray[np.int64],
        npt.NDArray[np.int8],
        npt.NDArray[np.uint16],
        npt.NDArray[np.uint32],
        npt.NDArray[np.uint64],
        npt.NDArray[np.uint8],
    ] = field(
        converter=TensorBufferExt.inner__field_converter_override  # type: ignore[misc]
    )
    """
    Must be one of:

    * U8 (npt.NDArray[np.uint8]):
        8bit unsigned integer.

    * U16 (npt.NDArray[np.uint16]):
        16bit unsigned integer.

    * U32 (npt.NDArray[np.uint32]):
        32bit unsigned integer.

    * U64 (npt.NDArray[np.uint64]):
        64bit unsigned integer.

    * I8 (npt.NDArray[np.int8]):
        8bit signed integer.

    * I16 (npt.NDArray[np.int16]):
        16bit signed integer.

    * I32 (npt.NDArray[np.int32]):
        32bit signed integer.

    * I64 (npt.NDArray[np.int64]):
        64bit signed integer.

    * F16 (npt.NDArray[np.float16]):
        16bit IEEE-754 floating point, also known as `half`.

    * F32 (npt.NDArray[np.float32]):
        32bit IEEE-754 floating point, also known as `float` or `single`.

    * F64 (npt.NDArray[np.float64]):
        64bit IEEE-754 floating point, also known as `double`.

    * JPEG (npt.NDArray[np.uint8]):
        Raw bytes of a JPEG file.

    * NV12 (npt.NDArray[np.uint8]):
        NV12 is a YUV 4:2:0 chroma downsamples format with 8 bits per channel.

        First comes entire image in Y, followed by interleaved lines ordered as U0, V0, U1, V1, etc.

    * YUY2 (npt.NDArray[np.uint8]):
        YUY2, also known as YUYV is a YUV 4:2:2 chrome downsampled format with 8 bits per channel.

        The order of the channels is Y0, U0, Y1, V0.
    """

    kind: Literal[
        "u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64", "f16", "f32", "f64", "jpeg", "nv12", "yuy2"
    ] = field(default="u8")
    """
    Possible values:

    * "U8":
        8bit unsigned integer.

    * "U16":
        16bit unsigned integer.

    * "U32":
        32bit unsigned integer.

    * "U64":
        64bit unsigned integer.

    * "I8":
        8bit signed integer.

    * "I16":
        16bit signed integer.

    * "I32":
        32bit signed integer.

    * "I64":
        64bit signed integer.

    * "F16":
        16bit IEEE-754 floating point, also known as `half`.

    * "F32":
        32bit IEEE-754 floating point, also known as `float` or `single`.

    * "F64":
        64bit IEEE-754 floating point, also known as `double`.

    * "JPEG":
        Raw bytes of a JPEG file.

    * "NV12":
        NV12 is a YUV 4:2:0 chroma downsamples format with 8 bits per channel.

        First comes entire image in Y, followed by interleaved lines ordered as U0, V0, U1, V1, etc.

    * "YUY2":
        YUY2, also known as YUYV is a YUV 4:2:2 chrome downsampled format with 8 bits per channel.

        The order of the channels is Y0, U0, Y1, V0.
    """


if TYPE_CHECKING:
    TensorBufferLike = Union[
        TensorBuffer,
        npt.NDArray[np.float16],
        npt.NDArray[np.float32],
        npt.NDArray[np.float64],
        npt.NDArray[np.int16],
        npt.NDArray[np.int32],
        npt.NDArray[np.int64],
        npt.NDArray[np.int8],
        npt.NDArray[np.uint16],
        npt.NDArray[np.uint32],
        npt.NDArray[np.uint64],
        npt.NDArray[np.uint8],
    ]
    TensorBufferArrayLike = Union[
        TensorBuffer,
        npt.NDArray[np.float16],
        npt.NDArray[np.float32],
        npt.NDArray[np.float64],
        npt.NDArray[np.int16],
        npt.NDArray[np.int32],
        npt.NDArray[np.int64],
        npt.NDArray[np.int8],
        npt.NDArray[np.uint16],
        npt.NDArray[np.uint32],
        npt.NDArray[np.uint64],
        npt.NDArray[np.uint8],
        Sequence[TensorBufferLike],
    ]
else:
    TensorBufferLike = Any
    TensorBufferArrayLike = Any


class TensorBufferType(BaseExtensionType):
    _TYPE_NAME: str = "rerun.datatypes.TensorBuffer"

    def __init__(self) -> None:
        pa.ExtensionType.__init__(
            self,
            pa.dense_union(
                [
                    pa.field("_null_markers", pa.null(), nullable=True, metadata={}),
                    pa.field(
                        "U8",
                        pa.list_(pa.field("item", pa.uint8(), nullable=False, metadata={})),
                        nullable=False,
                        metadata={},
                    ),
                    pa.field(
                        "U16",
                        pa.list_(pa.field("item", pa.uint16(), nullable=False, metadata={})),
                        nullable=False,
                        metadata={},
                    ),
                    pa.field(
                        "U32",
                        pa.list_(pa.field("item", pa.uint32(), nullable=False, metadata={})),
                        nullable=False,
                        metadata={},
                    ),
                    pa.field(
                        "U64",
                        pa.list_(pa.field("item", pa.uint64(), nullable=False, metadata={})),
                        nullable=False,
                        metadata={},
                    ),
                    pa.field(
                        "I8",
                        pa.list_(pa.field("item", pa.int8(), nullable=False, metadata={})),
                        nullable=False,
                        metadata={},
                    ),
                    pa.field(
                        "I16",
                        pa.list_(pa.field("item", pa.int16(), nullable=False, metadata={})),
                        nullable=False,
                        metadata={},
                    ),
                    pa.field(
                        "I32",
                        pa.list_(pa.field("item", pa.int32(), nullable=False, metadata={})),
                        nullable=False,
                        metadata={},
                    ),
                    pa.field(
                        "I64",
                        pa.list_(pa.field("item", pa.int64(), nullable=False, metadata={})),
                        nullable=False,
                        metadata={},
                    ),
                    pa.field(
                        "F16",
                        pa.list_(pa.field("item", pa.float16(), nullable=False, metadata={})),
                        nullable=False,
                        metadata={},
                    ),
                    pa.field(
                        "F32",
                        pa.list_(pa.field("item", pa.float32(), nullable=False, metadata={})),
                        nullable=False,
                        metadata={},
                    ),
                    pa.field(
                        "F64",
                        pa.list_(pa.field("item", pa.float64(), nullable=False, metadata={})),
                        nullable=False,
                        metadata={},
                    ),
                    pa.field(
                        "JPEG",
                        pa.list_(pa.field("item", pa.uint8(), nullable=False, metadata={})),
                        nullable=False,
                        metadata={},
                    ),
                    pa.field(
                        "NV12",
                        pa.list_(pa.field("item", pa.uint8(), nullable=False, metadata={})),
                        nullable=False,
                        metadata={},
                    ),
                    pa.field(
                        "YUY2",
                        pa.list_(pa.field("item", pa.uint8(), nullable=False, metadata={})),
                        nullable=False,
                        metadata={},
                    ),
                ]
            ),
            self._TYPE_NAME,
        )


class TensorBufferBatch(BaseBatch[TensorBufferArrayLike]):
    _ARROW_TYPE = TensorBufferType()

    @staticmethod
    def _native_to_pa_array(data: TensorBufferArrayLike, data_type: pa.DataType) -> pa.Array:
        raise NotImplementedError  # You need to implement native_to_pa_array_override in tensor_buffer_ext.py
