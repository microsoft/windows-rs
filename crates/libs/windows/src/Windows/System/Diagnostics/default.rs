impl ::core::cmp::PartialEq for DiagnosticActionResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DiagnosticActionResult {}
impl ::core::fmt::Debug for DiagnosticActionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DiagnosticActionResult").field(&self.0).finish()
    }
}
impl ::core::default::Default for DiagnosticActionState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DiagnosticActionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DiagnosticActionState").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for DiagnosticInvoker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DiagnosticInvoker {}
impl ::core::fmt::Debug for DiagnosticInvoker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DiagnosticInvoker").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ProcessCpuUsage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProcessCpuUsage {}
impl ::core::fmt::Debug for ProcessCpuUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessCpuUsage").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ProcessCpuUsageReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProcessCpuUsageReport {}
impl ::core::fmt::Debug for ProcessCpuUsageReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessCpuUsageReport").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ProcessDiagnosticInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProcessDiagnosticInfo {}
impl ::core::fmt::Debug for ProcessDiagnosticInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessDiagnosticInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ProcessDiskUsage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProcessDiskUsage {}
impl ::core::fmt::Debug for ProcessDiskUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessDiskUsage").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ProcessDiskUsageReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProcessDiskUsageReport {}
impl ::core::fmt::Debug for ProcessDiskUsageReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessDiskUsageReport").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ProcessMemoryUsage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProcessMemoryUsage {}
impl ::core::fmt::Debug for ProcessMemoryUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessMemoryUsage").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ProcessMemoryUsageReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProcessMemoryUsageReport {}
impl ::core::fmt::Debug for ProcessMemoryUsageReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProcessMemoryUsageReport").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for SystemCpuUsage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemCpuUsage {}
impl ::core::fmt::Debug for SystemCpuUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemCpuUsage").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for SystemCpuUsageReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemCpuUsageReport {}
impl ::core::fmt::Debug for SystemCpuUsageReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemCpuUsageReport").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for SystemDiagnosticInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemDiagnosticInfo {}
impl ::core::fmt::Debug for SystemDiagnosticInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemDiagnosticInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for SystemMemoryUsage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemMemoryUsage {}
impl ::core::fmt::Debug for SystemMemoryUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemMemoryUsage").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for SystemMemoryUsageReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemMemoryUsageReport {}
impl ::core::fmt::Debug for SystemMemoryUsageReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemMemoryUsageReport").field(&self.0).finish()
    }
}
