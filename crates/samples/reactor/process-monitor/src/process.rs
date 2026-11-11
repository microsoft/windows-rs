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
        let mut capture_time = FILETIME::default();
        unsafe { GetSystemTimePreciseAsFileTime(&mut capture_time) };
        let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS as u32, 0) };
        if snapshot == INVALID_HANDLE_VALUE {
            return Err(io::Error::last_os_error());
        }
        let snapshot = unsafe { OwnedHandle::from_raw_handle(snapshot) };
        let processors = unsafe { GetActiveProcessorCount(ALL_PROCESSOR_GROUPS as u16) };
        if processors == 0 {
            return Err(io::Error::last_os_error());
        }
        self.sequence = self.sequence.checked_add(1).unwrap();
        let mut result = Snapshot {
            version: 3,
            sequence: self.sequence,
            processors,
            processes: Vec::new(),
            system: Some(self.system(processors)),
        };
        let mut entry = PROCESSENTRY32W {
            dwSize: size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };
        let mut found = unsafe { Process32FirstW(snapshot.as_raw_handle(), &mut entry) };
        while found.as_bool() {
            if cancel.is_cancelled() {
                return Err(io::Error::new(
                    io::ErrorKind::Interrupted,
                    "Sampling cancelled",
                ));
            }
            let length = entry
                .szExeFile
                .iter()
                .position(|v| *v == 0)
                .unwrap_or(entry.szExeFile.len());
            let mut process = Observation {
                pid: entry.th32ProcessID,
                parent_pid: entry.th32ParentProcessID,
                created: None,
                name: String::from_utf16_lossy(&entry.szExeFile[..length]),
                threads: entry.cntThreads,
                sampled_at: 0,
                cpu_ticks: None,
                working_set: None,
                private_bytes: None,
                unavailable: None,
            };
            if let Err(error) = self.read_counters(&mut process, ticks(capture_time)) {
                process.unavailable = Some(error.to_string());
            }
            result.processes.push(process);
            found = unsafe { Process32NextW(snapshot.as_raw_handle(), &mut entry) };
        }
        let error = io::Error::last_os_error();
        if error.raw_os_error() != Some(ERROR_NO_MORE_FILES) {
            return Err(error);
        }
        Ok(result)
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
            sampled_at: self
                .started
                .elapsed()
                .as_nanos()
                .div_euclid(100)
                .try_into()
                .unwrap(),
            cpu_percent,
            cpu_error,
            logical_cpus,
            logical_cpu_error,
            memory,
            memory_error,
        }
    }

    fn read_counters(&self, process: &mut Observation, capture_time: u64) -> io::Result<()> {
        let handle = open(process.pid)?;
        let (created, cpu) = times(&handle)?;
        if created > capture_time {
            return Err(io::Error::other("Process changed during enumeration"));
        }
        process.created = Some(created);
        process.cpu_ticks = Some(cpu);
        process.sampled_at = self
            .started
            .elapsed()
            .as_nanos()
            .div_euclid(100)
            .try_into()
            .unwrap();
        let mut memory = PROCESS_MEMORY_COUNTERS_EX {
            cb: size_of::<PROCESS_MEMORY_COUNTERS_EX>() as u32,
            ..Default::default()
        };
        if !unsafe {
            GetProcessMemoryInfo(
                handle.as_raw_handle(),
                (&mut memory as *mut PROCESS_MEMORY_COUNTERS_EX).cast(),
                memory.cb,
            )
        }
        .as_bool()
        {
            return Err(io::Error::last_os_error());
        }
        process.working_set = Some(memory.WorkingSetSize as u64);
        process.private_bytes = Some(memory.PrivateUsage as u64);
        Ok(())
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
