#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for ADDRESS_LIST_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for ADDRESS_LIST_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.MediaType == other.MediaType && self.AddressList == other.AddressList
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for ADDRESS_LIST_DESCRIPTOR {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for ADDRESS_LIST_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADDRESS_LIST_DESCRIPTOR").field("MediaType", &self.MediaType).field("AddressList", &self.AddressList).finish()
    }
}
impl ::core::default::Default for ADSPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ADSPEC {
    fn eq(&self, other: &Self) -> bool {
        self.adspec_header == other.adspec_header && self.adspec_body == other.adspec_body
    }
}
impl ::core::cmp::Eq for ADSPEC {}
impl ::core::fmt::Debug for ADSPEC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADSPEC").field("adspec_header", &self.adspec_header).field("adspec_body", &self.adspec_body).finish()
    }
}
impl ::core::default::Default for AD_GENERAL_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AD_GENERAL_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.IntServAwareHopCount == other.IntServAwareHopCount && self.PathBandwidthEstimate == other.PathBandwidthEstimate && self.MinimumLatency == other.MinimumLatency && self.PathMTU == other.PathMTU && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for AD_GENERAL_PARAMS {}
impl ::core::fmt::Debug for AD_GENERAL_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AD_GENERAL_PARAMS").field("IntServAwareHopCount", &self.IntServAwareHopCount).field("PathBandwidthEstimate", &self.PathBandwidthEstimate).field("MinimumLatency", &self.MinimumLatency).field("PathMTU", &self.PathMTU).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for AD_GUARANTEED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AD_GUARANTEED {
    fn eq(&self, other: &Self) -> bool {
        self.CTotal == other.CTotal && self.DTotal == other.DTotal && self.CSum == other.CSum && self.DSum == other.DSum
    }
}
impl ::core::cmp::Eq for AD_GUARANTEED {}
impl ::core::fmt::Debug for AD_GUARANTEED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AD_GUARANTEED").field("CTotal", &self.CTotal).field("DTotal", &self.DTotal).field("CSum", &self.CSum).field("DSum", &self.DSum).finish()
    }
}
impl ::core::default::Default for CONTROL_SERVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CtrlLoadFlowspec {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CtrlLoadFlowspec {
    fn eq(&self, other: &Self) -> bool {
        self.CL_spec_serv_hdr == other.CL_spec_serv_hdr && self.CL_spec_parm_hdr == other.CL_spec_parm_hdr && self.CL_spec_parms == other.CL_spec_parms
    }
}
impl ::core::cmp::Eq for CtrlLoadFlowspec {}
impl ::core::fmt::Debug for CtrlLoadFlowspec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CtrlLoadFlowspec").field("CL_spec_serv_hdr", &self.CL_spec_serv_hdr).field("CL_spec_parm_hdr", &self.CL_spec_parm_hdr).field("CL_spec_parms", &self.CL_spec_parms).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for ENUMERATION_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for ENUMERATION_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.OwnerProcessId == other.OwnerProcessId && self.FlowNameLength == other.FlowNameLength && self.FlowName == other.FlowName && self.pFlow == other.pFlow && self.NumberOfFilters == other.NumberOfFilters && self.GenericFilter == other.GenericFilter
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for ENUMERATION_BUFFER {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for ENUMERATION_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMERATION_BUFFER").field("Length", &self.Length).field("OwnerProcessId", &self.OwnerProcessId).field("FlowNameLength", &self.FlowNameLength).field("FlowName", &self.FlowName).field("pFlow", &self.pFlow).field("NumberOfFilters", &self.NumberOfFilters).field("GenericFilter", &self.GenericFilter).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for ERROR_SPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for Error_Spec_IPv4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for FILTER_SPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for FLOWDESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for FLOWDESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.FlowSpec == other.FlowSpec && self.NumFilters == other.NumFilters && self.FilterList == other.FilterList
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for FLOWDESCRIPTOR {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for FLOWDESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLOWDESCRIPTOR").field("FlowSpec", &self.FlowSpec).field("NumFilters", &self.NumFilters).field("FilterList", &self.FilterList).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for FLOW_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FilterType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FilterType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FilterType").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for Filter_Spec_IPv4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for Filter_Spec_IPv4GPI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for Gads_parms_t {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for Gads_parms_t {
    fn eq(&self, other: &Self) -> bool {
        self.Gads_serv_hdr == other.Gads_serv_hdr && self.Gads_Ctot_hdr == other.Gads_Ctot_hdr && self.Gads_Ctot == other.Gads_Ctot && self.Gads_Dtot_hdr == other.Gads_Dtot_hdr && self.Gads_Dtot == other.Gads_Dtot && self.Gads_Csum_hdr == other.Gads_Csum_hdr && self.Gads_Csum == other.Gads_Csum && self.Gads_Dsum_hdr == other.Gads_Dsum_hdr && self.Gads_Dsum == other.Gads_Dsum
    }
}
impl ::core::cmp::Eq for Gads_parms_t {}
impl ::core::fmt::Debug for Gads_parms_t {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Gads_parms_t").field("Gads_serv_hdr", &self.Gads_serv_hdr).field("Gads_Ctot_hdr", &self.Gads_Ctot_hdr).field("Gads_Ctot", &self.Gads_Ctot).field("Gads_Dtot_hdr", &self.Gads_Dtot_hdr).field("Gads_Dtot", &self.Gads_Dtot).field("Gads_Csum_hdr", &self.Gads_Csum_hdr).field("Gads_Csum", &self.Gads_Csum).field("Gads_Dsum_hdr", &self.Gads_Dsum_hdr).field("Gads_Dsum", &self.Gads_Dsum).finish()
    }
}
impl ::core::default::Default for GenAdspecParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GenAdspecParams {
    fn eq(&self, other: &Self) -> bool {
        self.gen_parm_hdr == other.gen_parm_hdr && self.gen_parm_hopcnt_hdr == other.gen_parm_hopcnt_hdr && self.gen_parm_hopcnt == other.gen_parm_hopcnt && self.gen_parm_pathbw_hdr == other.gen_parm_pathbw_hdr && self.gen_parm_path_bw == other.gen_parm_path_bw && self.gen_parm_minlat_hdr == other.gen_parm_minlat_hdr && self.gen_parm_min_latency == other.gen_parm_min_latency && self.gen_parm_compmtu_hdr == other.gen_parm_compmtu_hdr && self.gen_parm_composed_MTU == other.gen_parm_composed_MTU
    }
}
impl ::core::cmp::Eq for GenAdspecParams {}
impl ::core::fmt::Debug for GenAdspecParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GenAdspecParams")
            .field("gen_parm_hdr", &self.gen_parm_hdr)
            .field("gen_parm_hopcnt_hdr", &self.gen_parm_hopcnt_hdr)
            .field("gen_parm_hopcnt", &self.gen_parm_hopcnt)
            .field("gen_parm_pathbw_hdr", &self.gen_parm_pathbw_hdr)
            .field("gen_parm_path_bw", &self.gen_parm_path_bw)
            .field("gen_parm_minlat_hdr", &self.gen_parm_minlat_hdr)
            .field("gen_parm_min_latency", &self.gen_parm_min_latency)
            .field("gen_parm_compmtu_hdr", &self.gen_parm_compmtu_hdr)
            .field("gen_parm_composed_MTU", &self.gen_parm_composed_MTU)
            .finish()
    }
}
impl ::core::default::Default for GenTspec {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GenTspec {
    fn eq(&self, other: &Self) -> bool {
        self.gen_Tspec_serv_hdr == other.gen_Tspec_serv_hdr && self.gen_Tspec_parm_hdr == other.gen_Tspec_parm_hdr && self.gen_Tspec_parms == other.gen_Tspec_parms
    }
}
impl ::core::cmp::Eq for GenTspec {}
impl ::core::fmt::Debug for GenTspec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GenTspec").field("gen_Tspec_serv_hdr", &self.gen_Tspec_serv_hdr).field("gen_Tspec_parm_hdr", &self.gen_Tspec_parm_hdr).field("gen_Tspec_parms", &self.gen_Tspec_parms).finish()
    }
}
impl ::core::default::Default for GenTspecParms {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GenTspecParms {
    fn eq(&self, other: &Self) -> bool {
        self.TB_Tspec_r == other.TB_Tspec_r && self.TB_Tspec_b == other.TB_Tspec_b && self.TB_Tspec_p == other.TB_Tspec_p && self.TB_Tspec_m == other.TB_Tspec_m && self.TB_Tspec_M == other.TB_Tspec_M
    }
}
impl ::core::cmp::Eq for GenTspecParms {}
impl ::core::fmt::Debug for GenTspecParms {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GenTspecParms").field("TB_Tspec_r", &self.TB_Tspec_r).field("TB_Tspec_b", &self.TB_Tspec_b).field("TB_Tspec_p", &self.TB_Tspec_p).field("TB_Tspec_m", &self.TB_Tspec_m).field("TB_Tspec_M", &self.TB_Tspec_M).finish()
    }
}
impl ::core::default::Default for GuarFlowSpec {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GuarFlowSpec {
    fn eq(&self, other: &Self) -> bool {
        self.Guar_serv_hdr == other.Guar_serv_hdr && self.Guar_Tspec_hdr == other.Guar_Tspec_hdr && self.Guar_Tspec_parms == other.Guar_Tspec_parms && self.Guar_Rspec_hdr == other.Guar_Rspec_hdr && self.Guar_Rspec == other.Guar_Rspec
    }
}
impl ::core::cmp::Eq for GuarFlowSpec {}
impl ::core::fmt::Debug for GuarFlowSpec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GuarFlowSpec").field("Guar_serv_hdr", &self.Guar_serv_hdr).field("Guar_Tspec_hdr", &self.Guar_Tspec_hdr).field("Guar_Tspec_parms", &self.Guar_Tspec_parms).field("Guar_Rspec_hdr", &self.Guar_Rspec_hdr).field("Guar_Rspec", &self.Guar_Rspec).finish()
    }
}
impl ::core::default::Default for GuarRspec {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GuarRspec {
    fn eq(&self, other: &Self) -> bool {
        self.Guar_R == other.Guar_R && self.Guar_S == other.Guar_S
    }
}
impl ::core::cmp::Eq for GuarRspec {}
impl ::core::fmt::Debug for GuarRspec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GuarRspec").field("Guar_R", &self.Guar_R).field("Guar_S", &self.Guar_S).finish()
    }
}
impl ::core::default::Default for IDPE_ATTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IDPE_ATTR {
    fn eq(&self, other: &Self) -> bool {
        self.PeAttribLength == other.PeAttribLength && self.PeAttribType == other.PeAttribType && self.PeAttribSubType == other.PeAttribSubType && self.PeAttribValue == other.PeAttribValue
    }
}
impl ::core::cmp::Eq for IDPE_ATTR {}
impl ::core::fmt::Debug for IDPE_ATTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IDPE_ATTR").field("PeAttribLength", &self.PeAttribLength).field("PeAttribType", &self.PeAttribType).field("PeAttribSubType", &self.PeAttribSubType).field("PeAttribValue", &self.PeAttribValue).finish()
    }
}
impl ::core::default::Default for ID_ERROR_OBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ID_ERROR_OBJECT {
    fn eq(&self, other: &Self) -> bool {
        self.usIdErrLength == other.usIdErrLength && self.ucAType == other.ucAType && self.ucSubType == other.ucSubType && self.usReserved == other.usReserved && self.usIdErrorValue == other.usIdErrorValue && self.ucIdErrData == other.ucIdErrData
    }
}
impl ::core::cmp::Eq for ID_ERROR_OBJECT {}
impl ::core::fmt::Debug for ID_ERROR_OBJECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ID_ERROR_OBJECT").field("usIdErrLength", &self.usIdErrLength).field("ucAType", &self.ucAType).field("ucSubType", &self.ucSubType).field("usReserved", &self.usReserved).field("usIdErrorValue", &self.usIdErrorValue).field("ucIdErrData", &self.ucIdErrData).finish()
    }
}
impl ::core::default::Default for IN_ADDR_IPV4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IN_ADDR_IPV6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IN_ADDR_IPV6 {
    fn eq(&self, other: &Self) -> bool {
        self.Addr == other.Addr
    }
}
impl ::core::cmp::Eq for IN_ADDR_IPV6 {}
impl ::core::fmt::Debug for IN_ADDR_IPV6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IN_ADDR_IPV6").field("Addr", &self.Addr).finish()
    }
}
impl ::core::default::Default for IPX_PATTERN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPX_PATTERN {
    fn eq(&self, other: &Self) -> bool {
        self.Src == other.Src && self.Dest == other.Dest
    }
}
impl ::core::cmp::Eq for IPX_PATTERN {}
impl ::core::fmt::Debug for IPX_PATTERN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPX_PATTERN").field("Src", &self.Src).field("Dest", &self.Dest).finish()
    }
}
impl ::core::default::Default for IPX_PATTERN_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPX_PATTERN_0 {
    fn eq(&self, other: &Self) -> bool {
        self.NetworkAddress == other.NetworkAddress && self.NodeAddress == other.NodeAddress && self.Socket == other.Socket
    }
}
impl ::core::cmp::Eq for IPX_PATTERN_0 {}
impl ::core::fmt::Debug for IPX_PATTERN_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPX_PATTERN_0").field("NetworkAddress", &self.NetworkAddress).field("NodeAddress", &self.NodeAddress).field("Socket", &self.Socket).finish()
    }
}
impl ::core::default::Default for IP_PATTERN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IS_ADSPEC_BODY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IS_ADSPEC_BODY {
    fn eq(&self, other: &Self) -> bool {
        self.adspec_mh == other.adspec_mh && self.adspec_genparms == other.adspec_genparms
    }
}
impl ::core::cmp::Eq for IS_ADSPEC_BODY {}
impl ::core::fmt::Debug for IS_ADSPEC_BODY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IS_ADSPEC_BODY").field("adspec_mh", &self.adspec_mh).field("adspec_genparms", &self.adspec_genparms).finish()
    }
}
impl ::core::default::Default for IS_FLOWSPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IntServFlowSpec {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IntServMainHdr {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IntServMainHdr {
    fn eq(&self, other: &Self) -> bool {
        self.ismh_version == other.ismh_version && self.ismh_unused == other.ismh_unused && self.ismh_len32b == other.ismh_len32b
    }
}
impl ::core::cmp::Eq for IntServMainHdr {}
impl ::core::fmt::Debug for IntServMainHdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IntServMainHdr").field("ismh_version", &self.ismh_version).field("ismh_unused", &self.ismh_unused).field("ismh_len32b", &self.ismh_len32b).finish()
    }
}
impl ::core::default::Default for IntServParmHdr {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IntServParmHdr {
    fn eq(&self, other: &Self) -> bool {
        self.isph_parm_num == other.isph_parm_num && self.isph_flags == other.isph_flags && self.isph_len32b == other.isph_len32b
    }
}
impl ::core::cmp::Eq for IntServParmHdr {}
impl ::core::fmt::Debug for IntServParmHdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IntServParmHdr").field("isph_parm_num", &self.isph_parm_num).field("isph_flags", &self.isph_flags).field("isph_len32b", &self.isph_len32b).finish()
    }
}
impl ::core::default::Default for IntServServiceHdr {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IntServServiceHdr {
    fn eq(&self, other: &Self) -> bool {
        self.issh_service == other.issh_service && self.issh_flags == other.issh_flags && self.issh_len32b == other.issh_len32b
    }
}
impl ::core::cmp::Eq for IntServServiceHdr {}
impl ::core::fmt::Debug for IntServServiceHdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IntServServiceHdr").field("issh_service", &self.issh_service).field("issh_flags", &self.issh_flags).field("issh_len32b", &self.issh_len32b).finish()
    }
}
impl ::core::default::Default for IntServTspecBody {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for LPMIPTABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LPM_INIT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PARAM_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PARAM_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.ParameterId == other.ParameterId && self.Length == other.Length && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for PARAM_BUFFER {}
impl ::core::fmt::Debug for PARAM_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PARAM_BUFFER").field("ParameterId", &self.ParameterId).field("Length", &self.Length).field("Buffer", &self.Buffer).finish()
    }
}
impl ::core::default::Default for POLICY_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for POLICY_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.PolicyObjHdr == other.PolicyObjHdr && self.usPeOffset == other.usPeOffset && self.usReserved == other.usReserved
    }
}
impl ::core::cmp::Eq for POLICY_DATA {}
impl ::core::fmt::Debug for POLICY_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLICY_DATA").field("PolicyObjHdr", &self.PolicyObjHdr).field("usPeOffset", &self.usPeOffset).field("usReserved", &self.usReserved).finish()
    }
}
impl ::core::default::Default for POLICY_DECISION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for POLICY_DECISION {
    fn eq(&self, other: &Self) -> bool {
        self.lpvResult == other.lpvResult && self.wPolicyErrCode == other.wPolicyErrCode && self.wPolicyErrValue == other.wPolicyErrValue
    }
}
impl ::core::cmp::Eq for POLICY_DECISION {}
impl ::core::fmt::Debug for POLICY_DECISION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLICY_DECISION").field("lpvResult", &self.lpvResult).field("wPolicyErrCode", &self.wPolicyErrCode).field("wPolicyErrValue", &self.wPolicyErrValue).finish()
    }
}
impl ::core::default::Default for POLICY_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for POLICY_ELEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.usPeLength == other.usPeLength && self.usPeType == other.usPeType && self.ucPeData == other.ucPeData
    }
}
impl ::core::cmp::Eq for POLICY_ELEMENT {}
impl ::core::fmt::Debug for POLICY_ELEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLICY_ELEMENT").field("usPeLength", &self.usPeLength).field("usPeType", &self.usPeType).field("ucPeData", &self.ucPeData).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for QOS_DESTADDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for QOS_DESTADDR {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectHdr == other.ObjectHdr && self.SocketAddress == other.SocketAddress && self.SocketAddressLength == other.SocketAddressLength
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for QOS_DESTADDR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for QOS_DESTADDR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QOS_DESTADDR").field("ObjectHdr", &self.ObjectHdr).field("SocketAddress", &self.SocketAddress).field("SocketAddressLength", &self.SocketAddressLength).finish()
    }
}
impl ::core::default::Default for QOS_DIFFSERV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for QOS_DIFFSERV {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectHdr == other.ObjectHdr && self.DSFieldCount == other.DSFieldCount && self.DiffservRule == other.DiffservRule
    }
}
impl ::core::cmp::Eq for QOS_DIFFSERV {}
impl ::core::fmt::Debug for QOS_DIFFSERV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QOS_DIFFSERV").field("ObjectHdr", &self.ObjectHdr).field("DSFieldCount", &self.DSFieldCount).field("DiffservRule", &self.DiffservRule).finish()
    }
}
impl ::core::default::Default for QOS_DIFFSERV_RULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for QOS_DIFFSERV_RULE {
    fn eq(&self, other: &Self) -> bool {
        self.InboundDSField == other.InboundDSField && self.ConformingOutboundDSField == other.ConformingOutboundDSField && self.NonConformingOutboundDSField == other.NonConformingOutboundDSField && self.ConformingUserPriority == other.ConformingUserPriority && self.NonConformingUserPriority == other.NonConformingUserPriority
    }
}
impl ::core::cmp::Eq for QOS_DIFFSERV_RULE {}
impl ::core::fmt::Debug for QOS_DIFFSERV_RULE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QOS_DIFFSERV_RULE").field("InboundDSField", &self.InboundDSField).field("ConformingOutboundDSField", &self.ConformingOutboundDSField).field("NonConformingOutboundDSField", &self.NonConformingOutboundDSField).field("ConformingUserPriority", &self.ConformingUserPriority).field("NonConformingUserPriority", &self.NonConformingUserPriority).finish()
    }
}
impl ::core::default::Default for QOS_DS_CLASS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for QOS_DS_CLASS {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectHdr == other.ObjectHdr && self.DSField == other.DSField
    }
}
impl ::core::cmp::Eq for QOS_DS_CLASS {}
impl ::core::fmt::Debug for QOS_DS_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QOS_DS_CLASS").field("ObjectHdr", &self.ObjectHdr).field("DSField", &self.DSField).finish()
    }
}
impl ::core::default::Default for QOS_FLOWRATE_OUTGOING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for QOS_FLOWRATE_OUTGOING {
    fn eq(&self, other: &Self) -> bool {
        self.Bandwidth == other.Bandwidth && self.ShapingBehavior == other.ShapingBehavior && self.Reason == other.Reason
    }
}
impl ::core::cmp::Eq for QOS_FLOWRATE_OUTGOING {}
impl ::core::fmt::Debug for QOS_FLOWRATE_OUTGOING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QOS_FLOWRATE_OUTGOING").field("Bandwidth", &self.Bandwidth).field("ShapingBehavior", &self.ShapingBehavior).field("Reason", &self.Reason).finish()
    }
}
impl ::core::default::Default for QOS_FLOWRATE_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for QOS_FLOWRATE_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QOS_FLOWRATE_REASON").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for QOS_FLOW_FUNDAMENTALS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for QOS_FLOW_FUNDAMENTALS {
    fn eq(&self, other: &Self) -> bool {
        self.BottleneckBandwidthSet == other.BottleneckBandwidthSet && self.BottleneckBandwidth == other.BottleneckBandwidth && self.AvailableBandwidthSet == other.AvailableBandwidthSet && self.AvailableBandwidth == other.AvailableBandwidth && self.RTTSet == other.RTTSet && self.RTT == other.RTT
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for QOS_FLOW_FUNDAMENTALS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for QOS_FLOW_FUNDAMENTALS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QOS_FLOW_FUNDAMENTALS").field("BottleneckBandwidthSet", &self.BottleneckBandwidthSet).field("BottleneckBandwidth", &self.BottleneckBandwidth).field("AvailableBandwidthSet", &self.AvailableBandwidthSet).field("AvailableBandwidth", &self.AvailableBandwidth).field("RTTSet", &self.RTTSet).field("RTT", &self.RTT).finish()
    }
}
impl ::core::default::Default for QOS_FRIENDLY_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for QOS_FRIENDLY_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectHdr == other.ObjectHdr && self.FriendlyName == other.FriendlyName
    }
}
impl ::core::cmp::Eq for QOS_FRIENDLY_NAME {}
impl ::core::fmt::Debug for QOS_FRIENDLY_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QOS_FRIENDLY_NAME").field("ObjectHdr", &self.ObjectHdr).field("FriendlyName", &self.FriendlyName).finish()
    }
}
impl ::core::default::Default for QOS_NOTIFY_FLOW {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for QOS_NOTIFY_FLOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QOS_NOTIFY_FLOW").field(&self.0).finish()
    }
}
impl ::core::default::Default for QOS_OBJECT_HDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for QOS_OBJECT_HDR {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectType == other.ObjectType && self.ObjectLength == other.ObjectLength
    }
}
impl ::core::cmp::Eq for QOS_OBJECT_HDR {}
impl ::core::fmt::Debug for QOS_OBJECT_HDR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QOS_OBJECT_HDR").field("ObjectType", &self.ObjectType).field("ObjectLength", &self.ObjectLength).finish()
    }
}
impl ::core::default::Default for QOS_PACKET_PRIORITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for QOS_PACKET_PRIORITY {
    fn eq(&self, other: &Self) -> bool {
        self.ConformantDSCPValue == other.ConformantDSCPValue && self.NonConformantDSCPValue == other.NonConformantDSCPValue && self.ConformantL2Value == other.ConformantL2Value && self.NonConformantL2Value == other.NonConformantL2Value
    }
}
impl ::core::cmp::Eq for QOS_PACKET_PRIORITY {}
impl ::core::fmt::Debug for QOS_PACKET_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QOS_PACKET_PRIORITY").field("ConformantDSCPValue", &self.ConformantDSCPValue).field("NonConformantDSCPValue", &self.NonConformantDSCPValue).field("ConformantL2Value", &self.ConformantL2Value).field("NonConformantL2Value", &self.NonConformantL2Value).finish()
    }
}
impl ::core::default::Default for QOS_QUERY_FLOW {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for QOS_QUERY_FLOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QOS_QUERY_FLOW").field(&self.0).finish()
    }
}
impl ::core::default::Default for QOS_SD_MODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for QOS_SD_MODE {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectHdr == other.ObjectHdr && self.ShapeDiscardMode == other.ShapeDiscardMode
    }
}
impl ::core::cmp::Eq for QOS_SD_MODE {}
impl ::core::fmt::Debug for QOS_SD_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QOS_SD_MODE").field("ObjectHdr", &self.ObjectHdr).field("ShapeDiscardMode", &self.ShapeDiscardMode).finish()
    }
}
impl ::core::default::Default for QOS_SET_FLOW {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for QOS_SET_FLOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QOS_SET_FLOW").field(&self.0).finish()
    }
}
impl ::core::default::Default for QOS_SHAPING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for QOS_SHAPING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QOS_SHAPING").field(&self.0).finish()
    }
}
impl ::core::default::Default for QOS_SHAPING_RATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for QOS_SHAPING_RATE {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectHdr == other.ObjectHdr && self.ShapingRate == other.ShapingRate
    }
}
impl ::core::cmp::Eq for QOS_SHAPING_RATE {}
impl ::core::fmt::Debug for QOS_SHAPING_RATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QOS_SHAPING_RATE").field("ObjectHdr", &self.ObjectHdr).field("ShapingRate", &self.ShapingRate).finish()
    }
}
impl ::core::default::Default for QOS_TCP_TRAFFIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for QOS_TCP_TRAFFIC {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectHdr == other.ObjectHdr
    }
}
impl ::core::cmp::Eq for QOS_TCP_TRAFFIC {}
impl ::core::fmt::Debug for QOS_TCP_TRAFFIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QOS_TCP_TRAFFIC").field("ObjectHdr", &self.ObjectHdr).finish()
    }
}
impl ::core::default::Default for QOS_TRAFFIC_CLASS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for QOS_TRAFFIC_CLASS {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectHdr == other.ObjectHdr && self.TrafficClass == other.TrafficClass
    }
}
impl ::core::cmp::Eq for QOS_TRAFFIC_CLASS {}
impl ::core::fmt::Debug for QOS_TRAFFIC_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QOS_TRAFFIC_CLASS").field("ObjectHdr", &self.ObjectHdr).field("TrafficClass", &self.TrafficClass).finish()
    }
}
impl ::core::default::Default for QOS_TRAFFIC_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for QOS_TRAFFIC_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QOS_TRAFFIC_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for QOS_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for QOS_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion
    }
}
impl ::core::cmp::Eq for QOS_VERSION {}
impl ::core::fmt::Debug for QOS_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QOS_VERSION").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).finish()
    }
}
impl ::core::default::Default for QualAppFlowSpec {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for QualAppFlowSpec {
    fn eq(&self, other: &Self) -> bool {
        self.Q_spec_serv_hdr == other.Q_spec_serv_hdr && self.Q_spec_parm_hdr == other.Q_spec_parm_hdr && self.Q_spec_parms == other.Q_spec_parms
    }
}
impl ::core::cmp::Eq for QualAppFlowSpec {}
impl ::core::fmt::Debug for QualAppFlowSpec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QualAppFlowSpec").field("Q_spec_serv_hdr", &self.Q_spec_serv_hdr).field("Q_spec_parm_hdr", &self.Q_spec_parm_hdr).field("Q_spec_parms", &self.Q_spec_parms).finish()
    }
}
impl ::core::default::Default for QualTspec {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for QualTspec {
    fn eq(&self, other: &Self) -> bool {
        self.qual_Tspec_serv_hdr == other.qual_Tspec_serv_hdr && self.qual_Tspec_parm_hdr == other.qual_Tspec_parm_hdr && self.qual_Tspec_parms == other.qual_Tspec_parms
    }
}
impl ::core::cmp::Eq for QualTspec {}
impl ::core::fmt::Debug for QualTspec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QualTspec").field("qual_Tspec_serv_hdr", &self.qual_Tspec_serv_hdr).field("qual_Tspec_parm_hdr", &self.qual_Tspec_parm_hdr).field("qual_Tspec_parms", &self.qual_Tspec_parms).finish()
    }
}
impl ::core::default::Default for QualTspecParms {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for QualTspecParms {
    fn eq(&self, other: &Self) -> bool {
        self.TB_Tspec_M == other.TB_Tspec_M
    }
}
impl ::core::cmp::Eq for QualTspecParms {}
impl ::core::fmt::Debug for QualTspecParms {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QualTspecParms").field("TB_Tspec_M", &self.TB_Tspec_M).finish()
    }
}
impl ::core::default::Default for RESV_STYLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RESV_STYLE {
    fn eq(&self, other: &Self) -> bool {
        self.style_header == other.style_header && self.style_word == other.style_word
    }
}
impl ::core::cmp::Eq for RESV_STYLE {}
impl ::core::fmt::Debug for RESV_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESV_STYLE").field("style_header", &self.style_header).field("style_word", &self.style_word).finish()
    }
}
impl ::core::default::Default for RSVP_ADSPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RSVP_FILTERSPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RSVP_FILTERSPEC_V4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RSVP_FILTERSPEC_V4_GPI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RSVP_FILTERSPEC_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RSVP_FILTERSPEC_V6 {
    fn eq(&self, other: &Self) -> bool {
        self.Address == other.Address && self.UnUsed == other.UnUsed && self.Port == other.Port
    }
}
impl ::core::cmp::Eq for RSVP_FILTERSPEC_V6 {}
impl ::core::fmt::Debug for RSVP_FILTERSPEC_V6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RSVP_FILTERSPEC_V6").field("Address", &self.Address).field("UnUsed", &self.UnUsed).field("Port", &self.Port).finish()
    }
}
impl ::core::default::Default for RSVP_FILTERSPEC_V6_FLOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RSVP_FILTERSPEC_V6_FLOW {
    fn eq(&self, other: &Self) -> bool {
        self.Address == other.Address && self.UnUsed == other.UnUsed && self.FlowLabel == other.FlowLabel
    }
}
impl ::core::cmp::Eq for RSVP_FILTERSPEC_V6_FLOW {}
impl ::core::fmt::Debug for RSVP_FILTERSPEC_V6_FLOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RSVP_FILTERSPEC_V6_FLOW").field("Address", &self.Address).field("UnUsed", &self.UnUsed).field("FlowLabel", &self.FlowLabel).finish()
    }
}
impl ::core::default::Default for RSVP_FILTERSPEC_V6_GPI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RSVP_FILTERSPEC_V6_GPI {
    fn eq(&self, other: &Self) -> bool {
        self.Address == other.Address && self.GeneralPortId == other.GeneralPortId
    }
}
impl ::core::cmp::Eq for RSVP_FILTERSPEC_V6_GPI {}
impl ::core::fmt::Debug for RSVP_FILTERSPEC_V6_GPI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RSVP_FILTERSPEC_V6_GPI").field("Address", &self.Address).field("GeneralPortId", &self.GeneralPortId).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for RSVP_HOP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for RSVP_MSG_OBJS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for RSVP_MSG_OBJS {
    fn eq(&self, other: &Self) -> bool {
        self.RsvpMsgType == other.RsvpMsgType && self.pRsvpSession == other.pRsvpSession && self.pRsvpFromHop == other.pRsvpFromHop && self.pRsvpToHop == other.pRsvpToHop && self.pResvStyle == other.pResvStyle && self.pRsvpScope == other.pRsvpScope && self.FlowDescCount == other.FlowDescCount && self.pFlowDescs == other.pFlowDescs && self.PdObjectCount == other.PdObjectCount && self.ppPdObjects == other.ppPdObjects && self.pErrorSpec == other.pErrorSpec && self.pAdspec == other.pAdspec
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for RSVP_MSG_OBJS {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for RSVP_MSG_OBJS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RSVP_MSG_OBJS")
            .field("RsvpMsgType", &self.RsvpMsgType)
            .field("pRsvpSession", &self.pRsvpSession)
            .field("pRsvpFromHop", &self.pRsvpFromHop)
            .field("pRsvpToHop", &self.pRsvpToHop)
            .field("pResvStyle", &self.pResvStyle)
            .field("pRsvpScope", &self.pRsvpScope)
            .field("FlowDescCount", &self.FlowDescCount)
            .field("pFlowDescs", &self.pFlowDescs)
            .field("PdObjectCount", &self.PdObjectCount)
            .field("ppPdObjects", &self.ppPdObjects)
            .field("pErrorSpec", &self.pErrorSpec)
            .field("pAdspec", &self.pAdspec)
            .finish()
    }
}
impl ::core::default::Default for RSVP_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RSVP_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Len == other.Len && self.Type == other.Type && self.Info == other.Info
    }
}
impl ::core::cmp::Eq for RSVP_POLICY {}
impl ::core::fmt::Debug for RSVP_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RSVP_POLICY").field("Len", &self.Len).field("Type", &self.Type).field("Info", &self.Info).finish()
    }
}
impl ::core::default::Default for RSVP_POLICY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RSVP_POLICY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectHdr == other.ObjectHdr && self.NumPolicyElement == other.NumPolicyElement && self.PolicyElement == other.PolicyElement
    }
}
impl ::core::cmp::Eq for RSVP_POLICY_INFO {}
impl ::core::fmt::Debug for RSVP_POLICY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RSVP_POLICY_INFO").field("ObjectHdr", &self.ObjectHdr).field("NumPolicyElement", &self.NumPolicyElement).field("PolicyElement", &self.PolicyElement).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for RSVP_RESERVE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for RSVP_RESERVE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectHdr == other.ObjectHdr && self.Style == other.Style && self.ConfirmRequest == other.ConfirmRequest && self.PolicyElementList == other.PolicyElementList && self.NumFlowDesc == other.NumFlowDesc && self.FlowDescList == other.FlowDescList
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for RSVP_RESERVE_INFO {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for RSVP_RESERVE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RSVP_RESERVE_INFO").field("ObjectHdr", &self.ObjectHdr).field("Style", &self.Style).field("ConfirmRequest", &self.ConfirmRequest).field("PolicyElementList", &self.PolicyElementList).field("NumFlowDesc", &self.NumFlowDesc).field("FlowDescList", &self.FlowDescList).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for RSVP_SCOPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for RSVP_SESSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RSVP_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RSVP_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectHdr == other.ObjectHdr && self.StatusCode == other.StatusCode && self.ExtendedStatus1 == other.ExtendedStatus1 && self.ExtendedStatus2 == other.ExtendedStatus2
    }
}
impl ::core::cmp::Eq for RSVP_STATUS_INFO {}
impl ::core::fmt::Debug for RSVP_STATUS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RSVP_STATUS_INFO").field("ObjectHdr", &self.ObjectHdr).field("StatusCode", &self.StatusCode).field("ExtendedStatus1", &self.ExtendedStatus1).field("ExtendedStatus2", &self.ExtendedStatus2).finish()
    }
}
impl ::core::default::Default for RsvpObjHdr {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RsvpObjHdr {
    fn eq(&self, other: &Self) -> bool {
        self.obj_length == other.obj_length && self.obj_class == other.obj_class && self.obj_ctype == other.obj_ctype
    }
}
impl ::core::cmp::Eq for RsvpObjHdr {}
impl ::core::fmt::Debug for RsvpObjHdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RsvpObjHdr").field("obj_length", &self.obj_length).field("obj_class", &self.obj_class).field("obj_ctype", &self.obj_ctype).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for Rsvp_Hop_IPv4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SENDER_TSPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SIPAEVENT_KSR_SIGNATURE_PAYLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SIPAEVENT_REVOCATION_LIST_PAYLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SIPAEVENT_SBCP_INFO_PAYLOAD_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SIPAEVENT_SI_POLICY_PAYLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SIPAEVENT_VSM_IDK_INFO_PAYLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SIPAEVENT_VSM_IDK_RSA_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for Scope_list_ipv4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for Session_IPv4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TCG_PCClientPCREventStruct {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TCG_PCClientTaggedEventStruct {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TCI_CLIENT_FUNC_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TC_GEN_FILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TC_GEN_FILTER {
    fn eq(&self, other: &Self) -> bool {
        self.AddressType == other.AddressType && self.PatternSize == other.PatternSize && self.Pattern == other.Pattern && self.Mask == other.Mask
    }
}
impl ::core::cmp::Eq for TC_GEN_FILTER {}
impl ::core::fmt::Debug for TC_GEN_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TC_GEN_FILTER").field("AddressType", &self.AddressType).field("PatternSize", &self.PatternSize).field("Pattern", &self.Pattern).field("Mask", &self.Mask).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for TC_GEN_FLOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for TC_GEN_FLOW {
    fn eq(&self, other: &Self) -> bool {
        self.SendingFlowspec == other.SendingFlowspec && self.ReceivingFlowspec == other.ReceivingFlowspec && self.TcObjectsLength == other.TcObjectsLength && self.TcObjects == other.TcObjects
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for TC_GEN_FLOW {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for TC_GEN_FLOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TC_GEN_FLOW").field("SendingFlowspec", &self.SendingFlowspec).field("ReceivingFlowspec", &self.ReceivingFlowspec).field("TcObjectsLength", &self.TcObjectsLength).field("TcObjects", &self.TcObjects).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for TC_IFC_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for TC_IFC_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.pInterfaceName == other.pInterfaceName && self.pInterfaceID == other.pInterfaceID && self.AddressListDesc == other.AddressListDesc
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for TC_IFC_DESCRIPTOR {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for TC_IFC_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TC_IFC_DESCRIPTOR").field("Length", &self.Length).field("pInterfaceName", &self.pInterfaceName).field("pInterfaceID", &self.pInterfaceID).field("AddressListDesc", &self.AddressListDesc).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for TC_SUPPORTED_INFO_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for TC_SUPPORTED_INFO_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.InstanceIDLength == other.InstanceIDLength && self.InstanceID == other.InstanceID && self.InterfaceLuid == other.InterfaceLuid && self.AddrListDesc == other.AddrListDesc
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for TC_SUPPORTED_INFO_BUFFER {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for TC_SUPPORTED_INFO_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TC_SUPPORTED_INFO_BUFFER").field("InstanceIDLength", &self.InstanceIDLength).field("InstanceID", &self.InstanceID).field("InterfaceLuid", &self.InterfaceLuid).field("AddrListDesc", &self.AddrListDesc).finish()
    }
}
impl ::core::default::Default for WBCL_Iterator {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WBCL_LogHdr {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for int_serv_wkp {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for int_serv_wkp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("int_serv_wkp").field(&self.0).finish()
    }
}
