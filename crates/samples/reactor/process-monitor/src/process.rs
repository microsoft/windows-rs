use crate::bindings::*;
use crate::model::{
    LogicalCpuObservation, MemoryObservation, Observation, ProcessId, ProcessorId, Snapshot,
    SystemObservation,
};
use std::io;
use std::os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle};
use std::time::{Duration, Instant};
use windows_reactor::CancellationToken;

pub struct Sampler {
    started: Instant,
    sequence: u64,
    cpu: Result<CpuSampler, String>,
}

impl Sampler {
    pub fn new() -> Self {
        Self {
            started: Instant::now(),
            sequence: 0,
            cpu: CpuSampler::new(),
        }
    }

    pub fn snapshot(&mut self, cancel: &CancellationToken) -> io::Result<Snapshot> {
        self.sample(|| cancel.is_cancelled())
    }

    fn sample(&mut self, is_cancelled: impl Fn() -> bool) -> io::Result<Snapshot> {
        let processors = unsafe { GetActiveProcessorCount(ALL_PROCESSOR_GROUPS as u16) };
        if processors == 0 {
            return Err(io::Error::last_os_error());
        }
        let buffer = system_processes()?;
        let sampled_at = self.elapsed();
        self.sequence = self.sequence.checked_add(1).unwrap();
        let mut result = Snapshot {
            version: 3,
            sequence: self.sequence,
            processors,
            processes: Vec::new(),
            system: Some(self.system(processors)),
        };
        let bytes = buffer.as_ptr().cast::<u8>();
        let mut offset = 0;
        loop {
            if is_cancelled() {
                return Err(io::Error::new(
                    io::ErrorKind::Interrupted,
                    "Sampling cancelled",
                ));
            }
            // Entries are aligned for SYSTEM_PROCESS_INFORMATION within the u64-aligned buffer.
            let entry = unsafe { &*bytes.add(offset).cast::<SYSTEM_PROCESS_INFORMATION>() };
            let times = ProcessTimes::from(entry);
            let name = if entry.ImageName.Buffer.is_null() {
                "System Idle Process".into()
            } else {
                String::from_utf16_lossy(unsafe {
                    std::slice::from_raw_parts(
                        entry.ImageName.Buffer.0,
                        usize::from(entry.ImageName.Length) / 2,
                    )
                })
            };
            result.processes.push(Observation {
                pid: entry.UniqueProcessId as usize as u32,
                parent_pid: entry.InheritedFromUniqueProcessId as usize as u32,
                created: Some(times.created),
                name,
                threads: entry.NumberOfThreads,
                sampled_at,
                cpu_ticks: Some(times.cpu),
                working_set: Some(entry.WorkingSetSize as u64),
                private_bytes: Some(entry.PrivatePageCount as u64),
                unavailable: None,
            });
            if entry.NextEntryOffset == 0 {
                break;
            }
            offset += entry.NextEntryOffset as usize;
        }
        Ok(result)
    }

    fn elapsed(&self) -> u64 {
        self.started
            .elapsed()
            .as_nanos()
            .div_euclid(100)
            .try_into()
            .unwrap()
    }

    fn system(&mut self, processors: u32) -> SystemObservation {
        let cpu = match &mut self.cpu {
            Ok(cpu) => cpu.sample(),
            Err(error) => CpuSample::failed(error.clone()),
        };
        let mut memory = MEMORYSTATUSEX {
            dwLength: size_of::<MEMORYSTATUSEX>() as u32,
            ..Default::default()
        };
        let memory = if unsafe { GlobalMemoryStatusEx(&mut memory) }.as_bool() {
            Ok(MemoryObservation {
                total: memory.ullTotalPhys,
                available: memory.ullAvailPhys,
            })
        } else {
            Err(io::Error::last_os_error().to_string())
        };
        let (cpu_percent, cpu_error) = match cpu.total {
            Ok(value) => (value, None),
            Err(error) => (None, Some(error)),
        };
        let (logical_cpus, logical_cpu_error) = match cpu.logical {
            Ok(Some(values)) if values.len() != processors as usize => (
                None,
                Some(format!(
                    "Expected {processors} logical processors, received {}",
                    values.len()
                )),
            ),
            Ok(values) => (values, None),
            Err(error) => (None, Some(error)),
        };
        let (memory, memory_error) = match memory {
            Ok(value) => (Some(value), None),
            Err(error) => (None, Some(error)),
        };
        SystemObservation {
            sampled_at: self.elapsed(),
            cpu_percent,
            cpu_error,
            logical_cpus,
            logical_cpu_error,
            memory,
            memory_error,
        }
    }
}

