# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/blueprint/components/included_space_views.fbs".

# You can extend this class by creating a "IncludedSpaceViewsExt" class in "included_space_views_ext.py".

from __future__ import annotations

from typing import Any, Sequence, Union

import pyarrow as pa
from attrs import define, field

from ... import datatypes
from ..._baseclasses import BaseBatch, BaseExtensionType, ComponentBatchMixin

__all__ = [
    "IncludedSpaceViews",
    "IncludedSpaceViewsArrayLike",
    "IncludedSpaceViewsBatch",
    "IncludedSpaceViewsLike",
    "IncludedSpaceViewsType",
]


@define(init=False)
class IncludedSpaceViews:
    """
    **Component**: The id of a `SpaceView`.

    Unstable. Used for the ongoing blueprint experimentations.
    """

    def __init__(self: Any, space_view_ids: IncludedSpaceViewsLike):
        """Create a new instance of the IncludedSpaceViews component."""

        # You can define your own __init__ function as a member of IncludedSpaceViewsExt in included_space_views_ext.py
        self.__attrs_init__(space_view_ids=space_view_ids)

    space_view_ids: list[datatypes.Uuid] = field()


IncludedSpaceViewsLike = IncludedSpaceViews
IncludedSpaceViewsArrayLike = Union[
    IncludedSpaceViews,
    Sequence[IncludedSpaceViewsLike],
]


class IncludedSpaceViewsType(BaseExtensionType):
    _TYPE_NAME: str = "rerun.blueprint.components.IncludedSpaceViews"

    def __init__(self) -> None:
        pa.ExtensionType.__init__(
            self,
            pa.list_(
                pa.field(
                    "item",
                    pa.struct(
                        [
                            pa.field(
                                "bytes",
                                pa.list_(pa.field("item", pa.uint8(), nullable=False, metadata={}), 16),
                                nullable=False,
                                metadata={},
                            )
                        ]
                    ),
                    nullable=False,
                    metadata={},
                )
            ),
            self._TYPE_NAME,
        )


class IncludedSpaceViewsBatch(BaseBatch[IncludedSpaceViewsArrayLike], ComponentBatchMixin):
    _ARROW_TYPE = IncludedSpaceViewsType()

    @staticmethod
    def _native_to_pa_array(data: IncludedSpaceViewsArrayLike, data_type: pa.DataType) -> pa.Array:
        raise NotImplementedError  # You need to implement native_to_pa_array_override in included_space_views_ext.py
