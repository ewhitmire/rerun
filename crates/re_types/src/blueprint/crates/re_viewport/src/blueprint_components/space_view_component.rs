// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/blueprint/space_view.fbs".

#![allow(trivial_numeric_casts)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

/// **Blueprint**: A view of a space.
///
/// Unstable. Used for the ongoing blueprint experimentations.
#[derive(Clone, Debug)]
pub struct SpaceView {
    pub space_view: crate::ArrowBuffer<u8>,
}

impl From<crate::ArrowBuffer<u8>> for SpaceView {
    #[inline]
    fn from(space_view: crate::ArrowBuffer<u8>) -> Self {
        Self { space_view }
    }
}

impl From<SpaceView> for crate::ArrowBuffer<u8> {
    #[inline]
    fn from(value: SpaceView) -> Self {
        value.space_view
    }
}

impl<'a> From<SpaceView> for ::std::borrow::Cow<'a, SpaceView> {
    #[inline]
    fn from(value: SpaceView) -> Self {
        std::borrow::Cow::Owned(value)
    }
}

impl<'a> From<&'a SpaceView> for ::std::borrow::Cow<'a, SpaceView> {
    #[inline]
    fn from(value: &'a SpaceView) -> Self {
        std::borrow::Cow::Borrowed(value)
    }
}