/// Returns the `SystemProcessInformation` entries in a u64-aligned buffer.
fn system_processes() -> io::Result<Vec<u64>> {
    let mut buffer = Vec::<u64>::new();
    loop {
        let capacity = u32::try_from(buffer.len() * size_of::<u64>()).unwrap();
        let mut length = 0;
        let status = unsafe {
            NtQuerySystemInformation(
                SystemProcessInformation,
                buffer.as_mut_ptr().cast(),
                capacity,
                &mut length,
            )
        };
        if status == STATUS_INFO_LENGTH_MISMATCH {
            // Processes can start between the size query and the next attempt.
            buffer = vec![0; (length as usize + 16 * 1024).div_ceil(size_of::<u64>())];
            continue;
        }
        status.ok().map_err(io::Error::other)?;
        return Ok(buffer);
    }
}

/// Process times from `SYSTEM_PROCESS_INFORMATION::Reserved1`, which the SDK leaves opaque. The
/// native layout is `WorkingSetPrivateSize`, `HardFaultCount`, `NumberOfThreadsHighWatermark`,
/// `CycleTime`, `CreateTime`, `UserTime`, and `KernelTime`.
struct ProcessTimes {
    created: u64,
    cpu: u64,
}

impl From<&SYSTEM_PROCESS_INFORMATION> for ProcessTimes {
    fn from(entry: &SYSTEM_PROCESS_INFORMATION) -> Self {
        let field = |offset: usize| {
            u64::from_le_bytes(entry.Reserved1[offset..offset + 8].try_into().unwrap())
        };
        Self {
            created: field(24),
            cpu: field(32).checked_add(field(40)).unwrap(),
        }
    }
}

struct CpuSampler {
    query: PDH_HQUERY,
    counter: PDH_HCOUNTER,
    logical_counter: Result<PDH_HCOUNTER, String>,
    last_collect: Option<Instant>,
    current: CpuSample,
}

#[derive(Clone)]
struct CpuSample {
    total: Result<Option<f64>, String>,
    logical: Result<Option<Vec<LogicalCpuObservation>>, String>,
}

impl CpuSample {
    fn pending() -> Self {
        Self {
            total: Ok(None),
            logical: Ok(None),
        }
    }

    fn failed(error: String) -> Self {
        Self {
            total: Err(error.clone()),
            logical: Err(error),
        }
    }
}

// PDH handles are not thread-affine. Only the owning sampler accesses this query, through &mut self.
unsafe impl Send for CpuSampler {}

impl CpuSampler {
    fn new() -> Result<Self, String> {
        let mut query = std::ptr::null_mut();
        pdh_status("PdhOpenQuery", unsafe {
            PdhOpenQueryW(windows_core::PCWSTR::null(), 0, &mut query)
        })?;
        let mut sampler = Self {
            query,
            counter: std::ptr::null_mut(),
            logical_counter: Ok(std::ptr::null_mut()),
            last_collect: None,
            current: CpuSample::pending(),
        };
        pdh_status("PdhAddEnglishCounter", unsafe {
            PdhAddEnglishCounterW(
                query,
                windows_core::w!("\\Processor Information(_Total)\\% Processor Time"),
                0,
                &mut sampler.counter,
            )
        })?;
        let mut logical = std::ptr::null_mut();
        sampler.logical_counter = pdh_status("PdhAddEnglishCounter (logical processors)", unsafe {
            PdhAddEnglishCounterW(
                query,
                windows_core::w!("\\Processor Information(*)\\% Processor Time"),
                0,
                &mut logical,
            )
        })
        .map(|()| logical);
        Ok(sampler)
    }

    fn sample(&mut self) -> CpuSample {
        // PDH rate counters need two samples at least a second apart.
        if self
            .last_collect
            .is_none_or(|last| last.elapsed() >= Duration::from_secs(1))
        {
            let first = self.last_collect.is_none();
            self.current = self.collect(first);
            self.last_collect = Some(Instant::now());
        }
        self.current.clone()
    }

    fn collect(&self, first: bool) -> CpuSample {
        if let Err(error) = pdh_status("PdhCollectQueryData", unsafe {
            PdhCollectQueryData(self.query)
        }) {
            return CpuSample::failed(error);
        }
        if first {
            return CpuSample::pending();
        }
        CpuSample {
            total: self.read_total().map(Some),
            logical: self.read_logical().map(Some),
        }
    }

