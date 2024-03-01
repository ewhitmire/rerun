# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/archetypes/scalar.fbs".

# You can extend this class by creating a "ScalarExt" class in "scalar_ext.py".

from __future__ import annotations

from typing import Any

from attrs import define, field

from .. import components
from .._baseclasses import Archetype
from ..error_utils import catch_and_log_exceptions

__all__ = ["Scalar"]


@define(str=False, repr=False, init=False)
class Scalar(Archetype):
    """
    **Archetype**: Log a double-precision scalar.

    The current timeline value will be used for the time/X-axis, hence scalars
    cannot be timeless.

    When used to produce a plot, this archetype is used to provide the data that
    is referenced by the `SeriesLine` or `SeriesPoint` archetypes. You can do
    this by logging both archetypes to the same path, or alternatively configuring
    the plot-specific archetypes through the blueprint.

    See also  [`SeriesPoint`][rerun.archetypes.SeriesPoint], [`SeriesLine`][rerun.archetypes.SeriesLine].

    Example
    -------
    ### Simple line plot:
    ```python
    import math

    import rerun as rr

    rr.init("rerun_example_scalar", spawn=True)

    # Log the data on a timeline called "step".
    for step in range(0, 64):
        rr.set_time_sequence("step", step)
        rr.log("scalar", rr.Scalar(math.sin(step / 10.0)))
    ```
    <center>
    <picture>
      <source media="(max-width: 480px)" srcset="https://static.rerun.io/scalar_simple/8bcc92f56268739f8cd24d60d1fe72a655f62a46/480w.png">
      <source media="(max-width: 768px)" srcset="https://static.rerun.io/scalar_simple/8bcc92f56268739f8cd24d60d1fe72a655f62a46/768w.png">
      <source media="(max-width: 1024px)" srcset="https://static.rerun.io/scalar_simple/8bcc92f56268739f8cd24d60d1fe72a655f62a46/1024w.png">
      <source media="(max-width: 1200px)" srcset="https://static.rerun.io/scalar_simple/8bcc92f56268739f8cd24d60d1fe72a655f62a46/1200w.png">
      <img src="https://static.rerun.io/scalar_simple/8bcc92f56268739f8cd24d60d1fe72a655f62a46/full.png" width="640">
    </picture>
    </center>

    """

    def __init__(self: Any, scalar: components.ScalarLike):
        """
        Create a new instance of the Scalar archetype.

        Parameters
        ----------
        scalar:
            The scalar value to log.

        """

        # You can define your own __init__ function as a member of ScalarExt in scalar_ext.py
        with catch_and_log_exceptions(context=self.__class__.__name__):
            self.__attrs_init__(scalar=scalar)
            return
        self.__attrs_clear__()

    def __attrs_clear__(self) -> None:
        """Convenience method for calling `__attrs_init__` with all `None`s."""
        self.__attrs_init__(
            scalar=None,  # type: ignore[arg-type]
        )

    @classmethod
    def _clear(cls) -> Scalar:
        """Produce an empty Scalar, bypassing `__init__`."""
        inst = cls.__new__(cls)
        inst.__attrs_clear__()
        return inst

    scalar: components.ScalarBatch = field(
        metadata={"component": "required"},
        converter=components.ScalarBatch._required,  # type: ignore[misc]
    )
    # The scalar value to log.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__
