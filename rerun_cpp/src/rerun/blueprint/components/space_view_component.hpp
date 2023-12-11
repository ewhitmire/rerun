// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/blueprint/components/space_view_component.fbs".

#pragma once

#include "../../blueprint/datatypes/space_view_component.hpp"
#include "../../result.hpp"

#include <cstdint>
#include <memory>
#include <utility>

namespace arrow {
    class Array;
    class DataType;
    class StructBuilder;
} // namespace arrow

namespace rerun::blueprint::components {
    /// **Component**: A view of a space.
    ///
    /// Unstable. Used for the ongoing blueprint experimentations.
    struct SpaceViewComponent {
        rerun::blueprint::datatypes::SpaceViewComponent component;

      public:
        SpaceViewComponent() = default;

        SpaceViewComponent(rerun::blueprint::datatypes::SpaceViewComponent component_)
            : component(std::move(component_)) {}

        SpaceViewComponent& operator=(rerun::blueprint::datatypes::SpaceViewComponent component_) {
            component = std::move(component_);
            return *this;
        }

        /// Cast to the underlying SpaceViewComponent datatype
        operator rerun::blueprint::datatypes::SpaceViewComponent() const {
            return component;
        }
    };
} // namespace rerun::blueprint::components

namespace rerun {
    template <typename T>
    struct Loggable;

    /// \private
    template <>
    struct Loggable<blueprint::components::SpaceViewComponent> {
        static constexpr const char Name[] = "rerun.blueprint.components.SpaceViewComponent";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype();

        /// Fills an arrow array builder with an array of this type.
        static rerun::Error fill_arrow_array_builder(
            arrow::StructBuilder* builder,
            const blueprint::components::SpaceViewComponent* elements, size_t num_elements
        );

        /// Serializes an array of `rerun::blueprint:: components::SpaceViewComponent` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const blueprint::components::SpaceViewComponent* instances, size_t num_instances
        );
    };
} // namespace rerun