    fn read_total(&self) -> Result<f64, String> {
        let mut value = PDH_FMT_COUNTERVALUE::default();
        pdh_status("PdhGetFormattedCounterValue", unsafe {
            PdhGetFormattedCounterValue(
                self.counter,
                PDH_FMT_DOUBLE,
                std::ptr::null_mut(),
                &mut value,
            )
        })?;
        counter_percent(&value)
    }

    fn read_logical(&self) -> Result<Vec<LogicalCpuObservation>, String> {
        let counter = *self.logical_counter.as_ref().map_err(Clone::clone)?;
        let mut bytes = 0;
        let mut count = 0;
        let status = unsafe {
            PdhGetFormattedCounterArrayW(
                counter,
                PDH_FMT_DOUBLE,
                &mut bytes,
                &mut count,
                std::ptr::null_mut(),
            )
        };
        if status as u32 != PDH_MORE_DATA {
            pdh_status("PdhGetFormattedCounterArray size", status)?;
            return Err("No logical CPU counters were returned".into());
        }
        // The API writes aligned items followed by their UTF-16 names into the same allocation.
        let mut buffer = vec![
            PDH_FMT_COUNTERVALUE_ITEM_W::default();
            (bytes as usize).div_ceil(size_of::<PDH_FMT_COUNTERVALUE_ITEM_W>())
        ];
        pdh_status("PdhGetFormattedCounterArray", unsafe {
            PdhGetFormattedCounterArrayW(
                counter,
                PDH_FMT_DOUBLE,
                &mut bytes,
                &mut count,
                buffer.as_mut_ptr(),
            )
        })?;
        let mut values = Vec::new();
        for item in &buffer[..count as usize] {
            let name = unsafe { item.szName.to_string() }.map_err(|error| error.to_string())?;
            if let Some(id) = processor_id(&name)? {
                let (percent, error) = match counter_percent(&item.FmtValue) {
                    Ok(value) => (Some(value), None),
                    Err(error) => (None, Some(error)),
                };
                values.push(LogicalCpuObservation { id, percent, error });
            }
        }
        values.sort_unstable_by_key(|cpu| cpu.id);
        if values.is_empty() || values.windows(2).any(|pair| pair[0].id == pair[1].id) {
            return Err("Missing or duplicate logical CPU counter identities".into());
        }
        Ok(values)
    }
}

fn processor_id(name: &str) -> Result<Option<ProcessorId>, String> {
    if name == "_Total" {
        return Ok(None);
    }
    let invalid = || format!("Invalid logical CPU instance: {name}");
    // PDH numbers processors within NUMA nodes, not Windows processor-affinity groups.
    let (node, number) = name.split_once(',').ok_or_else(invalid)?;
    let node: u32 = node.parse().map_err(|_| invalid())?;
    if number == "_Total" {
        return Ok(None);
    }
    let number: u32 = number.parse().map_err(|_| invalid())?;
    Ok(Some(ProcessorId { node, number }))
}

fn counter_percent(value: &PDH_FMT_COUNTERVALUE) -> Result<f64, String> {
    if value.CStatus != PDH_CSTATUS_VALID_DATA as u32
        && value.CStatus != PDH_CSTATUS_NEW_DATA as u32
    {
        return Err(format!("CPU counter status: {:#010x}", value.CStatus));
    }
    let percent = unsafe { value.Anonymous.doubleValue };
    if !percent.is_finite() || !(0.0..=100.0).contains(&percent) {
        return Err(format!("Invalid CPU counter value: {percent}"));
    }
    Ok(percent)
}

impl Drop for CpuSampler {
    fn drop(&mut self) {
        assert_eq!(unsafe { PdhCloseQuery(self.query) }, 0);
    }
}

fn pdh_status(operation: &str, status: i32) -> Result<(), String> {
    if status == 0 {
        Ok(())
    } else {
        Err(format!("{operation} failed: {status:#010x}"))
    }
}

fn open(pid: u32) -> io::Result<OwnedHandle> {
    let handle =
        unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION as u32, false.into(), pid) };
    if handle.is_null() {
        Err(io::Error::last_os_error())
    } else {
        Ok(unsafe { OwnedHandle::from_raw_handle(handle) })
    }
}

fn ticks(time: FILETIME) -> u64 {
    (u64::from(time.dwHighDateTime) << 32) | u64::from(time.dwLowDateTime)
}

