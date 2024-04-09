use itertools::{Either, Itertools};
use re_entity_db::{EntityPath, InstancePathHash};
use re_query_cache2::{range_zip_1x5, CachedRangeData};
use re_renderer::{LineDrawableBuilder, PickingLayerInstanceId, PointCloudBuilder};
use re_types::{
    archetypes::Points3D,
    components::{ClassId, Color, InstanceKey, KeypointId, Position3D, Radius, Text},
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
    visualizers::{
        load_keypoint_connections, process_annotation_and_keypoint_slices2, process_color_slice2,
        process_radius_slice2, UiLabel, UiLabelTarget,
    },
};

use super::{
    entity_iterator::ArchetypeResults, filter_visualizable_3d_entities, SpatialViewVisualizerData,
    SIZE_BOOST_IN_POINTS_FOR_POINT_OUTLINES,
};

pub struct Points3DVisualizer {
    /// If the number of points in the batch is > max_labels, don't render point labels.
    pub max_labels: usize,
    pub data: SpatialViewVisualizerData,
}

impl Default for Points3DVisualizer {
    fn default() -> Self {
        Self {
            max_labels: 10,
            data: SpatialViewVisualizerData::new(Some(SpatialSpaceViewKind::ThreeD)),
        }
    }
}

struct Points3DComponentData<'a> {
    // Point of views
    positions: &'a [Position3D],

    // Clamped to edge
    colors: &'a [Color],
    radii: &'a [Radius],
    labels: &'a [Text],
    keypoint_ids: &'a [KeypointId],
    class_ids: &'a [ClassId],
}

// NOTE: Do not put profile scopes in these methods. They are called for all entities and all
// timestamps within a time range -- it's _a lot_.
impl Points3DVisualizer {
    fn process_labels<'a>(
        entity_path: &'a EntityPath,
        positions: &'a [glam::Vec3],
        labels: &'a [Text],
        colors: &'a [egui::Color32],
        annotation_infos: &'a ResolvedAnnotationInfos,
        world_from_obj: glam::Affine3A,
    ) -> impl Iterator<Item = UiLabel> + 'a {
        if labels.is_empty() {
            return Either::Left(std::iter::empty());
        }

        // TODO: this needs a helper somewhere
        let labels = labels
            .iter()
            .chain(std::iter::repeat(labels.last().unwrap()))
            .take(positions.len());

        let ui_labels = itertools::izip!(annotation_infos.iter(), positions, labels, colors)
            .enumerate()
            .filter_map(move |(i, (annotation_info, point, label, color))| {
                let label = annotation_info.label(Some(label.as_str()));
                match (point, label) {
                    (point, Some(label)) => Some(UiLabel {
                        text: label,
                        color: *color,
                        target: UiLabelTarget::Position3D(world_from_obj.transform_point3(*point)),
                        labeled_instance: InstancePathHash::instance(
                            entity_path,
                            InstanceKey(i as _),
                        ),
                    }),
                    _ => None,
                }
            });

        Either::Right(ui_labels)
    }

    fn process_data<'a>(
        &mut self,
        point_builder: &mut PointCloudBuilder<'_>,
        line_builder: &mut LineDrawableBuilder<'_>,
        query: &ViewQuery<'_>,
        entity_path: &EntityPath,
        ent_context: &SpatialSceneEntityContext<'_>,
        data: impl Iterator<Item = Points3DComponentData<'a>>,
    ) -> Result<(), SpaceViewSystemExecutionError> {
        for data in data {
            let num_instances = data.positions.len();
            if num_instances == 0 {
                continue;
            }

            let picking_ids = (0..num_instances)
                .map(|i| PickingLayerInstanceId(i as _))
                .collect_vec();

            let (annotation_infos, keypoints) = process_annotation_and_keypoint_slices2(
                query.latest_at,
                num_instances,
                data.positions.iter().map(|p| p.0.into()),
                data.keypoint_ids,
                data.class_ids,
                &ent_context.annotations,
            );

            let positions = bytemuck::cast_slice(data.positions);
            let radii = process_radius_slice2(entity_path, num_instances, data.radii);
            let colors =
                process_color_slice2(entity_path, num_instances, &annotation_infos, data.colors);

            {
                let point_batch = point_builder
                    .batch(entity_path.to_string())
                    .world_from_obj(ent_context.world_from_entity)
                    .outline_mask_ids(ent_context.highlight.overall)
                    .picking_object_id(re_renderer::PickingLayerObjectId(entity_path.hash64()));

                let mut point_range_builder =
                    point_batch.add_points(positions, &radii, &colors, &picking_ids);

                // Determine if there's any sub-ranges that need extra highlighting.
                {
                    for (highlighted_key, instance_mask_ids) in &ent_context.highlight.instances {
                        let highlighted_point_index =
                            (highlighted_key.0 < num_instances as u64).then_some(highlighted_key.0);
                        if let Some(highlighted_point_index) = highlighted_point_index {
                            point_range_builder = point_range_builder
                                .push_additional_outline_mask_ids_for_range(
                                    highlighted_point_index as u32
                                        ..highlighted_point_index as u32 + 1,
                                    *instance_mask_ids,
                                );
                        }
                    }
                }
            }

            self.data.add_bounding_box_from_points(
                entity_path.hash(),
                positions.iter().copied(),
                ent_context.world_from_entity,
            );

            load_keypoint_connections(line_builder, ent_context, entity_path, &keypoints)?;

            if num_instances <= self.max_labels {
                // Max labels is small enough that we can afford iterating on the colors again.
                let colors = process_color_slice2(
                    entity_path,
                    num_instances,
                    &annotation_infos,
                    data.colors,
                );

                self.data.ui_labels.extend(Self::process_labels(
                    entity_path,
                    positions,
                    data.labels,
                    &colors,
                    &annotation_infos,
                    ent_context.world_from_entity,
                ));
            }
        }

        Ok(())
    }
}

