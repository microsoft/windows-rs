impl ::core::default::Default for ACTION_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ACTION_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.transport_id == other.transport_id && self.action_code == other.action_code && self.reserved == other.reserved
    }
}
impl ::core::cmp::Eq for ACTION_HEADER {}
impl ::core::fmt::Debug for ACTION_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTION_HEADER").field("transport_id", &self.transport_id).field("action_code", &self.action_code).field("reserved", &self.reserved).finish()
    }
}
impl ::core::default::Default for ADAPTER_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ADAPTER_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.adapter_address == other.adapter_address
            && self.rev_major == other.rev_major
            && self.reserved0 == other.reserved0
            && self.adapter_type == other.adapter_type
            && self.rev_minor == other.rev_minor
            && self.duration == other.duration
            && self.frmr_recv == other.frmr_recv
            && self.frmr_xmit == other.frmr_xmit
            && self.iframe_recv_err == other.iframe_recv_err
            && self.xmit_aborts == other.xmit_aborts
            && self.xmit_success == other.xmit_success
            && self.recv_success == other.recv_success
            && self.iframe_xmit_err == other.iframe_xmit_err
            && self.recv_buff_unavail == other.recv_buff_unavail
            && self.t1_timeouts == other.t1_timeouts
            && self.ti_timeouts == other.ti_timeouts
            && self.reserved1 == other.reserved1
            && self.free_ncbs == other.free_ncbs
            && self.max_cfg_ncbs == other.max_cfg_ncbs
            && self.max_ncbs == other.max_ncbs
            && self.xmit_buf_unavail == other.xmit_buf_unavail
            && self.max_dgram_size == other.max_dgram_size
            && self.pending_sess == other.pending_sess
            && self.max_cfg_sess == other.max_cfg_sess
            && self.max_sess == other.max_sess
            && self.max_sess_pkt_size == other.max_sess_pkt_size
            && self.name_count == other.name_count
    }
}
impl ::core::cmp::Eq for ADAPTER_STATUS {}
impl ::core::fmt::Debug for ADAPTER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADAPTER_STATUS")
            .field("adapter_address", &self.adapter_address)
            .field("rev_major", &self.rev_major)
            .field("reserved0", &self.reserved0)
            .field("adapter_type", &self.adapter_type)
            .field("rev_minor", &self.rev_minor)
            .field("duration", &self.duration)
            .field("frmr_recv", &self.frmr_recv)
            .field("frmr_xmit", &self.frmr_xmit)
            .field("iframe_recv_err", &self.iframe_recv_err)
            .field("xmit_aborts", &self.xmit_aborts)
            .field("xmit_success", &self.xmit_success)
            .field("recv_success", &self.recv_success)
            .field("iframe_xmit_err", &self.iframe_xmit_err)
            .field("recv_buff_unavail", &self.recv_buff_unavail)
            .field("t1_timeouts", &self.t1_timeouts)
            .field("ti_timeouts", &self.ti_timeouts)
            .field("reserved1", &self.reserved1)
            .field("free_ncbs", &self.free_ncbs)
            .field("max_cfg_ncbs", &self.max_cfg_ncbs)
            .field("max_ncbs", &self.max_ncbs)
            .field("xmit_buf_unavail", &self.xmit_buf_unavail)
            .field("max_dgram_size", &self.max_dgram_size)
            .field("pending_sess", &self.pending_sess)
            .field("max_cfg_sess", &self.max_cfg_sess)
            .field("max_sess", &self.max_sess)
            .field("max_sess_pkt_size", &self.max_sess_pkt_size)
            .field("name_count", &self.name_count)
            .finish()
    }
}
impl ::core::default::Default for FIND_NAME_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FIND_NAME_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.access_control == other.access_control && self.frame_control == other.frame_control && self.destination_addr == other.destination_addr && self.source_addr == other.source_addr && self.routing_info == other.routing_info
    }
}
impl ::core::cmp::Eq for FIND_NAME_BUFFER {}
impl ::core::fmt::Debug for FIND_NAME_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FIND_NAME_BUFFER").field("length", &self.length).field("access_control", &self.access_control).field("frame_control", &self.frame_control).field("destination_addr", &self.destination_addr).field("source_addr", &self.source_addr).field("routing_info", &self.routing_info).finish()
    }
}
impl ::core::default::Default for FIND_NAME_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FIND_NAME_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.node_count == other.node_count && self.reserved == other.reserved && self.unique_group == other.unique_group
    }
}
impl ::core::cmp::Eq for FIND_NAME_HEADER {}
impl ::core::fmt::Debug for FIND_NAME_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FIND_NAME_HEADER").field("node_count", &self.node_count).field("reserved", &self.reserved).field("unique_group", &self.unique_group).finish()
    }
}
impl ::core::default::Default for LANA_ENUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LANA_ENUM {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.lana == other.lana
    }
}
impl ::core::cmp::Eq for LANA_ENUM {}
impl ::core::fmt::Debug for LANA_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LANA_ENUM").field("length", &self.length).field("lana", &self.lana).finish()
    }
}
impl ::core::default::Default for NAME_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NAME_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.name_num == other.name_num && self.name_flags == other.name_flags
    }
}
impl ::core::cmp::Eq for NAME_BUFFER {}
impl ::core::fmt::Debug for NAME_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NAME_BUFFER").field("name", &self.name).field("name_num", &self.name_num).field("name_flags", &self.name_flags).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NCB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NCB {
    fn eq(&self, other: &Self) -> bool {
        self.ncb_command == other.ncb_command && self.ncb_retcode == other.ncb_retcode && self.ncb_lsn == other.ncb_lsn && self.ncb_num == other.ncb_num && self.ncb_buffer == other.ncb_buffer && self.ncb_length == other.ncb_length && self.ncb_callname == other.ncb_callname && self.ncb_name == other.ncb_name && self.ncb_rto == other.ncb_rto && self.ncb_sto == other.ncb_sto && self.ncb_post == other.ncb_post && self.ncb_lana_num == other.ncb_lana_num && self.ncb_cmd_cplt == other.ncb_cmd_cplt && self.ncb_reserve == other.ncb_reserve && self.ncb_event == other.ncb_event
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NCB {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NCB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NCB")
            .field("ncb_command", &self.ncb_command)
            .field("ncb_retcode", &self.ncb_retcode)
            .field("ncb_lsn", &self.ncb_lsn)
            .field("ncb_num", &self.ncb_num)
            .field("ncb_buffer", &self.ncb_buffer)
            .field("ncb_length", &self.ncb_length)
            .field("ncb_callname", &self.ncb_callname)
            .field("ncb_name", &self.ncb_name)
            .field("ncb_rto", &self.ncb_rto)
            .field("ncb_sto", &self.ncb_sto)
            .field("ncb_post", &self.ncb_post)
            .field("ncb_lana_num", &self.ncb_lana_num)
            .field("ncb_cmd_cplt", &self.ncb_cmd_cplt)
            .field("ncb_reserve", &self.ncb_reserve)
            .field("ncb_event", &self.ncb_event)
            .finish()
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NCB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NCB {
    fn eq(&self, other: &Self) -> bool {
        self.ncb_command == other.ncb_command && self.ncb_retcode == other.ncb_retcode && self.ncb_lsn == other.ncb_lsn && self.ncb_num == other.ncb_num && self.ncb_buffer == other.ncb_buffer && self.ncb_length == other.ncb_length && self.ncb_callname == other.ncb_callname && self.ncb_name == other.ncb_name && self.ncb_rto == other.ncb_rto && self.ncb_sto == other.ncb_sto && self.ncb_post == other.ncb_post && self.ncb_lana_num == other.ncb_lana_num && self.ncb_cmd_cplt == other.ncb_cmd_cplt && self.ncb_reserve == other.ncb_reserve && self.ncb_event == other.ncb_event
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NCB {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NCB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NCB")
            .field("ncb_command", &self.ncb_command)
            .field("ncb_retcode", &self.ncb_retcode)
            .field("ncb_lsn", &self.ncb_lsn)
            .field("ncb_num", &self.ncb_num)
            .field("ncb_buffer", &self.ncb_buffer)
            .field("ncb_length", &self.ncb_length)
            .field("ncb_callname", &self.ncb_callname)
            .field("ncb_name", &self.ncb_name)
            .field("ncb_rto", &self.ncb_rto)
            .field("ncb_sto", &self.ncb_sto)
            .field("ncb_post", &self.ncb_post)
            .field("ncb_lana_num", &self.ncb_lana_num)
            .field("ncb_cmd_cplt", &self.ncb_cmd_cplt)
            .field("ncb_reserve", &self.ncb_reserve)
            .field("ncb_event", &self.ncb_event)
            .finish()
    }
}
impl ::core::default::Default for SESSION_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SESSION_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.lsn == other.lsn && self.state == other.state && self.local_name == other.local_name && self.remote_name == other.remote_name && self.rcvs_outstanding == other.rcvs_outstanding && self.sends_outstanding == other.sends_outstanding
    }
}
impl ::core::cmp::Eq for SESSION_BUFFER {}
impl ::core::fmt::Debug for SESSION_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SESSION_BUFFER").field("lsn", &self.lsn).field("state", &self.state).field("local_name", &self.local_name).field("remote_name", &self.remote_name).field("rcvs_outstanding", &self.rcvs_outstanding).field("sends_outstanding", &self.sends_outstanding).finish()
    }
}
impl ::core::default::Default for SESSION_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SESSION_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.sess_name == other.sess_name && self.num_sess == other.num_sess && self.rcv_dg_outstanding == other.rcv_dg_outstanding && self.rcv_any_outstanding == other.rcv_any_outstanding
    }
}
impl ::core::cmp::Eq for SESSION_HEADER {}
impl ::core::fmt::Debug for SESSION_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SESSION_HEADER").field("sess_name", &self.sess_name).field("num_sess", &self.num_sess).field("rcv_dg_outstanding", &self.rcv_dg_outstanding).field("rcv_any_outstanding", &self.rcv_any_outstanding).finish()
    }
}
