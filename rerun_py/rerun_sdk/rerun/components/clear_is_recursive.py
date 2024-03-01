# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/components/clear_is_recursive.fbs".

# You can extend this class by creating a "ClearIsRecursiveExt" class in "clear_is_recursive_ext.py".

from __future__ import annotations

from typing import TYPE_CHECKING, Any, Sequence, Union

import numpy as np
import numpy.typing as npt
import pyarrow as pa
from attrs import define, field

from .._baseclasses import BaseBatch, BaseExtensionType, ComponentBatchMixin
from .clear_is_recursive_ext import ClearIsRecursiveExt

__all__ = [
    "ClearIsRecursive",
    "ClearIsRecursiveArrayLike",
    "ClearIsRecursiveBatch",
    "ClearIsRecursiveLike",
    "ClearIsRecursiveType",
]


@define(init=False)
class ClearIsRecursive(ClearIsRecursiveExt):
    """**Component**: Configures how a clear operation should behave - recursive or not."""

    def __init__(self: Any, recursive: ClearIsRecursiveLike):
        """
        Create a new instance of the ClearIsRecursive component.

        Parameters
        ----------
        recursive:
            If true, also clears all recursive children entities.

        """

        # You can define your own __init__ function as a member of ClearIsRecursiveExt in clear_is_recursive_ext.py
        self.__attrs_init__(recursive=recursive)

    recursive: bool = field(converter=bool)
    # If true, also clears all recursive children entities.
    #
    # (Docstring intentionally commented out to hide this field from the docs)


if TYPE_CHECKING:
    ClearIsRecursiveLike = Union[ClearIsRecursive, bool]
else:
    ClearIsRecursiveLike = Any

ClearIsRecursiveArrayLike = Union[ClearIsRecursive, Sequence[ClearIsRecursiveLike], bool, npt.NDArray[np.bool_]]


class ClearIsRecursiveType(BaseExtensionType):
    _TYPE_NAME: str = "rerun.components.ClearIsRecursive"

    def __init__(self) -> None:
        pa.ExtensionType.__init__(self, pa.bool_(), self._TYPE_NAME)


class ClearIsRecursiveBatch(BaseBatch[ClearIsRecursiveArrayLike], ComponentBatchMixin):
    _ARROW_TYPE = ClearIsRecursiveType()

    @staticmethod
    def _native_to_pa_array(data: ClearIsRecursiveArrayLike, data_type: pa.DataType) -> pa.Array:
        return ClearIsRecursiveExt.native_to_pa_array_override(data, data_type)
