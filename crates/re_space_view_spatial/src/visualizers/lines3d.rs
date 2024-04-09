use itertools::Either;
use re_entity_db::{EntityPath, InstancePathHash};
use re_query_cache2::{range_zip_1x5, CachedRangeData};
use re_renderer::PickingLayerInstanceId;
use re_types::{
    archetypes::LineStrips3D,
    components::{ClassId, Color, InstanceKey, KeypointId, LineStrip3D, Radius, Text},
    Loggable as _,
};
use re_viewer_context::{
    ApplicableEntities, IdentifiedViewSystem, ResolvedAnnotationInfos,
    SpaceViewSystemExecutionError, ViewContextCollection, ViewQuery, ViewerContext,
    VisualizableEntities, VisualizableFilterContext, VisualizerQueryInfo, VisualizerSystem,
};

use crate::{
    contexts::{EntityDepthOffsets, SpatialSceneEntityContext},
    view_kind::SpatialSpaceViewKind,
    visualizers::{UiLabel, UiLabelTarget},
};

use super::{
    entity_iterator::ArchetypeResults, filter_visualizable_3d_entities,
    process_annotation_and_keypoint_slices2, process_color_slice2, process_radius_slice2,
    SpatialViewVisualizerData, SIZE_BOOST_IN_POINTS_FOR_LINE_OUTLINES,
};

// ---

pub struct Lines3DVisualizer {
    /// If the number of arrows in the batch is > max_labels, don't render point labels.
    pub max_labels: usize,
    pub data: SpatialViewVisualizerData,
}

impl Default for Lines3DVisualizer {
    fn default() -> Self {
        Self {
            max_labels: 10,
            data: SpatialViewVisualizerData::new(Some(SpatialSpaceViewKind::ThreeD)),
        }
    }
}

// NOTE: Do not put profile scopes in these methods. They are called for all entities and all
// timestamps within a time range -- it's _a lot_.
impl Lines3DVisualizer {
    fn process_labels<'a>(
        entity_path: &'a EntityPath,
        strips: &'a [LineStrip3D],
        labels: &'a [Text],
        colors: &'a [egui::Color32],
        annotation_infos: &'a ResolvedAnnotationInfos,
        world_from_obj: glam::Affine3A,
    ) -> impl Iterator<Item = UiLabel> + 'a {
        if labels.is_empty() {
            return Either::Left(std::iter::empty());
        }

        // TODO: need a helper for this
        let labels = labels
            .iter()
            .chain(std::iter::repeat(labels.last().unwrap()))
            .take(strips.len());

        let ui_labels = itertools::izip!(annotation_infos.iter(), strips, labels, colors,)
            .enumerate()
            .filter_map(move |(i, (annotation_info, strip, label, color))| {
                let label = annotation_info.label(Some(label.as_str()));
                match (strip, label) {
                    (strip, Some(label)) => {
                        let midpoint = strip
                            .0
                            .iter()
                            .copied()
                            .map(glam::Vec3::from)
                            .sum::<glam::Vec3>()
                            / (strip.0.len() as f32);

                        Some(UiLabel {
                            text: label,
                            color: *color,
                            target: UiLabelTarget::Position3D(
                                world_from_obj.transform_point3(midpoint),
                            ),
                            labeled_instance: InstancePathHash::instance(
                                entity_path,
                                InstanceKey(i as _),
                            ),
                        })
                    }
                    _ => None,
                }
            });

        Either::Right(ui_labels)
    }

    fn process_data<'a>(
        &mut self,
        line_builder: &mut re_renderer::LineDrawableBuilder<'_>,
        query: &ViewQuery<'_>,
        data: impl Iterator<Item = Lines3DComponentData<'a>>,
        entity_path: &EntityPath,
        ent_context: &SpatialSceneEntityContext<'_>,
    ) {
        for data in data {
            let num_instances = data.strips.len();
            if num_instances == 0 {
                continue;
            }

            let (annotation_infos, _) = process_annotation_and_keypoint_slices2(
                query.latest_at,
                num_instances,
                data.strips.iter().map(|_| glam::Vec3::ZERO),
                data.keypoint_ids,
                data.class_ids,
                &ent_context.annotations,
            );

            let radii = process_radius_slice2(entity_path, num_instances, data.radii);
            let colors =
                process_color_slice2(entity_path, num_instances, &annotation_infos, data.colors);

            if num_instances <= self.max_labels {
                self.data.ui_labels.extend(Self::process_labels(
                    entity_path,
                    data.strips,
                    data.labels,
                    &colors,
                    &annotation_infos,
                    ent_context.world_from_entity,
                ));
            }

            let mut line_batch = line_builder
                .batch(entity_path.to_string())
                .depth_offset(ent_context.depth_offset)
                .world_from_obj(ent_context.world_from_entity)
                .outline_mask_ids(ent_context.highlight.overall)
                .picking_object_id(re_renderer::PickingLayerObjectId(entity_path.hash64()));

            let mut bounding_box = macaw::BoundingBox::nothing();

            let mut num_rendered_strips = 0usize;
            for (i, (strip, radius, color)) in
                itertools::izip!(data.strips, radii, colors).enumerate()
            {
                let lines = line_batch
                    .add_strip(strip.0.iter().copied().map(Into::into))
                    .color(color)
                    .radius(radius)
                    .picking_instance_id(PickingLayerInstanceId(i as _));

                if let Some(outline_mask_ids) =
                    ent_context.highlight.instances.get(&InstanceKey(i as _))
                {
                    lines.outline_mask_ids(*outline_mask_ids);
                }

                for p in &strip.0 {
                    bounding_box.extend((*p).into());
                }

                num_rendered_strips += 1;
            }
            debug_assert_eq!(data.strips.len(), num_rendered_strips, "the number of renderer strips after all post-processing is done should be equal to {} (got {num_rendered_strips} instead)", data.strips.len());

            self.data.add_bounding_box(
                entity_path.hash(),
                bounding_box,
                ent_context.world_from_entity,
            );
        }
    }
}