impl IdentifiedViewSystem for Points3DVisualizer {
    fn identifier() -> re_viewer_context::ViewSystemIdentifier {
        "Points3D".into()
    }
}

impl VisualizerSystem for Points3DVisualizer {
    fn visualizer_query_info(&self) -> VisualizerQueryInfo {
        VisualizerQueryInfo::from_archetype::<Points3D>()
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
        let mut point_builder = PointCloudBuilder::new(ctx.render_ctx);
        point_builder
            .radius_boost_in_ui_points_for_outlines(SIZE_BOOST_IN_POINTS_FOR_POINT_OUTLINES);

        // We need lines from keypoints. The number of lines we'll have is harder to predict, so we'll go
        // with the dynamic allocation approach.
        let mut line_builder = LineDrawableBuilder::new(ctx.render_ctx);
        line_builder
            .radius_boost_in_ui_points_for_outlines(SIZE_BOOST_IN_POINTS_FOR_POINT_OUTLINES);

        // TODO: any chance we can dedupe this...?
        super::entity_iterator::process_archetype::<Points3DVisualizer, Points3D, _>(
            ctx,
            view_query,
            view_ctx,
            view_ctx.get::<EntityDepthOffsets>()?.points,
            |ctx, entity_path, _entity_props, spatial_ctx, results| match results {
                ArchetypeResults::LatestAt(query, results) => {
                    re_tracing::profile_scope!(format!("{entity_path} @ {query:?}"));

                    let resolver = ctx.recording().resolver();

                    let positions: &[Position3D] = {
                        let Some(positions) = results.get(Position3D::name()) else {
                            return Ok(());
                        };
                        ArchetypeResults::latest_at(resolver, positions)?
                    };

                    if positions.is_empty() {
                        return Ok(());
                    }

                    point_builder.reserve(positions.len())?;

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

                    let data = Points3DComponentData {
                        positions,
                        colors,
                        radii,
                        labels,
                        keypoint_ids,
                        class_ids,
                    };

                    self.process_data(
                        &mut point_builder,
                        &mut line_builder,
                        view_query,
                        entity_path,
                        spatial_ctx,
                        std::iter::once(data),
                    )
                }

                ArchetypeResults::Range(query, results) => {
                    re_tracing::profile_scope!(format!("{entity_path} @ {query:?}"));

                    let resolver = ctx.recording().resolver();

                    let positions: CachedRangeData<'_, Position3D> = {
                        let Some(positions) = results.get(Position3D::name()) else {
                            return Ok(());
                        };
                        positions.to_dense(resolver)
                    };
                    ArchetypeResults::check_range(query, &positions)?;

                    let num_positions = positions
                        .range_indexed(query.range())
                        .map(|(_, positions)| positions.len())
                        .sum::<usize>();
                    if num_positions == 0 {
                        return Ok(());
                    }

                    point_builder.reserve(num_positions)?;

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
                        positions.range_indexed(query.range()),
                        colors.range_indexed(query.range()),
                        radii.range_indexed(query.range()),
                        labels.range_indexed(query.range()),
                        class_ids.range_indexed(query.range()),
                        keypoint_ids.range_indexed(query.range()),
                    )
                    .map(
                        |(_index, positions, colors, radii, labels, class_ids, keypoint_ids)| {
                            Points3DComponentData {
                                positions,
                                colors: colors.unwrap_or_default(),
                                radii: radii.unwrap_or_default(),
                                labels: labels.unwrap_or_default(),
                                class_ids: class_ids.unwrap_or_default(),
                                keypoint_ids: keypoint_ids.unwrap_or_default(),
                            }
                        },
                    );

                    self.process_data(
                        &mut point_builder,
                        &mut line_builder,
                        view_query,
                        entity_path,
                        spatial_ctx,
                        data,
                    )
                }
            },
        )?;

        Ok(vec![
            point_builder.into_draw_data()?.into(),
            line_builder.into_draw_data()?.into(),
        ])
    }

    fn data(&self) -> Option<&dyn std::any::Any> {
        Some(self.data.as_any())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
