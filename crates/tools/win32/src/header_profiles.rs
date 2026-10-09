pub fn prerequisites(header: &str) -> &'static [&'static str] {
    match header {
        "mfreadwrite.h" => &["mfidl.h"],
        "ntsecpkg.h" => &["ntsecapi.h", "sspi.h"],
        "dxva2api.h" => &["d3d9.h"],
        "icmpapi.h" => &["ipexport.h"],
        "vswriter.h" => &["vss.h"],
        "vsbackup.h" => &["vss.h", "vswriter.h"],
        "adshlp.h" => &["iads.h"],
        "dmort.h" => &["mediaobj.h"],
        "secext.h" => &["sspi.h"],
        "traffic.h" => &["ntddndis.h"],
        "ksmedia.h" => &["ks.h"],
        // d3dkmdt.h rejects direct inclusion; use the SDK's user-mode entry header.
        "d3dkmdt.h" => &["d3dkmthk.h"],
        "hidpi.h" => &["hidusage.h"],
        "richole.h" | "textserv.h" => &["richedit.h"],
        "commoncontrols.h" => &["commctrl.h"],
        "lmat.h" | "lmconfig.h" | "lmsvc.h" | "lmmsg.h" | "lmremutl.h" | "lmrepl.h"
        | "lmalert.h" | "lmjoin.h" => &["lmcons.h"],
        _ => &[],
    }
}