// ---

struct Lines3DComponentData<'a> {
    // Point of views
    pub strips: &'a [LineStrip3D],

    // Clamped to edge
    pub colors: &'a [Color],
    pub radii: &'a [Radius],
    pub labels: &'a [Text],
    pub keypoint_ids: &'a [KeypointId],
    pub class_ids: &'a [ClassId],
}

impl IdentifiedViewSystem for Lines3DVisualizer {
    fn identifier() -> re_viewer_context::ViewSystemIdentifier {
        "Lines3D".into()
    }
}

impl VisualizerSystem for Lines3DVisualizer {
    fn visualizer_query_info(&self) -> VisualizerQueryInfo {
        VisualizerQueryInfo::from_archetype::<LineStrips3D>()
    }

    fn filter_visualizable_entities(
        &self,
        entities: ApplicableEntities,
        context: &dyn VisualizableFilterContext,
    ) -> VisualizableEntities {
        re_tracing::profile_function!();
        filter_visualizable_3d_entities(entities, context)
    }

    fn execute(
        &mut self,
        ctx: &ViewerContext<'_>,
        view_query: &ViewQuery<'_>,
        view_ctx: &ViewContextCollection,
    ) -> Result<Vec<re_renderer::QueueableDrawData>, SpaceViewSystemExecutionError> {
        // Counting all lines (strips and vertices) ahead of time is a bit expensive since we need to do a full query for this.
        // We choose a semi-dynamic approach here, where we reserve on every new line batch.
        let mut line_builder = re_renderer::LineDrawableBuilder::new(ctx.render_ctx);
        line_builder.radius_boost_in_ui_points_for_outlines(SIZE_BOOST_IN_POINTS_FOR_LINE_OUTLINES);

        super::entity_iterator::process_archetype::<Lines3DVisualizer, LineStrips3D, _>(
            ctx,
            view_query,
            view_ctx,
            view_ctx.get::<EntityDepthOffsets>()?.points,
            |ctx, entity_path, _entity_props, spatial_ctx, results| match results {
                ArchetypeResults::LatestAt(query, results) => {
                    re_tracing::profile_scope!(format!("{entity_path} @ {query:?}"));

                    let resolver = ctx.recording().resolver();

                    let strips: &[LineStrip3D] = {
                        let Some(strips) = results.get(LineStrip3D::name()) else {
                            return Ok(());
                        };
                        ArchetypeResults::latest_at(resolver, strips)?
                    };

                    if strips.is_empty() {
                        return Ok(());
                    }

                    line_builder.reserve_strips(strips.len())?;
                    line_builder.reserve_vertices(
                        strips.iter().map(|strip| strip.0.len()).sum::<usize>(),
                    )?;

                    let colors =
                        ArchetypeResults::latest_at(resolver, results.get_or_empty(Color::name()))?;
                    let radii = ArchetypeResults::latest_at(
                        resolver,
                        results.get_or_empty(Radius::name()),
                    )?;
                    let labels =
                        ArchetypeResults::latest_at(resolver, results.get_or_empty(Text::name()))?;
                    let class_ids = ArchetypeResults::latest_at(
                        resolver,
                        results.get_or_empty(ClassId::name()),
                    )?;
                    let keypoint_ids = ArchetypeResults::latest_at(
                        resolver,
                        results.get_or_empty(KeypointId::name()),
                    )?;

                    let data = Lines3DComponentData {
                        strips,
                        colors,
                        radii,
                        labels,
                        keypoint_ids,
                        class_ids,
                    };

                    self.process_data(
                        &mut line_builder,
                        view_query,
                        std::iter::once(data),
                        entity_path,
                        spatial_ctx,
                    );

                    Ok(())
                }

                ArchetypeResults::Range(query, results) => {
                    re_tracing::profile_scope!(format!("{entity_path} @ {query:?}"));

                    let resolver = ctx.recording().resolver();

                    let strips: CachedRangeData<'_, LineStrip3D> = {
                        let Some(strips) = results.get(LineStrip3D::name()) else {
                            return Ok(());
                        };
                        strips.to_dense(resolver)
                    };
                    ArchetypeResults::check_range(query, &strips)?;

                    let num_strips = strips
                        .range_indexed(query.range())
                        .map(|(_, strips)| strips.len())
                        .sum::<usize>();
                    if num_strips == 0 {
                        return Ok(());
                    }

                    let num_vertices = strips
                        .range_indexed(query.range())
                        .map(|(_, strips)| strips.iter().map(|strip| strip.0.len()).sum::<usize>())
                        .sum::<usize>();

                    line_builder.reserve_strips(num_strips)?;
                    line_builder.reserve_vertices(num_vertices)?;

                    let colors = results.get_or_empty(Color::name()).to_dense(resolver);
                    ArchetypeResults::check_range(query, &colors)?;
                    let radii = results.get_or_empty(Radius::name()).to_dense(resolver);
                    ArchetypeResults::check_range(query, &radii)?;
                    let labels = results.get_or_empty(Text::name()).to_dense(resolver);
                    ArchetypeResults::check_range(query, &labels)?;
                    let class_ids = results.get_or_empty(ClassId::name()).to_dense(resolver);
                    ArchetypeResults::check_range(query, &class_ids)?;
                    let keypoint_ids = results.get_or_empty(KeypointId::name()).to_dense(resolver);
                    ArchetypeResults::check_range(query, &keypoint_ids)?;

                    let data = range_zip_1x5(
                        strips.range_indexed(query.range()),
                        colors.range_indexed(query.range()),
                        radii.range_indexed(query.range()),
                        labels.range_indexed(query.range()),
                        class_ids.range_indexed(query.range()),
                        keypoint_ids.range_indexed(query.range()),
                    )
                    .map(
                        |(_index, strips, colors, radii, labels, class_ids, keypoint_ids)| {
                            Lines3DComponentData {
                                strips,
                                colors: colors.unwrap_or_default(),
                                radii: radii.unwrap_or_default(),
                                labels: labels.unwrap_or_default(),
                                class_ids: class_ids.unwrap_or_default(),
                                keypoint_ids: keypoint_ids.unwrap_or_default(),
                            }
                        },
                    );

                    self.process_data(
                        &mut line_builder,
                        view_query,
                        data,
                        entity_path,
                        spatial_ctx,
                    );

                    Ok(())
                }
            },
        )?;

        Ok(vec![(line_builder.into_draw_data()?.into())])
    }

    fn data(&self) -> Option<&dyn std::any::Any> {
        Some(self.data.as_any())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
