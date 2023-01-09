impl ::core::default::Default for ACCESS_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ACCESS_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.acc0_resource_name == other.acc0_resource_name
    }
}
impl ::core::cmp::Eq for ACCESS_INFO_0 {}
impl ::core::fmt::Debug for ACCESS_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACCESS_INFO_0").field("acc0_resource_name", &self.acc0_resource_name).finish()
    }
}
impl ::core::default::Default for ACCESS_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ACCESS_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.acc1_resource_name == other.acc1_resource_name && self.acc1_attr == other.acc1_attr && self.acc1_count == other.acc1_count
    }
}
impl ::core::cmp::Eq for ACCESS_INFO_1 {}
impl ::core::fmt::Debug for ACCESS_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACCESS_INFO_1").field("acc1_resource_name", &self.acc1_resource_name).field("acc1_attr", &self.acc1_attr).field("acc1_count", &self.acc1_count).finish()
    }
}
impl ::core::default::Default for ACCESS_INFO_1002 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ACCESS_INFO_1002 {
    fn eq(&self, other: &Self) -> bool {
        self.acc1002_attr == other.acc1002_attr
    }
}
impl ::core::cmp::Eq for ACCESS_INFO_1002 {}
impl ::core::fmt::Debug for ACCESS_INFO_1002 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACCESS_INFO_1002").field("acc1002_attr", &self.acc1002_attr).finish()
    }
}
impl ::core::default::Default for ACCESS_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ACCESS_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.acl_ugname == other.acl_ugname && self.acl_access == other.acl_access
    }
}
impl ::core::cmp::Eq for ACCESS_LIST {}
impl ::core::fmt::Debug for ACCESS_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACCESS_LIST").field("acl_ugname", &self.acl_ugname).field("acl_access", &self.acl_access).finish()
    }
}
impl ::core::default::Default for ADMIN_OTHER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ADMIN_OTHER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.alrtad_errcode == other.alrtad_errcode && self.alrtad_numstrings == other.alrtad_numstrings
    }
}
impl ::core::cmp::Eq for ADMIN_OTHER_INFO {}
impl ::core::fmt::Debug for ADMIN_OTHER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADMIN_OTHER_INFO").field("alrtad_errcode", &self.alrtad_errcode).field("alrtad_numstrings", &self.alrtad_numstrings).finish()
    }
}
impl ::core::default::Default for AE_ACCLIM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AE_ACCLIM {
    fn eq(&self, other: &Self) -> bool {
        self.ae_al_compname == other.ae_al_compname && self.ae_al_username == other.ae_al_username && self.ae_al_resname == other.ae_al_resname && self.ae_al_limit == other.ae_al_limit
    }
}
impl ::core::cmp::Eq for AE_ACCLIM {}
impl ::core::fmt::Debug for AE_ACCLIM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_ACCLIM").field("ae_al_compname", &self.ae_al_compname).field("ae_al_username", &self.ae_al_username).field("ae_al_resname", &self.ae_al_resname).field("ae_al_limit", &self.ae_al_limit).finish()
    }
}
impl ::core::default::Default for AE_ACLMOD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AE_ACLMOD {
    fn eq(&self, other: &Self) -> bool {
        self.ae_am_compname == other.ae_am_compname && self.ae_am_username == other.ae_am_username && self.ae_am_resname == other.ae_am_resname && self.ae_am_action == other.ae_am_action && self.ae_am_datalen == other.ae_am_datalen
    }
}
impl ::core::cmp::Eq for AE_ACLMOD {}
impl ::core::fmt::Debug for AE_ACLMOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_ACLMOD").field("ae_am_compname", &self.ae_am_compname).field("ae_am_username", &self.ae_am_username).field("ae_am_resname", &self.ae_am_resname).field("ae_am_action", &self.ae_am_action).field("ae_am_datalen", &self.ae_am_datalen).finish()
    }
}
impl ::core::default::Default for AE_CLOSEFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AE_CLOSEFILE {
    fn eq(&self, other: &Self) -> bool {
        self.ae_cf_compname == other.ae_cf_compname && self.ae_cf_username == other.ae_cf_username && self.ae_cf_resname == other.ae_cf_resname && self.ae_cf_fileid == other.ae_cf_fileid && self.ae_cf_duration == other.ae_cf_duration && self.ae_cf_reason == other.ae_cf_reason
    }
}
impl ::core::cmp::Eq for AE_CLOSEFILE {}
impl ::core::fmt::Debug for AE_CLOSEFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_CLOSEFILE").field("ae_cf_compname", &self.ae_cf_compname).field("ae_cf_username", &self.ae_cf_username).field("ae_cf_resname", &self.ae_cf_resname).field("ae_cf_fileid", &self.ae_cf_fileid).field("ae_cf_duration", &self.ae_cf_duration).field("ae_cf_reason", &self.ae_cf_reason).finish()
    }
}
impl ::core::default::Default for AE_CONNREJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AE_CONNREJ {
    fn eq(&self, other: &Self) -> bool {
        self.ae_cr_compname == other.ae_cr_compname && self.ae_cr_username == other.ae_cr_username && self.ae_cr_netname == other.ae_cr_netname && self.ae_cr_reason == other.ae_cr_reason
    }
}
impl ::core::cmp::Eq for AE_CONNREJ {}
impl ::core::fmt::Debug for AE_CONNREJ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_CONNREJ").field("ae_cr_compname", &self.ae_cr_compname).field("ae_cr_username", &self.ae_cr_username).field("ae_cr_netname", &self.ae_cr_netname).field("ae_cr_reason", &self.ae_cr_reason).finish()
    }
}
impl ::core::default::Default for AE_CONNSTART {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AE_CONNSTART {
    fn eq(&self, other: &Self) -> bool {
        self.ae_ct_compname == other.ae_ct_compname && self.ae_ct_username == other.ae_ct_username && self.ae_ct_netname == other.ae_ct_netname && self.ae_ct_connid == other.ae_ct_connid
    }
}
impl ::core::cmp::Eq for AE_CONNSTART {}
impl ::core::fmt::Debug for AE_CONNSTART {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_CONNSTART").field("ae_ct_compname", &self.ae_ct_compname).field("ae_ct_username", &self.ae_ct_username).field("ae_ct_netname", &self.ae_ct_netname).field("ae_ct_connid", &self.ae_ct_connid).finish()
    }
}
impl ::core::default::Default for AE_CONNSTOP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AE_CONNSTOP {
    fn eq(&self, other: &Self) -> bool {
        self.ae_cp_compname == other.ae_cp_compname && self.ae_cp_username == other.ae_cp_username && self.ae_cp_netname == other.ae_cp_netname && self.ae_cp_connid == other.ae_cp_connid && self.ae_cp_reason == other.ae_cp_reason
    }
}
impl ::core::cmp::Eq for AE_CONNSTOP {}
impl ::core::fmt::Debug for AE_CONNSTOP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_CONNSTOP").field("ae_cp_compname", &self.ae_cp_compname).field("ae_cp_username", &self.ae_cp_username).field("ae_cp_netname", &self.ae_cp_netname).field("ae_cp_connid", &self.ae_cp_connid).field("ae_cp_reason", &self.ae_cp_reason).finish()
    }
}
impl ::core::default::Default for AE_GENERIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AE_GENERIC {
    fn eq(&self, other: &Self) -> bool {
        self.ae_ge_msgfile == other.ae_ge_msgfile && self.ae_ge_msgnum == other.ae_ge_msgnum && self.ae_ge_params == other.ae_ge_params && self.ae_ge_param1 == other.ae_ge_param1 && self.ae_ge_param2 == other.ae_ge_param2 && self.ae_ge_param3 == other.ae_ge_param3 && self.ae_ge_param4 == other.ae_ge_param4 && self.ae_ge_param5 == other.ae_ge_param5 && self.ae_ge_param6 == other.ae_ge_param6 && self.ae_ge_param7 == other.ae_ge_param7 && self.ae_ge_param8 == other.ae_ge_param8 && self.ae_ge_param9 == other.ae_ge_param9
    }
}
impl ::core::cmp::Eq for AE_GENERIC {}
impl ::core::fmt::Debug for AE_GENERIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_GENERIC")
            .field("ae_ge_msgfile", &self.ae_ge_msgfile)
            .field("ae_ge_msgnum", &self.ae_ge_msgnum)
            .field("ae_ge_params", &self.ae_ge_params)
            .field("ae_ge_param1", &self.ae_ge_param1)
            .field("ae_ge_param2", &self.ae_ge_param2)
            .field("ae_ge_param3", &self.ae_ge_param3)
            .field("ae_ge_param4", &self.ae_ge_param4)
            .field("ae_ge_param5", &self.ae_ge_param5)
            .field("ae_ge_param6", &self.ae_ge_param6)
            .field("ae_ge_param7", &self.ae_ge_param7)
            .field("ae_ge_param8", &self.ae_ge_param8)
            .field("ae_ge_param9", &self.ae_ge_param9)
            .finish()
    }
}
impl ::core::default::Default for AE_LOCKOUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AE_LOCKOUT {
    fn eq(&self, other: &Self) -> bool {
        self.ae_lk_compname == other.ae_lk_compname && self.ae_lk_username == other.ae_lk_username && self.ae_lk_action == other.ae_lk_action && self.ae_lk_bad_pw_count == other.ae_lk_bad_pw_count
    }
}
impl ::core::cmp::Eq for AE_LOCKOUT {}
impl ::core::fmt::Debug for AE_LOCKOUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_LOCKOUT").field("ae_lk_compname", &self.ae_lk_compname).field("ae_lk_username", &self.ae_lk_username).field("ae_lk_action", &self.ae_lk_action).field("ae_lk_bad_pw_count", &self.ae_lk_bad_pw_count).finish()
    }
}
impl ::core::default::Default for AE_NETLOGOFF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AE_NETLOGOFF {
    fn eq(&self, other: &Self) -> bool {
        self.ae_nf_compname == other.ae_nf_compname && self.ae_nf_username == other.ae_nf_username && self.ae_nf_reserved1 == other.ae_nf_reserved1 && self.ae_nf_reserved2 == other.ae_nf_reserved2
    }
}
impl ::core::cmp::Eq for AE_NETLOGOFF {}
impl ::core::fmt::Debug for AE_NETLOGOFF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_NETLOGOFF").field("ae_nf_compname", &self.ae_nf_compname).field("ae_nf_username", &self.ae_nf_username).field("ae_nf_reserved1", &self.ae_nf_reserved1).field("ae_nf_reserved2", &self.ae_nf_reserved2).finish()
    }
}
impl ::core::default::Default for AE_NETLOGON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AE_NETLOGON {
    fn eq(&self, other: &Self) -> bool {
        self.ae_no_compname == other.ae_no_compname && self.ae_no_username == other.ae_no_username && self.ae_no_privilege == other.ae_no_privilege && self.ae_no_authflags == other.ae_no_authflags
    }
}
impl ::core::cmp::Eq for AE_NETLOGON {}
impl ::core::fmt::Debug for AE_NETLOGON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_NETLOGON").field("ae_no_compname", &self.ae_no_compname).field("ae_no_username", &self.ae_no_username).field("ae_no_privilege", &self.ae_no_privilege).field("ae_no_authflags", &self.ae_no_authflags).finish()
    }
}
impl ::core::default::Default for AE_RESACCESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AE_RESACCESS {
    fn eq(&self, other: &Self) -> bool {
        self.ae_ra_compname == other.ae_ra_compname && self.ae_ra_username == other.ae_ra_username && self.ae_ra_resname == other.ae_ra_resname && self.ae_ra_operation == other.ae_ra_operation && self.ae_ra_returncode == other.ae_ra_returncode && self.ae_ra_restype == other.ae_ra_restype && self.ae_ra_fileid == other.ae_ra_fileid
    }
}
impl ::core::cmp::Eq for AE_RESACCESS {}
impl ::core::fmt::Debug for AE_RESACCESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_RESACCESS").field("ae_ra_compname", &self.ae_ra_compname).field("ae_ra_username", &self.ae_ra_username).field("ae_ra_resname", &self.ae_ra_resname).field("ae_ra_operation", &self.ae_ra_operation).field("ae_ra_returncode", &self.ae_ra_returncode).field("ae_ra_restype", &self.ae_ra_restype).field("ae_ra_fileid", &self.ae_ra_fileid).finish()
    }
}
impl ::core::default::Default for AE_RESACCESSREJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AE_RESACCESSREJ {
    fn eq(&self, other: &Self) -> bool {
        self.ae_rr_compname == other.ae_rr_compname && self.ae_rr_username == other.ae_rr_username && self.ae_rr_resname == other.ae_rr_resname && self.ae_rr_operation == other.ae_rr_operation
    }
}
impl ::core::cmp::Eq for AE_RESACCESSREJ {}
impl ::core::fmt::Debug for AE_RESACCESSREJ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_RESACCESSREJ").field("ae_rr_compname", &self.ae_rr_compname).field("ae_rr_username", &self.ae_rr_username).field("ae_rr_resname", &self.ae_rr_resname).field("ae_rr_operation", &self.ae_rr_operation).finish()
    }
}
impl ::core::default::Default for AE_SERVICESTAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AE_SERVICESTAT {
    fn eq(&self, other: &Self) -> bool {
        self.ae_ss_compname == other.ae_ss_compname && self.ae_ss_username == other.ae_ss_username && self.ae_ss_svcname == other.ae_ss_svcname && self.ae_ss_status == other.ae_ss_status && self.ae_ss_code == other.ae_ss_code && self.ae_ss_text == other.ae_ss_text && self.ae_ss_returnval == other.ae_ss_returnval
    }
}
impl ::core::cmp::Eq for AE_SERVICESTAT {}
impl ::core::fmt::Debug for AE_SERVICESTAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_SERVICESTAT").field("ae_ss_compname", &self.ae_ss_compname).field("ae_ss_username", &self.ae_ss_username).field("ae_ss_svcname", &self.ae_ss_svcname).field("ae_ss_status", &self.ae_ss_status).field("ae_ss_code", &self.ae_ss_code).field("ae_ss_text", &self.ae_ss_text).field("ae_ss_returnval", &self.ae_ss_returnval).finish()
    }
}
impl ::core::default::Default for AE_SESSLOGOFF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AE_SESSLOGOFF {
    fn eq(&self, other: &Self) -> bool {
        self.ae_sf_compname == other.ae_sf_compname && self.ae_sf_username == other.ae_sf_username && self.ae_sf_reason == other.ae_sf_reason
    }
}
impl ::core::cmp::Eq for AE_SESSLOGOFF {}
impl ::core::fmt::Debug for AE_SESSLOGOFF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_SESSLOGOFF").field("ae_sf_compname", &self.ae_sf_compname).field("ae_sf_username", &self.ae_sf_username).field("ae_sf_reason", &self.ae_sf_reason).finish()
    }
}
impl ::core::default::Default for AE_SESSLOGON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AE_SESSLOGON {
    fn eq(&self, other: &Self) -> bool {
        self.ae_so_compname == other.ae_so_compname && self.ae_so_username == other.ae_so_username && self.ae_so_privilege == other.ae_so_privilege
    }
}
impl ::core::cmp::Eq for AE_SESSLOGON {}
impl ::core::fmt::Debug for AE_SESSLOGON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_SESSLOGON").field("ae_so_compname", &self.ae_so_compname).field("ae_so_username", &self.ae_so_username).field("ae_so_privilege", &self.ae_so_privilege).finish()
    }
}
impl ::core::default::Default for AE_SESSPWERR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AE_SESSPWERR {
    fn eq(&self, other: &Self) -> bool {
        self.ae_sp_compname == other.ae_sp_compname && self.ae_sp_username == other.ae_sp_username
    }
}
impl ::core::cmp::Eq for AE_SESSPWERR {}
impl ::core::fmt::Debug for AE_SESSPWERR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_SESSPWERR").field("ae_sp_compname", &self.ae_sp_compname).field("ae_sp_username", &self.ae_sp_username).finish()
    }
}
impl ::core::default::Default for AE_SRVSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AE_SRVSTATUS {
    fn eq(&self, other: &Self) -> bool {
        self.ae_sv_status == other.ae_sv_status
    }
}
impl ::core::cmp::Eq for AE_SRVSTATUS {}
impl ::core::fmt::Debug for AE_SRVSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_SRVSTATUS").field("ae_sv_status", &self.ae_sv_status).finish()
    }
}
impl ::core::default::Default for AE_UASMOD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AE_UASMOD {
    fn eq(&self, other: &Self) -> bool {
        self.ae_um_compname == other.ae_um_compname && self.ae_um_username == other.ae_um_username && self.ae_um_resname == other.ae_um_resname && self.ae_um_rectype == other.ae_um_rectype && self.ae_um_action == other.ae_um_action && self.ae_um_datalen == other.ae_um_datalen
    }
}
impl ::core::cmp::Eq for AE_UASMOD {}
impl ::core::fmt::Debug for AE_UASMOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_UASMOD").field("ae_um_compname", &self.ae_um_compname).field("ae_um_username", &self.ae_um_username).field("ae_um_resname", &self.ae_um_resname).field("ae_um_rectype", &self.ae_um_rectype).field("ae_um_action", &self.ae_um_action).field("ae_um_datalen", &self.ae_um_datalen).finish()
    }
}
impl ::core::default::Default for AF_OP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AF_OP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AF_OP").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for AF_OP {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AF_OP {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AF_OP {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AF_OP {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AF_OP {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for AT_ENUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AT_ENUM {
    fn eq(&self, other: &Self) -> bool {
        self.JobId == other.JobId && self.JobTime == other.JobTime && self.DaysOfMonth == other.DaysOfMonth && self.DaysOfWeek == other.DaysOfWeek && self.Flags == other.Flags && self.Command == other.Command
    }
}
impl ::core::cmp::Eq for AT_ENUM {}
impl ::core::fmt::Debug for AT_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AT_ENUM").field("JobId", &self.JobId).field("JobTime", &self.JobTime).field("DaysOfMonth", &self.DaysOfMonth).field("DaysOfWeek", &self.DaysOfWeek).field("Flags", &self.Flags).field("Command", &self.Command).finish()
    }
}
impl ::core::default::Default for AT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.JobTime == other.JobTime && self.DaysOfMonth == other.DaysOfMonth && self.DaysOfWeek == other.DaysOfWeek && self.Flags == other.Flags && self.Command == other.Command
    }
}
impl ::core::cmp::Eq for AT_INFO {}
impl ::core::fmt::Debug for AT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AT_INFO").field("JobTime", &self.JobTime).field("DaysOfMonth", &self.DaysOfMonth).field("DaysOfWeek", &self.DaysOfWeek).field("Flags", &self.Flags).field("Command", &self.Command).finish()
    }
}
impl ::core::default::Default for AUDIT_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AUDIT_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.ae_len == other.ae_len && self.ae_reserved == other.ae_reserved && self.ae_time == other.ae_time && self.ae_type == other.ae_type && self.ae_data_offset == other.ae_data_offset && self.ae_data_size == other.ae_data_size
    }
}
impl ::core::cmp::Eq for AUDIT_ENTRY {}
impl ::core::fmt::Debug for AUDIT_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIT_ENTRY").field("ae_len", &self.ae_len).field("ae_reserved", &self.ae_reserved).field("ae_time", &self.ae_time).field("ae_type", &self.ae_type).field("ae_data_offset", &self.ae_data_offset).field("ae_data_size", &self.ae_data_size).finish()
    }
}
impl ::core::default::Default for BIND_FLAGS1 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BIND_FLAGS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BIND_FLAGS1").field(&self.0).finish()
    }
}
impl ::core::default::Default for COMPONENT_CHARACTERISTICS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COMPONENT_CHARACTERISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMPONENT_CHARACTERISTICS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CONFIG_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CONFIG_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.cfgi0_key == other.cfgi0_key && self.cfgi0_data == other.cfgi0_data
    }
}
impl ::core::cmp::Eq for CONFIG_INFO_0 {}
impl ::core::fmt::Debug for CONFIG_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONFIG_INFO_0").field("cfgi0_key", &self.cfgi0_key).field("cfgi0_data", &self.cfgi0_data).finish()
    }
}
impl ::core::default::Default for DEFAULT_PAGES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEFAULT_PAGES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEFAULT_PAGES").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for DSREG_JOIN_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for DSREG_JOIN_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.joinType == other.joinType && self.pJoinCertificate == other.pJoinCertificate && self.pszDeviceId == other.pszDeviceId && self.pszIdpDomain == other.pszIdpDomain && self.pszTenantId == other.pszTenantId && self.pszJoinUserEmail == other.pszJoinUserEmail && self.pszTenantDisplayName == other.pszTenantDisplayName && self.pszMdmEnrollmentUrl == other.pszMdmEnrollmentUrl && self.pszMdmTermsOfUseUrl == other.pszMdmTermsOfUseUrl && self.pszMdmComplianceUrl == other.pszMdmComplianceUrl && self.pszUserSettingSyncUrl == other.pszUserSettingSyncUrl && self.pUserInfo == other.pUserInfo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for DSREG_JOIN_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for DSREG_JOIN_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSREG_JOIN_INFO")
            .field("joinType", &self.joinType)
            .field("pJoinCertificate", &self.pJoinCertificate)
            .field("pszDeviceId", &self.pszDeviceId)
            .field("pszIdpDomain", &self.pszIdpDomain)
            .field("pszTenantId", &self.pszTenantId)
            .field("pszJoinUserEmail", &self.pszJoinUserEmail)
            .field("pszTenantDisplayName", &self.pszTenantDisplayName)
            .field("pszMdmEnrollmentUrl", &self.pszMdmEnrollmentUrl)
            .field("pszMdmTermsOfUseUrl", &self.pszMdmTermsOfUseUrl)
            .field("pszMdmComplianceUrl", &self.pszMdmComplianceUrl)
            .field("pszUserSettingSyncUrl", &self.pszUserSettingSyncUrl)
            .field("pUserInfo", &self.pUserInfo)
            .finish()
    }
}
impl ::core::default::Default for DSREG_JOIN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DSREG_JOIN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DSREG_JOIN_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DSREG_USER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DSREG_USER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszUserEmail == other.pszUserEmail && self.pszUserKeyId == other.pszUserKeyId && self.pszUserKeyName == other.pszUserKeyName
    }
}
impl ::core::cmp::Eq for DSREG_USER_INFO {}
impl ::core::fmt::Debug for DSREG_USER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSREG_USER_INFO").field("pszUserEmail", &self.pszUserEmail).field("pszUserKeyId", &self.pszUserKeyId).field("pszUserKeyName", &self.pszUserKeyName).finish()
    }
}
impl ::core::default::Default for ENUM_BINDING_PATHS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ENUM_BINDING_PATHS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENUM_BINDING_PATHS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for ERRLOG_OTHER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ERRLOG_OTHER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.alrter_errcode == other.alrter_errcode && self.alrter_offset == other.alrter_offset
    }
}
impl ::core::cmp::Eq for ERRLOG_OTHER_INFO {}
impl ::core::fmt::Debug for ERRLOG_OTHER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ERRLOG_OTHER_INFO").field("alrter_errcode", &self.alrter_errcode).field("alrter_offset", &self.alrter_offset).finish()
    }
}
impl ::core::default::Default for ERROR_LOG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ERROR_LOG {
    fn eq(&self, other: &Self) -> bool {
        self.el_len == other.el_len && self.el_reserved == other.el_reserved && self.el_time == other.el_time && self.el_error == other.el_error && self.el_name == other.el_name && self.el_text == other.el_text && self.el_data == other.el_data && self.el_data_size == other.el_data_size && self.el_nstrings == other.el_nstrings
    }
}
impl ::core::cmp::Eq for ERROR_LOG {}
impl ::core::fmt::Debug for ERROR_LOG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ERROR_LOG").field("el_len", &self.el_len).field("el_reserved", &self.el_reserved).field("el_time", &self.el_time).field("el_error", &self.el_error).field("el_name", &self.el_name).field("el_text", &self.el_text).field("el_data", &self.el_data).field("el_data_size", &self.el_data_size).field("el_nstrings", &self.el_nstrings).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FLAT_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FLAT_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.MaximumLength == other.MaximumLength && self.Length == other.Length && self.Buffer == other.Buffer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FLAT_STRING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FLAT_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLAT_STRING").field("MaximumLength", &self.MaximumLength).field("Length", &self.Length).field("Buffer", &self.Buffer).finish()
    }
}
impl ::core::default::Default for FORCE_LEVEL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FORCE_LEVEL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FORCE_LEVEL_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for GROUP_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GROUP_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.grpi0_name == other.grpi0_name
    }
}
impl ::core::cmp::Eq for GROUP_INFO_0 {}
impl ::core::fmt::Debug for GROUP_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_INFO_0").field("grpi0_name", &self.grpi0_name).finish()
    }
}
impl ::core::default::Default for GROUP_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GROUP_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.grpi1_name == other.grpi1_name && self.grpi1_comment == other.grpi1_comment
    }
}
impl ::core::cmp::Eq for GROUP_INFO_1 {}
impl ::core::fmt::Debug for GROUP_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_INFO_1").field("grpi1_name", &self.grpi1_name).field("grpi1_comment", &self.grpi1_comment).finish()
    }
}
impl ::core::default::Default for GROUP_INFO_1002 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GROUP_INFO_1002 {
    fn eq(&self, other: &Self) -> bool {
        self.grpi1002_comment == other.grpi1002_comment
    }
}
impl ::core::cmp::Eq for GROUP_INFO_1002 {}
impl ::core::fmt::Debug for GROUP_INFO_1002 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_INFO_1002").field("grpi1002_comment", &self.grpi1002_comment).finish()
    }
}
impl ::core::default::Default for GROUP_INFO_1005 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GROUP_INFO_1005 {
    fn eq(&self, other: &Self) -> bool {
        self.grpi1005_attributes == other.grpi1005_attributes
    }
}
impl ::core::cmp::Eq for GROUP_INFO_1005 {}
impl ::core::fmt::Debug for GROUP_INFO_1005 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_INFO_1005").field("grpi1005_attributes", &self.grpi1005_attributes).finish()
    }
}
impl ::core::default::Default for GROUP_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GROUP_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.grpi2_name == other.grpi2_name && self.grpi2_comment == other.grpi2_comment && self.grpi2_group_id == other.grpi2_group_id && self.grpi2_attributes == other.grpi2_attributes
    }
}
impl ::core::cmp::Eq for GROUP_INFO_2 {}
impl ::core::fmt::Debug for GROUP_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_INFO_2").field("grpi2_name", &self.grpi2_name).field("grpi2_comment", &self.grpi2_comment).field("grpi2_group_id", &self.grpi2_group_id).field("grpi2_attributes", &self.grpi2_attributes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GROUP_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GROUP_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        self.grpi3_name == other.grpi3_name && self.grpi3_comment == other.grpi3_comment && self.grpi3_group_sid == other.grpi3_group_sid && self.grpi3_attributes == other.grpi3_attributes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GROUP_INFO_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GROUP_INFO_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_INFO_3").field("grpi3_name", &self.grpi3_name).field("grpi3_comment", &self.grpi3_comment).field("grpi3_group_sid", &self.grpi3_group_sid).field("grpi3_attributes", &self.grpi3_attributes).finish()
    }
}
impl ::core::default::Default for GROUP_USERS_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GROUP_USERS_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.grui0_name == other.grui0_name
    }
}
impl ::core::cmp::Eq for GROUP_USERS_INFO_0 {}
impl ::core::fmt::Debug for GROUP_USERS_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_USERS_INFO_0").field("grui0_name", &self.grui0_name).finish()
    }
}
impl ::core::default::Default for GROUP_USERS_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GROUP_USERS_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.grui1_name == other.grui1_name && self.grui1_attributes == other.grui1_attributes
    }
}
impl ::core::cmp::Eq for GROUP_USERS_INFO_1 {}
impl ::core::fmt::Debug for GROUP_USERS_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_USERS_INFO_1").field("grui1_name", &self.grui1_name).field("grui1_attributes", &self.grui1_attributes).finish()
    }
}
impl ::core::default::Default for HARDWARE_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HARDWARE_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.Address == other.Address
    }
}
impl ::core::cmp::Eq for HARDWARE_ADDRESS {}
impl ::core::fmt::Debug for HARDWARE_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HARDWARE_ADDRESS").field("Address", &self.Address).finish()
    }
}
impl ::core::default::Default for HLOG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HLOG {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.last_flags == other.last_flags && self.offset == other.offset && self.rec_offset == other.rec_offset
    }
}
impl ::core::cmp::Eq for HLOG {}
impl ::core::fmt::Debug for HLOG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HLOG").field("time", &self.time).field("last_flags", &self.last_flags).field("offset", &self.offset).field("rec_offset", &self.rec_offset).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumNetCfgBindingInterface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumNetCfgBindingInterface {}
impl ::core::fmt::Debug for IEnumNetCfgBindingInterface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumNetCfgBindingInterface").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumNetCfgBindingPath {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumNetCfgBindingPath {}
impl ::core::fmt::Debug for IEnumNetCfgBindingPath {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumNetCfgBindingPath").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumNetCfgComponent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumNetCfgComponent {}
impl ::core::fmt::Debug for IEnumNetCfgComponent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumNetCfgComponent").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INetCfg {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfg {}
impl ::core::fmt::Debug for INetCfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfg").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INetCfgBindingInterface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgBindingInterface {}
impl ::core::fmt::Debug for INetCfgBindingInterface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgBindingInterface").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INetCfgBindingPath {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgBindingPath {}
impl ::core::fmt::Debug for INetCfgBindingPath {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgBindingPath").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INetCfgClass {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgClass {}
impl ::core::fmt::Debug for INetCfgClass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgClass").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INetCfgClassSetup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgClassSetup {}
impl ::core::fmt::Debug for INetCfgClassSetup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgClassSetup").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INetCfgClassSetup2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgClassSetup2 {}
impl ::core::fmt::Debug for INetCfgClassSetup2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgClassSetup2").field(&self.0).finish()
    }
}
impl INetCfgClassSetup2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SelectAndInstall<P0>(&self, hwndparent: P0, pobotoken: ::core::option::Option<*const OBO_TOKEN>, ppnccitem: ::core::option::Option<*mut ::core::option::Option<INetCfgComponent>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.SelectAndInstall)(::windows::core::Vtable::as_raw(self), hwndparent.into(), ::core::mem::transmute(pobotoken.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppnccitem.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Install<P0, P1, P2>(&self, pszwinfid: P0, pobotoken: ::core::option::Option<*const OBO_TOKEN>, dwsetupflags: u32, dwupgradefrombuildno: u32, pszwanswerfile: P1, pszwanswersections: P2, ppnccitem: ::core::option::Option<*mut ::core::option::Option<INetCfgComponent>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Install)(::windows::core::Vtable::as_raw(self), pszwinfid.into().abi(), ::core::mem::transmute(pobotoken.unwrap_or(::std::ptr::null())), dwsetupflags, dwupgradefrombuildno, pszwanswerfile.into().abi(), pszwanswersections.into().abi(), ::core::mem::transmute(ppnccitem.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeInstall<P0>(&self, pcomponent: P0, pobotoken: ::core::option::Option<*const OBO_TOKEN>, pmszwrefs: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<INetCfgComponent>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DeInstall)(::windows::core::Vtable::as_raw(self), pcomponent.into().abi(), ::core::mem::transmute(pobotoken.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pmszwrefs.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
impl ::core::cmp::PartialEq for INetCfgComponent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgComponent {}
impl ::core::fmt::Debug for INetCfgComponent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgComponent").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INetCfgComponentBindings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgComponentBindings {}
impl ::core::fmt::Debug for INetCfgComponentBindings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgComponentBindings").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INetCfgComponentControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgComponentControl {}
impl ::core::fmt::Debug for INetCfgComponentControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgComponentControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INetCfgComponentNotifyBinding {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgComponentNotifyBinding {}
impl ::core::fmt::Debug for INetCfgComponentNotifyBinding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgComponentNotifyBinding").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INetCfgComponentNotifyGlobal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgComponentNotifyGlobal {}
impl ::core::fmt::Debug for INetCfgComponentNotifyGlobal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgComponentNotifyGlobal").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INetCfgComponentPropertyUi {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgComponentPropertyUi {}
impl ::core::fmt::Debug for INetCfgComponentPropertyUi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgComponentPropertyUi").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INetCfgComponentSetup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgComponentSetup {}
impl ::core::fmt::Debug for INetCfgComponentSetup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgComponentSetup").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INetCfgComponentSysPrep {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgComponentSysPrep {}
impl ::core::fmt::Debug for INetCfgComponentSysPrep {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgComponentSysPrep").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INetCfgComponentUpperEdge {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgComponentUpperEdge {}
impl ::core::fmt::Debug for INetCfgComponentUpperEdge {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgComponentUpperEdge").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INetCfgLock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgLock {}
impl ::core::fmt::Debug for INetCfgLock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgLock").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INetCfgPnpReconfigCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgPnpReconfigCallback {}
impl ::core::fmt::Debug for INetCfgPnpReconfigCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgPnpReconfigCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INetCfgSysPrep {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetCfgSysPrep {}
impl ::core::fmt::Debug for INetCfgSysPrep {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetCfgSysPrep").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INetLanConnectionUiInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetLanConnectionUiInfo {}
impl ::core::fmt::Debug for INetLanConnectionUiInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetLanConnectionUiInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INetRasConnectionIpUiInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetRasConnectionIpUiInfo {}
impl ::core::fmt::Debug for INetRasConnectionIpUiInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetRasConnectionIpUiInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IProvisioningDomain {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProvisioningDomain {}
impl ::core::fmt::Debug for IProvisioningDomain {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProvisioningDomain").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IProvisioningProfileWireless {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProvisioningProfileWireless {}
impl ::core::fmt::Debug for IProvisioningProfileWireless {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProvisioningProfileWireless").field(&self.0).finish()
    }
}
impl ::core::default::Default for LOCALGROUP_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LOCALGROUP_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.lgrpi0_name == other.lgrpi0_name
    }
}
impl ::core::cmp::Eq for LOCALGROUP_INFO_0 {}
impl ::core::fmt::Debug for LOCALGROUP_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOCALGROUP_INFO_0").field("lgrpi0_name", &self.lgrpi0_name).finish()
    }
}
impl ::core::default::Default for LOCALGROUP_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LOCALGROUP_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.lgrpi1_name == other.lgrpi1_name && self.lgrpi1_comment == other.lgrpi1_comment
    }
}
impl ::core::cmp::Eq for LOCALGROUP_INFO_1 {}
impl ::core::fmt::Debug for LOCALGROUP_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOCALGROUP_INFO_1").field("lgrpi1_name", &self.lgrpi1_name).field("lgrpi1_comment", &self.lgrpi1_comment).finish()
    }
}
impl ::core::default::Default for LOCALGROUP_INFO_1002 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LOCALGROUP_INFO_1002 {
    fn eq(&self, other: &Self) -> bool {
        self.lgrpi1002_comment == other.lgrpi1002_comment
    }
}
impl ::core::cmp::Eq for LOCALGROUP_INFO_1002 {}
impl ::core::fmt::Debug for LOCALGROUP_INFO_1002 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOCALGROUP_INFO_1002").field("lgrpi1002_comment", &self.lgrpi1002_comment).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LOCALGROUP_MEMBERS_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LOCALGROUP_MEMBERS_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.lgrmi0_sid == other.lgrmi0_sid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LOCALGROUP_MEMBERS_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LOCALGROUP_MEMBERS_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOCALGROUP_MEMBERS_INFO_0").field("lgrmi0_sid", &self.lgrmi0_sid).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for LOCALGROUP_MEMBERS_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for LOCALGROUP_MEMBERS_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.lgrmi1_sid == other.lgrmi1_sid && self.lgrmi1_sidusage == other.lgrmi1_sidusage && self.lgrmi1_name == other.lgrmi1_name
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for LOCALGROUP_MEMBERS_INFO_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for LOCALGROUP_MEMBERS_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOCALGROUP_MEMBERS_INFO_1").field("lgrmi1_sid", &self.lgrmi1_sid).field("lgrmi1_sidusage", &self.lgrmi1_sidusage).field("lgrmi1_name", &self.lgrmi1_name).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for LOCALGROUP_MEMBERS_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for LOCALGROUP_MEMBERS_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.lgrmi2_sid == other.lgrmi2_sid && self.lgrmi2_sidusage == other.lgrmi2_sidusage && self.lgrmi2_domainandname == other.lgrmi2_domainandname
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for LOCALGROUP_MEMBERS_INFO_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for LOCALGROUP_MEMBERS_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOCALGROUP_MEMBERS_INFO_2").field("lgrmi2_sid", &self.lgrmi2_sid).field("lgrmi2_sidusage", &self.lgrmi2_sidusage).field("lgrmi2_domainandname", &self.lgrmi2_domainandname).finish()
    }
}
impl ::core::default::Default for LOCALGROUP_MEMBERS_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LOCALGROUP_MEMBERS_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        self.lgrmi3_domainandname == other.lgrmi3_domainandname
    }
}
impl ::core::cmp::Eq for LOCALGROUP_MEMBERS_INFO_3 {}
impl ::core::fmt::Debug for LOCALGROUP_MEMBERS_INFO_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOCALGROUP_MEMBERS_INFO_3").field("lgrmi3_domainandname", &self.lgrmi3_domainandname).finish()
    }
}
impl ::core::default::Default for LOCALGROUP_USERS_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LOCALGROUP_USERS_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.lgrui0_name == other.lgrui0_name
    }
}
impl ::core::cmp::Eq for LOCALGROUP_USERS_INFO_0 {}
impl ::core::fmt::Debug for LOCALGROUP_USERS_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOCALGROUP_USERS_INFO_0").field("lgrui0_name", &self.lgrui0_name).finish()
    }
}
impl ::core::default::Default for MPR_PROTOCOL_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MPR_PROTOCOL_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwProtocolId == other.dwProtocolId && self.wszProtocol == other.wszProtocol && self.wszDLLName == other.wszDLLName
    }
}
impl ::core::cmp::Eq for MPR_PROTOCOL_0 {}
impl ::core::fmt::Debug for MPR_PROTOCOL_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_PROTOCOL_0").field("dwProtocolId", &self.dwProtocolId).field("wszProtocol", &self.wszProtocol).field("wszDLLName", &self.wszDLLName).finish()
    }
}
impl ::core::default::Default for MSA_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MSA_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.State == other.State
    }
}
impl ::core::cmp::Eq for MSA_INFO_0 {}
impl ::core::fmt::Debug for MSA_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSA_INFO_0").field("State", &self.State).finish()
    }
}
impl ::core::default::Default for MSA_INFO_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MSA_INFO_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSA_INFO_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for MSA_INFO_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MSA_INFO_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSA_INFO_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MSG_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MSG_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.msgi0_name == other.msgi0_name
    }
}
impl ::core::cmp::Eq for MSG_INFO_0 {}
impl ::core::fmt::Debug for MSG_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSG_INFO_0").field("msgi0_name", &self.msgi0_name).finish()
    }
}
impl ::core::default::Default for MSG_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MSG_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.msgi1_name == other.msgi1_name && self.msgi1_forward_flag == other.msgi1_forward_flag && self.msgi1_forward == other.msgi1_forward
    }
}
impl ::core::cmp::Eq for MSG_INFO_1 {}
impl ::core::fmt::Debug for MSG_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSG_INFO_1").field("msgi1_name", &self.msgi1_name).field("msgi1_forward_flag", &self.msgi1_forward_flag).field("msgi1_forward", &self.msgi1_forward).finish()
    }
}
impl ::core::default::Default for NCPNP_RECONFIG_LAYER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NCPNP_RECONFIG_LAYER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NCPNP_RECONFIG_LAYER").field(&self.0).finish()
    }
}
impl ::core::default::Default for NCRP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NCRP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NCRP_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for NETLOGON_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NETLOGON_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.netlog1_flags == other.netlog1_flags && self.netlog1_pdc_connection_status == other.netlog1_pdc_connection_status
    }
}
impl ::core::cmp::Eq for NETLOGON_INFO_1 {}
impl ::core::fmt::Debug for NETLOGON_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETLOGON_INFO_1").field("netlog1_flags", &self.netlog1_flags).field("netlog1_pdc_connection_status", &self.netlog1_pdc_connection_status).finish()
    }
}
impl ::core::default::Default for NETLOGON_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NETLOGON_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.netlog2_flags == other.netlog2_flags && self.netlog2_pdc_connection_status == other.netlog2_pdc_connection_status && self.netlog2_trusted_dc_name == other.netlog2_trusted_dc_name && self.netlog2_tc_connection_status == other.netlog2_tc_connection_status
    }
}
impl ::core::cmp::Eq for NETLOGON_INFO_2 {}
impl ::core::fmt::Debug for NETLOGON_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETLOGON_INFO_2").field("netlog2_flags", &self.netlog2_flags).field("netlog2_pdc_connection_status", &self.netlog2_pdc_connection_status).field("netlog2_trusted_dc_name", &self.netlog2_trusted_dc_name).field("netlog2_tc_connection_status", &self.netlog2_tc_connection_status).finish()
    }
}
impl ::core::default::Default for NETLOGON_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NETLOGON_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        self.netlog3_flags == other.netlog3_flags && self.netlog3_logon_attempts == other.netlog3_logon_attempts && self.netlog3_reserved1 == other.netlog3_reserved1 && self.netlog3_reserved2 == other.netlog3_reserved2 && self.netlog3_reserved3 == other.netlog3_reserved3 && self.netlog3_reserved4 == other.netlog3_reserved4 && self.netlog3_reserved5 == other.netlog3_reserved5
    }
}
impl ::core::cmp::Eq for NETLOGON_INFO_3 {}
impl ::core::fmt::Debug for NETLOGON_INFO_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETLOGON_INFO_3").field("netlog3_flags", &self.netlog3_flags).field("netlog3_logon_attempts", &self.netlog3_logon_attempts).field("netlog3_reserved1", &self.netlog3_reserved1).field("netlog3_reserved2", &self.netlog3_reserved2).field("netlog3_reserved3", &self.netlog3_reserved3).field("netlog3_reserved4", &self.netlog3_reserved4).field("netlog3_reserved5", &self.netlog3_reserved5).finish()
    }
}
impl ::core::default::Default for NETLOGON_INFO_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NETLOGON_INFO_4 {
    fn eq(&self, other: &Self) -> bool {
        self.netlog4_trusted_dc_name == other.netlog4_trusted_dc_name && self.netlog4_trusted_domain_name == other.netlog4_trusted_domain_name
    }
}
impl ::core::cmp::Eq for NETLOGON_INFO_4 {}
impl ::core::fmt::Debug for NETLOGON_INFO_4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETLOGON_INFO_4").field("netlog4_trusted_dc_name", &self.netlog4_trusted_dc_name).field("netlog4_trusted_domain_name", &self.netlog4_trusted_domain_name).finish()
    }
}
impl ::core::default::Default for NETSETUP_JOIN_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NETSETUP_JOIN_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETSETUP_JOIN_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for NETSETUP_NAME_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NETSETUP_NAME_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETSETUP_NAME_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NETSETUP_PROVISION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NETSETUP_PROVISION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETSETUP_PROVISION").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for NETSETUP_PROVISION {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NETSETUP_PROVISION {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NETSETUP_PROVISION {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NETSETUP_PROVISION {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NETSETUP_PROVISION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for NETSETUP_PROVISIONING_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NETSETUP_PROVISIONING_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.lpDomain == other.lpDomain && self.lpHostName == other.lpHostName && self.lpMachineAccountOU == other.lpMachineAccountOU && self.lpDcName == other.lpDcName && self.dwProvisionOptions == other.dwProvisionOptions && self.aCertTemplateNames == other.aCertTemplateNames && self.cCertTemplateNames == other.cCertTemplateNames && self.aMachinePolicyNames == other.aMachinePolicyNames && self.cMachinePolicyNames == other.cMachinePolicyNames && self.aMachinePolicyPaths == other.aMachinePolicyPaths && self.cMachinePolicyPaths == other.cMachinePolicyPaths && self.lpNetbiosName == other.lpNetbiosName && self.lpSiteName == other.lpSiteName && self.lpPrimaryDNSDomain == other.lpPrimaryDNSDomain
    }
}
impl ::core::cmp::Eq for NETSETUP_PROVISIONING_PARAMS {}
impl ::core::fmt::Debug for NETSETUP_PROVISIONING_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETSETUP_PROVISIONING_PARAMS")
            .field("dwVersion", &self.dwVersion)
            .field("lpDomain", &self.lpDomain)
            .field("lpHostName", &self.lpHostName)
            .field("lpMachineAccountOU", &self.lpMachineAccountOU)
            .field("lpDcName", &self.lpDcName)
            .field("dwProvisionOptions", &self.dwProvisionOptions)
            .field("aCertTemplateNames", &self.aCertTemplateNames)
            .field("cCertTemplateNames", &self.cCertTemplateNames)
            .field("aMachinePolicyNames", &self.aMachinePolicyNames)
            .field("cMachinePolicyNames", &self.cMachinePolicyNames)
            .field("aMachinePolicyPaths", &self.aMachinePolicyPaths)
            .field("cMachinePolicyPaths", &self.cMachinePolicyPaths)
            .field("lpNetbiosName", &self.lpNetbiosName)
            .field("lpSiteName", &self.lpSiteName)
            .field("lpPrimaryDNSDomain", &self.lpPrimaryDNSDomain)
            .finish()
    }
}
impl ::core::default::Default for NETWORK_INSTALL_TIME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NETWORK_INSTALL_TIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETWORK_INSTALL_TIME").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NETWORK_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NETWORK_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NETWORK_NAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NETWORK_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETWORK_NAME").field("Name", &self.Name).finish()
    }
}
impl ::core::default::Default for NETWORK_UPGRADE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NETWORK_UPGRADE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETWORK_UPGRADE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NET_COMPUTER_NAME_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NET_COMPUTER_NAME_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_COMPUTER_NAME_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NET_DISPLAY_GROUP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NET_DISPLAY_GROUP {
    fn eq(&self, other: &Self) -> bool {
        self.grpi3_name == other.grpi3_name && self.grpi3_comment == other.grpi3_comment && self.grpi3_group_id == other.grpi3_group_id && self.grpi3_attributes == other.grpi3_attributes && self.grpi3_next_index == other.grpi3_next_index
    }
}
impl ::core::cmp::Eq for NET_DISPLAY_GROUP {}
impl ::core::fmt::Debug for NET_DISPLAY_GROUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NET_DISPLAY_GROUP").field("grpi3_name", &self.grpi3_name).field("grpi3_comment", &self.grpi3_comment).field("grpi3_group_id", &self.grpi3_group_id).field("grpi3_attributes", &self.grpi3_attributes).field("grpi3_next_index", &self.grpi3_next_index).finish()
    }
}
impl ::core::default::Default for NET_DISPLAY_MACHINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NET_DISPLAY_MACHINE {
    fn eq(&self, other: &Self) -> bool {
        self.usri2_name == other.usri2_name && self.usri2_comment == other.usri2_comment && self.usri2_flags == other.usri2_flags && self.usri2_user_id == other.usri2_user_id && self.usri2_next_index == other.usri2_next_index
    }
}
impl ::core::cmp::Eq for NET_DISPLAY_MACHINE {}
impl ::core::fmt::Debug for NET_DISPLAY_MACHINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NET_DISPLAY_MACHINE").field("usri2_name", &self.usri2_name).field("usri2_comment", &self.usri2_comment).field("usri2_flags", &self.usri2_flags).field("usri2_user_id", &self.usri2_user_id).field("usri2_next_index", &self.usri2_next_index).finish()
    }
}
impl ::core::default::Default for NET_DISPLAY_USER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NET_DISPLAY_USER {
    fn eq(&self, other: &Self) -> bool {
        self.usri1_name == other.usri1_name && self.usri1_comment == other.usri1_comment && self.usri1_flags == other.usri1_flags && self.usri1_full_name == other.usri1_full_name && self.usri1_user_id == other.usri1_user_id && self.usri1_next_index == other.usri1_next_index
    }
}
impl ::core::cmp::Eq for NET_DISPLAY_USER {}
impl ::core::fmt::Debug for NET_DISPLAY_USER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NET_DISPLAY_USER").field("usri1_name", &self.usri1_name).field("usri1_comment", &self.usri1_comment).field("usri1_flags", &self.usri1_flags).field("usri1_full_name", &self.usri1_full_name).field("usri1_user_id", &self.usri1_user_id).field("usri1_next_index", &self.usri1_next_index).finish()
    }
}
impl ::core::default::Default for NET_JOIN_DOMAIN_JOIN_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NET_JOIN_DOMAIN_JOIN_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_JOIN_DOMAIN_JOIN_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for NET_JOIN_DOMAIN_JOIN_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NET_JOIN_DOMAIN_JOIN_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NET_JOIN_DOMAIN_JOIN_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NET_JOIN_DOMAIN_JOIN_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NET_JOIN_DOMAIN_JOIN_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for NET_REQUEST_PROVISION_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NET_REQUEST_PROVISION_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_REQUEST_PROVISION_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for NET_REQUEST_PROVISION_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NET_REQUEST_PROVISION_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NET_REQUEST_PROVISION_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NET_REQUEST_PROVISION_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NET_REQUEST_PROVISION_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for NET_SERVER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NET_SERVER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_SERVER_TYPE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for NET_SERVER_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NET_SERVER_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NET_SERVER_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NET_SERVER_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NET_SERVER_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for NET_USER_ENUM_FILTER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NET_USER_ENUM_FILTER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_USER_ENUM_FILTER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for NET_USER_ENUM_FILTER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NET_USER_ENUM_FILTER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NET_USER_ENUM_FILTER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NET_USER_ENUM_FILTER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NET_USER_ENUM_FILTER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NET_VALIDATE_AUTHENTICATION_INPUT_ARG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NET_VALIDATE_AUTHENTICATION_INPUT_ARG {
    fn eq(&self, other: &Self) -> bool {
        self.InputPersistedFields == other.InputPersistedFields && self.PasswordMatched == other.PasswordMatched
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NET_VALIDATE_AUTHENTICATION_INPUT_ARG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NET_VALIDATE_AUTHENTICATION_INPUT_ARG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NET_VALIDATE_AUTHENTICATION_INPUT_ARG").field("InputPersistedFields", &self.InputPersistedFields).field("PasswordMatched", &self.PasswordMatched).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NET_VALIDATE_OUTPUT_ARG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NET_VALIDATE_OUTPUT_ARG {
    fn eq(&self, other: &Self) -> bool {
        self.ChangedPersistedFields == other.ChangedPersistedFields && self.ValidationStatus == other.ValidationStatus
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NET_VALIDATE_OUTPUT_ARG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NET_VALIDATE_OUTPUT_ARG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NET_VALIDATE_OUTPUT_ARG").field("ChangedPersistedFields", &self.ChangedPersistedFields).field("ValidationStatus", &self.ValidationStatus).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG {
    fn eq(&self, other: &Self) -> bool {
        self.InputPersistedFields == other.InputPersistedFields && self.ClearPassword == other.ClearPassword && self.UserAccountName == other.UserAccountName && self.HashedPassword == other.HashedPassword && self.PasswordMatch == other.PasswordMatch
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG").field("InputPersistedFields", &self.InputPersistedFields).field("ClearPassword", &self.ClearPassword).field("UserAccountName", &self.UserAccountName).field("HashedPassword", &self.HashedPassword).field("PasswordMatch", &self.PasswordMatch).finish()
    }
}
impl ::core::default::Default for NET_VALIDATE_PASSWORD_HASH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NET_VALIDATE_PASSWORD_HASH {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.Hash == other.Hash
    }
}
impl ::core::cmp::Eq for NET_VALIDATE_PASSWORD_HASH {}
impl ::core::fmt::Debug for NET_VALIDATE_PASSWORD_HASH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NET_VALIDATE_PASSWORD_HASH").field("Length", &self.Length).field("Hash", &self.Hash).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NET_VALIDATE_PASSWORD_RESET_INPUT_ARG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NET_VALIDATE_PASSWORD_RESET_INPUT_ARG {
    fn eq(&self, other: &Self) -> bool {
        self.InputPersistedFields == other.InputPersistedFields && self.ClearPassword == other.ClearPassword && self.UserAccountName == other.UserAccountName && self.HashedPassword == other.HashedPassword && self.PasswordMustChangeAtNextLogon == other.PasswordMustChangeAtNextLogon && self.ClearLockout == other.ClearLockout
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NET_VALIDATE_PASSWORD_RESET_INPUT_ARG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NET_VALIDATE_PASSWORD_RESET_INPUT_ARG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NET_VALIDATE_PASSWORD_RESET_INPUT_ARG").field("InputPersistedFields", &self.InputPersistedFields).field("ClearPassword", &self.ClearPassword).field("UserAccountName", &self.UserAccountName).field("HashedPassword", &self.HashedPassword).field("PasswordMustChangeAtNextLogon", &self.PasswordMustChangeAtNextLogon).field("ClearLockout", &self.ClearLockout).finish()
    }
}
impl ::core::default::Default for NET_VALIDATE_PASSWORD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NET_VALIDATE_PASSWORD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_VALIDATE_PASSWORD_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NET_VALIDATE_PERSISTED_FIELDS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NET_VALIDATE_PERSISTED_FIELDS {
    fn eq(&self, other: &Self) -> bool {
        self.PresentFields == other.PresentFields && self.PasswordLastSet == other.PasswordLastSet && self.BadPasswordTime == other.BadPasswordTime && self.LockoutTime == other.LockoutTime && self.BadPasswordCount == other.BadPasswordCount && self.PasswordHistoryLength == other.PasswordHistoryLength && self.PasswordHistory == other.PasswordHistory
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NET_VALIDATE_PERSISTED_FIELDS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NET_VALIDATE_PERSISTED_FIELDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NET_VALIDATE_PERSISTED_FIELDS").field("PresentFields", &self.PresentFields).field("PasswordLastSet", &self.PasswordLastSet).field("BadPasswordTime", &self.BadPasswordTime).field("LockoutTime", &self.LockoutTime).field("BadPasswordCount", &self.BadPasswordCount).field("PasswordHistoryLength", &self.PasswordHistoryLength).field("PasswordHistory", &self.PasswordHistory).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OBO_TOKEN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OBO_TOKEN {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.pncc == other.pncc && self.pszwManufacturer == other.pszwManufacturer && self.pszwProduct == other.pszwProduct && self.pszwDisplayName == other.pszwDisplayName && self.fRegistered == other.fRegistered
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OBO_TOKEN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OBO_TOKEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OBO_TOKEN").field("Type", &self.Type).field("pncc", &self.pncc).field("pszwManufacturer", &self.pszwManufacturer).field("pszwProduct", &self.pszwProduct).field("pszwDisplayName", &self.pszwDisplayName).field("fRegistered", &self.fRegistered).finish()
    }
}
impl ::core::default::Default for OBO_TOKEN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OBO_TOKEN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OBO_TOKEN_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PRINT_OTHER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PRINT_OTHER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.alrtpr_jobid == other.alrtpr_jobid && self.alrtpr_status == other.alrtpr_status && self.alrtpr_submitted == other.alrtpr_submitted && self.alrtpr_size == other.alrtpr_size
    }
}
impl ::core::cmp::Eq for PRINT_OTHER_INFO {}
impl ::core::fmt::Debug for PRINT_OTHER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINT_OTHER_INFO").field("alrtpr_jobid", &self.alrtpr_jobid).field("alrtpr_status", &self.alrtpr_status).field("alrtpr_submitted", &self.alrtpr_submitted).field("alrtpr_size", &self.alrtpr_size).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASCON_IPUI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASCON_IPUI {
    fn eq(&self, other: &Self) -> bool {
        self.guidConnection == other.guidConnection && self.fIPv6Cfg == other.fIPv6Cfg && self.dwFlags == other.dwFlags && self.pszwIpAddr == other.pszwIpAddr && self.pszwDnsAddr == other.pszwDnsAddr && self.pszwDns2Addr == other.pszwDns2Addr && self.pszwWinsAddr == other.pszwWinsAddr && self.pszwWins2Addr == other.pszwWins2Addr && self.pszwDnsSuffix == other.pszwDnsSuffix && self.pszwIpv6Addr == other.pszwIpv6Addr && self.dwIpv6PrefixLength == other.dwIpv6PrefixLength && self.pszwIpv6DnsAddr == other.pszwIpv6DnsAddr && self.pszwIpv6Dns2Addr == other.pszwIpv6Dns2Addr && self.dwIPv4InfMetric == other.dwIPv4InfMetric && self.dwIPv6InfMetric == other.dwIPv6InfMetric
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASCON_IPUI {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RASCON_IPUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASCON_IPUI")
            .field("guidConnection", &self.guidConnection)
            .field("fIPv6Cfg", &self.fIPv6Cfg)
            .field("dwFlags", &self.dwFlags)
            .field("pszwIpAddr", &self.pszwIpAddr)
            .field("pszwDnsAddr", &self.pszwDnsAddr)
            .field("pszwDns2Addr", &self.pszwDns2Addr)
            .field("pszwWinsAddr", &self.pszwWinsAddr)
            .field("pszwWins2Addr", &self.pszwWins2Addr)
            .field("pszwDnsSuffix", &self.pszwDnsSuffix)
            .field("pszwIpv6Addr", &self.pszwIpv6Addr)
            .field("dwIpv6PrefixLength", &self.dwIpv6PrefixLength)
            .field("pszwIpv6DnsAddr", &self.pszwIpv6DnsAddr)
            .field("pszwIpv6Dns2Addr", &self.pszwIpv6Dns2Addr)
            .field("dwIPv4InfMetric", &self.dwIPv4InfMetric)
            .field("dwIPv6InfMetric", &self.dwIPv6InfMetric)
            .finish()
    }
}
impl ::core::default::Default for RASCON_UIINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RASCON_UIINFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RASCON_UIINFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for REPL_EDIR_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for REPL_EDIR_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.rped0_dirname == other.rped0_dirname
    }
}
impl ::core::cmp::Eq for REPL_EDIR_INFO_0 {}
impl ::core::fmt::Debug for REPL_EDIR_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPL_EDIR_INFO_0").field("rped0_dirname", &self.rped0_dirname).finish()
    }
}
impl ::core::default::Default for REPL_EDIR_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for REPL_EDIR_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.rped1_dirname == other.rped1_dirname && self.rped1_integrity == other.rped1_integrity && self.rped1_extent == other.rped1_extent
    }
}
impl ::core::cmp::Eq for REPL_EDIR_INFO_1 {}
impl ::core::fmt::Debug for REPL_EDIR_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPL_EDIR_INFO_1").field("rped1_dirname", &self.rped1_dirname).field("rped1_integrity", &self.rped1_integrity).field("rped1_extent", &self.rped1_extent).finish()
    }
}
impl ::core::default::Default for REPL_EDIR_INFO_1000 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for REPL_EDIR_INFO_1000 {
    fn eq(&self, other: &Self) -> bool {
        self.rped1000_integrity == other.rped1000_integrity
    }
}
impl ::core::cmp::Eq for REPL_EDIR_INFO_1000 {}
impl ::core::fmt::Debug for REPL_EDIR_INFO_1000 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPL_EDIR_INFO_1000").field("rped1000_integrity", &self.rped1000_integrity).finish()
    }
}
impl ::core::default::Default for REPL_EDIR_INFO_1001 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for REPL_EDIR_INFO_1001 {
    fn eq(&self, other: &Self) -> bool {
        self.rped1001_extent == other.rped1001_extent
    }
}
impl ::core::cmp::Eq for REPL_EDIR_INFO_1001 {}
impl ::core::fmt::Debug for REPL_EDIR_INFO_1001 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPL_EDIR_INFO_1001").field("rped1001_extent", &self.rped1001_extent).finish()
    }
}
impl ::core::default::Default for REPL_EDIR_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for REPL_EDIR_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.rped2_dirname == other.rped2_dirname && self.rped2_integrity == other.rped2_integrity && self.rped2_extent == other.rped2_extent && self.rped2_lockcount == other.rped2_lockcount && self.rped2_locktime == other.rped2_locktime
    }
}
impl ::core::cmp::Eq for REPL_EDIR_INFO_2 {}
impl ::core::fmt::Debug for REPL_EDIR_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPL_EDIR_INFO_2").field("rped2_dirname", &self.rped2_dirname).field("rped2_integrity", &self.rped2_integrity).field("rped2_extent", &self.rped2_extent).field("rped2_lockcount", &self.rped2_lockcount).field("rped2_locktime", &self.rped2_locktime).finish()
    }
}
impl ::core::default::Default for REPL_IDIR_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for REPL_IDIR_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.rpid0_dirname == other.rpid0_dirname
    }
}
impl ::core::cmp::Eq for REPL_IDIR_INFO_0 {}
impl ::core::fmt::Debug for REPL_IDIR_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPL_IDIR_INFO_0").field("rpid0_dirname", &self.rpid0_dirname).finish()
    }
}
impl ::core::default::Default for REPL_IDIR_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for REPL_IDIR_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.rpid1_dirname == other.rpid1_dirname && self.rpid1_state == other.rpid1_state && self.rpid1_mastername == other.rpid1_mastername && self.rpid1_last_update_time == other.rpid1_last_update_time && self.rpid1_lockcount == other.rpid1_lockcount && self.rpid1_locktime == other.rpid1_locktime
    }
}
impl ::core::cmp::Eq for REPL_IDIR_INFO_1 {}
impl ::core::fmt::Debug for REPL_IDIR_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPL_IDIR_INFO_1").field("rpid1_dirname", &self.rpid1_dirname).field("rpid1_state", &self.rpid1_state).field("rpid1_mastername", &self.rpid1_mastername).field("rpid1_last_update_time", &self.rpid1_last_update_time).field("rpid1_lockcount", &self.rpid1_lockcount).field("rpid1_locktime", &self.rpid1_locktime).finish()
    }
}
impl ::core::default::Default for REPL_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for REPL_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.rp0_role == other.rp0_role && self.rp0_exportpath == other.rp0_exportpath && self.rp0_exportlist == other.rp0_exportlist && self.rp0_importpath == other.rp0_importpath && self.rp0_importlist == other.rp0_importlist && self.rp0_logonusername == other.rp0_logonusername && self.rp0_interval == other.rp0_interval && self.rp0_pulse == other.rp0_pulse && self.rp0_guardtime == other.rp0_guardtime && self.rp0_random == other.rp0_random
    }
}
impl ::core::cmp::Eq for REPL_INFO_0 {}
impl ::core::fmt::Debug for REPL_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPL_INFO_0").field("rp0_role", &self.rp0_role).field("rp0_exportpath", &self.rp0_exportpath).field("rp0_exportlist", &self.rp0_exportlist).field("rp0_importpath", &self.rp0_importpath).field("rp0_importlist", &self.rp0_importlist).field("rp0_logonusername", &self.rp0_logonusername).field("rp0_interval", &self.rp0_interval).field("rp0_pulse", &self.rp0_pulse).field("rp0_guardtime", &self.rp0_guardtime).field("rp0_random", &self.rp0_random).finish()
    }
}
impl ::core::default::Default for REPL_INFO_1000 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for REPL_INFO_1000 {
    fn eq(&self, other: &Self) -> bool {
        self.rp1000_interval == other.rp1000_interval
    }
}
impl ::core::cmp::Eq for REPL_INFO_1000 {}
impl ::core::fmt::Debug for REPL_INFO_1000 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPL_INFO_1000").field("rp1000_interval", &self.rp1000_interval).finish()
    }
}
impl ::core::default::Default for REPL_INFO_1001 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for REPL_INFO_1001 {
    fn eq(&self, other: &Self) -> bool {
        self.rp1001_pulse == other.rp1001_pulse
    }
}
impl ::core::cmp::Eq for REPL_INFO_1001 {}
impl ::core::fmt::Debug for REPL_INFO_1001 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPL_INFO_1001").field("rp1001_pulse", &self.rp1001_pulse).finish()
    }
}
impl ::core::default::Default for REPL_INFO_1002 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for REPL_INFO_1002 {
    fn eq(&self, other: &Self) -> bool {
        self.rp1002_guardtime == other.rp1002_guardtime
    }
}
impl ::core::cmp::Eq for REPL_INFO_1002 {}
impl ::core::fmt::Debug for REPL_INFO_1002 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPL_INFO_1002").field("rp1002_guardtime", &self.rp1002_guardtime).finish()
    }
}
impl ::core::default::Default for REPL_INFO_1003 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for REPL_INFO_1003 {
    fn eq(&self, other: &Self) -> bool {
        self.rp1003_random == other.rp1003_random
    }
}
impl ::core::cmp::Eq for REPL_INFO_1003 {}
impl ::core::fmt::Debug for REPL_INFO_1003 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPL_INFO_1003").field("rp1003_random", &self.rp1003_random).finish()
    }
}
impl ::core::default::Default for RTR_INFO_BLOCK_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RTR_INFO_BLOCK_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.TocEntriesCount == other.TocEntriesCount && self.TocEntry == other.TocEntry
    }
}
impl ::core::cmp::Eq for RTR_INFO_BLOCK_HEADER {}
impl ::core::fmt::Debug for RTR_INFO_BLOCK_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTR_INFO_BLOCK_HEADER").field("Version", &self.Version).field("Size", &self.Size).field("TocEntriesCount", &self.TocEntriesCount).field("TocEntry", &self.TocEntry).finish()
    }
}
impl ::core::default::Default for RTR_TOC_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RTR_TOC_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.InfoType == other.InfoType && self.InfoSize == other.InfoSize && self.Count == other.Count && self.Offset == other.Offset
    }
}
impl ::core::cmp::Eq for RTR_TOC_ENTRY {}
impl ::core::fmt::Debug for RTR_TOC_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTR_TOC_ENTRY").field("InfoType", &self.InfoType).field("InfoSize", &self.InfoSize).field("Count", &self.Count).field("Offset", &self.Offset).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_100 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_100 {
    fn eq(&self, other: &Self) -> bool {
        self.sv100_platform_id == other.sv100_platform_id && self.sv100_name == other.sv100_name
    }
}
impl ::core::cmp::Eq for SERVER_INFO_100 {}
impl ::core::fmt::Debug for SERVER_INFO_100 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_100").field("sv100_platform_id", &self.sv100_platform_id).field("sv100_name", &self.sv100_name).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1005 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1005 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1005_comment == other.sv1005_comment
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1005 {}
impl ::core::fmt::Debug for SERVER_INFO_1005 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1005").field("sv1005_comment", &self.sv1005_comment).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_101 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_101 {
    fn eq(&self, other: &Self) -> bool {
        self.sv101_platform_id == other.sv101_platform_id && self.sv101_name == other.sv101_name && self.sv101_version_major == other.sv101_version_major && self.sv101_version_minor == other.sv101_version_minor && self.sv101_type == other.sv101_type && self.sv101_comment == other.sv101_comment
    }
}
impl ::core::cmp::Eq for SERVER_INFO_101 {}
impl ::core::fmt::Debug for SERVER_INFO_101 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_101").field("sv101_platform_id", &self.sv101_platform_id).field("sv101_name", &self.sv101_name).field("sv101_version_major", &self.sv101_version_major).field("sv101_version_minor", &self.sv101_version_minor).field("sv101_type", &self.sv101_type).field("sv101_comment", &self.sv101_comment).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1010 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1010 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1010_disc == other.sv1010_disc
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1010 {}
impl ::core::fmt::Debug for SERVER_INFO_1010 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1010").field("sv1010_disc", &self.sv1010_disc).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1016 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1016 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1016_hidden == other.sv1016_hidden
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1016 {}
impl ::core::fmt::Debug for SERVER_INFO_1016 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1016").field("sv1016_hidden", &self.sv1016_hidden).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1017 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1017 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1017_announce == other.sv1017_announce
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1017 {}
impl ::core::fmt::Debug for SERVER_INFO_1017 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1017").field("sv1017_announce", &self.sv1017_announce).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1018 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1018 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1018_anndelta == other.sv1018_anndelta
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1018 {}
impl ::core::fmt::Debug for SERVER_INFO_1018 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1018").field("sv1018_anndelta", &self.sv1018_anndelta).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_102 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_102 {
    fn eq(&self, other: &Self) -> bool {
        self.sv102_platform_id == other.sv102_platform_id && self.sv102_name == other.sv102_name && self.sv102_version_major == other.sv102_version_major && self.sv102_version_minor == other.sv102_version_minor && self.sv102_type == other.sv102_type && self.sv102_comment == other.sv102_comment && self.sv102_users == other.sv102_users && self.sv102_disc == other.sv102_disc && self.sv102_hidden == other.sv102_hidden && self.sv102_announce == other.sv102_announce && self.sv102_anndelta == other.sv102_anndelta && self.sv102_licenses == other.sv102_licenses && self.sv102_userpath == other.sv102_userpath
    }
}
impl ::core::cmp::Eq for SERVER_INFO_102 {}
impl ::core::fmt::Debug for SERVER_INFO_102 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_102")
            .field("sv102_platform_id", &self.sv102_platform_id)
            .field("sv102_name", &self.sv102_name)
            .field("sv102_version_major", &self.sv102_version_major)
            .field("sv102_version_minor", &self.sv102_version_minor)
            .field("sv102_type", &self.sv102_type)
            .field("sv102_comment", &self.sv102_comment)
            .field("sv102_users", &self.sv102_users)
            .field("sv102_disc", &self.sv102_disc)
            .field("sv102_hidden", &self.sv102_hidden)
            .field("sv102_announce", &self.sv102_announce)
            .field("sv102_anndelta", &self.sv102_anndelta)
            .field("sv102_licenses", &self.sv102_licenses)
            .field("sv102_userpath", &self.sv102_userpath)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_103 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_103 {
    fn eq(&self, other: &Self) -> bool {
        self.sv103_platform_id == other.sv103_platform_id && self.sv103_name == other.sv103_name && self.sv103_version_major == other.sv103_version_major && self.sv103_version_minor == other.sv103_version_minor && self.sv103_type == other.sv103_type && self.sv103_comment == other.sv103_comment && self.sv103_users == other.sv103_users && self.sv103_disc == other.sv103_disc && self.sv103_hidden == other.sv103_hidden && self.sv103_announce == other.sv103_announce && self.sv103_anndelta == other.sv103_anndelta && self.sv103_licenses == other.sv103_licenses && self.sv103_userpath == other.sv103_userpath && self.sv103_capabilities == other.sv103_capabilities
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_103 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_103 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_103")
            .field("sv103_platform_id", &self.sv103_platform_id)
            .field("sv103_name", &self.sv103_name)
            .field("sv103_version_major", &self.sv103_version_major)
            .field("sv103_version_minor", &self.sv103_version_minor)
            .field("sv103_type", &self.sv103_type)
            .field("sv103_comment", &self.sv103_comment)
            .field("sv103_users", &self.sv103_users)
            .field("sv103_disc", &self.sv103_disc)
            .field("sv103_hidden", &self.sv103_hidden)
            .field("sv103_announce", &self.sv103_announce)
            .field("sv103_anndelta", &self.sv103_anndelta)
            .field("sv103_licenses", &self.sv103_licenses)
            .field("sv103_userpath", &self.sv103_userpath)
            .field("sv103_capabilities", &self.sv103_capabilities)
            .finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1107 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1107 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1107_users == other.sv1107_users
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1107 {}
impl ::core::fmt::Debug for SERVER_INFO_1107 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1107").field("sv1107_users", &self.sv1107_users).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1501 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1501 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1501_sessopens == other.sv1501_sessopens
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1501 {}
impl ::core::fmt::Debug for SERVER_INFO_1501 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1501").field("sv1501_sessopens", &self.sv1501_sessopens).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1502 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1502 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1502_sessvcs == other.sv1502_sessvcs
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1502 {}
impl ::core::fmt::Debug for SERVER_INFO_1502 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1502").field("sv1502_sessvcs", &self.sv1502_sessvcs).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1503 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1503 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1503_opensearch == other.sv1503_opensearch
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1503 {}
impl ::core::fmt::Debug for SERVER_INFO_1503 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1503").field("sv1503_opensearch", &self.sv1503_opensearch).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1506 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1506 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1506_maxworkitems == other.sv1506_maxworkitems
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1506 {}
impl ::core::fmt::Debug for SERVER_INFO_1506 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1506").field("sv1506_maxworkitems", &self.sv1506_maxworkitems).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1509 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1509 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1509_maxrawbuflen == other.sv1509_maxrawbuflen
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1509 {}
impl ::core::fmt::Debug for SERVER_INFO_1509 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1509").field("sv1509_maxrawbuflen", &self.sv1509_maxrawbuflen).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1510 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1510 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1510_sessusers == other.sv1510_sessusers
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1510 {}
impl ::core::fmt::Debug for SERVER_INFO_1510 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1510").field("sv1510_sessusers", &self.sv1510_sessusers).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1511 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1511 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1511_sessconns == other.sv1511_sessconns
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1511 {}
impl ::core::fmt::Debug for SERVER_INFO_1511 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1511").field("sv1511_sessconns", &self.sv1511_sessconns).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1512 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1512 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1512_maxnonpagedmemoryusage == other.sv1512_maxnonpagedmemoryusage
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1512 {}
impl ::core::fmt::Debug for SERVER_INFO_1512 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1512").field("sv1512_maxnonpagedmemoryusage", &self.sv1512_maxnonpagedmemoryusage).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1513 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1513 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1513_maxpagedmemoryusage == other.sv1513_maxpagedmemoryusage
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1513 {}
impl ::core::fmt::Debug for SERVER_INFO_1513 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1513").field("sv1513_maxpagedmemoryusage", &self.sv1513_maxpagedmemoryusage).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1514 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1514 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1514_enablesoftcompat == other.sv1514_enablesoftcompat
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1514 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_1514 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1514").field("sv1514_enablesoftcompat", &self.sv1514_enablesoftcompat).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1515 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1515 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1515_enableforcedlogoff == other.sv1515_enableforcedlogoff
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1515 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_1515 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1515").field("sv1515_enableforcedlogoff", &self.sv1515_enableforcedlogoff).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1516 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1516 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1516_timesource == other.sv1516_timesource
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1516 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_1516 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1516").field("sv1516_timesource", &self.sv1516_timesource).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1518 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1518 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1518_lmannounce == other.sv1518_lmannounce
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1518 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_1518 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1518").field("sv1518_lmannounce", &self.sv1518_lmannounce).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1520 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1520 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1520_maxcopyreadlen == other.sv1520_maxcopyreadlen
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1520 {}
impl ::core::fmt::Debug for SERVER_INFO_1520 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1520").field("sv1520_maxcopyreadlen", &self.sv1520_maxcopyreadlen).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1521 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1521 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1521_maxcopywritelen == other.sv1521_maxcopywritelen
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1521 {}
impl ::core::fmt::Debug for SERVER_INFO_1521 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1521").field("sv1521_maxcopywritelen", &self.sv1521_maxcopywritelen).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1522 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1522 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1522_minkeepsearch == other.sv1522_minkeepsearch
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1522 {}
impl ::core::fmt::Debug for SERVER_INFO_1522 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1522").field("sv1522_minkeepsearch", &self.sv1522_minkeepsearch).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1523 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1523 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1523_maxkeepsearch == other.sv1523_maxkeepsearch
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1523 {}
impl ::core::fmt::Debug for SERVER_INFO_1523 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1523").field("sv1523_maxkeepsearch", &self.sv1523_maxkeepsearch).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1524 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1524 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1524_minkeepcomplsearch == other.sv1524_minkeepcomplsearch
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1524 {}
impl ::core::fmt::Debug for SERVER_INFO_1524 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1524").field("sv1524_minkeepcomplsearch", &self.sv1524_minkeepcomplsearch).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1525 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1525 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1525_maxkeepcomplsearch == other.sv1525_maxkeepcomplsearch
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1525 {}
impl ::core::fmt::Debug for SERVER_INFO_1525 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1525").field("sv1525_maxkeepcomplsearch", &self.sv1525_maxkeepcomplsearch).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1528 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1528 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1528_scavtimeout == other.sv1528_scavtimeout
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1528 {}
impl ::core::fmt::Debug for SERVER_INFO_1528 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1528").field("sv1528_scavtimeout", &self.sv1528_scavtimeout).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1529 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1529 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1529_minrcvqueue == other.sv1529_minrcvqueue
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1529 {}
impl ::core::fmt::Debug for SERVER_INFO_1529 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1529").field("sv1529_minrcvqueue", &self.sv1529_minrcvqueue).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1530 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1530 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1530_minfreeworkitems == other.sv1530_minfreeworkitems
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1530 {}
impl ::core::fmt::Debug for SERVER_INFO_1530 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1530").field("sv1530_minfreeworkitems", &self.sv1530_minfreeworkitems).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1533 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1533 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1533_maxmpxct == other.sv1533_maxmpxct
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1533 {}
impl ::core::fmt::Debug for SERVER_INFO_1533 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1533").field("sv1533_maxmpxct", &self.sv1533_maxmpxct).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1534 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1534 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1534_oplockbreakwait == other.sv1534_oplockbreakwait
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1534 {}
impl ::core::fmt::Debug for SERVER_INFO_1534 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1534").field("sv1534_oplockbreakwait", &self.sv1534_oplockbreakwait).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1535 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1535 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1535_oplockbreakresponsewait == other.sv1535_oplockbreakresponsewait
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1535 {}
impl ::core::fmt::Debug for SERVER_INFO_1535 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1535").field("sv1535_oplockbreakresponsewait", &self.sv1535_oplockbreakresponsewait).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1536 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1536 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1536_enableoplocks == other.sv1536_enableoplocks
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1536 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_1536 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1536").field("sv1536_enableoplocks", &self.sv1536_enableoplocks).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1537 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1537 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1537_enableoplockforceclose == other.sv1537_enableoplockforceclose
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1537 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_1537 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1537").field("sv1537_enableoplockforceclose", &self.sv1537_enableoplockforceclose).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1538 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1538 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1538_enablefcbopens == other.sv1538_enablefcbopens
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1538 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_1538 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1538").field("sv1538_enablefcbopens", &self.sv1538_enablefcbopens).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1539 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1539 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1539_enableraw == other.sv1539_enableraw
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1539 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_1539 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1539").field("sv1539_enableraw", &self.sv1539_enableraw).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1540 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1540 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1540_enablesharednetdrives == other.sv1540_enablesharednetdrives
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1540 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_1540 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1540").field("sv1540_enablesharednetdrives", &self.sv1540_enablesharednetdrives).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1541 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1541 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1541_minfreeconnections == other.sv1541_minfreeconnections
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1541 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_1541 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1541").field("sv1541_minfreeconnections", &self.sv1541_minfreeconnections).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1542 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1542 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1542_maxfreeconnections == other.sv1542_maxfreeconnections
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1542 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_1542 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1542").field("sv1542_maxfreeconnections", &self.sv1542_maxfreeconnections).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1543 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1543 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1543_initsesstable == other.sv1543_initsesstable
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1543 {}
impl ::core::fmt::Debug for SERVER_INFO_1543 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1543").field("sv1543_initsesstable", &self.sv1543_initsesstable).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1544 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1544 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1544_initconntable == other.sv1544_initconntable
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1544 {}
impl ::core::fmt::Debug for SERVER_INFO_1544 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1544").field("sv1544_initconntable", &self.sv1544_initconntable).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1545 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1545 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1545_initfiletable == other.sv1545_initfiletable
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1545 {}
impl ::core::fmt::Debug for SERVER_INFO_1545 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1545").field("sv1545_initfiletable", &self.sv1545_initfiletable).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1546 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1546 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1546_initsearchtable == other.sv1546_initsearchtable
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1546 {}
impl ::core::fmt::Debug for SERVER_INFO_1546 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1546").field("sv1546_initsearchtable", &self.sv1546_initsearchtable).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1547 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1547 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1547_alertschedule == other.sv1547_alertschedule
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1547 {}
impl ::core::fmt::Debug for SERVER_INFO_1547 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1547").field("sv1547_alertschedule", &self.sv1547_alertschedule).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1548 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1548 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1548_errorthreshold == other.sv1548_errorthreshold
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1548 {}
impl ::core::fmt::Debug for SERVER_INFO_1548 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1548").field("sv1548_errorthreshold", &self.sv1548_errorthreshold).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1549 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1549 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1549_networkerrorthreshold == other.sv1549_networkerrorthreshold
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1549 {}
impl ::core::fmt::Debug for SERVER_INFO_1549 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1549").field("sv1549_networkerrorthreshold", &self.sv1549_networkerrorthreshold).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1550 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1550 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1550_diskspacethreshold == other.sv1550_diskspacethreshold
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1550 {}
impl ::core::fmt::Debug for SERVER_INFO_1550 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1550").field("sv1550_diskspacethreshold", &self.sv1550_diskspacethreshold).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1552 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1552 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1552_maxlinkdelay == other.sv1552_maxlinkdelay
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1552 {}
impl ::core::fmt::Debug for SERVER_INFO_1552 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1552").field("sv1552_maxlinkdelay", &self.sv1552_maxlinkdelay).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1553 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1553 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1553_minlinkthroughput == other.sv1553_minlinkthroughput
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1553 {}
impl ::core::fmt::Debug for SERVER_INFO_1553 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1553").field("sv1553_minlinkthroughput", &self.sv1553_minlinkthroughput).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1554 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1554 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1554_linkinfovalidtime == other.sv1554_linkinfovalidtime
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1554 {}
impl ::core::fmt::Debug for SERVER_INFO_1554 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1554").field("sv1554_linkinfovalidtime", &self.sv1554_linkinfovalidtime).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1555 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1555 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1555_scavqosinfoupdatetime == other.sv1555_scavqosinfoupdatetime
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1555 {}
impl ::core::fmt::Debug for SERVER_INFO_1555 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1555").field("sv1555_scavqosinfoupdatetime", &self.sv1555_scavqosinfoupdatetime).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1556 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1556 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1556_maxworkitemidletime == other.sv1556_maxworkitemidletime
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1556 {}
impl ::core::fmt::Debug for SERVER_INFO_1556 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1556").field("sv1556_maxworkitemidletime", &self.sv1556_maxworkitemidletime).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1557 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1557 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1557_maxrawworkitems == other.sv1557_maxrawworkitems
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1557 {}
impl ::core::fmt::Debug for SERVER_INFO_1557 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1557").field("sv1557_maxrawworkitems", &self.sv1557_maxrawworkitems).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1560 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1560 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1560_producttype == other.sv1560_producttype
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1560 {}
impl ::core::fmt::Debug for SERVER_INFO_1560 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1560").field("sv1560_producttype", &self.sv1560_producttype).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1561 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1561 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1561_serversize == other.sv1561_serversize
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1561 {}
impl ::core::fmt::Debug for SERVER_INFO_1561 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1561").field("sv1561_serversize", &self.sv1561_serversize).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1562 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1562 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1562_connectionlessautodisc == other.sv1562_connectionlessautodisc
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1562 {}
impl ::core::fmt::Debug for SERVER_INFO_1562 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1562").field("sv1562_connectionlessautodisc", &self.sv1562_connectionlessautodisc).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1563 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1563 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1563_sharingviolationretries == other.sv1563_sharingviolationretries
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1563 {}
impl ::core::fmt::Debug for SERVER_INFO_1563 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1563").field("sv1563_sharingviolationretries", &self.sv1563_sharingviolationretries).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1564 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1564 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1564_sharingviolationdelay == other.sv1564_sharingviolationdelay
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1564 {}
impl ::core::fmt::Debug for SERVER_INFO_1564 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1564").field("sv1564_sharingviolationdelay", &self.sv1564_sharingviolationdelay).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1565 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1565 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1565_maxglobalopensearch == other.sv1565_maxglobalopensearch
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1565 {}
impl ::core::fmt::Debug for SERVER_INFO_1565 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1565").field("sv1565_maxglobalopensearch", &self.sv1565_maxglobalopensearch).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1566 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1566 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1566_removeduplicatesearches == other.sv1566_removeduplicatesearches
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1566 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_1566 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1566").field("sv1566_removeduplicatesearches", &self.sv1566_removeduplicatesearches).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1567 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1567 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1567_lockviolationretries == other.sv1567_lockviolationretries
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1567 {}
impl ::core::fmt::Debug for SERVER_INFO_1567 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1567").field("sv1567_lockviolationretries", &self.sv1567_lockviolationretries).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1568 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1568 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1568_lockviolationoffset == other.sv1568_lockviolationoffset
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1568 {}
impl ::core::fmt::Debug for SERVER_INFO_1568 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1568").field("sv1568_lockviolationoffset", &self.sv1568_lockviolationoffset).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1569 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1569 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1569_lockviolationdelay == other.sv1569_lockviolationdelay
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1569 {}
impl ::core::fmt::Debug for SERVER_INFO_1569 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1569").field("sv1569_lockviolationdelay", &self.sv1569_lockviolationdelay).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1570 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1570 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1570_mdlreadswitchover == other.sv1570_mdlreadswitchover
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1570 {}
impl ::core::fmt::Debug for SERVER_INFO_1570 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1570").field("sv1570_mdlreadswitchover", &self.sv1570_mdlreadswitchover).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1571 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1571 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1571_cachedopenlimit == other.sv1571_cachedopenlimit
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1571 {}
impl ::core::fmt::Debug for SERVER_INFO_1571 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1571").field("sv1571_cachedopenlimit", &self.sv1571_cachedopenlimit).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1572 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1572 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1572_criticalthreads == other.sv1572_criticalthreads
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1572 {}
impl ::core::fmt::Debug for SERVER_INFO_1572 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1572").field("sv1572_criticalthreads", &self.sv1572_criticalthreads).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1573 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1573 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1573_restrictnullsessaccess == other.sv1573_restrictnullsessaccess
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1573 {}
impl ::core::fmt::Debug for SERVER_INFO_1573 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1573").field("sv1573_restrictnullsessaccess", &self.sv1573_restrictnullsessaccess).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1574 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1574 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1574_enablewfw311directipx == other.sv1574_enablewfw311directipx
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1574 {}
impl ::core::fmt::Debug for SERVER_INFO_1574 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1574").field("sv1574_enablewfw311directipx", &self.sv1574_enablewfw311directipx).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1575 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1575 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1575_otherqueueaffinity == other.sv1575_otherqueueaffinity
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1575 {}
impl ::core::fmt::Debug for SERVER_INFO_1575 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1575").field("sv1575_otherqueueaffinity", &self.sv1575_otherqueueaffinity).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1576 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1576 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1576_queuesamplesecs == other.sv1576_queuesamplesecs
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1576 {}
impl ::core::fmt::Debug for SERVER_INFO_1576 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1576").field("sv1576_queuesamplesecs", &self.sv1576_queuesamplesecs).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1577 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1577 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1577_balancecount == other.sv1577_balancecount
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1577 {}
impl ::core::fmt::Debug for SERVER_INFO_1577 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1577").field("sv1577_balancecount", &self.sv1577_balancecount).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1578 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1578 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1578_preferredaffinity == other.sv1578_preferredaffinity
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1578 {}
impl ::core::fmt::Debug for SERVER_INFO_1578 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1578").field("sv1578_preferredaffinity", &self.sv1578_preferredaffinity).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1579 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1579 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1579_maxfreerfcbs == other.sv1579_maxfreerfcbs
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1579 {}
impl ::core::fmt::Debug for SERVER_INFO_1579 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1579").field("sv1579_maxfreerfcbs", &self.sv1579_maxfreerfcbs).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1580 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1580 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1580_maxfreemfcbs == other.sv1580_maxfreemfcbs
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1580 {}
impl ::core::fmt::Debug for SERVER_INFO_1580 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1580").field("sv1580_maxfreemfcbs", &self.sv1580_maxfreemfcbs).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1581 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1581 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1581_maxfreemlcbs == other.sv1581_maxfreemlcbs
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1581 {}
impl ::core::fmt::Debug for SERVER_INFO_1581 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1581").field("sv1581_maxfreemlcbs", &self.sv1581_maxfreemlcbs).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1582 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1582 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1582_maxfreepagedpoolchunks == other.sv1582_maxfreepagedpoolchunks
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1582 {}
impl ::core::fmt::Debug for SERVER_INFO_1582 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1582").field("sv1582_maxfreepagedpoolchunks", &self.sv1582_maxfreepagedpoolchunks).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1583 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1583 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1583_minpagedpoolchunksize == other.sv1583_minpagedpoolchunksize
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1583 {}
impl ::core::fmt::Debug for SERVER_INFO_1583 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1583").field("sv1583_minpagedpoolchunksize", &self.sv1583_minpagedpoolchunksize).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1584 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1584 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1584_maxpagedpoolchunksize == other.sv1584_maxpagedpoolchunksize
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1584 {}
impl ::core::fmt::Debug for SERVER_INFO_1584 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1584").field("sv1584_maxpagedpoolchunksize", &self.sv1584_maxpagedpoolchunksize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1585 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1585 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1585_sendsfrompreferredprocessor == other.sv1585_sendsfrompreferredprocessor
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1585 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_1585 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1585").field("sv1585_sendsfrompreferredprocessor", &self.sv1585_sendsfrompreferredprocessor).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1586 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1586 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1586_maxthreadsperqueue == other.sv1586_maxthreadsperqueue
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1586 {}
impl ::core::fmt::Debug for SERVER_INFO_1586 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1586").field("sv1586_maxthreadsperqueue", &self.sv1586_maxthreadsperqueue).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1587 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1587 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1587_cacheddirectorylimit == other.sv1587_cacheddirectorylimit
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1587 {}
impl ::core::fmt::Debug for SERVER_INFO_1587 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1587").field("sv1587_cacheddirectorylimit", &self.sv1587_cacheddirectorylimit).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1588 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1588 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1588_maxcopylength == other.sv1588_maxcopylength
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1588 {}
impl ::core::fmt::Debug for SERVER_INFO_1588 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1588").field("sv1588_maxcopylength", &self.sv1588_maxcopylength).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1590 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1590 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1590_enablecompression == other.sv1590_enablecompression
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1590 {}
impl ::core::fmt::Debug for SERVER_INFO_1590 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1590").field("sv1590_enablecompression", &self.sv1590_enablecompression).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1591 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1591 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1591_autosharewks == other.sv1591_autosharewks
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1591 {}
impl ::core::fmt::Debug for SERVER_INFO_1591 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1591").field("sv1591_autosharewks", &self.sv1591_autosharewks).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1592 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1592 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1592_autosharewks == other.sv1592_autosharewks
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1592 {}
impl ::core::fmt::Debug for SERVER_INFO_1592 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1592").field("sv1592_autosharewks", &self.sv1592_autosharewks).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1593 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1593 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1593_enablesecuritysignature == other.sv1593_enablesecuritysignature
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1593 {}
impl ::core::fmt::Debug for SERVER_INFO_1593 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1593").field("sv1593_enablesecuritysignature", &self.sv1593_enablesecuritysignature).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1594 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1594 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1594_requiresecuritysignature == other.sv1594_requiresecuritysignature
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1594 {}
impl ::core::fmt::Debug for SERVER_INFO_1594 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1594").field("sv1594_requiresecuritysignature", &self.sv1594_requiresecuritysignature).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1595 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1595 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1595_minclientbuffersize == other.sv1595_minclientbuffersize
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1595 {}
impl ::core::fmt::Debug for SERVER_INFO_1595 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1595").field("sv1595_minclientbuffersize", &self.sv1595_minclientbuffersize).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1596 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1596 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1596_ConnectionNoSessionsTimeout == other.sv1596_ConnectionNoSessionsTimeout
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1596 {}
impl ::core::fmt::Debug for SERVER_INFO_1596 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1596").field("sv1596_ConnectionNoSessionsTimeout", &self.sv1596_ConnectionNoSessionsTimeout).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1597 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1597 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1597_IdleThreadTimeOut == other.sv1597_IdleThreadTimeOut
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1597 {}
impl ::core::fmt::Debug for SERVER_INFO_1597 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1597").field("sv1597_IdleThreadTimeOut", &self.sv1597_IdleThreadTimeOut).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1598 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1598 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1598_enableW9xsecuritysignature == other.sv1598_enableW9xsecuritysignature
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1598 {}
impl ::core::fmt::Debug for SERVER_INFO_1598 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1598").field("sv1598_enableW9xsecuritysignature", &self.sv1598_enableW9xsecuritysignature).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1599 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1599 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1598_enforcekerberosreauthentication == other.sv1598_enforcekerberosreauthentication
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1599 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_1599 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1599").field("sv1598_enforcekerberosreauthentication", &self.sv1598_enforcekerberosreauthentication).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1600 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1600 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1598_disabledos == other.sv1598_disabledos
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1600 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_1600 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1600").field("sv1598_disabledos", &self.sv1598_disabledos).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_1601 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_1601 {
    fn eq(&self, other: &Self) -> bool {
        self.sv1598_lowdiskspaceminimum == other.sv1598_lowdiskspaceminimum
    }
}
impl ::core::cmp::Eq for SERVER_INFO_1601 {}
impl ::core::fmt::Debug for SERVER_INFO_1601 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1601").field("sv1598_lowdiskspaceminimum", &self.sv1598_lowdiskspaceminimum).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_1602 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_1602 {
    fn eq(&self, other: &Self) -> bool {
        self.sv_1598_disablestrictnamechecking == other.sv_1598_disablestrictnamechecking
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_1602 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_1602 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_1602").field("sv_1598_disablestrictnamechecking", &self.sv_1598_disablestrictnamechecking).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_402 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_402 {
    fn eq(&self, other: &Self) -> bool {
        self.sv402_ulist_mtime == other.sv402_ulist_mtime
            && self.sv402_glist_mtime == other.sv402_glist_mtime
            && self.sv402_alist_mtime == other.sv402_alist_mtime
            && self.sv402_alerts == other.sv402_alerts
            && self.sv402_security == other.sv402_security
            && self.sv402_numadmin == other.sv402_numadmin
            && self.sv402_lanmask == other.sv402_lanmask
            && self.sv402_guestacct == other.sv402_guestacct
            && self.sv402_chdevs == other.sv402_chdevs
            && self.sv402_chdevq == other.sv402_chdevq
            && self.sv402_chdevjobs == other.sv402_chdevjobs
            && self.sv402_connections == other.sv402_connections
            && self.sv402_shares == other.sv402_shares
            && self.sv402_openfiles == other.sv402_openfiles
            && self.sv402_sessopens == other.sv402_sessopens
            && self.sv402_sessvcs == other.sv402_sessvcs
            && self.sv402_sessreqs == other.sv402_sessreqs
            && self.sv402_opensearch == other.sv402_opensearch
            && self.sv402_activelocks == other.sv402_activelocks
            && self.sv402_numreqbuf == other.sv402_numreqbuf
            && self.sv402_sizreqbuf == other.sv402_sizreqbuf
            && self.sv402_numbigbuf == other.sv402_numbigbuf
            && self.sv402_numfiletasks == other.sv402_numfiletasks
            && self.sv402_alertsched == other.sv402_alertsched
            && self.sv402_erroralert == other.sv402_erroralert
            && self.sv402_logonalert == other.sv402_logonalert
            && self.sv402_accessalert == other.sv402_accessalert
            && self.sv402_diskalert == other.sv402_diskalert
            && self.sv402_netioalert == other.sv402_netioalert
            && self.sv402_maxauditsz == other.sv402_maxauditsz
            && self.sv402_srvheuristics == other.sv402_srvheuristics
    }
}
impl ::core::cmp::Eq for SERVER_INFO_402 {}
impl ::core::fmt::Debug for SERVER_INFO_402 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_402")
            .field("sv402_ulist_mtime", &self.sv402_ulist_mtime)
            .field("sv402_glist_mtime", &self.sv402_glist_mtime)
            .field("sv402_alist_mtime", &self.sv402_alist_mtime)
            .field("sv402_alerts", &self.sv402_alerts)
            .field("sv402_security", &self.sv402_security)
            .field("sv402_numadmin", &self.sv402_numadmin)
            .field("sv402_lanmask", &self.sv402_lanmask)
            .field("sv402_guestacct", &self.sv402_guestacct)
            .field("sv402_chdevs", &self.sv402_chdevs)
            .field("sv402_chdevq", &self.sv402_chdevq)
            .field("sv402_chdevjobs", &self.sv402_chdevjobs)
            .field("sv402_connections", &self.sv402_connections)
            .field("sv402_shares", &self.sv402_shares)
            .field("sv402_openfiles", &self.sv402_openfiles)
            .field("sv402_sessopens", &self.sv402_sessopens)
            .field("sv402_sessvcs", &self.sv402_sessvcs)
            .field("sv402_sessreqs", &self.sv402_sessreqs)
            .field("sv402_opensearch", &self.sv402_opensearch)
            .field("sv402_activelocks", &self.sv402_activelocks)
            .field("sv402_numreqbuf", &self.sv402_numreqbuf)
            .field("sv402_sizreqbuf", &self.sv402_sizreqbuf)
            .field("sv402_numbigbuf", &self.sv402_numbigbuf)
            .field("sv402_numfiletasks", &self.sv402_numfiletasks)
            .field("sv402_alertsched", &self.sv402_alertsched)
            .field("sv402_erroralert", &self.sv402_erroralert)
            .field("sv402_logonalert", &self.sv402_logonalert)
            .field("sv402_accessalert", &self.sv402_accessalert)
            .field("sv402_diskalert", &self.sv402_diskalert)
            .field("sv402_netioalert", &self.sv402_netioalert)
            .field("sv402_maxauditsz", &self.sv402_maxauditsz)
            .field("sv402_srvheuristics", &self.sv402_srvheuristics)
            .finish()
    }
}
impl ::core::default::Default for SERVER_INFO_403 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_INFO_403 {
    fn eq(&self, other: &Self) -> bool {
        self.sv403_ulist_mtime == other.sv403_ulist_mtime
            && self.sv403_glist_mtime == other.sv403_glist_mtime
            && self.sv403_alist_mtime == other.sv403_alist_mtime
            && self.sv403_alerts == other.sv403_alerts
            && self.sv403_security == other.sv403_security
            && self.sv403_numadmin == other.sv403_numadmin
            && self.sv403_lanmask == other.sv403_lanmask
            && self.sv403_guestacct == other.sv403_guestacct
            && self.sv403_chdevs == other.sv403_chdevs
            && self.sv403_chdevq == other.sv403_chdevq
            && self.sv403_chdevjobs == other.sv403_chdevjobs
            && self.sv403_connections == other.sv403_connections
            && self.sv403_shares == other.sv403_shares
            && self.sv403_openfiles == other.sv403_openfiles
            && self.sv403_sessopens == other.sv403_sessopens
            && self.sv403_sessvcs == other.sv403_sessvcs
            && self.sv403_sessreqs == other.sv403_sessreqs
            && self.sv403_opensearch == other.sv403_opensearch
            && self.sv403_activelocks == other.sv403_activelocks
            && self.sv403_numreqbuf == other.sv403_numreqbuf
            && self.sv403_sizreqbuf == other.sv403_sizreqbuf
            && self.sv403_numbigbuf == other.sv403_numbigbuf
            && self.sv403_numfiletasks == other.sv403_numfiletasks
            && self.sv403_alertsched == other.sv403_alertsched
            && self.sv403_erroralert == other.sv403_erroralert
            && self.sv403_logonalert == other.sv403_logonalert
            && self.sv403_accessalert == other.sv403_accessalert
            && self.sv403_diskalert == other.sv403_diskalert
            && self.sv403_netioalert == other.sv403_netioalert
            && self.sv403_maxauditsz == other.sv403_maxauditsz
            && self.sv403_srvheuristics == other.sv403_srvheuristics
            && self.sv403_auditedevents == other.sv403_auditedevents
            && self.sv403_autoprofile == other.sv403_autoprofile
            && self.sv403_autopath == other.sv403_autopath
    }
}
impl ::core::cmp::Eq for SERVER_INFO_403 {}
impl ::core::fmt::Debug for SERVER_INFO_403 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_403")
            .field("sv403_ulist_mtime", &self.sv403_ulist_mtime)
            .field("sv403_glist_mtime", &self.sv403_glist_mtime)
            .field("sv403_alist_mtime", &self.sv403_alist_mtime)
            .field("sv403_alerts", &self.sv403_alerts)
            .field("sv403_security", &self.sv403_security)
            .field("sv403_numadmin", &self.sv403_numadmin)
            .field("sv403_lanmask", &self.sv403_lanmask)
            .field("sv403_guestacct", &self.sv403_guestacct)
            .field("sv403_chdevs", &self.sv403_chdevs)
            .field("sv403_chdevq", &self.sv403_chdevq)
            .field("sv403_chdevjobs", &self.sv403_chdevjobs)
            .field("sv403_connections", &self.sv403_connections)
            .field("sv403_shares", &self.sv403_shares)
            .field("sv403_openfiles", &self.sv403_openfiles)
            .field("sv403_sessopens", &self.sv403_sessopens)
            .field("sv403_sessvcs", &self.sv403_sessvcs)
            .field("sv403_sessreqs", &self.sv403_sessreqs)
            .field("sv403_opensearch", &self.sv403_opensearch)
            .field("sv403_activelocks", &self.sv403_activelocks)
            .field("sv403_numreqbuf", &self.sv403_numreqbuf)
            .field("sv403_sizreqbuf", &self.sv403_sizreqbuf)
            .field("sv403_numbigbuf", &self.sv403_numbigbuf)
            .field("sv403_numfiletasks", &self.sv403_numfiletasks)
            .field("sv403_alertsched", &self.sv403_alertsched)
            .field("sv403_erroralert", &self.sv403_erroralert)
            .field("sv403_logonalert", &self.sv403_logonalert)
            .field("sv403_accessalert", &self.sv403_accessalert)
            .field("sv403_diskalert", &self.sv403_diskalert)
            .field("sv403_netioalert", &self.sv403_netioalert)
            .field("sv403_maxauditsz", &self.sv403_maxauditsz)
            .field("sv403_srvheuristics", &self.sv403_srvheuristics)
            .field("sv403_auditedevents", &self.sv403_auditedevents)
            .field("sv403_autoprofile", &self.sv403_autoprofile)
            .field("sv403_autopath", &self.sv403_autopath)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_502 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_502 {
    fn eq(&self, other: &Self) -> bool {
        self.sv502_sessopens == other.sv502_sessopens
            && self.sv502_sessvcs == other.sv502_sessvcs
            && self.sv502_opensearch == other.sv502_opensearch
            && self.sv502_sizreqbuf == other.sv502_sizreqbuf
            && self.sv502_initworkitems == other.sv502_initworkitems
            && self.sv502_maxworkitems == other.sv502_maxworkitems
            && self.sv502_rawworkitems == other.sv502_rawworkitems
            && self.sv502_irpstacksize == other.sv502_irpstacksize
            && self.sv502_maxrawbuflen == other.sv502_maxrawbuflen
            && self.sv502_sessusers == other.sv502_sessusers
            && self.sv502_sessconns == other.sv502_sessconns
            && self.sv502_maxpagedmemoryusage == other.sv502_maxpagedmemoryusage
            && self.sv502_maxnonpagedmemoryusage == other.sv502_maxnonpagedmemoryusage
            && self.sv502_enablesoftcompat == other.sv502_enablesoftcompat
            && self.sv502_enableforcedlogoff == other.sv502_enableforcedlogoff
            && self.sv502_timesource == other.sv502_timesource
            && self.sv502_acceptdownlevelapis == other.sv502_acceptdownlevelapis
            && self.sv502_lmannounce == other.sv502_lmannounce
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_502 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_502 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_502")
            .field("sv502_sessopens", &self.sv502_sessopens)
            .field("sv502_sessvcs", &self.sv502_sessvcs)
            .field("sv502_opensearch", &self.sv502_opensearch)
            .field("sv502_sizreqbuf", &self.sv502_sizreqbuf)
            .field("sv502_initworkitems", &self.sv502_initworkitems)
            .field("sv502_maxworkitems", &self.sv502_maxworkitems)
            .field("sv502_rawworkitems", &self.sv502_rawworkitems)
            .field("sv502_irpstacksize", &self.sv502_irpstacksize)
            .field("sv502_maxrawbuflen", &self.sv502_maxrawbuflen)
            .field("sv502_sessusers", &self.sv502_sessusers)
            .field("sv502_sessconns", &self.sv502_sessconns)
            .field("sv502_maxpagedmemoryusage", &self.sv502_maxpagedmemoryusage)
            .field("sv502_maxnonpagedmemoryusage", &self.sv502_maxnonpagedmemoryusage)
            .field("sv502_enablesoftcompat", &self.sv502_enablesoftcompat)
            .field("sv502_enableforcedlogoff", &self.sv502_enableforcedlogoff)
            .field("sv502_timesource", &self.sv502_timesource)
            .field("sv502_acceptdownlevelapis", &self.sv502_acceptdownlevelapis)
            .field("sv502_lmannounce", &self.sv502_lmannounce)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_503 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_503 {
    fn eq(&self, other: &Self) -> bool {
        self.sv503_sessopens == other.sv503_sessopens
            && self.sv503_sessvcs == other.sv503_sessvcs
            && self.sv503_opensearch == other.sv503_opensearch
            && self.sv503_sizreqbuf == other.sv503_sizreqbuf
            && self.sv503_initworkitems == other.sv503_initworkitems
            && self.sv503_maxworkitems == other.sv503_maxworkitems
            && self.sv503_rawworkitems == other.sv503_rawworkitems
            && self.sv503_irpstacksize == other.sv503_irpstacksize
            && self.sv503_maxrawbuflen == other.sv503_maxrawbuflen
            && self.sv503_sessusers == other.sv503_sessusers
            && self.sv503_sessconns == other.sv503_sessconns
            && self.sv503_maxpagedmemoryusage == other.sv503_maxpagedmemoryusage
            && self.sv503_maxnonpagedmemoryusage == other.sv503_maxnonpagedmemoryusage
            && self.sv503_enablesoftcompat == other.sv503_enablesoftcompat
            && self.sv503_enableforcedlogoff == other.sv503_enableforcedlogoff
            && self.sv503_timesource == other.sv503_timesource
            && self.sv503_acceptdownlevelapis == other.sv503_acceptdownlevelapis
            && self.sv503_lmannounce == other.sv503_lmannounce
            && self.sv503_domain == other.sv503_domain
            && self.sv503_maxcopyreadlen == other.sv503_maxcopyreadlen
            && self.sv503_maxcopywritelen == other.sv503_maxcopywritelen
            && self.sv503_minkeepsearch == other.sv503_minkeepsearch
            && self.sv503_maxkeepsearch == other.sv503_maxkeepsearch
            && self.sv503_minkeepcomplsearch == other.sv503_minkeepcomplsearch
            && self.sv503_maxkeepcomplsearch == other.sv503_maxkeepcomplsearch
            && self.sv503_threadcountadd == other.sv503_threadcountadd
            && self.sv503_numblockthreads == other.sv503_numblockthreads
            && self.sv503_scavtimeout == other.sv503_scavtimeout
            && self.sv503_minrcvqueue == other.sv503_minrcvqueue
            && self.sv503_minfreeworkitems == other.sv503_minfreeworkitems
            && self.sv503_xactmemsize == other.sv503_xactmemsize
            && self.sv503_threadpriority == other.sv503_threadpriority
            && self.sv503_maxmpxct == other.sv503_maxmpxct
            && self.sv503_oplockbreakwait == other.sv503_oplockbreakwait
            && self.sv503_oplockbreakresponsewait == other.sv503_oplockbreakresponsewait
            && self.sv503_enableoplocks == other.sv503_enableoplocks
            && self.sv503_enableoplockforceclose == other.sv503_enableoplockforceclose
            && self.sv503_enablefcbopens == other.sv503_enablefcbopens
            && self.sv503_enableraw == other.sv503_enableraw
            && self.sv503_enablesharednetdrives == other.sv503_enablesharednetdrives
            && self.sv503_minfreeconnections == other.sv503_minfreeconnections
            && self.sv503_maxfreeconnections == other.sv503_maxfreeconnections
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_503 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_503 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_503")
            .field("sv503_sessopens", &self.sv503_sessopens)
            .field("sv503_sessvcs", &self.sv503_sessvcs)
            .field("sv503_opensearch", &self.sv503_opensearch)
            .field("sv503_sizreqbuf", &self.sv503_sizreqbuf)
            .field("sv503_initworkitems", &self.sv503_initworkitems)
            .field("sv503_maxworkitems", &self.sv503_maxworkitems)
            .field("sv503_rawworkitems", &self.sv503_rawworkitems)
            .field("sv503_irpstacksize", &self.sv503_irpstacksize)
            .field("sv503_maxrawbuflen", &self.sv503_maxrawbuflen)
            .field("sv503_sessusers", &self.sv503_sessusers)
            .field("sv503_sessconns", &self.sv503_sessconns)
            .field("sv503_maxpagedmemoryusage", &self.sv503_maxpagedmemoryusage)
            .field("sv503_maxnonpagedmemoryusage", &self.sv503_maxnonpagedmemoryusage)
            .field("sv503_enablesoftcompat", &self.sv503_enablesoftcompat)
            .field("sv503_enableforcedlogoff", &self.sv503_enableforcedlogoff)
            .field("sv503_timesource", &self.sv503_timesource)
            .field("sv503_acceptdownlevelapis", &self.sv503_acceptdownlevelapis)
            .field("sv503_lmannounce", &self.sv503_lmannounce)
            .field("sv503_domain", &self.sv503_domain)
            .field("sv503_maxcopyreadlen", &self.sv503_maxcopyreadlen)
            .field("sv503_maxcopywritelen", &self.sv503_maxcopywritelen)
            .field("sv503_minkeepsearch", &self.sv503_minkeepsearch)
            .field("sv503_maxkeepsearch", &self.sv503_maxkeepsearch)
            .field("sv503_minkeepcomplsearch", &self.sv503_minkeepcomplsearch)
            .field("sv503_maxkeepcomplsearch", &self.sv503_maxkeepcomplsearch)
            .field("sv503_threadcountadd", &self.sv503_threadcountadd)
            .field("sv503_numblockthreads", &self.sv503_numblockthreads)
            .field("sv503_scavtimeout", &self.sv503_scavtimeout)
            .field("sv503_minrcvqueue", &self.sv503_minrcvqueue)
            .field("sv503_minfreeworkitems", &self.sv503_minfreeworkitems)
            .field("sv503_xactmemsize", &self.sv503_xactmemsize)
            .field("sv503_threadpriority", &self.sv503_threadpriority)
            .field("sv503_maxmpxct", &self.sv503_maxmpxct)
            .field("sv503_oplockbreakwait", &self.sv503_oplockbreakwait)
            .field("sv503_oplockbreakresponsewait", &self.sv503_oplockbreakresponsewait)
            .field("sv503_enableoplocks", &self.sv503_enableoplocks)
            .field("sv503_enableoplockforceclose", &self.sv503_enableoplockforceclose)
            .field("sv503_enablefcbopens", &self.sv503_enablefcbopens)
            .field("sv503_enableraw", &self.sv503_enableraw)
            .field("sv503_enablesharednetdrives", &self.sv503_enablesharednetdrives)
            .field("sv503_minfreeconnections", &self.sv503_minfreeconnections)
            .field("sv503_maxfreeconnections", &self.sv503_maxfreeconnections)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_598 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_598 {
    fn eq(&self, other: &Self) -> bool {
        self.sv598_maxrawworkitems == other.sv598_maxrawworkitems
            && self.sv598_maxthreadsperqueue == other.sv598_maxthreadsperqueue
            && self.sv598_producttype == other.sv598_producttype
            && self.sv598_serversize == other.sv598_serversize
            && self.sv598_connectionlessautodisc == other.sv598_connectionlessautodisc
            && self.sv598_sharingviolationretries == other.sv598_sharingviolationretries
            && self.sv598_sharingviolationdelay == other.sv598_sharingviolationdelay
            && self.sv598_maxglobalopensearch == other.sv598_maxglobalopensearch
            && self.sv598_removeduplicatesearches == other.sv598_removeduplicatesearches
            && self.sv598_lockviolationoffset == other.sv598_lockviolationoffset
            && self.sv598_lockviolationdelay == other.sv598_lockviolationdelay
            && self.sv598_mdlreadswitchover == other.sv598_mdlreadswitchover
            && self.sv598_cachedopenlimit == other.sv598_cachedopenlimit
            && self.sv598_otherqueueaffinity == other.sv598_otherqueueaffinity
            && self.sv598_restrictnullsessaccess == other.sv598_restrictnullsessaccess
            && self.sv598_enablewfw311directipx == other.sv598_enablewfw311directipx
            && self.sv598_queuesamplesecs == other.sv598_queuesamplesecs
            && self.sv598_balancecount == other.sv598_balancecount
            && self.sv598_preferredaffinity == other.sv598_preferredaffinity
            && self.sv598_maxfreerfcbs == other.sv598_maxfreerfcbs
            && self.sv598_maxfreemfcbs == other.sv598_maxfreemfcbs
            && self.sv598_maxfreelfcbs == other.sv598_maxfreelfcbs
            && self.sv598_maxfreepagedpoolchunks == other.sv598_maxfreepagedpoolchunks
            && self.sv598_minpagedpoolchunksize == other.sv598_minpagedpoolchunksize
            && self.sv598_maxpagedpoolchunksize == other.sv598_maxpagedpoolchunksize
            && self.sv598_sendsfrompreferredprocessor == other.sv598_sendsfrompreferredprocessor
            && self.sv598_cacheddirectorylimit == other.sv598_cacheddirectorylimit
            && self.sv598_maxcopylength == other.sv598_maxcopylength
            && self.sv598_enablecompression == other.sv598_enablecompression
            && self.sv598_autosharewks == other.sv598_autosharewks
            && self.sv598_autoshareserver == other.sv598_autoshareserver
            && self.sv598_enablesecuritysignature == other.sv598_enablesecuritysignature
            && self.sv598_requiresecuritysignature == other.sv598_requiresecuritysignature
            && self.sv598_minclientbuffersize == other.sv598_minclientbuffersize
            && self.sv598_serverguid == other.sv598_serverguid
            && self.sv598_ConnectionNoSessionsTimeout == other.sv598_ConnectionNoSessionsTimeout
            && self.sv598_IdleThreadTimeOut == other.sv598_IdleThreadTimeOut
            && self.sv598_enableW9xsecuritysignature == other.sv598_enableW9xsecuritysignature
            && self.sv598_enforcekerberosreauthentication == other.sv598_enforcekerberosreauthentication
            && self.sv598_disabledos == other.sv598_disabledos
            && self.sv598_lowdiskspaceminimum == other.sv598_lowdiskspaceminimum
            && self.sv598_disablestrictnamechecking == other.sv598_disablestrictnamechecking
            && self.sv598_enableauthenticateusersharing == other.sv598_enableauthenticateusersharing
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_598 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_598 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_598")
            .field("sv598_maxrawworkitems", &self.sv598_maxrawworkitems)
            .field("sv598_maxthreadsperqueue", &self.sv598_maxthreadsperqueue)
            .field("sv598_producttype", &self.sv598_producttype)
            .field("sv598_serversize", &self.sv598_serversize)
            .field("sv598_connectionlessautodisc", &self.sv598_connectionlessautodisc)
            .field("sv598_sharingviolationretries", &self.sv598_sharingviolationretries)
            .field("sv598_sharingviolationdelay", &self.sv598_sharingviolationdelay)
            .field("sv598_maxglobalopensearch", &self.sv598_maxglobalopensearch)
            .field("sv598_removeduplicatesearches", &self.sv598_removeduplicatesearches)
            .field("sv598_lockviolationoffset", &self.sv598_lockviolationoffset)
            .field("sv598_lockviolationdelay", &self.sv598_lockviolationdelay)
            .field("sv598_mdlreadswitchover", &self.sv598_mdlreadswitchover)
            .field("sv598_cachedopenlimit", &self.sv598_cachedopenlimit)
            .field("sv598_otherqueueaffinity", &self.sv598_otherqueueaffinity)
            .field("sv598_restrictnullsessaccess", &self.sv598_restrictnullsessaccess)
            .field("sv598_enablewfw311directipx", &self.sv598_enablewfw311directipx)
            .field("sv598_queuesamplesecs", &self.sv598_queuesamplesecs)
            .field("sv598_balancecount", &self.sv598_balancecount)
            .field("sv598_preferredaffinity", &self.sv598_preferredaffinity)
            .field("sv598_maxfreerfcbs", &self.sv598_maxfreerfcbs)
            .field("sv598_maxfreemfcbs", &self.sv598_maxfreemfcbs)
            .field("sv598_maxfreelfcbs", &self.sv598_maxfreelfcbs)
            .field("sv598_maxfreepagedpoolchunks", &self.sv598_maxfreepagedpoolchunks)
            .field("sv598_minpagedpoolchunksize", &self.sv598_minpagedpoolchunksize)
            .field("sv598_maxpagedpoolchunksize", &self.sv598_maxpagedpoolchunksize)
            .field("sv598_sendsfrompreferredprocessor", &self.sv598_sendsfrompreferredprocessor)
            .field("sv598_cacheddirectorylimit", &self.sv598_cacheddirectorylimit)
            .field("sv598_maxcopylength", &self.sv598_maxcopylength)
            .field("sv598_enablecompression", &self.sv598_enablecompression)
            .field("sv598_autosharewks", &self.sv598_autosharewks)
            .field("sv598_autoshareserver", &self.sv598_autoshareserver)
            .field("sv598_enablesecuritysignature", &self.sv598_enablesecuritysignature)
            .field("sv598_requiresecuritysignature", &self.sv598_requiresecuritysignature)
            .field("sv598_minclientbuffersize", &self.sv598_minclientbuffersize)
            .field("sv598_serverguid", &self.sv598_serverguid)
            .field("sv598_ConnectionNoSessionsTimeout", &self.sv598_ConnectionNoSessionsTimeout)
            .field("sv598_IdleThreadTimeOut", &self.sv598_IdleThreadTimeOut)
            .field("sv598_enableW9xsecuritysignature", &self.sv598_enableW9xsecuritysignature)
            .field("sv598_enforcekerberosreauthentication", &self.sv598_enforcekerberosreauthentication)
            .field("sv598_disabledos", &self.sv598_disabledos)
            .field("sv598_lowdiskspaceminimum", &self.sv598_lowdiskspaceminimum)
            .field("sv598_disablestrictnamechecking", &self.sv598_disablestrictnamechecking)
            .field("sv598_enableauthenticateusersharing", &self.sv598_enableauthenticateusersharing)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVER_INFO_599 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVER_INFO_599 {
    fn eq(&self, other: &Self) -> bool {
        self.sv599_sessopens == other.sv599_sessopens
            && self.sv599_sessvcs == other.sv599_sessvcs
            && self.sv599_opensearch == other.sv599_opensearch
            && self.sv599_sizreqbuf == other.sv599_sizreqbuf
            && self.sv599_initworkitems == other.sv599_initworkitems
            && self.sv599_maxworkitems == other.sv599_maxworkitems
            && self.sv599_rawworkitems == other.sv599_rawworkitems
            && self.sv599_irpstacksize == other.sv599_irpstacksize
            && self.sv599_maxrawbuflen == other.sv599_maxrawbuflen
            && self.sv599_sessusers == other.sv599_sessusers
            && self.sv599_sessconns == other.sv599_sessconns
            && self.sv599_maxpagedmemoryusage == other.sv599_maxpagedmemoryusage
            && self.sv599_maxnonpagedmemoryusage == other.sv599_maxnonpagedmemoryusage
            && self.sv599_enablesoftcompat == other.sv599_enablesoftcompat
            && self.sv599_enableforcedlogoff == other.sv599_enableforcedlogoff
            && self.sv599_timesource == other.sv599_timesource
            && self.sv599_acceptdownlevelapis == other.sv599_acceptdownlevelapis
            && self.sv599_lmannounce == other.sv599_lmannounce
            && self.sv599_domain == other.sv599_domain
            && self.sv599_maxcopyreadlen == other.sv599_maxcopyreadlen
            && self.sv599_maxcopywritelen == other.sv599_maxcopywritelen
            && self.sv599_minkeepsearch == other.sv599_minkeepsearch
            && self.sv599_maxkeepsearch == other.sv599_maxkeepsearch
            && self.sv599_minkeepcomplsearch == other.sv599_minkeepcomplsearch
            && self.sv599_maxkeepcomplsearch == other.sv599_maxkeepcomplsearch
            && self.sv599_threadcountadd == other.sv599_threadcountadd
            && self.sv599_numblockthreads == other.sv599_numblockthreads
            && self.sv599_scavtimeout == other.sv599_scavtimeout
            && self.sv599_minrcvqueue == other.sv599_minrcvqueue
            && self.sv599_minfreeworkitems == other.sv599_minfreeworkitems
            && self.sv599_xactmemsize == other.sv599_xactmemsize
            && self.sv599_threadpriority == other.sv599_threadpriority
            && self.sv599_maxmpxct == other.sv599_maxmpxct
            && self.sv599_oplockbreakwait == other.sv599_oplockbreakwait
            && self.sv599_oplockbreakresponsewait == other.sv599_oplockbreakresponsewait
            && self.sv599_enableoplocks == other.sv599_enableoplocks
            && self.sv599_enableoplockforceclose == other.sv599_enableoplockforceclose
            && self.sv599_enablefcbopens == other.sv599_enablefcbopens
            && self.sv599_enableraw == other.sv599_enableraw
            && self.sv599_enablesharednetdrives == other.sv599_enablesharednetdrives
            && self.sv599_minfreeconnections == other.sv599_minfreeconnections
            && self.sv599_maxfreeconnections == other.sv599_maxfreeconnections
            && self.sv599_initsesstable == other.sv599_initsesstable
            && self.sv599_initconntable == other.sv599_initconntable
            && self.sv599_initfiletable == other.sv599_initfiletable
            && self.sv599_initsearchtable == other.sv599_initsearchtable
            && self.sv599_alertschedule == other.sv599_alertschedule
            && self.sv599_errorthreshold == other.sv599_errorthreshold
            && self.sv599_networkerrorthreshold == other.sv599_networkerrorthreshold
            && self.sv599_diskspacethreshold == other.sv599_diskspacethreshold
            && self.sv599_reserved == other.sv599_reserved
            && self.sv599_maxlinkdelay == other.sv599_maxlinkdelay
            && self.sv599_minlinkthroughput == other.sv599_minlinkthroughput
            && self.sv599_linkinfovalidtime == other.sv599_linkinfovalidtime
            && self.sv599_scavqosinfoupdatetime == other.sv599_scavqosinfoupdatetime
            && self.sv599_maxworkitemidletime == other.sv599_maxworkitemidletime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVER_INFO_599 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVER_INFO_599 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_INFO_599")
            .field("sv599_sessopens", &self.sv599_sessopens)
            .field("sv599_sessvcs", &self.sv599_sessvcs)
            .field("sv599_opensearch", &self.sv599_opensearch)
            .field("sv599_sizreqbuf", &self.sv599_sizreqbuf)
            .field("sv599_initworkitems", &self.sv599_initworkitems)
            .field("sv599_maxworkitems", &self.sv599_maxworkitems)
            .field("sv599_rawworkitems", &self.sv599_rawworkitems)
            .field("sv599_irpstacksize", &self.sv599_irpstacksize)
            .field("sv599_maxrawbuflen", &self.sv599_maxrawbuflen)
            .field("sv599_sessusers", &self.sv599_sessusers)
            .field("sv599_sessconns", &self.sv599_sessconns)
            .field("sv599_maxpagedmemoryusage", &self.sv599_maxpagedmemoryusage)
            .field("sv599_maxnonpagedmemoryusage", &self.sv599_maxnonpagedmemoryusage)
            .field("sv599_enablesoftcompat", &self.sv599_enablesoftcompat)
            .field("sv599_enableforcedlogoff", &self.sv599_enableforcedlogoff)
            .field("sv599_timesource", &self.sv599_timesource)
            .field("sv599_acceptdownlevelapis", &self.sv599_acceptdownlevelapis)
            .field("sv599_lmannounce", &self.sv599_lmannounce)
            .field("sv599_domain", &self.sv599_domain)
            .field("sv599_maxcopyreadlen", &self.sv599_maxcopyreadlen)
            .field("sv599_maxcopywritelen", &self.sv599_maxcopywritelen)
            .field("sv599_minkeepsearch", &self.sv599_minkeepsearch)
            .field("sv599_maxkeepsearch", &self.sv599_maxkeepsearch)
            .field("sv599_minkeepcomplsearch", &self.sv599_minkeepcomplsearch)
            .field("sv599_maxkeepcomplsearch", &self.sv599_maxkeepcomplsearch)
            .field("sv599_threadcountadd", &self.sv599_threadcountadd)
            .field("sv599_numblockthreads", &self.sv599_numblockthreads)
            .field("sv599_scavtimeout", &self.sv599_scavtimeout)
            .field("sv599_minrcvqueue", &self.sv599_minrcvqueue)
            .field("sv599_minfreeworkitems", &self.sv599_minfreeworkitems)
            .field("sv599_xactmemsize", &self.sv599_xactmemsize)
            .field("sv599_threadpriority", &self.sv599_threadpriority)
            .field("sv599_maxmpxct", &self.sv599_maxmpxct)
            .field("sv599_oplockbreakwait", &self.sv599_oplockbreakwait)
            .field("sv599_oplockbreakresponsewait", &self.sv599_oplockbreakresponsewait)
            .field("sv599_enableoplocks", &self.sv599_enableoplocks)
            .field("sv599_enableoplockforceclose", &self.sv599_enableoplockforceclose)
            .field("sv599_enablefcbopens", &self.sv599_enablefcbopens)
            .field("sv599_enableraw", &self.sv599_enableraw)
            .field("sv599_enablesharednetdrives", &self.sv599_enablesharednetdrives)
            .field("sv599_minfreeconnections", &self.sv599_minfreeconnections)
            .field("sv599_maxfreeconnections", &self.sv599_maxfreeconnections)
            .field("sv599_initsesstable", &self.sv599_initsesstable)
            .field("sv599_initconntable", &self.sv599_initconntable)
            .field("sv599_initfiletable", &self.sv599_initfiletable)
            .field("sv599_initsearchtable", &self.sv599_initsearchtable)
            .field("sv599_alertschedule", &self.sv599_alertschedule)
            .field("sv599_errorthreshold", &self.sv599_errorthreshold)
            .field("sv599_networkerrorthreshold", &self.sv599_networkerrorthreshold)
            .field("sv599_diskspacethreshold", &self.sv599_diskspacethreshold)
            .field("sv599_reserved", &self.sv599_reserved)
            .field("sv599_maxlinkdelay", &self.sv599_maxlinkdelay)
            .field("sv599_minlinkthroughput", &self.sv599_minlinkthroughput)
            .field("sv599_linkinfovalidtime", &self.sv599_linkinfovalidtime)
            .field("sv599_scavqosinfoupdatetime", &self.sv599_scavqosinfoupdatetime)
            .field("sv599_maxworkitemidletime", &self.sv599_maxworkitemidletime)
            .finish()
    }
}
impl ::core::default::Default for SERVER_INFO_HIDDEN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SERVER_INFO_HIDDEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERVER_INFO_HIDDEN").field(&self.0).finish()
    }
}
impl ::core::default::Default for SERVER_INFO_SECURITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SERVER_INFO_SECURITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERVER_INFO_SECURITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for SERVER_TRANSPORT_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_TRANSPORT_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.svti0_numberofvcs == other.svti0_numberofvcs && self.svti0_transportname == other.svti0_transportname && self.svti0_transportaddress == other.svti0_transportaddress && self.svti0_transportaddresslength == other.svti0_transportaddresslength && self.svti0_networkaddress == other.svti0_networkaddress
    }
}
impl ::core::cmp::Eq for SERVER_TRANSPORT_INFO_0 {}
impl ::core::fmt::Debug for SERVER_TRANSPORT_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_TRANSPORT_INFO_0").field("svti0_numberofvcs", &self.svti0_numberofvcs).field("svti0_transportname", &self.svti0_transportname).field("svti0_transportaddress", &self.svti0_transportaddress).field("svti0_transportaddresslength", &self.svti0_transportaddresslength).field("svti0_networkaddress", &self.svti0_networkaddress).finish()
    }
}
impl ::core::default::Default for SERVER_TRANSPORT_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_TRANSPORT_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.svti1_numberofvcs == other.svti1_numberofvcs && self.svti1_transportname == other.svti1_transportname && self.svti1_transportaddress == other.svti1_transportaddress && self.svti1_transportaddresslength == other.svti1_transportaddresslength && self.svti1_networkaddress == other.svti1_networkaddress && self.svti1_domain == other.svti1_domain
    }
}
impl ::core::cmp::Eq for SERVER_TRANSPORT_INFO_1 {}
impl ::core::fmt::Debug for SERVER_TRANSPORT_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_TRANSPORT_INFO_1").field("svti1_numberofvcs", &self.svti1_numberofvcs).field("svti1_transportname", &self.svti1_transportname).field("svti1_transportaddress", &self.svti1_transportaddress).field("svti1_transportaddresslength", &self.svti1_transportaddresslength).field("svti1_networkaddress", &self.svti1_networkaddress).field("svti1_domain", &self.svti1_domain).finish()
    }
}
impl ::core::default::Default for SERVER_TRANSPORT_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_TRANSPORT_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.svti2_numberofvcs == other.svti2_numberofvcs && self.svti2_transportname == other.svti2_transportname && self.svti2_transportaddress == other.svti2_transportaddress && self.svti2_transportaddresslength == other.svti2_transportaddresslength && self.svti2_networkaddress == other.svti2_networkaddress && self.svti2_domain == other.svti2_domain && self.svti2_flags == other.svti2_flags
    }
}
impl ::core::cmp::Eq for SERVER_TRANSPORT_INFO_2 {}
impl ::core::fmt::Debug for SERVER_TRANSPORT_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_TRANSPORT_INFO_2").field("svti2_numberofvcs", &self.svti2_numberofvcs).field("svti2_transportname", &self.svti2_transportname).field("svti2_transportaddress", &self.svti2_transportaddress).field("svti2_transportaddresslength", &self.svti2_transportaddresslength).field("svti2_networkaddress", &self.svti2_networkaddress).field("svti2_domain", &self.svti2_domain).field("svti2_flags", &self.svti2_flags).finish()
    }
}
impl ::core::default::Default for SERVER_TRANSPORT_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVER_TRANSPORT_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        self.svti3_numberofvcs == other.svti3_numberofvcs && self.svti3_transportname == other.svti3_transportname && self.svti3_transportaddress == other.svti3_transportaddress && self.svti3_transportaddresslength == other.svti3_transportaddresslength && self.svti3_networkaddress == other.svti3_networkaddress && self.svti3_domain == other.svti3_domain && self.svti3_flags == other.svti3_flags && self.svti3_passwordlength == other.svti3_passwordlength && self.svti3_password == other.svti3_password
    }
}
impl ::core::cmp::Eq for SERVER_TRANSPORT_INFO_3 {}
impl ::core::fmt::Debug for SERVER_TRANSPORT_INFO_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVER_TRANSPORT_INFO_3")
            .field("svti3_numberofvcs", &self.svti3_numberofvcs)
            .field("svti3_transportname", &self.svti3_transportname)
            .field("svti3_transportaddress", &self.svti3_transportaddress)
            .field("svti3_transportaddresslength", &self.svti3_transportaddresslength)
            .field("svti3_networkaddress", &self.svti3_networkaddress)
            .field("svti3_domain", &self.svti3_domain)
            .field("svti3_flags", &self.svti3_flags)
            .field("svti3_passwordlength", &self.svti3_passwordlength)
            .field("svti3_password", &self.svti3_password)
            .finish()
    }
}
impl ::core::default::Default for SERVICE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVICE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.svci0_name == other.svci0_name
    }
}
impl ::core::cmp::Eq for SERVICE_INFO_0 {}
impl ::core::fmt::Debug for SERVICE_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_INFO_0").field("svci0_name", &self.svci0_name).finish()
    }
}
impl ::core::default::Default for SERVICE_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVICE_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.svci1_name == other.svci1_name && self.svci1_status == other.svci1_status && self.svci1_code == other.svci1_code && self.svci1_pid == other.svci1_pid
    }
}
impl ::core::cmp::Eq for SERVICE_INFO_1 {}
impl ::core::fmt::Debug for SERVICE_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_INFO_1").field("svci1_name", &self.svci1_name).field("svci1_status", &self.svci1_status).field("svci1_code", &self.svci1_code).field("svci1_pid", &self.svci1_pid).finish()
    }
}
impl ::core::default::Default for SERVICE_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SERVICE_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.svci2_name == other.svci2_name && self.svci2_status == other.svci2_status && self.svci2_code == other.svci2_code && self.svci2_pid == other.svci2_pid && self.svci2_text == other.svci2_text && self.svci2_specific_error == other.svci2_specific_error && self.svci2_display_name == other.svci2_display_name
    }
}
impl ::core::cmp::Eq for SERVICE_INFO_2 {}
impl ::core::fmt::Debug for SERVICE_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_INFO_2").field("svci2_name", &self.svci2_name).field("svci2_status", &self.svci2_status).field("svci2_code", &self.svci2_code).field("svci2_pid", &self.svci2_pid).field("svci2_text", &self.svci2_text).field("svci2_specific_error", &self.svci2_specific_error).field("svci2_display_name", &self.svci2_display_name).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SMB_COMPRESSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SMB_COMPRESSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Switch == other.Switch && self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2 && self.Reserved3 == other.Reserved3
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SMB_COMPRESSION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SMB_COMPRESSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SMB_COMPRESSION_INFO").field("Switch", &self.Switch).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("Reserved3", &self.Reserved3).finish()
    }
}
impl ::core::default::Default for SMB_TREE_CONNECT_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SMB_TREE_CONNECT_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.EABufferOffset == other.EABufferOffset && self.EABufferLen == other.EABufferLen && self.CreateOptions == other.CreateOptions && self.TreeConnectAttributes == other.TreeConnectAttributes
    }
}
impl ::core::cmp::Eq for SMB_TREE_CONNECT_PARAMETERS {}
impl ::core::fmt::Debug for SMB_TREE_CONNECT_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SMB_TREE_CONNECT_PARAMETERS").field("EABufferOffset", &self.EABufferOffset).field("EABufferLen", &self.EABufferLen).field("CreateOptions", &self.CreateOptions).field("TreeConnectAttributes", &self.TreeConnectAttributes).finish()
    }
}
impl ::core::default::Default for SMB_USE_OPTION_COMPRESSION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SMB_USE_OPTION_COMPRESSION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Tag == other.Tag && self.Length == other.Length && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for SMB_USE_OPTION_COMPRESSION_PARAMETERS {}
impl ::core::fmt::Debug for SMB_USE_OPTION_COMPRESSION_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SMB_USE_OPTION_COMPRESSION_PARAMETERS").field("Tag", &self.Tag).field("Length", &self.Length).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for STD_ALERT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STD_ALERT {
    fn eq(&self, other: &Self) -> bool {
        self.alrt_timestamp == other.alrt_timestamp && self.alrt_eventname == other.alrt_eventname && self.alrt_servicename == other.alrt_servicename
    }
}
impl ::core::cmp::Eq for STD_ALERT {}
impl ::core::fmt::Debug for STD_ALERT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STD_ALERT").field("alrt_timestamp", &self.alrt_timestamp).field("alrt_eventname", &self.alrt_eventname).field("alrt_servicename", &self.alrt_servicename).finish()
    }
}
impl ::core::default::Default for SUPPORTS_BINDING_INTERFACE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SUPPORTS_BINDING_INTERFACE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SUPPORTS_BINDING_INTERFACE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for TIME_OF_DAY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TIME_OF_DAY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.tod_elapsedt == other.tod_elapsedt && self.tod_msecs == other.tod_msecs && self.tod_hours == other.tod_hours && self.tod_mins == other.tod_mins && self.tod_secs == other.tod_secs && self.tod_hunds == other.tod_hunds && self.tod_timezone == other.tod_timezone && self.tod_tinterval == other.tod_tinterval && self.tod_day == other.tod_day && self.tod_month == other.tod_month && self.tod_year == other.tod_year && self.tod_weekday == other.tod_weekday
    }
}
impl ::core::cmp::Eq for TIME_OF_DAY_INFO {}
impl ::core::fmt::Debug for TIME_OF_DAY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TIME_OF_DAY_INFO")
            .field("tod_elapsedt", &self.tod_elapsedt)
            .field("tod_msecs", &self.tod_msecs)
            .field("tod_hours", &self.tod_hours)
            .field("tod_mins", &self.tod_mins)
            .field("tod_secs", &self.tod_secs)
            .field("tod_hunds", &self.tod_hunds)
            .field("tod_timezone", &self.tod_timezone)
            .field("tod_tinterval", &self.tod_tinterval)
            .field("tod_day", &self.tod_day)
            .field("tod_month", &self.tod_month)
            .field("tod_year", &self.tod_year)
            .field("tod_weekday", &self.tod_weekday)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TRANSPORT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TRANSPORT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.SkipCertificateCheck == other.SkipCertificateCheck
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TRANSPORT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TRANSPORT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSPORT_INFO").field("Type", &self.Type).field("SkipCertificateCheck", &self.SkipCertificateCheck).finish()
    }
}
impl ::core::default::Default for TRANSPORT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TRANSPORT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRANSPORT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for USER_ACCOUNT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USER_ACCOUNT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USER_ACCOUNT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for USER_ACCOUNT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for USER_ACCOUNT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for USER_ACCOUNT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for USER_ACCOUNT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for USER_ACCOUNT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for USER_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.usri0_name == other.usri0_name
    }
}
impl ::core::cmp::Eq for USER_INFO_0 {}
impl ::core::fmt::Debug for USER_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_0").field("usri0_name", &self.usri0_name).finish()
    }
}
impl ::core::default::Default for USER_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1_name == other.usri1_name && self.usri1_password == other.usri1_password && self.usri1_password_age == other.usri1_password_age && self.usri1_priv == other.usri1_priv && self.usri1_home_dir == other.usri1_home_dir && self.usri1_comment == other.usri1_comment && self.usri1_flags == other.usri1_flags && self.usri1_script_path == other.usri1_script_path
    }
}
impl ::core::cmp::Eq for USER_INFO_1 {}
impl ::core::fmt::Debug for USER_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1").field("usri1_name", &self.usri1_name).field("usri1_password", &self.usri1_password).field("usri1_password_age", &self.usri1_password_age).field("usri1_priv", &self.usri1_priv).field("usri1_home_dir", &self.usri1_home_dir).field("usri1_comment", &self.usri1_comment).field("usri1_flags", &self.usri1_flags).field("usri1_script_path", &self.usri1_script_path).finish()
    }
}
impl ::core::default::Default for USER_INFO_10 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_INFO_10 {
    fn eq(&self, other: &Self) -> bool {
        self.usri10_name == other.usri10_name && self.usri10_comment == other.usri10_comment && self.usri10_usr_comment == other.usri10_usr_comment && self.usri10_full_name == other.usri10_full_name
    }
}
impl ::core::cmp::Eq for USER_INFO_10 {}
impl ::core::fmt::Debug for USER_INFO_10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_10").field("usri10_name", &self.usri10_name).field("usri10_comment", &self.usri10_comment).field("usri10_usr_comment", &self.usri10_usr_comment).field("usri10_full_name", &self.usri10_full_name).finish()
    }
}
impl ::core::default::Default for USER_INFO_1003 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_INFO_1003 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1003_password == other.usri1003_password
    }
}
impl ::core::cmp::Eq for USER_INFO_1003 {}
impl ::core::fmt::Debug for USER_INFO_1003 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1003").field("usri1003_password", &self.usri1003_password).finish()
    }
}
impl ::core::default::Default for USER_INFO_1005 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_INFO_1005 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1005_priv == other.usri1005_priv
    }
}
impl ::core::cmp::Eq for USER_INFO_1005 {}
impl ::core::fmt::Debug for USER_INFO_1005 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1005").field("usri1005_priv", &self.usri1005_priv).finish()
    }
}
impl ::core::default::Default for USER_INFO_1006 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_INFO_1006 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1006_home_dir == other.usri1006_home_dir
    }
}
impl ::core::cmp::Eq for USER_INFO_1006 {}
impl ::core::fmt::Debug for USER_INFO_1006 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1006").field("usri1006_home_dir", &self.usri1006_home_dir).finish()
    }
}
impl ::core::default::Default for USER_INFO_1007 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_INFO_1007 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1007_comment == other.usri1007_comment
    }
}
impl ::core::cmp::Eq for USER_INFO_1007 {}
impl ::core::fmt::Debug for USER_INFO_1007 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1007").field("usri1007_comment", &self.usri1007_comment).finish()
    }
}
impl ::core::default::Default for USER_INFO_1008 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_INFO_1008 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1008_flags == other.usri1008_flags
    }
}
impl ::core::cmp::Eq for USER_INFO_1008 {}
impl ::core::fmt::Debug for USER_INFO_1008 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1008").field("usri1008_flags", &self.usri1008_flags).finish()
    }
}
impl ::core::default::Default for USER_INFO_1009 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_INFO_1009 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1009_script_path == other.usri1009_script_path
    }
}
impl ::core::cmp::Eq for USER_INFO_1009 {}
impl ::core::fmt::Debug for USER_INFO_1009 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1009").field("usri1009_script_path", &self.usri1009_script_path).finish()
    }
}
impl ::core::default::Default for USER_INFO_1010 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_INFO_1010 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1010_auth_flags == other.usri1010_auth_flags
    }
}
impl ::core::cmp::Eq for USER_INFO_1010 {}
impl ::core::fmt::Debug for USER_INFO_1010 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1010").field("usri1010_auth_flags", &self.usri1010_auth_flags).finish()
    }
}
impl ::core::default::Default for USER_INFO_1011 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_INFO_1011 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1011_full_name == other.usri1011_full_name
    }
}
impl ::core::cmp::Eq for USER_INFO_1011 {}
impl ::core::fmt::Debug for USER_INFO_1011 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1011").field("usri1011_full_name", &self.usri1011_full_name).finish()
    }
}
impl ::core::default::Default for USER_INFO_1012 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_INFO_1012 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1012_usr_comment == other.usri1012_usr_comment
    }
}
impl ::core::cmp::Eq for USER_INFO_1012 {}
impl ::core::fmt::Debug for USER_INFO_1012 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1012").field("usri1012_usr_comment", &self.usri1012_usr_comment).finish()
    }
}
impl ::core::default::Default for USER_INFO_1013 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_INFO_1013 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1013_parms == other.usri1013_parms
    }
}
impl ::core::cmp::Eq for USER_INFO_1013 {}
impl ::core::fmt::Debug for USER_INFO_1013 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1013").field("usri1013_parms", &self.usri1013_parms).finish()
    }
}
impl ::core::default::Default for USER_INFO_1014 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_INFO_1014 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1014_workstations == other.usri1014_workstations
    }
}
impl ::core::cmp::Eq for USER_INFO_1014 {}
impl ::core::fmt::Debug for USER_INFO_1014 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1014").field("usri1014_workstations", &self.usri1014_workstations).finish()
    }
}
impl ::core::default::Default for USER_INFO_1017 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_INFO_1017 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1017_acct_expires == other.usri1017_acct_expires
    }
}
impl ::core::cmp::Eq for USER_INFO_1017 {}
impl ::core::fmt::Debug for USER_INFO_1017 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1017").field("usri1017_acct_expires", &self.usri1017_acct_expires).finish()
    }
}
impl ::core::default::Default for USER_INFO_1018 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_INFO_1018 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1018_max_storage == other.usri1018_max_storage
    }
}
impl ::core::cmp::Eq for USER_INFO_1018 {}
impl ::core::fmt::Debug for USER_INFO_1018 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1018").field("usri1018_max_storage", &self.usri1018_max_storage).finish()
    }
}
impl ::core::default::Default for USER_INFO_1020 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_INFO_1020 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1020_units_per_week == other.usri1020_units_per_week && self.usri1020_logon_hours == other.usri1020_logon_hours
    }
}
impl ::core::cmp::Eq for USER_INFO_1020 {}
impl ::core::fmt::Debug for USER_INFO_1020 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1020").field("usri1020_units_per_week", &self.usri1020_units_per_week).field("usri1020_logon_hours", &self.usri1020_logon_hours).finish()
    }
}
impl ::core::default::Default for USER_INFO_1023 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_INFO_1023 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1023_logon_server == other.usri1023_logon_server
    }
}
impl ::core::cmp::Eq for USER_INFO_1023 {}
impl ::core::fmt::Debug for USER_INFO_1023 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1023").field("usri1023_logon_server", &self.usri1023_logon_server).finish()
    }
}
impl ::core::default::Default for USER_INFO_1024 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_INFO_1024 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1024_country_code == other.usri1024_country_code
    }
}
impl ::core::cmp::Eq for USER_INFO_1024 {}
impl ::core::fmt::Debug for USER_INFO_1024 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1024").field("usri1024_country_code", &self.usri1024_country_code).finish()
    }
}
impl ::core::default::Default for USER_INFO_1025 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_INFO_1025 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1025_code_page == other.usri1025_code_page
    }
}
impl ::core::cmp::Eq for USER_INFO_1025 {}
impl ::core::fmt::Debug for USER_INFO_1025 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1025").field("usri1025_code_page", &self.usri1025_code_page).finish()
    }
}
impl ::core::default::Default for USER_INFO_1051 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_INFO_1051 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1051_primary_group_id == other.usri1051_primary_group_id
    }
}
impl ::core::cmp::Eq for USER_INFO_1051 {}
impl ::core::fmt::Debug for USER_INFO_1051 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1051").field("usri1051_primary_group_id", &self.usri1051_primary_group_id).finish()
    }
}
impl ::core::default::Default for USER_INFO_1052 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_INFO_1052 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1052_profile == other.usri1052_profile
    }
}
impl ::core::cmp::Eq for USER_INFO_1052 {}
impl ::core::fmt::Debug for USER_INFO_1052 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1052").field("usri1052_profile", &self.usri1052_profile).finish()
    }
}
impl ::core::default::Default for USER_INFO_1053 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_INFO_1053 {
    fn eq(&self, other: &Self) -> bool {
        self.usri1053_home_dir_drive == other.usri1053_home_dir_drive
    }
}
impl ::core::cmp::Eq for USER_INFO_1053 {}
impl ::core::fmt::Debug for USER_INFO_1053 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_1053").field("usri1053_home_dir_drive", &self.usri1053_home_dir_drive).finish()
    }
}
impl ::core::default::Default for USER_INFO_11 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_INFO_11 {
    fn eq(&self, other: &Self) -> bool {
        self.usri11_name == other.usri11_name
            && self.usri11_comment == other.usri11_comment
            && self.usri11_usr_comment == other.usri11_usr_comment
            && self.usri11_full_name == other.usri11_full_name
            && self.usri11_priv == other.usri11_priv
            && self.usri11_auth_flags == other.usri11_auth_flags
            && self.usri11_password_age == other.usri11_password_age
            && self.usri11_home_dir == other.usri11_home_dir
            && self.usri11_parms == other.usri11_parms
            && self.usri11_last_logon == other.usri11_last_logon
            && self.usri11_last_logoff == other.usri11_last_logoff
            && self.usri11_bad_pw_count == other.usri11_bad_pw_count
            && self.usri11_num_logons == other.usri11_num_logons
            && self.usri11_logon_server == other.usri11_logon_server
            && self.usri11_country_code == other.usri11_country_code
            && self.usri11_workstations == other.usri11_workstations
            && self.usri11_max_storage == other.usri11_max_storage
            && self.usri11_units_per_week == other.usri11_units_per_week
            && self.usri11_logon_hours == other.usri11_logon_hours
            && self.usri11_code_page == other.usri11_code_page
    }
}
impl ::core::cmp::Eq for USER_INFO_11 {}
impl ::core::fmt::Debug for USER_INFO_11 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_11")
            .field("usri11_name", &self.usri11_name)
            .field("usri11_comment", &self.usri11_comment)
            .field("usri11_usr_comment", &self.usri11_usr_comment)
            .field("usri11_full_name", &self.usri11_full_name)
            .field("usri11_priv", &self.usri11_priv)
            .field("usri11_auth_flags", &self.usri11_auth_flags)
            .field("usri11_password_age", &self.usri11_password_age)
            .field("usri11_home_dir", &self.usri11_home_dir)
            .field("usri11_parms", &self.usri11_parms)
            .field("usri11_last_logon", &self.usri11_last_logon)
            .field("usri11_last_logoff", &self.usri11_last_logoff)
            .field("usri11_bad_pw_count", &self.usri11_bad_pw_count)
            .field("usri11_num_logons", &self.usri11_num_logons)
            .field("usri11_logon_server", &self.usri11_logon_server)
            .field("usri11_country_code", &self.usri11_country_code)
            .field("usri11_workstations", &self.usri11_workstations)
            .field("usri11_max_storage", &self.usri11_max_storage)
            .field("usri11_units_per_week", &self.usri11_units_per_week)
            .field("usri11_logon_hours", &self.usri11_logon_hours)
            .field("usri11_code_page", &self.usri11_code_page)
            .finish()
    }
}
impl ::core::default::Default for USER_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.usri2_name == other.usri2_name
            && self.usri2_password == other.usri2_password
            && self.usri2_password_age == other.usri2_password_age
            && self.usri2_priv == other.usri2_priv
            && self.usri2_home_dir == other.usri2_home_dir
            && self.usri2_comment == other.usri2_comment
            && self.usri2_flags == other.usri2_flags
            && self.usri2_script_path == other.usri2_script_path
            && self.usri2_auth_flags == other.usri2_auth_flags
            && self.usri2_full_name == other.usri2_full_name
            && self.usri2_usr_comment == other.usri2_usr_comment
            && self.usri2_parms == other.usri2_parms
            && self.usri2_workstations == other.usri2_workstations
            && self.usri2_last_logon == other.usri2_last_logon
            && self.usri2_last_logoff == other.usri2_last_logoff
            && self.usri2_acct_expires == other.usri2_acct_expires
            && self.usri2_max_storage == other.usri2_max_storage
            && self.usri2_units_per_week == other.usri2_units_per_week
            && self.usri2_logon_hours == other.usri2_logon_hours
            && self.usri2_bad_pw_count == other.usri2_bad_pw_count
            && self.usri2_num_logons == other.usri2_num_logons
            && self.usri2_logon_server == other.usri2_logon_server
            && self.usri2_country_code == other.usri2_country_code
            && self.usri2_code_page == other.usri2_code_page
    }
}
impl ::core::cmp::Eq for USER_INFO_2 {}
impl ::core::fmt::Debug for USER_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_2")
            .field("usri2_name", &self.usri2_name)
            .field("usri2_password", &self.usri2_password)
            .field("usri2_password_age", &self.usri2_password_age)
            .field("usri2_priv", &self.usri2_priv)
            .field("usri2_home_dir", &self.usri2_home_dir)
            .field("usri2_comment", &self.usri2_comment)
            .field("usri2_flags", &self.usri2_flags)
            .field("usri2_script_path", &self.usri2_script_path)
            .field("usri2_auth_flags", &self.usri2_auth_flags)
            .field("usri2_full_name", &self.usri2_full_name)
            .field("usri2_usr_comment", &self.usri2_usr_comment)
            .field("usri2_parms", &self.usri2_parms)
            .field("usri2_workstations", &self.usri2_workstations)
            .field("usri2_last_logon", &self.usri2_last_logon)
            .field("usri2_last_logoff", &self.usri2_last_logoff)
            .field("usri2_acct_expires", &self.usri2_acct_expires)
            .field("usri2_max_storage", &self.usri2_max_storage)
            .field("usri2_units_per_week", &self.usri2_units_per_week)
            .field("usri2_logon_hours", &self.usri2_logon_hours)
            .field("usri2_bad_pw_count", &self.usri2_bad_pw_count)
            .field("usri2_num_logons", &self.usri2_num_logons)
            .field("usri2_logon_server", &self.usri2_logon_server)
            .field("usri2_country_code", &self.usri2_country_code)
            .field("usri2_code_page", &self.usri2_code_page)
            .finish()
    }
}
impl ::core::default::Default for USER_INFO_20 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_INFO_20 {
    fn eq(&self, other: &Self) -> bool {
        self.usri20_name == other.usri20_name && self.usri20_full_name == other.usri20_full_name && self.usri20_comment == other.usri20_comment && self.usri20_flags == other.usri20_flags && self.usri20_user_id == other.usri20_user_id
    }
}
impl ::core::cmp::Eq for USER_INFO_20 {}
impl ::core::fmt::Debug for USER_INFO_20 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_20").field("usri20_name", &self.usri20_name).field("usri20_full_name", &self.usri20_full_name).field("usri20_comment", &self.usri20_comment).field("usri20_flags", &self.usri20_flags).field("usri20_user_id", &self.usri20_user_id).finish()
    }
}
impl ::core::default::Default for USER_INFO_21 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_INFO_21 {
    fn eq(&self, other: &Self) -> bool {
        self.usri21_password == other.usri21_password
    }
}
impl ::core::cmp::Eq for USER_INFO_21 {}
impl ::core::fmt::Debug for USER_INFO_21 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_21").field("usri21_password", &self.usri21_password).finish()
    }
}
impl ::core::default::Default for USER_INFO_22 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_INFO_22 {
    fn eq(&self, other: &Self) -> bool {
        self.usri22_name == other.usri22_name
            && self.usri22_password == other.usri22_password
            && self.usri22_password_age == other.usri22_password_age
            && self.usri22_priv == other.usri22_priv
            && self.usri22_home_dir == other.usri22_home_dir
            && self.usri22_comment == other.usri22_comment
            && self.usri22_flags == other.usri22_flags
            && self.usri22_script_path == other.usri22_script_path
            && self.usri22_auth_flags == other.usri22_auth_flags
            && self.usri22_full_name == other.usri22_full_name
            && self.usri22_usr_comment == other.usri22_usr_comment
            && self.usri22_parms == other.usri22_parms
            && self.usri22_workstations == other.usri22_workstations
            && self.usri22_last_logon == other.usri22_last_logon
            && self.usri22_last_logoff == other.usri22_last_logoff
            && self.usri22_acct_expires == other.usri22_acct_expires
            && self.usri22_max_storage == other.usri22_max_storage
            && self.usri22_units_per_week == other.usri22_units_per_week
            && self.usri22_logon_hours == other.usri22_logon_hours
            && self.usri22_bad_pw_count == other.usri22_bad_pw_count
            && self.usri22_num_logons == other.usri22_num_logons
            && self.usri22_logon_server == other.usri22_logon_server
            && self.usri22_country_code == other.usri22_country_code
            && self.usri22_code_page == other.usri22_code_page
    }
}
impl ::core::cmp::Eq for USER_INFO_22 {}
impl ::core::fmt::Debug for USER_INFO_22 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_22")
            .field("usri22_name", &self.usri22_name)
            .field("usri22_password", &self.usri22_password)
            .field("usri22_password_age", &self.usri22_password_age)
            .field("usri22_priv", &self.usri22_priv)
            .field("usri22_home_dir", &self.usri22_home_dir)
            .field("usri22_comment", &self.usri22_comment)
            .field("usri22_flags", &self.usri22_flags)
            .field("usri22_script_path", &self.usri22_script_path)
            .field("usri22_auth_flags", &self.usri22_auth_flags)
            .field("usri22_full_name", &self.usri22_full_name)
            .field("usri22_usr_comment", &self.usri22_usr_comment)
            .field("usri22_parms", &self.usri22_parms)
            .field("usri22_workstations", &self.usri22_workstations)
            .field("usri22_last_logon", &self.usri22_last_logon)
            .field("usri22_last_logoff", &self.usri22_last_logoff)
            .field("usri22_acct_expires", &self.usri22_acct_expires)
            .field("usri22_max_storage", &self.usri22_max_storage)
            .field("usri22_units_per_week", &self.usri22_units_per_week)
            .field("usri22_logon_hours", &self.usri22_logon_hours)
            .field("usri22_bad_pw_count", &self.usri22_bad_pw_count)
            .field("usri22_num_logons", &self.usri22_num_logons)
            .field("usri22_logon_server", &self.usri22_logon_server)
            .field("usri22_country_code", &self.usri22_country_code)
            .field("usri22_code_page", &self.usri22_code_page)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_INFO_23 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_INFO_23 {
    fn eq(&self, other: &Self) -> bool {
        self.usri23_name == other.usri23_name && self.usri23_full_name == other.usri23_full_name && self.usri23_comment == other.usri23_comment && self.usri23_flags == other.usri23_flags && self.usri23_user_sid == other.usri23_user_sid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_INFO_23 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for USER_INFO_23 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_23").field("usri23_name", &self.usri23_name).field("usri23_full_name", &self.usri23_full_name).field("usri23_comment", &self.usri23_comment).field("usri23_flags", &self.usri23_flags).field("usri23_user_sid", &self.usri23_user_sid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_INFO_24 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_INFO_24 {
    fn eq(&self, other: &Self) -> bool {
        self.usri24_internet_identity == other.usri24_internet_identity && self.usri24_flags == other.usri24_flags && self.usri24_internet_provider_name == other.usri24_internet_provider_name && self.usri24_internet_principal_name == other.usri24_internet_principal_name && self.usri24_user_sid == other.usri24_user_sid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_INFO_24 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for USER_INFO_24 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_24").field("usri24_internet_identity", &self.usri24_internet_identity).field("usri24_flags", &self.usri24_flags).field("usri24_internet_provider_name", &self.usri24_internet_provider_name).field("usri24_internet_principal_name", &self.usri24_internet_principal_name).field("usri24_user_sid", &self.usri24_user_sid).finish()
    }
}
impl ::core::default::Default for USER_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        self.usri3_name == other.usri3_name
            && self.usri3_password == other.usri3_password
            && self.usri3_password_age == other.usri3_password_age
            && self.usri3_priv == other.usri3_priv
            && self.usri3_home_dir == other.usri3_home_dir
            && self.usri3_comment == other.usri3_comment
            && self.usri3_flags == other.usri3_flags
            && self.usri3_script_path == other.usri3_script_path
            && self.usri3_auth_flags == other.usri3_auth_flags
            && self.usri3_full_name == other.usri3_full_name
            && self.usri3_usr_comment == other.usri3_usr_comment
            && self.usri3_parms == other.usri3_parms
            && self.usri3_workstations == other.usri3_workstations
            && self.usri3_last_logon == other.usri3_last_logon
            && self.usri3_last_logoff == other.usri3_last_logoff
            && self.usri3_acct_expires == other.usri3_acct_expires
            && self.usri3_max_storage == other.usri3_max_storage
            && self.usri3_units_per_week == other.usri3_units_per_week
            && self.usri3_logon_hours == other.usri3_logon_hours
            && self.usri3_bad_pw_count == other.usri3_bad_pw_count
            && self.usri3_num_logons == other.usri3_num_logons
            && self.usri3_logon_server == other.usri3_logon_server
            && self.usri3_country_code == other.usri3_country_code
            && self.usri3_code_page == other.usri3_code_page
            && self.usri3_user_id == other.usri3_user_id
            && self.usri3_primary_group_id == other.usri3_primary_group_id
            && self.usri3_profile == other.usri3_profile
            && self.usri3_home_dir_drive == other.usri3_home_dir_drive
            && self.usri3_password_expired == other.usri3_password_expired
    }
}
impl ::core::cmp::Eq for USER_INFO_3 {}
impl ::core::fmt::Debug for USER_INFO_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_3")
            .field("usri3_name", &self.usri3_name)
            .field("usri3_password", &self.usri3_password)
            .field("usri3_password_age", &self.usri3_password_age)
            .field("usri3_priv", &self.usri3_priv)
            .field("usri3_home_dir", &self.usri3_home_dir)
            .field("usri3_comment", &self.usri3_comment)
            .field("usri3_flags", &self.usri3_flags)
            .field("usri3_script_path", &self.usri3_script_path)
            .field("usri3_auth_flags", &self.usri3_auth_flags)
            .field("usri3_full_name", &self.usri3_full_name)
            .field("usri3_usr_comment", &self.usri3_usr_comment)
            .field("usri3_parms", &self.usri3_parms)
            .field("usri3_workstations", &self.usri3_workstations)
            .field("usri3_last_logon", &self.usri3_last_logon)
            .field("usri3_last_logoff", &self.usri3_last_logoff)
            .field("usri3_acct_expires", &self.usri3_acct_expires)
            .field("usri3_max_storage", &self.usri3_max_storage)
            .field("usri3_units_per_week", &self.usri3_units_per_week)
            .field("usri3_logon_hours", &self.usri3_logon_hours)
            .field("usri3_bad_pw_count", &self.usri3_bad_pw_count)
            .field("usri3_num_logons", &self.usri3_num_logons)
            .field("usri3_logon_server", &self.usri3_logon_server)
            .field("usri3_country_code", &self.usri3_country_code)
            .field("usri3_code_page", &self.usri3_code_page)
            .field("usri3_user_id", &self.usri3_user_id)
            .field("usri3_primary_group_id", &self.usri3_primary_group_id)
            .field("usri3_profile", &self.usri3_profile)
            .field("usri3_home_dir_drive", &self.usri3_home_dir_drive)
            .field("usri3_password_expired", &self.usri3_password_expired)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_INFO_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_INFO_4 {
    fn eq(&self, other: &Self) -> bool {
        self.usri4_name == other.usri4_name
            && self.usri4_password == other.usri4_password
            && self.usri4_password_age == other.usri4_password_age
            && self.usri4_priv == other.usri4_priv
            && self.usri4_home_dir == other.usri4_home_dir
            && self.usri4_comment == other.usri4_comment
            && self.usri4_flags == other.usri4_flags
            && self.usri4_script_path == other.usri4_script_path
            && self.usri4_auth_flags == other.usri4_auth_flags
            && self.usri4_full_name == other.usri4_full_name
            && self.usri4_usr_comment == other.usri4_usr_comment
            && self.usri4_parms == other.usri4_parms
            && self.usri4_workstations == other.usri4_workstations
            && self.usri4_last_logon == other.usri4_last_logon
            && self.usri4_last_logoff == other.usri4_last_logoff
            && self.usri4_acct_expires == other.usri4_acct_expires
            && self.usri4_max_storage == other.usri4_max_storage
            && self.usri4_units_per_week == other.usri4_units_per_week
            && self.usri4_logon_hours == other.usri4_logon_hours
            && self.usri4_bad_pw_count == other.usri4_bad_pw_count
            && self.usri4_num_logons == other.usri4_num_logons
            && self.usri4_logon_server == other.usri4_logon_server
            && self.usri4_country_code == other.usri4_country_code
            && self.usri4_code_page == other.usri4_code_page
            && self.usri4_user_sid == other.usri4_user_sid
            && self.usri4_primary_group_id == other.usri4_primary_group_id
            && self.usri4_profile == other.usri4_profile
            && self.usri4_home_dir_drive == other.usri4_home_dir_drive
            && self.usri4_password_expired == other.usri4_password_expired
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_INFO_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for USER_INFO_4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_INFO_4")
            .field("usri4_name", &self.usri4_name)
            .field("usri4_password", &self.usri4_password)
            .field("usri4_password_age", &self.usri4_password_age)
            .field("usri4_priv", &self.usri4_priv)
            .field("usri4_home_dir", &self.usri4_home_dir)
            .field("usri4_comment", &self.usri4_comment)
            .field("usri4_flags", &self.usri4_flags)
            .field("usri4_script_path", &self.usri4_script_path)
            .field("usri4_auth_flags", &self.usri4_auth_flags)
            .field("usri4_full_name", &self.usri4_full_name)
            .field("usri4_usr_comment", &self.usri4_usr_comment)
            .field("usri4_parms", &self.usri4_parms)
            .field("usri4_workstations", &self.usri4_workstations)
            .field("usri4_last_logon", &self.usri4_last_logon)
            .field("usri4_last_logoff", &self.usri4_last_logoff)
            .field("usri4_acct_expires", &self.usri4_acct_expires)
            .field("usri4_max_storage", &self.usri4_max_storage)
            .field("usri4_units_per_week", &self.usri4_units_per_week)
            .field("usri4_logon_hours", &self.usri4_logon_hours)
            .field("usri4_bad_pw_count", &self.usri4_bad_pw_count)
            .field("usri4_num_logons", &self.usri4_num_logons)
            .field("usri4_logon_server", &self.usri4_logon_server)
            .field("usri4_country_code", &self.usri4_country_code)
            .field("usri4_code_page", &self.usri4_code_page)
            .field("usri4_user_sid", &self.usri4_user_sid)
            .field("usri4_primary_group_id", &self.usri4_primary_group_id)
            .field("usri4_profile", &self.usri4_profile)
            .field("usri4_home_dir_drive", &self.usri4_home_dir_drive)
            .field("usri4_password_expired", &self.usri4_password_expired)
            .finish()
    }
}
impl ::core::default::Default for USER_MODALS_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_MODALS_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.usrmod0_min_passwd_len == other.usrmod0_min_passwd_len && self.usrmod0_max_passwd_age == other.usrmod0_max_passwd_age && self.usrmod0_min_passwd_age == other.usrmod0_min_passwd_age && self.usrmod0_force_logoff == other.usrmod0_force_logoff && self.usrmod0_password_hist_len == other.usrmod0_password_hist_len
    }
}
impl ::core::cmp::Eq for USER_MODALS_INFO_0 {}
impl ::core::fmt::Debug for USER_MODALS_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_MODALS_INFO_0").field("usrmod0_min_passwd_len", &self.usrmod0_min_passwd_len).field("usrmod0_max_passwd_age", &self.usrmod0_max_passwd_age).field("usrmod0_min_passwd_age", &self.usrmod0_min_passwd_age).field("usrmod0_force_logoff", &self.usrmod0_force_logoff).field("usrmod0_password_hist_len", &self.usrmod0_password_hist_len).finish()
    }
}
impl ::core::default::Default for USER_MODALS_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_MODALS_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.usrmod1_role == other.usrmod1_role && self.usrmod1_primary == other.usrmod1_primary
    }
}
impl ::core::cmp::Eq for USER_MODALS_INFO_1 {}
impl ::core::fmt::Debug for USER_MODALS_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_MODALS_INFO_1").field("usrmod1_role", &self.usrmod1_role).field("usrmod1_primary", &self.usrmod1_primary).finish()
    }
}
impl ::core::default::Default for USER_MODALS_INFO_1001 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_MODALS_INFO_1001 {
    fn eq(&self, other: &Self) -> bool {
        self.usrmod1001_min_passwd_len == other.usrmod1001_min_passwd_len
    }
}
impl ::core::cmp::Eq for USER_MODALS_INFO_1001 {}
impl ::core::fmt::Debug for USER_MODALS_INFO_1001 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_MODALS_INFO_1001").field("usrmod1001_min_passwd_len", &self.usrmod1001_min_passwd_len).finish()
    }
}
impl ::core::default::Default for USER_MODALS_INFO_1002 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_MODALS_INFO_1002 {
    fn eq(&self, other: &Self) -> bool {
        self.usrmod1002_max_passwd_age == other.usrmod1002_max_passwd_age
    }
}
impl ::core::cmp::Eq for USER_MODALS_INFO_1002 {}
impl ::core::fmt::Debug for USER_MODALS_INFO_1002 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_MODALS_INFO_1002").field("usrmod1002_max_passwd_age", &self.usrmod1002_max_passwd_age).finish()
    }
}
impl ::core::default::Default for USER_MODALS_INFO_1003 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_MODALS_INFO_1003 {
    fn eq(&self, other: &Self) -> bool {
        self.usrmod1003_min_passwd_age == other.usrmod1003_min_passwd_age
    }
}
impl ::core::cmp::Eq for USER_MODALS_INFO_1003 {}
impl ::core::fmt::Debug for USER_MODALS_INFO_1003 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_MODALS_INFO_1003").field("usrmod1003_min_passwd_age", &self.usrmod1003_min_passwd_age).finish()
    }
}
impl ::core::default::Default for USER_MODALS_INFO_1004 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_MODALS_INFO_1004 {
    fn eq(&self, other: &Self) -> bool {
        self.usrmod1004_force_logoff == other.usrmod1004_force_logoff
    }
}
impl ::core::cmp::Eq for USER_MODALS_INFO_1004 {}
impl ::core::fmt::Debug for USER_MODALS_INFO_1004 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_MODALS_INFO_1004").field("usrmod1004_force_logoff", &self.usrmod1004_force_logoff).finish()
    }
}
impl ::core::default::Default for USER_MODALS_INFO_1005 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_MODALS_INFO_1005 {
    fn eq(&self, other: &Self) -> bool {
        self.usrmod1005_password_hist_len == other.usrmod1005_password_hist_len
    }
}
impl ::core::cmp::Eq for USER_MODALS_INFO_1005 {}
impl ::core::fmt::Debug for USER_MODALS_INFO_1005 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_MODALS_INFO_1005").field("usrmod1005_password_hist_len", &self.usrmod1005_password_hist_len).finish()
    }
}
impl ::core::default::Default for USER_MODALS_INFO_1006 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_MODALS_INFO_1006 {
    fn eq(&self, other: &Self) -> bool {
        self.usrmod1006_role == other.usrmod1006_role
    }
}
impl ::core::cmp::Eq for USER_MODALS_INFO_1006 {}
impl ::core::fmt::Debug for USER_MODALS_INFO_1006 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_MODALS_INFO_1006").field("usrmod1006_role", &self.usrmod1006_role).finish()
    }
}
impl ::core::default::Default for USER_MODALS_INFO_1007 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_MODALS_INFO_1007 {
    fn eq(&self, other: &Self) -> bool {
        self.usrmod1007_primary == other.usrmod1007_primary
    }
}
impl ::core::cmp::Eq for USER_MODALS_INFO_1007 {}
impl ::core::fmt::Debug for USER_MODALS_INFO_1007 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_MODALS_INFO_1007").field("usrmod1007_primary", &self.usrmod1007_primary).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USER_MODALS_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USER_MODALS_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.usrmod2_domain_name == other.usrmod2_domain_name && self.usrmod2_domain_id == other.usrmod2_domain_id
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USER_MODALS_INFO_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for USER_MODALS_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_MODALS_INFO_2").field("usrmod2_domain_name", &self.usrmod2_domain_name).field("usrmod2_domain_id", &self.usrmod2_domain_id).finish()
    }
}
impl ::core::default::Default for USER_MODALS_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_MODALS_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        self.usrmod3_lockout_duration == other.usrmod3_lockout_duration && self.usrmod3_lockout_observation_window == other.usrmod3_lockout_observation_window && self.usrmod3_lockout_threshold == other.usrmod3_lockout_threshold
    }
}
impl ::core::cmp::Eq for USER_MODALS_INFO_3 {}
impl ::core::fmt::Debug for USER_MODALS_INFO_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_MODALS_INFO_3").field("usrmod3_lockout_duration", &self.usrmod3_lockout_duration).field("usrmod3_lockout_observation_window", &self.usrmod3_lockout_observation_window).field("usrmod3_lockout_threshold", &self.usrmod3_lockout_threshold).finish()
    }
}
impl ::core::default::Default for USER_MODALS_ROLES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USER_MODALS_ROLES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USER_MODALS_ROLES").field(&self.0).finish()
    }
}
impl ::core::default::Default for USER_OTHER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USER_OTHER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.alrtus_errcode == other.alrtus_errcode && self.alrtus_numstrings == other.alrtus_numstrings
    }
}
impl ::core::cmp::Eq for USER_OTHER_INFO {}
impl ::core::fmt::Debug for USER_OTHER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_OTHER_INFO").field("alrtus_errcode", &self.alrtus_errcode).field("alrtus_numstrings", &self.alrtus_numstrings).finish()
    }
}
impl ::core::default::Default for USER_PRIV {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USER_PRIV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USER_PRIV").field(&self.0).finish()
    }
}
impl ::core::default::Default for USE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.ui0_local == other.ui0_local && self.ui0_remote == other.ui0_remote
    }
}
impl ::core::cmp::Eq for USE_INFO_0 {}
impl ::core::fmt::Debug for USE_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USE_INFO_0").field("ui0_local", &self.ui0_local).field("ui0_remote", &self.ui0_remote).finish()
    }
}
impl ::core::default::Default for USE_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USE_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.ui1_local == other.ui1_local && self.ui1_remote == other.ui1_remote && self.ui1_password == other.ui1_password && self.ui1_status == other.ui1_status && self.ui1_asg_type == other.ui1_asg_type && self.ui1_refcount == other.ui1_refcount && self.ui1_usecount == other.ui1_usecount
    }
}
impl ::core::cmp::Eq for USE_INFO_1 {}
impl ::core::fmt::Debug for USE_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USE_INFO_1").field("ui1_local", &self.ui1_local).field("ui1_remote", &self.ui1_remote).field("ui1_password", &self.ui1_password).field("ui1_status", &self.ui1_status).field("ui1_asg_type", &self.ui1_asg_type).field("ui1_refcount", &self.ui1_refcount).field("ui1_usecount", &self.ui1_usecount).finish()
    }
}
impl ::core::default::Default for USE_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USE_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.ui2_local == other.ui2_local && self.ui2_remote == other.ui2_remote && self.ui2_password == other.ui2_password && self.ui2_status == other.ui2_status && self.ui2_asg_type == other.ui2_asg_type && self.ui2_refcount == other.ui2_refcount && self.ui2_usecount == other.ui2_usecount && self.ui2_username == other.ui2_username && self.ui2_domainname == other.ui2_domainname
    }
}
impl ::core::cmp::Eq for USE_INFO_2 {}
impl ::core::fmt::Debug for USE_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USE_INFO_2").field("ui2_local", &self.ui2_local).field("ui2_remote", &self.ui2_remote).field("ui2_password", &self.ui2_password).field("ui2_status", &self.ui2_status).field("ui2_asg_type", &self.ui2_asg_type).field("ui2_refcount", &self.ui2_refcount).field("ui2_usecount", &self.ui2_usecount).field("ui2_username", &self.ui2_username).field("ui2_domainname", &self.ui2_domainname).finish()
    }
}
impl ::core::default::Default for USE_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USE_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        self.ui3_ui2 == other.ui3_ui2 && self.ui3_flags == other.ui3_flags
    }
}
impl ::core::cmp::Eq for USE_INFO_3 {}
impl ::core::fmt::Debug for USE_INFO_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USE_INFO_3").field("ui3_ui2", &self.ui3_ui2).field("ui3_flags", &self.ui3_flags).finish()
    }
}
impl ::core::default::Default for USE_INFO_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USE_INFO_4 {
    fn eq(&self, other: &Self) -> bool {
        self.ui4_ui3 == other.ui4_ui3 && self.ui4_auth_identity_length == other.ui4_auth_identity_length && self.ui4_auth_identity == other.ui4_auth_identity
    }
}
impl ::core::cmp::Eq for USE_INFO_4 {}
impl ::core::fmt::Debug for USE_INFO_4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USE_INFO_4").field("ui4_ui3", &self.ui4_ui3).field("ui4_auth_identity_length", &self.ui4_auth_identity_length).field("ui4_auth_identity", &self.ui4_auth_identity).finish()
    }
}
impl ::core::default::Default for USE_INFO_5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USE_INFO_5 {
    fn eq(&self, other: &Self) -> bool {
        self.ui4_ui3 == other.ui4_ui3 && self.ui4_auth_identity_length == other.ui4_auth_identity_length && self.ui4_auth_identity == other.ui4_auth_identity && self.ui5_security_descriptor_length == other.ui5_security_descriptor_length && self.ui5_security_descriptor == other.ui5_security_descriptor && self.ui5_use_options_length == other.ui5_use_options_length && self.ui5_use_options == other.ui5_use_options
    }
}
impl ::core::cmp::Eq for USE_INFO_5 {}
impl ::core::fmt::Debug for USE_INFO_5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USE_INFO_5").field("ui4_ui3", &self.ui4_ui3).field("ui4_auth_identity_length", &self.ui4_auth_identity_length).field("ui4_auth_identity", &self.ui4_auth_identity).field("ui5_security_descriptor_length", &self.ui5_security_descriptor_length).field("ui5_security_descriptor", &self.ui5_security_descriptor).field("ui5_use_options_length", &self.ui5_use_options_length).field("ui5_use_options", &self.ui5_use_options).finish()
    }
}
impl ::core::default::Default for USE_INFO_ASG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USE_INFO_ASG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USE_INFO_ASG_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for USE_OPTION_DEFERRED_CONNECTION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USE_OPTION_DEFERRED_CONNECTION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Tag == other.Tag && self.Length == other.Length && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for USE_OPTION_DEFERRED_CONNECTION_PARAMETERS {}
impl ::core::fmt::Debug for USE_OPTION_DEFERRED_CONNECTION_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USE_OPTION_DEFERRED_CONNECTION_PARAMETERS").field("Tag", &self.Tag).field("Length", &self.Length).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for USE_OPTION_GENERIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USE_OPTION_GENERIC {
    fn eq(&self, other: &Self) -> bool {
        self.Tag == other.Tag && self.Length == other.Length && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for USE_OPTION_GENERIC {}
impl ::core::fmt::Debug for USE_OPTION_GENERIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USE_OPTION_GENERIC").field("Tag", &self.Tag).field("Length", &self.Length).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for USE_OPTION_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USE_OPTION_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.Tag == other.Tag && self.pInfo == other.pInfo && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for USE_OPTION_PROPERTIES {}
impl ::core::fmt::Debug for USE_OPTION_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USE_OPTION_PROPERTIES").field("Tag", &self.Tag).field("pInfo", &self.pInfo).field("Length", &self.Length).finish()
    }
}
impl ::core::default::Default for USE_OPTION_TRANSPORT_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USE_OPTION_TRANSPORT_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Tag == other.Tag && self.Length == other.Length && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for USE_OPTION_TRANSPORT_PARAMETERS {}
impl ::core::fmt::Debug for USE_OPTION_TRANSPORT_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USE_OPTION_TRANSPORT_PARAMETERS").field("Tag", &self.Tag).field("Length", &self.Length).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for WKSTA_INFO_100 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WKSTA_INFO_100 {
    fn eq(&self, other: &Self) -> bool {
        self.wki100_platform_id == other.wki100_platform_id && self.wki100_computername == other.wki100_computername && self.wki100_langroup == other.wki100_langroup && self.wki100_ver_major == other.wki100_ver_major && self.wki100_ver_minor == other.wki100_ver_minor
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_100 {}
impl ::core::fmt::Debug for WKSTA_INFO_100 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_100").field("wki100_platform_id", &self.wki100_platform_id).field("wki100_computername", &self.wki100_computername).field("wki100_langroup", &self.wki100_langroup).field("wki100_ver_major", &self.wki100_ver_major).field("wki100_ver_minor", &self.wki100_ver_minor).finish()
    }
}
impl ::core::default::Default for WKSTA_INFO_101 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WKSTA_INFO_101 {
    fn eq(&self, other: &Self) -> bool {
        self.wki101_platform_id == other.wki101_platform_id && self.wki101_computername == other.wki101_computername && self.wki101_langroup == other.wki101_langroup && self.wki101_ver_major == other.wki101_ver_major && self.wki101_ver_minor == other.wki101_ver_minor && self.wki101_lanroot == other.wki101_lanroot
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_101 {}
impl ::core::fmt::Debug for WKSTA_INFO_101 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_101").field("wki101_platform_id", &self.wki101_platform_id).field("wki101_computername", &self.wki101_computername).field("wki101_langroup", &self.wki101_langroup).field("wki101_ver_major", &self.wki101_ver_major).field("wki101_ver_minor", &self.wki101_ver_minor).field("wki101_lanroot", &self.wki101_lanroot).finish()
    }
}
impl ::core::default::Default for WKSTA_INFO_1010 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1010 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1010_char_wait == other.wki1010_char_wait
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1010 {}
impl ::core::fmt::Debug for WKSTA_INFO_1010 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1010").field("wki1010_char_wait", &self.wki1010_char_wait).finish()
    }
}
impl ::core::default::Default for WKSTA_INFO_1011 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1011 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1011_collection_time == other.wki1011_collection_time
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1011 {}
impl ::core::fmt::Debug for WKSTA_INFO_1011 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1011").field("wki1011_collection_time", &self.wki1011_collection_time).finish()
    }
}
impl ::core::default::Default for WKSTA_INFO_1012 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1012 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1012_maximum_collection_count == other.wki1012_maximum_collection_count
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1012 {}
impl ::core::fmt::Debug for WKSTA_INFO_1012 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1012").field("wki1012_maximum_collection_count", &self.wki1012_maximum_collection_count).finish()
    }
}
impl ::core::default::Default for WKSTA_INFO_1013 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1013 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1013_keep_conn == other.wki1013_keep_conn
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1013 {}
impl ::core::fmt::Debug for WKSTA_INFO_1013 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1013").field("wki1013_keep_conn", &self.wki1013_keep_conn).finish()
    }
}
impl ::core::default::Default for WKSTA_INFO_1018 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1018 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1018_sess_timeout == other.wki1018_sess_timeout
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1018 {}
impl ::core::fmt::Debug for WKSTA_INFO_1018 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1018").field("wki1018_sess_timeout", &self.wki1018_sess_timeout).finish()
    }
}
impl ::core::default::Default for WKSTA_INFO_102 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WKSTA_INFO_102 {
    fn eq(&self, other: &Self) -> bool {
        self.wki102_platform_id == other.wki102_platform_id && self.wki102_computername == other.wki102_computername && self.wki102_langroup == other.wki102_langroup && self.wki102_ver_major == other.wki102_ver_major && self.wki102_ver_minor == other.wki102_ver_minor && self.wki102_lanroot == other.wki102_lanroot && self.wki102_logged_on_users == other.wki102_logged_on_users
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_102 {}
impl ::core::fmt::Debug for WKSTA_INFO_102 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_102").field("wki102_platform_id", &self.wki102_platform_id).field("wki102_computername", &self.wki102_computername).field("wki102_langroup", &self.wki102_langroup).field("wki102_ver_major", &self.wki102_ver_major).field("wki102_ver_minor", &self.wki102_ver_minor).field("wki102_lanroot", &self.wki102_lanroot).field("wki102_logged_on_users", &self.wki102_logged_on_users).finish()
    }
}
impl ::core::default::Default for WKSTA_INFO_1023 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1023 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1023_siz_char_buf == other.wki1023_siz_char_buf
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1023 {}
impl ::core::fmt::Debug for WKSTA_INFO_1023 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1023").field("wki1023_siz_char_buf", &self.wki1023_siz_char_buf).finish()
    }
}
impl ::core::default::Default for WKSTA_INFO_1027 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1027 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1027_errlog_sz == other.wki1027_errlog_sz
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1027 {}
impl ::core::fmt::Debug for WKSTA_INFO_1027 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1027").field("wki1027_errlog_sz", &self.wki1027_errlog_sz).finish()
    }
}
impl ::core::default::Default for WKSTA_INFO_1028 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1028 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1028_print_buf_time == other.wki1028_print_buf_time
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1028 {}
impl ::core::fmt::Debug for WKSTA_INFO_1028 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1028").field("wki1028_print_buf_time", &self.wki1028_print_buf_time).finish()
    }
}
impl ::core::default::Default for WKSTA_INFO_1032 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1032 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1032_wrk_heuristics == other.wki1032_wrk_heuristics
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1032 {}
impl ::core::fmt::Debug for WKSTA_INFO_1032 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1032").field("wki1032_wrk_heuristics", &self.wki1032_wrk_heuristics).finish()
    }
}
impl ::core::default::Default for WKSTA_INFO_1033 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1033 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1033_max_threads == other.wki1033_max_threads
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1033 {}
impl ::core::fmt::Debug for WKSTA_INFO_1033 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1033").field("wki1033_max_threads", &self.wki1033_max_threads).finish()
    }
}
impl ::core::default::Default for WKSTA_INFO_1041 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1041 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1041_lock_quota == other.wki1041_lock_quota
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1041 {}
impl ::core::fmt::Debug for WKSTA_INFO_1041 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1041").field("wki1041_lock_quota", &self.wki1041_lock_quota).finish()
    }
}
impl ::core::default::Default for WKSTA_INFO_1042 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1042 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1042_lock_increment == other.wki1042_lock_increment
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1042 {}
impl ::core::fmt::Debug for WKSTA_INFO_1042 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1042").field("wki1042_lock_increment", &self.wki1042_lock_increment).finish()
    }
}
impl ::core::default::Default for WKSTA_INFO_1043 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1043 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1043_lock_maximum == other.wki1043_lock_maximum
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1043 {}
impl ::core::fmt::Debug for WKSTA_INFO_1043 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1043").field("wki1043_lock_maximum", &self.wki1043_lock_maximum).finish()
    }
}
impl ::core::default::Default for WKSTA_INFO_1044 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1044 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1044_pipe_increment == other.wki1044_pipe_increment
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1044 {}
impl ::core::fmt::Debug for WKSTA_INFO_1044 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1044").field("wki1044_pipe_increment", &self.wki1044_pipe_increment).finish()
    }
}
impl ::core::default::Default for WKSTA_INFO_1045 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1045 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1045_pipe_maximum == other.wki1045_pipe_maximum
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1045 {}
impl ::core::fmt::Debug for WKSTA_INFO_1045 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1045").field("wki1045_pipe_maximum", &self.wki1045_pipe_maximum).finish()
    }
}
impl ::core::default::Default for WKSTA_INFO_1046 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1046 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1046_dormant_file_limit == other.wki1046_dormant_file_limit
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1046 {}
impl ::core::fmt::Debug for WKSTA_INFO_1046 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1046").field("wki1046_dormant_file_limit", &self.wki1046_dormant_file_limit).finish()
    }
}
impl ::core::default::Default for WKSTA_INFO_1047 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1047 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1047_cache_file_timeout == other.wki1047_cache_file_timeout
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1047 {}
impl ::core::fmt::Debug for WKSTA_INFO_1047 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1047").field("wki1047_cache_file_timeout", &self.wki1047_cache_file_timeout).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1048 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1048 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1048_use_opportunistic_locking == other.wki1048_use_opportunistic_locking
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1048 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WKSTA_INFO_1048 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1048").field("wki1048_use_opportunistic_locking", &self.wki1048_use_opportunistic_locking).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1049 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1049 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1049_use_unlock_behind == other.wki1049_use_unlock_behind
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1049 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WKSTA_INFO_1049 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1049").field("wki1049_use_unlock_behind", &self.wki1049_use_unlock_behind).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1050 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1050 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1050_use_close_behind == other.wki1050_use_close_behind
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1050 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WKSTA_INFO_1050 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1050").field("wki1050_use_close_behind", &self.wki1050_use_close_behind).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1051 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1051 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1051_buf_named_pipes == other.wki1051_buf_named_pipes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1051 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WKSTA_INFO_1051 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1051").field("wki1051_buf_named_pipes", &self.wki1051_buf_named_pipes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1052 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1052 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1052_use_lock_read_unlock == other.wki1052_use_lock_read_unlock
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1052 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WKSTA_INFO_1052 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1052").field("wki1052_use_lock_read_unlock", &self.wki1052_use_lock_read_unlock).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1053 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1053 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1053_utilize_nt_caching == other.wki1053_utilize_nt_caching
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1053 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WKSTA_INFO_1053 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1053").field("wki1053_utilize_nt_caching", &self.wki1053_utilize_nt_caching).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1054 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1054 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1054_use_raw_read == other.wki1054_use_raw_read
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1054 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WKSTA_INFO_1054 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1054").field("wki1054_use_raw_read", &self.wki1054_use_raw_read).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1055 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1055 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1055_use_raw_write == other.wki1055_use_raw_write
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1055 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WKSTA_INFO_1055 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1055").field("wki1055_use_raw_write", &self.wki1055_use_raw_write).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1056 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1056 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1056_use_write_raw_data == other.wki1056_use_write_raw_data
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1056 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WKSTA_INFO_1056 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1056").field("wki1056_use_write_raw_data", &self.wki1056_use_write_raw_data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1057 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1057 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1057_use_encryption == other.wki1057_use_encryption
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1057 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WKSTA_INFO_1057 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1057").field("wki1057_use_encryption", &self.wki1057_use_encryption).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1058 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1058 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1058_buf_files_deny_write == other.wki1058_buf_files_deny_write
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1058 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WKSTA_INFO_1058 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1058").field("wki1058_buf_files_deny_write", &self.wki1058_buf_files_deny_write).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1059 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1059 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1059_buf_read_only_files == other.wki1059_buf_read_only_files
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1059 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WKSTA_INFO_1059 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1059").field("wki1059_buf_read_only_files", &self.wki1059_buf_read_only_files).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1060 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1060 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1060_force_core_create_mode == other.wki1060_force_core_create_mode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1060 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WKSTA_INFO_1060 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1060").field("wki1060_force_core_create_mode", &self.wki1060_force_core_create_mode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_1061 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_1061 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1061_use_512_byte_max_transfer == other.wki1061_use_512_byte_max_transfer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_1061 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WKSTA_INFO_1061 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1061").field("wki1061_use_512_byte_max_transfer", &self.wki1061_use_512_byte_max_transfer).finish()
    }
}
impl ::core::default::Default for WKSTA_INFO_1062 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WKSTA_INFO_1062 {
    fn eq(&self, other: &Self) -> bool {
        self.wki1062_read_ahead_throughput == other.wki1062_read_ahead_throughput
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_1062 {}
impl ::core::fmt::Debug for WKSTA_INFO_1062 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_1062").field("wki1062_read_ahead_throughput", &self.wki1062_read_ahead_throughput).finish()
    }
}
impl ::core::default::Default for WKSTA_INFO_302 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WKSTA_INFO_302 {
    fn eq(&self, other: &Self) -> bool {
        self.wki302_char_wait == other.wki302_char_wait
            && self.wki302_collection_time == other.wki302_collection_time
            && self.wki302_maximum_collection_count == other.wki302_maximum_collection_count
            && self.wki302_keep_conn == other.wki302_keep_conn
            && self.wki302_keep_search == other.wki302_keep_search
            && self.wki302_max_cmds == other.wki302_max_cmds
            && self.wki302_num_work_buf == other.wki302_num_work_buf
            && self.wki302_siz_work_buf == other.wki302_siz_work_buf
            && self.wki302_max_wrk_cache == other.wki302_max_wrk_cache
            && self.wki302_sess_timeout == other.wki302_sess_timeout
            && self.wki302_siz_error == other.wki302_siz_error
            && self.wki302_num_alerts == other.wki302_num_alerts
            && self.wki302_num_services == other.wki302_num_services
            && self.wki302_errlog_sz == other.wki302_errlog_sz
            && self.wki302_print_buf_time == other.wki302_print_buf_time
            && self.wki302_num_char_buf == other.wki302_num_char_buf
            && self.wki302_siz_char_buf == other.wki302_siz_char_buf
            && self.wki302_wrk_heuristics == other.wki302_wrk_heuristics
            && self.wki302_mailslots == other.wki302_mailslots
            && self.wki302_num_dgram_buf == other.wki302_num_dgram_buf
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_302 {}
impl ::core::fmt::Debug for WKSTA_INFO_302 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_302")
            .field("wki302_char_wait", &self.wki302_char_wait)
            .field("wki302_collection_time", &self.wki302_collection_time)
            .field("wki302_maximum_collection_count", &self.wki302_maximum_collection_count)
            .field("wki302_keep_conn", &self.wki302_keep_conn)
            .field("wki302_keep_search", &self.wki302_keep_search)
            .field("wki302_max_cmds", &self.wki302_max_cmds)
            .field("wki302_num_work_buf", &self.wki302_num_work_buf)
            .field("wki302_siz_work_buf", &self.wki302_siz_work_buf)
            .field("wki302_max_wrk_cache", &self.wki302_max_wrk_cache)
            .field("wki302_sess_timeout", &self.wki302_sess_timeout)
            .field("wki302_siz_error", &self.wki302_siz_error)
            .field("wki302_num_alerts", &self.wki302_num_alerts)
            .field("wki302_num_services", &self.wki302_num_services)
            .field("wki302_errlog_sz", &self.wki302_errlog_sz)
            .field("wki302_print_buf_time", &self.wki302_print_buf_time)
            .field("wki302_num_char_buf", &self.wki302_num_char_buf)
            .field("wki302_siz_char_buf", &self.wki302_siz_char_buf)
            .field("wki302_wrk_heuristics", &self.wki302_wrk_heuristics)
            .field("wki302_mailslots", &self.wki302_mailslots)
            .field("wki302_num_dgram_buf", &self.wki302_num_dgram_buf)
            .finish()
    }
}
impl ::core::default::Default for WKSTA_INFO_402 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WKSTA_INFO_402 {
    fn eq(&self, other: &Self) -> bool {
        self.wki402_char_wait == other.wki402_char_wait
            && self.wki402_collection_time == other.wki402_collection_time
            && self.wki402_maximum_collection_count == other.wki402_maximum_collection_count
            && self.wki402_keep_conn == other.wki402_keep_conn
            && self.wki402_keep_search == other.wki402_keep_search
            && self.wki402_max_cmds == other.wki402_max_cmds
            && self.wki402_num_work_buf == other.wki402_num_work_buf
            && self.wki402_siz_work_buf == other.wki402_siz_work_buf
            && self.wki402_max_wrk_cache == other.wki402_max_wrk_cache
            && self.wki402_sess_timeout == other.wki402_sess_timeout
            && self.wki402_siz_error == other.wki402_siz_error
            && self.wki402_num_alerts == other.wki402_num_alerts
            && self.wki402_num_services == other.wki402_num_services
            && self.wki402_errlog_sz == other.wki402_errlog_sz
            && self.wki402_print_buf_time == other.wki402_print_buf_time
            && self.wki402_num_char_buf == other.wki402_num_char_buf
            && self.wki402_siz_char_buf == other.wki402_siz_char_buf
            && self.wki402_wrk_heuristics == other.wki402_wrk_heuristics
            && self.wki402_mailslots == other.wki402_mailslots
            && self.wki402_num_dgram_buf == other.wki402_num_dgram_buf
            && self.wki402_max_threads == other.wki402_max_threads
    }
}
impl ::core::cmp::Eq for WKSTA_INFO_402 {}
impl ::core::fmt::Debug for WKSTA_INFO_402 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_402")
            .field("wki402_char_wait", &self.wki402_char_wait)
            .field("wki402_collection_time", &self.wki402_collection_time)
            .field("wki402_maximum_collection_count", &self.wki402_maximum_collection_count)
            .field("wki402_keep_conn", &self.wki402_keep_conn)
            .field("wki402_keep_search", &self.wki402_keep_search)
            .field("wki402_max_cmds", &self.wki402_max_cmds)
            .field("wki402_num_work_buf", &self.wki402_num_work_buf)
            .field("wki402_siz_work_buf", &self.wki402_siz_work_buf)
            .field("wki402_max_wrk_cache", &self.wki402_max_wrk_cache)
            .field("wki402_sess_timeout", &self.wki402_sess_timeout)
            .field("wki402_siz_error", &self.wki402_siz_error)
            .field("wki402_num_alerts", &self.wki402_num_alerts)
            .field("wki402_num_services", &self.wki402_num_services)
            .field("wki402_errlog_sz", &self.wki402_errlog_sz)
            .field("wki402_print_buf_time", &self.wki402_print_buf_time)
            .field("wki402_num_char_buf", &self.wki402_num_char_buf)
            .field("wki402_siz_char_buf", &self.wki402_siz_char_buf)
            .field("wki402_wrk_heuristics", &self.wki402_wrk_heuristics)
            .field("wki402_mailslots", &self.wki402_mailslots)
            .field("wki402_num_dgram_buf", &self.wki402_num_dgram_buf)
            .field("wki402_max_threads", &self.wki402_max_threads)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_INFO_502 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_INFO_502 {
    fn eq(&self, other: &Self) -> bool {
        self.wki502_char_wait == other.wki502_char_wait
            && self.wki502_collection_time == other.wki502_collection_time
            && self.wki502_maximum_collection_count == other.wki502_maximum_collection_count
            && self.wki502_keep_conn == other.wki502_keep_conn
            && self.wki502_max_cmds == other.wki502_max_cmds
            && self.wki502_sess_timeout == other.wki502_sess_timeout
            && self.wki502_siz_char_buf == other.wki502_siz_char_buf
            && self.wki502_max_threads == other.wki502_max_threads
            && self.wki502_lock_quota == other.wki502_lock_quota
            && self.wki502_lock_increment == other.wki502_lock_increment
            && self.wki502_lock_maximum == other.wki502_lock_maximum
            && self.wki502_pipe_increment == other.wki502_pipe_increment
            && self.wki502_pipe_maximum == other.wki502_pipe_maximum
            && self.wki502_cache_file_timeout == other.wki502_cache_file_timeout
            && self.wki502_dormant_file_limit == other.wki502_dormant_file_limit
            && self.wki502_read_ahead_throughput == other.wki502_read_ahead_throughput
            && self.wki502_num_mailslot_buffers == other.wki502_num_mailslot_buffers
            && self.wki502_num_srv_announce_buffers == other.wki502_num_srv_announce_buffers
            && self.wki502_max_illegal_datagram_events == other.wki502_max_illegal_datagram_events
            && self.wki502_illegal_datagram_event_reset_frequency == other.wki502_illegal_datagram_event_reset_frequency
            && self.wki502_log_election_packets == other.wki502_log_election_packets
            && self.wki502_use_opportunistic_locking == other.wki502_use_opportunistic_locking
            && self.wki502_use_unlock_behind == other.wki502_use_unlock_behind
            && self.wki502_use_close_behind == other.wki502_use_close_behind
            && self.wki502_buf_named_pipes == other.wki502_buf_named_pipes
            && self.wki502_use_lock_read_unlock == other.wki502_use_lock_read_unlock
            && self.wki502_utilize_nt_caching == other.wki502_utilize_nt_caching
            && self.wki502_use_raw_read == other.wki502_use_raw_read
            && self.wki502_use_raw_write == other.wki502_use_raw_write
            && self.wki502_use_write_raw_data == other.wki502_use_write_raw_data
            && self.wki502_use_encryption == other.wki502_use_encryption
            && self.wki502_buf_files_deny_write == other.wki502_buf_files_deny_write
            && self.wki502_buf_read_only_files == other.wki502_buf_read_only_files
            && self.wki502_force_core_create_mode == other.wki502_force_core_create_mode
            && self.wki502_use_512_byte_max_transfer == other.wki502_use_512_byte_max_transfer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_INFO_502 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WKSTA_INFO_502 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_INFO_502")
            .field("wki502_char_wait", &self.wki502_char_wait)
            .field("wki502_collection_time", &self.wki502_collection_time)
            .field("wki502_maximum_collection_count", &self.wki502_maximum_collection_count)
            .field("wki502_keep_conn", &self.wki502_keep_conn)
            .field("wki502_max_cmds", &self.wki502_max_cmds)
            .field("wki502_sess_timeout", &self.wki502_sess_timeout)
            .field("wki502_siz_char_buf", &self.wki502_siz_char_buf)
            .field("wki502_max_threads", &self.wki502_max_threads)
            .field("wki502_lock_quota", &self.wki502_lock_quota)
            .field("wki502_lock_increment", &self.wki502_lock_increment)
            .field("wki502_lock_maximum", &self.wki502_lock_maximum)
            .field("wki502_pipe_increment", &self.wki502_pipe_increment)
            .field("wki502_pipe_maximum", &self.wki502_pipe_maximum)
            .field("wki502_cache_file_timeout", &self.wki502_cache_file_timeout)
            .field("wki502_dormant_file_limit", &self.wki502_dormant_file_limit)
            .field("wki502_read_ahead_throughput", &self.wki502_read_ahead_throughput)
            .field("wki502_num_mailslot_buffers", &self.wki502_num_mailslot_buffers)
            .field("wki502_num_srv_announce_buffers", &self.wki502_num_srv_announce_buffers)
            .field("wki502_max_illegal_datagram_events", &self.wki502_max_illegal_datagram_events)
            .field("wki502_illegal_datagram_event_reset_frequency", &self.wki502_illegal_datagram_event_reset_frequency)
            .field("wki502_log_election_packets", &self.wki502_log_election_packets)
            .field("wki502_use_opportunistic_locking", &self.wki502_use_opportunistic_locking)
            .field("wki502_use_unlock_behind", &self.wki502_use_unlock_behind)
            .field("wki502_use_close_behind", &self.wki502_use_close_behind)
            .field("wki502_buf_named_pipes", &self.wki502_buf_named_pipes)
            .field("wki502_use_lock_read_unlock", &self.wki502_use_lock_read_unlock)
            .field("wki502_utilize_nt_caching", &self.wki502_utilize_nt_caching)
            .field("wki502_use_raw_read", &self.wki502_use_raw_read)
            .field("wki502_use_raw_write", &self.wki502_use_raw_write)
            .field("wki502_use_write_raw_data", &self.wki502_use_write_raw_data)
            .field("wki502_use_encryption", &self.wki502_use_encryption)
            .field("wki502_buf_files_deny_write", &self.wki502_buf_files_deny_write)
            .field("wki502_buf_read_only_files", &self.wki502_buf_read_only_files)
            .field("wki502_force_core_create_mode", &self.wki502_force_core_create_mode)
            .field("wki502_use_512_byte_max_transfer", &self.wki502_use_512_byte_max_transfer)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WKSTA_TRANSPORT_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WKSTA_TRANSPORT_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.wkti0_quality_of_service == other.wkti0_quality_of_service && self.wkti0_number_of_vcs == other.wkti0_number_of_vcs && self.wkti0_transport_name == other.wkti0_transport_name && self.wkti0_transport_address == other.wkti0_transport_address && self.wkti0_wan_ish == other.wkti0_wan_ish
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WKSTA_TRANSPORT_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WKSTA_TRANSPORT_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_TRANSPORT_INFO_0").field("wkti0_quality_of_service", &self.wkti0_quality_of_service).field("wkti0_number_of_vcs", &self.wkti0_number_of_vcs).field("wkti0_transport_name", &self.wkti0_transport_name).field("wkti0_transport_address", &self.wkti0_transport_address).field("wkti0_wan_ish", &self.wkti0_wan_ish).finish()
    }
}
impl ::core::default::Default for WKSTA_USER_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WKSTA_USER_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.wkui0_username == other.wkui0_username
    }
}
impl ::core::cmp::Eq for WKSTA_USER_INFO_0 {}
impl ::core::fmt::Debug for WKSTA_USER_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_USER_INFO_0").field("wkui0_username", &self.wkui0_username).finish()
    }
}
impl ::core::default::Default for WKSTA_USER_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WKSTA_USER_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.wkui1_username == other.wkui1_username && self.wkui1_logon_domain == other.wkui1_logon_domain && self.wkui1_oth_domains == other.wkui1_oth_domains && self.wkui1_logon_server == other.wkui1_logon_server
    }
}
impl ::core::cmp::Eq for WKSTA_USER_INFO_1 {}
impl ::core::fmt::Debug for WKSTA_USER_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_USER_INFO_1").field("wkui1_username", &self.wkui1_username).field("wkui1_logon_domain", &self.wkui1_logon_domain).field("wkui1_oth_domains", &self.wkui1_oth_domains).field("wkui1_logon_server", &self.wkui1_logon_server).finish()
    }
}
impl ::core::default::Default for WKSTA_USER_INFO_1101 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WKSTA_USER_INFO_1101 {
    fn eq(&self, other: &Self) -> bool {
        self.wkui1101_oth_domains == other.wkui1101_oth_domains
    }
}
impl ::core::cmp::Eq for WKSTA_USER_INFO_1101 {}
impl ::core::fmt::Debug for WKSTA_USER_INFO_1101 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WKSTA_USER_INFO_1101").field("wkui1101_oth_domains", &self.wkui1101_oth_domains).finish()
    }
}
