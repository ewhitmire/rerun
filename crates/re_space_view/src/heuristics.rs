use re_log_types::EntityPathFilter;
use re_viewer_context::{
    ApplicableEntities, IdentifiedViewSystem, RecommendedSpaceView, SpaceViewClass,
    SpaceViewSpawnHeuristics, ViewerContext, VisualizableEntities, VisualizerSystem,
};

use crate::visualizable;

/// Spawns a space view for each single entity which is visualizable & indicator-matching for a given visualizer.
///
/// This is used as utility by *some* space view types that want
/// to spawn a space view for every single entity that is visualizable with a given visualizer.
pub fn suggest_space_view_for_each_entity<TVisualizer>(
    ctx: &ViewerContext<'_>,
    space_view: &impl SpaceViewClass,
) -> SpaceViewSpawnHeuristics
where
    TVisualizer: VisualizerSystem + IdentifiedViewSystem + Default,
{
    re_tracing::profile_function!();

    let Some(indicator_matching_entities) = ctx
        .indicated_entities_per_visualizer
        .get(&TVisualizer::identifier())
    else {
        return Default::default();
    };
    let Some(applicable_entities) = ctx
        .applicable_entities_per_visualizer
        .get(&TVisualizer::identifier())
    else {
        return Default::default();
    };

    let visualizer = TVisualizer::default();
    let recommended_space_views = applicable_entities
        .intersection(indicator_matching_entities)
        .filter_map(|entity| {
            let context = space_view.visualizable_filter_context(entity, ctx.entity_db);

            let applicable_entities = ApplicableEntities(std::iter::once(entity.clone()).collect());
            let filtered_entities =
                visualizer.filter_visualizable_entities(&applicable_entities, context.as_ref());
            let visualizable_entities =
                VisualizableEntities::new(applicable_entities.clone(), &filtered_entities);

            if visualizable_entities.is_empty() {
                None
            } else {
                Some(RecommendedSpaceView {
                    root: entity.clone(),
                    query_filter: EntityPathFilter::single_entity_filter(entity),
                })
            }
        })
        .collect();

    re_viewer_context::SpaceViewSpawnHeuristics {
        recommended_space_views,
    }
}