impl crate::Loggable for SpaceView {
    type Name = crate::ComponentName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.blueprint.SpaceView".into()
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        use ::arrow2::datatypes::*;
        DataType::Struct(vec![Field {
            name: "space_view".to_owned(),
            data_type: DataType::List(Box::new(Field {
                name: "item".to_owned(),
                data_type: DataType::UInt8,
                is_nullable: false,
                metadata: [].into(),
            })),
            is_nullable: false,
            metadata: [].into(),
        }])
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> crate::SerializationResult<Box<dyn ::arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        re_tracing::profile_function!();
        use crate::{Loggable as _, ResultExt as _};
        use ::arrow2::{array::*, datatypes::*};
        Ok({
            let (somes, data): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    (datum.is_some(), datum)
                })
                .unzip();
            let bitmap: Option<::arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            StructArray::new(
                <crate::blueprint::SpaceView>::arrow_datatype(),
                vec![{
                    let (somes, space_view): (Vec<_>, Vec<_>) = data
                        .iter()
                        .map(|datum| {
                            let datum = datum.as_ref().map(|datum| {
                                let Self { space_view, .. } = &**datum;
                                space_view.clone()
                            });
                            (datum.is_some(), datum)
                        })
                        .unzip();
                    let space_view_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                        let any_nones = somes.iter().any(|some| !*some);
                        any_nones.then(|| somes.into())
                    };
                    {
                        use arrow2::{buffer::Buffer, offset::OffsetsBuffer};
                        let space_view_inner_data: Buffer<_> = space_view
                            .iter()
                            .flatten()
                            .map(|b| b.as_slice())
                            .collect::<Vec<_>>()
                            .concat()
                            .into();
                        let space_view_inner_bitmap: Option<::arrow2::bitmap::Bitmap> = None;
                        let offsets = ::arrow2::offset::Offsets::<i32>::try_from_lengths(
                            space_view.iter().map(|opt| {
                                opt.as_ref()
                                    .map(|datum| datum.num_instances())
                                    .unwrap_or_default()
                            }),
                        )
                        .unwrap()
                        .into();
                        ListArray::new(
                            DataType::List(Box::new(Field {
                                name: "item".to_owned(),
                                data_type: DataType::UInt8,
                                is_nullable: false,
                                metadata: [].into(),
                            })),
                            offsets,
                            PrimitiveArray::new(
                                DataType::UInt8,
                                space_view_inner_data,
                                space_view_inner_bitmap,
                            )
                            .boxed(),
                            space_view_bitmap,
                        )
                        .boxed()
                    }
                }],
                bitmap,
            )
            .boxed()
        })
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn from_arrow_opt(
        arrow_data: &dyn ::arrow2::array::Array,
    ) -> crate::DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        re_tracing::profile_function!();
        use crate::{Loggable as _, ResultExt as _};
        use ::arrow2::{array::*, buffer::*, datatypes::*};
        Ok({
            let arrow_data = arrow_data
                .as_any()
                .downcast_ref::<::arrow2::array::StructArray>()
                .ok_or_else(|| {
                    crate::DeserializationError::datatype_mismatch(
                        DataType::Struct(vec![Field {
                            name: "space_view".to_owned(),
                            data_type: DataType::List(Box::new(Field {
                                name: "item".to_owned(),
                                data_type: DataType::UInt8,
                                is_nullable: false,
                                metadata: [].into(),
                            })),
                            is_nullable: false,
                            metadata: [].into(),
                        }]),
                        arrow_data.data_type().clone(),
                    )
                })
                .with_context("rerun.blueprint.SpaceView")?;
            if arrow_data.is_empty() {
                Vec::new()
            } else {
                let (arrow_data_fields, arrow_data_arrays) =
                    (arrow_data.fields(), arrow_data.values());
                let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data_fields
                    .iter()
                    .map(|field| field.name.as_str())
                    .zip(arrow_data_arrays)
                    .collect();
                let space_view = {
                    if !arrays_by_name.contains_key("space_view") {
                        return Err(crate::DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "space_view",
                        ))
                        .with_context("rerun.blueprint.SpaceView");
                    }
                    let arrow_data = &**arrays_by_name["space_view"];
                    {
                        let arrow_data = arrow_data
                            .as_any()
                            .downcast_ref::<::arrow2::array::ListArray<i32>>()
                            .ok_or_else(|| {
                                crate::DeserializationError::datatype_mismatch(
                                    DataType::List(Box::new(Field {
                                        name: "item".to_owned(),
                                        data_type: DataType::UInt8,
                                        is_nullable: false,
                                        metadata: [].into(),
                                    })),
                                    arrow_data.data_type().clone(),
                                )
                            })
                            .with_context("rerun.blueprint.SpaceView#space_view")?;
                        if arrow_data.is_empty() {
                            Vec::new()
                        } else {
                            let arrow_data_inner = {
                                let arrow_data_inner = &**arrow_data.values();
                                arrow_data_inner
                                    .as_any()
                                    .downcast_ref::<UInt8Array>()
                                    .ok_or_else(|| {
                                        crate::DeserializationError::datatype_mismatch(
                                            DataType::UInt8,
                                            arrow_data_inner.data_type().clone(),
                                        )
                                    })
                                    .with_context("rerun.blueprint.SpaceView#space_view")?
                                    .values()
                            };
                            let offsets = arrow_data.offsets();
                            arrow2::bitmap::utils::ZipValidity::new_with_validity(
                                offsets.iter().zip(offsets.lengths()),
                                arrow_data.validity(),
                            )
                            .map(|elem| {
                                elem.map(|(start, len)| {
                                    let start = *start as usize;
                                    let end = start + len;
                                    if end as usize > arrow_data_inner.len() {
                                        return Err(crate::DeserializationError::offset_slice_oob(
                                            (start, end),
                                            arrow_data_inner.len(),
                                        ));
                                    }

                                    #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                    let data = unsafe {
                                        arrow_data_inner
                                            .clone()
                                            .sliced_unchecked(start as usize, end - start as usize)
                                    };
                                    let data = crate::ArrowBuffer::from(data);
                                    Ok(data)
                                })
                                .transpose()
                            })
                            .collect::<crate::DeserializationResult<Vec<Option<_>>>>()?
                        }
                        .into_iter()
                    }
                };
                arrow2::bitmap::utils::ZipValidity::new_with_validity(
                    ::itertools::izip!(space_view),
                    arrow_data.validity(),
                )
                .map(|opt| {
                    opt.map(|(space_view)| {
                        Ok(Self {
                            space_view: space_view
                                .ok_or_else(crate::DeserializationError::missing_data)
                                .with_context("rerun.blueprint.SpaceView#space_view")?,
                        })
                    })
                    .transpose()
                })
                .collect::<crate::DeserializationResult<Vec<_>>>()
                .with_context("rerun.blueprint.SpaceView")?
            }
        })
    }
}
