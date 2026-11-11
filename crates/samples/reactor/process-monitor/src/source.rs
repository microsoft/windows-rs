use crate::Options;
use crate::model::{
    LogicalCpuObservation, MemoryObservation, Observation, ProcessorId, Snapshot, SystemObservation,
};
use crate::process::Sampler;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use windows_reactor::CancellationToken;

enum Input {
    Live(Sampler),
    Replay(BufReader<File>),
    Demo(u64),
}

pub struct Source {
    input: Input,
    recording: Option<BufWriter<File>>,
}

impl Source {
    pub fn open(options: &Options) -> Result<Self, String> {
        let input = if let Some(path) = &options.replay {
            Input::Replay(BufReader::new(File::open(path).map_err(|e| e.to_string())?))
        } else if options.demo {
            Input::Demo(0)
        } else {
            Input::Live(Sampler::new())
        };
        let recording = options
            .record
            .as_ref()
            .map(|path| {
                OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(path)
                    .map(BufWriter::new)
            })
            .transpose()
            .map_err(|e| e.to_string())?;
        Ok(Self { input, recording })
    }

    pub fn next(&mut self, cancel: &CancellationToken) -> Result<Option<Snapshot>, String> {
        let snapshot = match &mut self.input {
            Input::Live(sampler) => Some(sampler.snapshot(cancel).map_err(|e| e.to_string())?),
            Input::Replay(reader) => read_snapshot(reader)?,
            Input::Demo(frame) if *frame < 60 => {
                *frame += 1;
                Some(demo(*frame))
            }
            Input::Demo(_) => None,
        };
        if let Some(snapshot) = &snapshot {
            snapshot.validate()?;
            if let Some(recording) = &mut self.recording {
                serde_json::to_writer(&mut *recording, snapshot).map_err(|e| e.to_string())?;
                recording.write_all(b"\n").map_err(|e| e.to_string())?;
                recording.flush().map_err(|e| e.to_string())?;
            }
        }
        Ok(snapshot)
    }
}

fn read_snapshot(reader: &mut impl BufRead) -> Result<Option<Snapshot>, String> {
    let mut line = String::new();
    if reader.read_line(&mut line).map_err(|e| e.to_string())? == 0 {
        return Ok(None);
    }
    let snapshot: Snapshot = serde_json::from_str(&line).map_err(|e| e.to_string())?;
    snapshot.validate()?;
    Ok(Some(snapshot))
}

pub fn demo(sequence: u64) -> Snapshot {
    let processes = (0..256u32)
        .filter(|pid| *pid < 240 || (sequence / 5 + u64::from(*pid)) % 3 != 0)
        .map(|pid| {
            let generation = if pid >= 240 { sequence / 10 } else { 0 };
            Observation {
                pid: pid + 100,
                parent_pid: if pid == 0 { 0 } else { 100 + (pid - 1) / 4 },
                created: Some(1000 + u64::from(pid) + generation * 10_000),
                name: if pid == 0 {
                    "demo-system.exe".into()
                } else {
                    format!("worker-{pid:03}.exe")
                },
                threads: 1 + pid % 8,
                sampled_at: sequence * 10_000_000,
                cpu_ticks: Some(sequence * (u64::from(pid % 8) + 1) * 100_000),
                working_set: Some((20 + (u64::from(pid) * 7 + sequence) % 400) * 1_048_576),
                private_bytes: Some((30 + u64::from(pid) * 3) * 1_048_576),
                unavailable: None,
            }
        })
        .collect();
    Snapshot {
        version: 3,
        sequence,
        processors: 8,
        processes,
        system: Some(SystemObservation {
            sampled_at: sequence * crate::model::SECOND,
            cpu_percent: (sequence > 1)
                .then(|| 15.0 + ((sequence as f64 / 4.0).sin() + 1.0) * 25.0),
            cpu_error: None,
            logical_cpus: Some(
                (0..8)
                    .map(|index| LogicalCpuObservation {
                        id: ProcessorId {
                            node: index / 4,
                            number: index % 4,
                        },
                        percent: (sequence > 1).then(|| {
                            10.0 + ((sequence as f64 / 4.0 + f64::from(index)).sin() + 1.0) * 40.0
                        }),
                        error: None,
                    })
                    .collect(),
            ),
            logical_cpu_error: None,
            memory: Some(MemoryObservation {
                total: 32 * 1024 * 1024 * 1024,
                available: (18 * 1024 - sequence * 12) * 1024 * 1024,
            }),
            memory_error: None,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Model;

    #[test]
    fn replay_round_trip_preserves_data_and_end_of_stream() {
        let mut bytes = Vec::new();
        for sequence in 1..=15 {
            serde_json::to_writer(&mut bytes, &demo(sequence)).unwrap();
            bytes.push(b'\n');
        }
        let mut reader = &bytes[..];
        let mut replayed = Model::default();
        let mut direct = Model::default();
        for sequence in 1..=15 {
            let frame = read_snapshot(&mut reader).unwrap().unwrap();
            assert_eq!(frame.sequence, sequence);
            replayed.apply(frame).unwrap();
            direct.apply(demo(sequence)).unwrap();
            assert_eq!(
                replayed
                    .visible
                    .iter()
                    .map(|r| r.process.id)
                    .collect::<Vec<_>>(),
                direct
                    .visible
                    .iter()
                    .map(|r| r.process.id)
                    .collect::<Vec<_>>(),
            );
            assert_eq!(replayed.key_revision, direct.key_revision);
            assert_eq!(replayed.history, direct.history);
        }
        assert!(read_snapshot(&mut reader).unwrap().is_none());
    }

    #[test]
    fn legacy_recordings_have_no_invented_system_metrics() {
        let mut value = serde_json::to_value(demo(1)).unwrap();
        value["version"] = 1.into();
        value.as_object_mut().unwrap().remove("system");
        let bytes = serde_json::to_vec(&value).unwrap();
        let mut model = Model::default();
        model
            .apply(read_snapshot(&mut &bytes[..]).unwrap().unwrap())
            .unwrap();
        assert!(!model.visible.is_empty());
        assert!(model.history.latest.is_none());
        assert!(model.history.points.is_empty());
    }

    #[test]
    fn version_two_replays_without_invented_logical_data() {
        let mut value = serde_json::to_value(demo(2)).unwrap();
        value["version"] = 2.into();
        value["system"]
            .as_object_mut()
            .unwrap()
            .remove("logical_cpus");
        let bytes = serde_json::to_vec(&value).unwrap();
        let frame = read_snapshot(&mut &bytes[..]).unwrap().unwrap();
        let encoded = serde_json::to_value(&frame).unwrap();
        assert!(encoded["system"].get("logical_cpus").is_none());
        assert!(encoded["system"].get("logical_cpu_error").is_none());
        let mut model = Model::default();
        model.apply(frame).unwrap();
        assert!(model.history.points.back().unwrap().cpu.is_some());
        assert_eq!(model.history.logical_ids().len(), 0);
    }

    #[test]
    fn malformed_replay_is_not_an_empty_success() {
        assert!(read_snapshot(&mut &b"{broken\n"[..]).is_err());
        let mut sample = demo(1);
        sample.processors = 0;
        let bytes = serde_json::to_vec(&sample).unwrap();
        assert!(read_snapshot(&mut &bytes[..]).is_err());
    }
}
