#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Accessibility"))]
pub trait IAccessibleWinSATImpl: Sized + IAccessibleImpl + IDispatchImpl {
    fn SetAccessiblityData();
}
pub trait IInitiateWinSATAssessmentImpl: Sized {
    fn InitiateAssessment();
    fn InitiateFormalAssessment();
    fn CancelAssessment();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IProvideWinSATAssessmentInfoImpl: Sized + IDispatchImpl {
    fn Score();
    fn Title();
    fn Description();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IProvideWinSATResultsInfoImpl: Sized + IDispatchImpl {
    fn GetAssessmentInfo();
    fn AssessmentState();
    fn AssessmentDateTime();
    fn SystemRating();
    fn RatingStateDesc();
}
pub trait IProvideWinSATVisualsImpl: Sized {
    fn Bitmap();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IQueryAllWinSATAssessmentsImpl: Sized + IDispatchImpl {
    fn AllXML();
}
pub trait IQueryOEMWinSATCustomizationImpl: Sized {
    fn GetOEMPrePopulationInfo();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IQueryRecentWinSATAssessmentImpl: Sized + IDispatchImpl {
    fn XML();
    fn Info();
}
pub trait IWinSATInitiateEventsImpl: Sized {
    fn WinSATComplete();
    fn WinSATUpdate();
}
