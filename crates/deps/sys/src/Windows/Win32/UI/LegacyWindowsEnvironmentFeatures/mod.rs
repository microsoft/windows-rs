#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn EVCCBF_LASTNOTIFICATION();
    fn EVCF_DONTSHOWIFZERO();
    fn EVCF_ENABLEBYDEFAULT();
    fn EVCF_ENABLEBYDEFAULT_AUTO();
    fn EVCF_HASSETTINGS();
    fn EVCF_OUTOFDISKSPACE();
    fn EVCF_REMOVEFROMLIST();
    fn EVCF_SETTINGSMODE();
    fn EVCF_SYSTEMAUTORUN();
    fn EVCF_USERCONSENTOBTAINED();
    fn IADesktopP2();
    fn IActiveDesktopP();
    fn IBriefcaseInitiator();
    fn IEmptyVolumeCache();
    fn IEmptyVolumeCache2();
    fn IEmptyVolumeCacheCallBack();
    fn IReconcilableObject();
    fn IReconcileInitiator();
    fn REC_E_ABORTED();
    fn REC_E_INEEDTODOTHEUPDATES();
    fn REC_E_NOCALLBACK();
    fn REC_E_NORESIDUES();
    fn REC_E_TOODIFFERENT();
    fn REC_S_IDIDTHEUPDATES();
    fn REC_S_NOTCOMPLETE();
    fn REC_S_NOTCOMPLETEBUTPROPAGATE();
    fn STATEBITS_FLAT();
    fn _reconcilef();
}
