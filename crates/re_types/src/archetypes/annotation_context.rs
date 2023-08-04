// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

#![allow(trivial_numeric_casts)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

/// The `AnnotationContext` provides additional information on how to display entities.
///
/// Entities can use `ClassId`s and `KeypointId`s to provide annotations, and
/// the labels and colors will be looked up in the appropriate
/// `AnnotationContext`. We use the *first* annotation context we find in the
/// path-hierarchy when searching up through the ancestors of a given entity
/// path.
///
/// ## Example
///
/// ```ignore
/// //! Log rectangles with different colors and labels using annotation context
/// use rerun::{
///    archetypes::AnnotationContext,
///    components::{ClassId, Color, Label, Rect2D},
///    datatypes::{AnnotationInfo, Vec4D},
///    MsgSender, RecordingStreamBuilder,
/// };
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///    let (rec_stream, storage) = RecordingStreamBuilder::new("annotation_context_rects").memory()?;
///
///    // Log an annotation context to assign a label and color to each class
///    let annotation = AnnotationContext::new([
///        AnnotationInfo {
///            id: 1,
///            label: Some(Label("red".into())),
///            color: Some(Color::from_rgb(255, 0, 0)),
///        },
///        AnnotationInfo {
///            id: 2,
///            label: Some(Label("green".into())),
///            color: Some(Color::from_rgb(0, 255, 0)),
///        },
///    ]);
///
///    MsgSender::from_archetype("/", &annotation)?.send(&rec_stream)?;
///
///    // Log a batch of 2 rectangles with different class IDs
///    MsgSender::new("detections")
///        .with_component(&[
///            Rect2D::XYWH(Vec4D([-2., -2., 3., 3.]).into()),
///            Rect2D::XYWH(Vec4D([0., 0., 2., 2.]).into()),
///        ])?
///        .with_component(&[ClassId(1), ClassId(2)])?
///        .send(&rec_stream)?;
///
///    // Log an extra rect to set the view bounds
///    MsgSender::new("bounds")
///        .with_component(&[Rect2D::XCYCWH(Vec4D([0.0, 0.0, 5.0, 5.0]).into())])?
///        .send(&rec_stream)?;
///
///    rerun::native_viewer::show(storage.take())?;
///    Ok(())
/// }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnnotationContext<'s> {
    pub context: crate::components::AnnotationContext<'s>,
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.annotation_context".into()]);
static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 0usize]> =
    once_cell::sync::Lazy::new(|| []);
static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 0usize]> =
    once_cell::sync::Lazy::new(|| []);
static ALL_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.annotation_context".into()]);

impl<'s> AnnotationContext<'s> {
    pub const NUM_COMPONENTS: usize = 1usize;
}

impl<'s> crate::Archetype<'s> for AnnotationContext<'s> {
    #[inline]
    fn name() -> crate::ArchetypeName {
        crate::ArchetypeName::Borrowed("rerun.archetypes.AnnotationContext")
    }

    #[inline]
    fn required_components() -> &'static [crate::ComponentName] {
        REQUIRED_COMPONENTS.as_slice()
    }

    #[inline]
    fn recommended_components() -> &'static [crate::ComponentName] {
        RECOMMENDED_COMPONENTS.as_slice()
    }

    #[inline]
    fn optional_components() -> &'static [crate::ComponentName] {
        OPTIONAL_COMPONENTS.as_slice()
    }

    #[inline]
    fn all_components() -> &'static [crate::ComponentName] {
        ALL_COMPONENTS.as_slice()
    }

    #[inline]
    fn try_to_arrow(
        &'s self,
    ) -> crate::SerializationResult<
        Vec<(::arrow2::datatypes::Field, Box<dyn ::arrow2::array::Array>)>,
    > {
        use crate::Loggable as _;
        Ok([{
            Some({
                let array =
                    <crate::components::AnnotationContext<'s>>::try_to_arrow([&self.context], None);
                array.map(|array| {
                    let datatype = ::arrow2::datatypes::DataType::Extension(
                        "rerun.components.AnnotationContext".into(),
                        Box::new(array.data_type().clone()),
                        Some("rerun.annotation_context".into()),
                    );
                    (
                        ::arrow2::datatypes::Field::new("context", datatype, false),
                        array,
                    )
                })
            })
            .transpose()
            .map_err(|err| crate::SerializationError::Context {
                location: "rerun.archetypes.AnnotationContext#context".into(),
                source: Box::new(err),
            })?
        }]
        .into_iter()
        .flatten()
        .collect())
    }

    #[inline]
    fn try_from_arrow(
        data: impl IntoIterator<Item = (::arrow2::datatypes::Field, &'s dyn ::arrow2::array::Array)>,
    ) -> crate::DeserializationResult<Self> {
        use crate::Loggable as _;
        let arrays_by_name: ::std::collections::HashMap<_, _> = data
            .into_iter()
            .map(|(field, array)| (field.name, array))
            .collect();
        let context = {
            let array = arrays_by_name
                .get("context")
                .ok_or_else(|| crate::DeserializationError::MissingData {
                    backtrace: ::backtrace::Backtrace::new_unresolved(),
                })
                .map_err(|err| crate::DeserializationError::Context {
                    location: "rerun.archetypes.AnnotationContext#context".into(),
                    source: Box::new(err),
                })?;
            <crate::components::AnnotationContext<'s>>::try_from_arrow_opt(&**array)
                .map_err(|err| crate::DeserializationError::Context {
                    location: "rerun.archetypes.AnnotationContext#context".into(),
                    source: Box::new(err),
                })?
                .into_iter()
                .next()
                .flatten()
                .ok_or_else(|| crate::DeserializationError::MissingData {
                    backtrace: ::backtrace::Backtrace::new_unresolved(),
                })
                .map_err(|err| crate::DeserializationError::Context {
                    location: "rerun.archetypes.AnnotationContext#context".into(),
                    source: Box::new(err),
                })?
        };
        Ok(Self { context })
    }
}

impl<'s> AnnotationContext<'s> {
    pub fn new(context: impl Into<crate::components::AnnotationContext<'s>>) -> Self {
        Self {
            context: context.into(),
        }
    }
}
