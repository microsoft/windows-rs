use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::rc::Rc;
use windows_reactor::Key;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Observation {
    pub pid: u32,
    pub parent_pid: u32,
    pub created: Option<u64>,
    pub name: String,
    pub threads: u32,
    pub sampled_at: u64,
    pub cpu_ticks: Option<u64>,
    pub working_set: Option<u64>,
    pub private_bytes: Option<u64>,
    pub unavailable: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub version: u32,
    pub sequence: u64,
    pub processors: u32,
    pub processes: Vec<Observation>,
    #[serde(default)]
    pub system: Option<SystemObservation>,
}

impl Snapshot {
    pub fn validate(&self) -> Result<(), String> {
        if !matches!(self.version, 1..=3) || self.processors == 0 || self.sequence == 0 {
            return Err("Invalid snapshot version, sequence, or processor count".into());
        }
        match (self.version, &self.system) {
            (1, None) => {}
            (2, Some(system))
                if system.logical_cpus.is_none() && system.logical_cpu_error.is_none() =>
            {
                system.validate(self.processors)?;
            }
            (3, Some(system)) => system.validate(self.processors)?,
            _ => return Err("System metrics do not match the snapshot version".into()),
        }
        let mut pids = HashSet::new();
        for process in &self.processes {
            if !pids.insert(process.pid) {
                return Err(format!("Duplicate PID {} in snapshot", process.pid));
            }
            if process.created.is_none()
                && (process.cpu_ticks.is_some() || process.unavailable.is_none())
            {
                return Err("Unverified process identity requires an unavailable reason".into());
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MemoryObservation {
    pub total: u64,
    pub available: u64,
}

impl MemoryObservation {
    pub fn percent(&self) -> f64 {
        (self.total - self.available) as f64 / self.total as f64 * 100.0
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LogicalCpuObservation {
    pub id: ProcessorId,
    pub percent: Option<f64>,
    pub error: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProcessorId {
    pub node: u32,
    pub number: u32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SystemObservation {
    pub sampled_at: u64,
    pub cpu_percent: Option<f64>,
    pub cpu_error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logical_cpus: Option<Vec<LogicalCpuObservation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logical_cpu_error: Option<String>,
    pub memory: Option<MemoryObservation>,
    pub memory_error: Option<String>,
}

impl SystemObservation {
    fn validate(&self, processors: u32) -> Result<(), String> {
        if self
            .cpu_percent
            .is_some_and(|p| !p.is_finite() || !(0.0..=100.0).contains(&p))
            || (self.cpu_percent.is_some() && self.cpu_error.is_some())
        {
            return Err("Invalid system CPU observation".into());
        }
        if let Some(logical) = &self.logical_cpus {
            if logical.len() != processors as usize || self.logical_cpu_error.is_some() {
                return Err("Invalid logical processor count or error".into());
            }
            let mut ids = std::collections::BTreeSet::new();
            for cpu in logical {
                if !ids.insert(cpu.id)
                    || cpu
                        .percent
                        .is_some_and(|p| !p.is_finite() || !(0.0..=100.0).contains(&p))
                    || (cpu.percent.is_some() && cpu.error.is_some())
                {
                    return Err(format!("Invalid logical CPU observation: {:?}", cpu.id));
                }
            }
        }
        if self
            .memory
            .as_ref()
            .is_some_and(|m| m.total == 0 || m.available > m.total)
            || self.memory.is_some() == self.memory_error.is_some()
        {
            return Err("Invalid system memory observation".into());
        }
        Ok(())
    }
}

pub const SECOND: u64 = 10_000_000;
pub const HISTORY_SECONDS: u64 = 60;

#[derive(Clone, Debug, PartialEq)]
pub struct HistoryPoint {
    pub sampled_at: u64,
    pub cpu: Option<f64>,
    pub memory: Option<f64>,
    pub logical_cpus: Vec<(ProcessorId, Option<f64>)>,
}

impl HistoryPoint {
    pub fn logical_cpu(&self, id: ProcessorId) -> Option<f64> {
        self.logical_cpus
            .binary_search_by_key(&id, |(id, _)| *id)
            .ok()
            .and_then(|index| self.logical_cpus[index].1)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct History {
    pub latest: Option<SystemObservation>,
    pub points: VecDeque<HistoryPoint>,
}

impl History {
    pub fn logical_ids(&self) -> impl ExactSizeIterator<Item = ProcessorId> + '_ {
        self.points
            .iter()
            .rev()
            .find(|point| !point.logical_cpus.is_empty())
            .map_or(&[][..], |point| point.logical_cpus.as_slice())
            .iter()
            .map(|(id, _)| *id)
    }

    fn push(&mut self, sample: SystemObservation) {
        let mut logical_cpus: Vec<_> = sample
            .logical_cpus
            .iter()
            .flatten()
            .map(|cpu| (cpu.id, cpu.percent))
            .collect();
        logical_cpus.sort_unstable_by_key(|(id, _)| *id);
        let point = HistoryPoint {
            sampled_at: sample.sampled_at,
            cpu: sample.cpu_percent,
            memory: sample.memory.as_ref().map(MemoryObservation::percent),
            logical_cpus,
        };
        // Keep the latest observation in each second, including unavailable values.
        if self
            .points
            .back()
            .is_some_and(|p| p.sampled_at / SECOND == point.sampled_at / SECOND)
        {
            self.points.pop_back();
        }
        while self
            .points
            .front()
            .is_some_and(|p| point.sampled_at - p.sampled_at > HISTORY_SECONDS * SECOND)
        {
            self.points.pop_front();
        }
        self.points.push_back(point);
        self.latest = Some(sample);
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProcessId {
    pub pid: u32,
    pub created: Option<u64>,
    pub observation: u64,
}

#[derive(Clone, PartialEq)]
pub struct Process {
    pub id: ProcessId,
    pub key: Key,
    pub parent_pid: u32,
    pub name: String,
    name_lowercase: String,
    pub threads: u32,
    pub cpu_tenths: Option<u32>,
    pub working_set: Option<u64>,
    pub private_bytes: Option<u64>,
    pub unavailable: Option<String>,
}

#[derive(Clone, PartialEq)]
pub struct Row {
    pub process: Rc<Process>,
    pub depth: usize,
    pub has_children: bool,
    pub expanded: bool,
}

#[derive(Clone, Copy, Default, PartialEq)]
pub enum Sort {
    #[default]
    Name,
    Pid,
    Cpu,
    Memory,
    Private,
}

pub struct Model {
    pub processes: BTreeMap<ProcessId, Rc<Process>>,
    pub visible: Rc<Vec<Row>>,
    pub selected: Option<ProcessId>,
    pub filter: String,
    pub sort: Sort,
    pub descending: bool,
    pub tree: bool,
    pub collapsed: HashSet<ProcessId>,
    pub key_revision: u64,
    pub history: Rc<History>,
    previous: HashMap<ProcessId, (u64, u64)>,
    sequence: u64,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            processes: BTreeMap::new(),
            visible: Rc::default(),
            selected: None,
            filter: String::new(),
            sort: Sort::Name,
            descending: false,
            tree: true,
            collapsed: HashSet::new(),
            key_revision: 0,
            history: Rc::default(),
            previous: HashMap::new(),
            sequence: 0,
        }
    }
}

impl Model {
    pub fn sort_by(&mut self, sort: Sort) {
        self.descending = if self.sort == sort {
            !self.descending
        } else {
            matches!(sort, Sort::Cpu | Sort::Memory | Sort::Private)
        };
        self.sort = sort;
        self.rebuild();
    }

    pub fn apply(&mut self, snapshot: Snapshot) -> Result<(), String> {
        snapshot.validate()?;
        if snapshot.sequence <= self.sequence {
            return Err("Snapshot sequence did not advance".into());
        }
        if let (Some(next), Some(previous)) = (&snapshot.system, &self.history.latest)
            && next.sampled_at <= previous.sampled_at
        {
            return Err("System sample time did not advance".into());
        }
        let mut processes = BTreeMap::new();
        let mut counters = HashMap::new();
        for item in snapshot.processes {
            let id = ProcessId {
                pid: item.pid,
                created: item.created,
                observation: if item.created.is_some() {
                    0
                } else {
                    snapshot.sequence
                },
            };
            let mut cpu_tenths = None;
            if let Some(ticks) = item.cpu_ticks {
                if let Some(&(previous_time, previous_cpu)) = self.previous.get(&id) {
                    let elapsed = item
                        .sampled_at
                        .checked_sub(previous_time)
                        .filter(|v| *v > 0);
                    let delta = ticks.checked_sub(previous_cpu);
                    let (Some(elapsed), Some(delta)) = (elapsed, delta) else {
                        return Err(format!("Invalid counter interval for PID {}", id.pid));
                    };
                    cpu_tenths = Some(
                        (delta as f64 / elapsed as f64 / f64::from(snapshot.processors) * 1000.0)
                            .round() as u32,
                    );
                }
                counters.insert(id, (item.sampled_at, ticks));
            }
            let old = self.processes.get(&id);
            let process = Process {
                id,
                key: old.map_or_else(
                    || Key::from(format!("{}:{:?}:{}", id.pid, id.created, id.observation)),
                    |old| old.key.clone(),
                ),
                parent_pid: item.parent_pid,
                name_lowercase: item.name.to_lowercase(),
                name: item.name,
                threads: item.threads,
                cpu_tenths,
                working_set: item.working_set,
                private_bytes: item.private_bytes,
                unavailable: item.unavailable,
            };
            let process = match old {
                Some(old) if **old == process => Rc::clone(old),
                _ => Rc::new(process),
            };
            processes.insert(id, process);
        }
        if let Some(system) = snapshot.system {
            Rc::make_mut(&mut self.history).push(system);
        } else if self.history.latest.is_some() {
            self.history = Rc::default();
        }
        self.sequence = snapshot.sequence;
        self.processes = processes;
        self.previous = counters;
        self.selected = self.selected.filter(|id| self.processes.contains_key(id));
        self.collapsed.retain(|id| self.processes.contains_key(id));
        self.rebuild();
        Ok(())
    }

    pub fn rebuild(&mut self) {
        let mut ordered: Vec<_> = self.processes.values().cloned().collect();
        ordered.sort_by(|a, b| {
            let order = match self.sort {
                Sort::Name => compare(
                    Some(&a.name_lowercase),
                    Some(&b.name_lowercase),
                    self.descending,
                ),
                Sort::Pid => compare(Some(a.id.pid), Some(b.id.pid), self.descending),
                Sort::Cpu => compare(a.cpu_tenths, b.cpu_tenths, self.descending),
                Sort::Memory => compare(a.working_set, b.working_set, self.descending),
                Sort::Private => compare(a.private_bytes, b.private_bytes, self.descending),
            };
            order.then_with(|| a.id.cmp(&b.id))
        });
        let by_pid: HashMap<_, _> = ordered.iter().map(|p| (p.id.pid, p.id)).collect();
        let parents: HashMap<_, _> = ordered
            .iter()
            .filter_map(|process| {
                let parent = *by_pid.get(&process.parent_pid)?;
                // A reused or unverified parent PID is not evidence of ancestry.
                (self.tree && parent.created? < process.id.created? && parent.pid != process.id.pid)
                    .then_some((process.id, parent))
            })
            .collect();
        let filter = self.filter.to_lowercase();
        let mut included = HashSet::new();
        for process in &ordered {
            if filter.is_empty()
                || process.name_lowercase.contains(&filter)
                || process.id.pid.to_string().contains(&filter)
            {
                let mut id = process.id;
                loop {
                    included.insert(id);
                    let Some(parent) = parents.get(&id) else {
                        break;
                    };
                    id = *parent;
                }
            }
        }
        let mut children: HashMap<Option<ProcessId>, Vec<Rc<Process>>> = HashMap::new();
        for process in ordered {
            if included.contains(&process.id) {
                children
                    .entry(parents.get(&process.id).copied())
                    .or_default()
                    .push(process);
            }
        }
        let mut pending: Vec<_> = children
            .get(&None)
            .into_iter()
            .flatten()
            .rev()
            .map(|p| (Rc::clone(p), 0))
            .collect();
        let mut rows = Vec::new();
        while let Some((process, depth)) = pending.pop() {
            let descendants = children.get(&Some(process.id));
            let expanded = !filter.is_empty() || !self.collapsed.contains(&process.id);
            if expanded && let Some(descendants) = descendants {
                pending.extend(descendants.iter().rev().map(|p| (Rc::clone(p), depth + 1)));
            }
            rows.push(Row {
                process,
                depth,
                has_children: descendants.is_some(),
                expanded,
            });
        }
        if !self
            .visible
            .iter()
            .map(|r| r.process.id)
            .eq(rows.iter().map(|r| r.process.id))
        {
            self.key_revision = self.key_revision.checked_add(1).unwrap();
        }
        self.visible = Rc::new(rows);
    }
}

fn compare<T: Ord>(a: Option<T>, b: Option<T>, descending: bool) -> std::cmp::Ordering {
    use std::cmp::Ordering;
    match (a, b) {
        (Some(a), Some(b)) if descending => b.cmp(&a),
        (Some(a), Some(b)) => a.cmp(&b),
        (None, None) => Ordering::Equal,
        (None, _) => Ordering::Greater,
        (_, None) => Ordering::Less,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snapshot(sequence: u64, created: u64, cpu: u64) -> Snapshot {
        Snapshot {
            version: 1,
            sequence,
            processors: 4,
            processes: vec![Observation {
                pid: 10,
                parent_pid: 0,
                created: Some(created),
                name: "worker.exe".into(),
                threads: 2,
                sampled_at: sequence * 10_000_000,
                cpu_ticks: Some(cpu),
                working_set: Some(4096),
                private_bytes: Some(8192),
                unavailable: None,
            }],
            system: None,
        }
    }

    #[test]
    fn history_retains_one_observation_per_second_for_sixty_seconds() {
        let mut history = History::default();
        let mut sample = crate::source::demo(1).system.unwrap();
        for tick in 1..=1800 {
            sample.sampled_at = tick * SECOND / 10;
            sample.cpu_percent = Some(25.0);
            history.push(sample.clone());
            assert!(history.points.len() <= HISTORY_SECONDS as usize + 1);
            assert!(
                history
                    .points
                    .iter()
                    .map(|point| point.logical_cpus.len())
                    .sum::<usize>()
                    <= 8 * (HISTORY_SECONDS as usize + 1)
            );
        }
        assert!(history.points.front().unwrap().sampled_at >= 120 * SECOND);
        sample.sampled_at += SECOND / 10;
        sample.cpu_percent = None;
        sample.cpu_error = Some("Counter unavailable".into());
        history.push(sample);
        assert_eq!(history.points.back().unwrap().cpu, None);
        assert_eq!(
            history.points.back().unwrap().sampled_at,
            180 * SECOND + SECOND / 10
        );
    }

    #[test]
    fn logical_history_preserves_identity_gaps_and_topology_changes() {
        let first = ProcessorId { node: 0, number: 0 };
        let second = ProcessorId { node: 1, number: 0 };
        let added = ProcessorId {
            node: 2,
            number: 127,
        };
        let mut model = Model::default();
        let mut frame = crate::source::demo(2);
        let cpus = frame
            .system
            .as_mut()
            .unwrap()
            .logical_cpus
            .as_mut()
            .unwrap();
        cpus[0].percent = Some(10.0);
        cpus[4].percent = Some(90.0);
        cpus.reverse();
        model.apply(frame).unwrap();
        let point = model.history.points.back().unwrap();
        assert_eq!(point.logical_cpu(first), Some(10.0));
        assert_eq!(point.logical_cpu(second), Some(90.0));
        assert_eq!(point.logical_cpu(added), None);

        let mut frame = crate::source::demo(3);
        let cpus = frame
            .system
            .as_mut()
            .unwrap()
            .logical_cpus
            .as_mut()
            .unwrap();
        cpus[0].percent = None;
        cpus[0].error = Some("Counter unavailable".into());
        model.apply(frame).unwrap();
        assert_eq!(
            model.history.points.back().unwrap().logical_cpu(first),
            None
        );
        assert!(
            model
                .history
                .points
                .back()
                .unwrap()
                .logical_cpu(second)
                .is_some()
        );

        let mut frame = crate::source::demo(4);
        let system = frame.system.as_mut().unwrap();
        system.logical_cpus = None;
        system.logical_cpu_error = Some("Collection failed".into());
        model.apply(frame).unwrap();
        assert_eq!(model.history.logical_ids().len(), 8);
        assert_eq!(
            model.history.points.back().unwrap().logical_cpu(second),
            None
        );

        let mut frame = crate::source::demo(5);
        frame.processors = 2;
        frame.system.as_mut().unwrap().logical_cpus = Some(vec![
            LogicalCpuObservation {
                id: added,
                percent: None,
                error: None,
            },
            LogicalCpuObservation {
                id: second,
                percent: Some(50.0),
                error: None,
            },
        ]);
        model.apply(frame).unwrap();
        assert_eq!(
            model.history.logical_ids().collect::<Vec<_>>(),
            [second, added]
        );
        assert_eq!(
            model.history.points.front().unwrap().logical_cpu(added),
            None
        );
    }

    #[test]
    fn invalid_logical_samples_leave_history_unchanged() {
        let mut model = Model::default();
        model.apply(crate::source::demo(1)).unwrap();
        let history = Rc::clone(&model.history);
        for case in 0..4 {
            let mut frame = crate::source::demo(2);
            let cpus = frame
                .system
                .as_mut()
                .unwrap()
                .logical_cpus
                .as_mut()
                .unwrap();
            match case {
                0 => cpus[1].id = cpus[0].id,
                1 => {
                    cpus.pop();
                }
                2 => cpus[0].percent = Some(f64::NAN),
                3 => cpus[0].error = Some("Cannot also have a value".into()),
                _ => unreachable!(),
            }
            assert!(model.apply(frame).is_err());
            assert!(Rc::ptr_eq(&history, &model.history));
        }
    }

    #[test]
    fn invalid_system_samples_leave_model_and_history_unchanged() {
        let mut model = Model::default();
        model.apply(crate::source::demo(1)).unwrap();
        let history = Rc::clone(&model.history);
        let rows = Rc::clone(&model.visible);
        let mut sample = crate::source::demo(2);
        sample.system.as_mut().unwrap().sampled_at = SECOND;
        assert!(model.apply(sample).is_err());
        let mut sample = crate::source::demo(2);
        sample.system.as_mut().unwrap().cpu_percent = Some(f64::NAN);
        assert!(model.apply(sample).is_err());
        let mut sample = crate::source::demo(2);
        let memory = sample.system.as_mut().unwrap().memory.as_mut().unwrap();
        memory.available = memory.total + 1;
        assert!(model.apply(sample).is_err());
        assert!(Rc::ptr_eq(&history, &model.history));
        assert!(Rc::ptr_eq(&rows, &model.visible));
        model.apply(crate::source::demo(2)).unwrap();
        assert_eq!(model.history.points.len(), 2);
    }

    #[test]
    fn sorting_changes_direction_and_keeps_unavailable_values_last() {
        let mut sample = snapshot(1, 100, 0);
        for (pid, memory) in [(20, Some(8192)), (30, None)] {
            let mut item = sample.processes[0].clone();
            item.pid = pid;
            item.name = format!("worker-{pid}.exe");
            item.working_set = memory;
            sample.processes.push(item);
        }
        let mut model = Model::default();
        model.apply(sample).unwrap();
        let selected = model.visible[0].process.id;
        model.selected = Some(selected);
        let ids = |model: &Model| {
            model
                .visible
                .iter()
                .map(|r| r.process.id.pid)
                .collect::<Vec<_>>()
        };
        model.sort_by(Sort::Memory);
        assert_eq!(ids(&model), [20, 10, 30]);
        let revision = model.key_revision;
        model.sort_by(Sort::Memory);
        assert_eq!(ids(&model), [10, 20, 30]);
        assert!(model.key_revision > revision);
        assert_eq!(model.selected, Some(selected));
        model.sort_by(Sort::Pid);
        assert_eq!(ids(&model), [10, 20, 30]);
        model.sort_by(Sort::Pid);
        assert_eq!(ids(&model), [30, 20, 10]);
    }

    #[test]
    fn cpu_and_selection_follow_process_instances() {
        let mut model = Model::default();
        model.apply(snapshot(1, 100, 0)).unwrap();
        let id = model.visible[0].process.id;
        model.selected = Some(id);
        assert_eq!(model.visible[0].process.cpu_tenths, None);
        let revision = model.key_revision;
        model.apply(snapshot(2, 100, 10_000_000)).unwrap();
        assert_eq!(model.visible[0].process.cpu_tenths, Some(250));
        assert_eq!(model.selected, Some(id));
        assert_eq!(model.key_revision, revision);
        model.apply(snapshot(3, 200, 1)).unwrap();
        assert_eq!(model.selected, None);
        assert_eq!(model.visible[0].process.cpu_tenths, None);
        assert_ne!(model.visible[0].process.id, id);
    }

    #[test]
    fn tree_filter_retains_ancestors_but_not_reused_parents() {
        let mut sample = snapshot(1, 100, 0);
        sample.processes[0].parent_pid = 20;
        let mut parent = sample.processes[0].clone();
        parent.pid = 20;
        parent.parent_pid = 0;
        parent.created = Some(50);
        parent.name = "parent.exe".into();
        sample.processes.push(parent);
        let mut model = Model {
            filter: "worker".into(),
            ..Default::default()
        };
        model.apply(sample.clone()).unwrap();
        assert_eq!(
            model.visible.iter().map(|r| r.depth).collect::<Vec<_>>(),
            [0, 1]
        );
        sample.sequence = 2;
        sample.processes[0].sampled_at += 10_000_000;
        sample.processes[1].created = Some(150);
        model.apply(sample).unwrap();
        assert_eq!(model.visible.len(), 1);
        assert_eq!(model.visible[0].depth, 0);
    }

    #[test]
    fn inaccessible_identity_does_not_inherit_selection() {
        let mut sample = snapshot(1, 100, 0);
        sample.processes[0].created = None;
        sample.processes[0].cpu_ticks = None;
        sample.processes[0].unavailable = Some("Access denied".into());
        let mut model = Model::default();
        model.apply(sample.clone()).unwrap();
        model.selected = Some(model.visible[0].process.id);
        sample.sequence = 2;
        model.apply(sample).unwrap();
        assert_eq!(model.selected, None);
    }

    #[test]
    fn invalid_snapshot_does_not_replace_displayed_state() {
        let mut model = Model::default();
        model.apply(snapshot(1, 100, 100)).unwrap();
        let before = Rc::clone(&model.visible);
        assert!(model.apply(snapshot(2, 100, 99)).is_err());
        assert!(Rc::ptr_eq(&before, &model.visible));
        let mut sample = snapshot(2, 100, 200);
        sample.processes.push(sample.processes[0].clone());
        assert!(model.apply(sample).is_err());
    }
}
