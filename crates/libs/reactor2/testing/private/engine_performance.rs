use super::*;

#[derive(Clone, Copy, Default)]
pub(crate) struct PerformanceStats {
    pub(crate) tree_build: std::time::Duration,
    pub(crate) elements_diffed: u64,
    pub(crate) elements_skipped: u64,
    pub(crate) elements_created: u64,
}

impl<R: NativeRuntime> Engine<R> {
    pub(crate) fn begin_performance_pass(&mut self) {
        self.performance = PerformanceStats::default();
    }

    pub(crate) fn record_tree_build(&mut self, elapsed: std::time::Duration) {
        self.performance.tree_build += elapsed;
    }

    pub(crate) fn record_element_created(&mut self) {
        self.performance.elements_created += 1;
    }

    pub(crate) fn begin_element_diff(&mut self) -> (usize, u64) {
        self.performance.elements_diffed += 1;
        (self.pending.len(), self.performance.elements_created)
    }

    pub(crate) fn finish_element_diff(&mut self, before: (usize, u64)) {
        if before == (self.pending.len(), self.performance.elements_created) {
            self.performance.elements_skipped += 1;
        }
    }

    pub(crate) fn performance_stats(&self) -> PerformanceStats {
        self.performance
    }
}
