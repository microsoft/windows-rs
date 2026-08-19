use super::*;

impl<R: NativeRuntime> Reactor<R> {
    pub(crate) fn set_render_complete<F>(&mut self, callback: F)
    where
        F: Fn(&crate::performance::RenderMetrics) + 'static,
    {
        self.render_complete = Some(Rc::new(callback));
    }

    pub(super) fn finish_performance_pass(&self, before_effects: Duration, effects: Duration) {
        let Some(callback) = self.render_complete.as_ref().map(Rc::clone) else {
            return;
        };
        let stats = self.engine.performance_stats();
        let metrics = crate::performance::RenderMetrics {
            tree_build_ms: stats.tree_build.as_secs_f64() * 1000.0,
            reconcile_ms: before_effects
                .saturating_sub(stats.tree_build)
                .as_secs_f64()
                * 1000.0,
            effects_ms: effects.as_secs_f64() * 1000.0,
            elements_diffed: stats.elements_diffed,
            elements_skipped: stats.elements_skipped,
            elements_created: stats.elements_created,
        };
        callback(&metrics);
    }
}