fn times(handle: &OwnedHandle) -> io::Result<(u64, u64)> {
    let mut created = FILETIME::default();
    let mut exited = FILETIME::default();
    let mut kernel = FILETIME::default();
    let mut user = FILETIME::default();
    if !unsafe {
        GetProcessTimes(
            handle.as_raw_handle(),
            &mut created,
            &mut exited,
            &mut kernel,
            &mut user,
        )
    }
    .as_bool()
    {
        return Err(io::Error::last_os_error());
    }
    if ticks(exited) != 0 {
        return Err(io::Error::other("Process exited"));
    }
    Ok((
        ticks(created),
        ticks(kernel).checked_add(ticks(user)).unwrap(),
    ))
}

pub fn image_path(id: ProcessId) -> Result<String, String> {
    let created = id.created.ok_or("Process identity is unavailable")?;
    let handle = open(id.pid).map_err(|error| error.to_string())?;
    if times(&handle).map_err(|error| error.to_string())?.0 != created {
        return Err("Process exited or its PID was reused".into());
    }
    let mut buffer = vec![0u16; 32_768];
    let mut length = buffer.len() as u32;
    if !unsafe {
        QueryFullProcessImageNameW(
            handle.as_raw_handle(),
            0,
            windows_core::PWSTR(buffer.as_mut_ptr()),
            &mut length,
        )
    }
    .as_bool()
    {
        return Err(io::Error::last_os_error().to_string());
    }
    Ok(String::from_utf16_lossy(&buffer[..length as usize]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sampling_preserves_unopenable_process_identity() {
        assert!(open(0).is_err());
        let mut sampler = Sampler::new();
        let mut model = crate::model::Model::default();
        model.apply(sampler.sample(|| false).unwrap()).unwrap();
        let idle = *model.processes.keys().find(|id| id.pid == 0).unwrap();
        assert!(idle.created.is_some());
        model.selected = Some(idle);
        model.apply(sampler.sample(|| false).unwrap()).unwrap();
        assert_eq!(model.selected, Some(idle));
        assert!(model.processes.contains_key(&idle));
    }

    #[test]
    fn cancelled_sampling_does_not_return_a_partial_snapshot() {
        let mut sampler = Sampler::new();
        let calls = std::cell::Cell::new(0);
        let error = sampler
            .sample(|| {
                calls.set(calls.get() + 1);
                calls.get() == 2
            })
            .unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::Interrupted);
    }

    #[test]
    fn system_processes_match_process_times() {
        let pid = std::process::id();
        let handle = open(pid).unwrap();
        let (created, before) = times(&handle).unwrap();
        let buffer = system_processes().unwrap();
        let (_, after) = times(&handle).unwrap();
        let bytes = buffer.as_ptr().cast::<u8>();
        let mut offset = 0;
        let current = loop {
            let entry = unsafe { &*bytes.add(offset).cast::<SYSTEM_PROCESS_INFORMATION>() };
            if entry.UniqueProcessId as usize as u32 == pid {
                break entry;
            }
            assert_ne!(entry.NextEntryOffset, 0);
            offset += entry.NextEntryOffset as usize;
        };
        let times = ProcessTimes::from(current);
        assert_eq!(times.created, created);
        assert!((before..=after).contains(&times.cpu));
    }

    #[test]
    fn logical_counter_names_preserve_nodes_and_exclude_totals() {
        assert_eq!(processor_id("_Total").unwrap(), None);
        assert_eq!(processor_id("2,_Total").unwrap(), None);
        assert_eq!(
            processor_id("2,127").unwrap(),
            Some(ProcessorId {
                node: 2,
                number: 127
            })
        );
        for invalid in ["0", "0,-1", "-1,0", "0,0,1", "other", "0,4294967296"] {
            assert!(processor_id(invalid).is_err());
        }
    }

    #[test]
    fn logical_counters_cover_active_processors() {
        let mut sampler = CpuSampler::new().unwrap();
        assert!(sampler.sample().logical.unwrap().is_none());
        std::thread::sleep(Duration::from_millis(1100));
        let cpus = sampler.sample().logical.unwrap().unwrap();
        assert_eq!(
            cpus.len(),
            unsafe { GetActiveProcessorCount(ALL_PROCESSOR_GROUPS as u16) } as usize
        );
        assert!(cpus.windows(2).all(|pair| pair[0].id < pair[1].id));
        assert!(cpus.iter().any(|cpu| cpu.percent.is_some()));
    }

    #[test]
    fn details_verify_creation_time_before_querying_path() {
        let pid = std::process::id();
        let created = times(&open(pid).unwrap()).unwrap().0;
        let id = ProcessId {
            pid,
            created: Some(created),
            observation: 0,
        };
        assert!(!image_path(id).unwrap().is_empty());
        assert!(
            image_path(ProcessId {
                created: Some(created + 1),
                ..id
            })
            .is_err()
        );
    }
}
