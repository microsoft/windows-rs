#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Calendar(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Calendar,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl Calendar {
    pub fn new() -> Result<Self, windows_result::HRESULT> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> Result<R, windows_result::HRESULT>,
    >(
        callback: F,
    ) -> Result<R, windows_result::HRESULT> {
        static SHARED: windows_core::imp::FactoryCache<
            Calendar,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Clone(&self) -> Result<Calendar, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Clone)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetToMin(&self) -> Result<(), windows_result::HRESULT> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetToMin)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn SetToMax(&self) -> Result<(), windows_result::HRESULT> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetToMax)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn NumeralSystem(&self) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NumeralSystem)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetNumeralSystem(
        &self,
        value: &windows_core::HSTRING,
    ) -> Result<(), windows_result::HRESULT> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetNumeralSystem)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn GetCalendarSystem(&self) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCalendarSystem)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn ChangeCalendarSystem(
        &self,
        value: &windows_core::HSTRING,
    ) -> Result<(), windows_result::HRESULT> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).ChangeCalendarSystem)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn GetClock(&self) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetClock)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn ChangeClock(
        &self,
        value: &windows_core::HSTRING,
    ) -> Result<(), windows_result::HRESULT> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).ChangeClock)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn SetToNow(&self) -> Result<(), windows_result::HRESULT> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetToNow)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn FirstEra(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirstEra)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn LastEra(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastEra)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn NumberOfEras(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NumberOfEras)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Era(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Era)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetEra(&self, value: i32) -> Result<(), windows_result::HRESULT> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetEra)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AddEras(&self, eras: i32) -> Result<(), windows_result::HRESULT> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).AddEras)(
                windows_core::Interface::as_raw(this),
                eras,
            )
            .ok()
        }
    }
    pub fn EraAsFullString(&self) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EraAsFullString)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn EraAsString(
        &self,
        ideallength: i32,
    ) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EraAsString)(
                windows_core::Interface::as_raw(this),
                ideallength,
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn FirstYearInThisEra(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirstYearInThisEra)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn LastYearInThisEra(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastYearInThisEra)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn NumberOfYearsInThisEra(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NumberOfYearsInThisEra)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Year(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Year)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetYear(&self, value: i32) -> Result<(), windows_result::HRESULT> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetYear)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AddYears(&self, years: i32) -> Result<(), windows_result::HRESULT> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).AddYears)(
                windows_core::Interface::as_raw(this),
                years,
            )
            .ok()
        }
    }
    pub fn YearAsString(&self) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).YearAsString)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn YearAsTruncatedString(
        &self,
        remainingdigits: i32,
    ) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).YearAsTruncatedString)(
                windows_core::Interface::as_raw(this),
                remainingdigits,
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn YearAsPaddedString(
        &self,
        mindigits: i32,
    ) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).YearAsPaddedString)(
                windows_core::Interface::as_raw(this),
                mindigits,
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn FirstMonthInThisYear(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirstMonthInThisYear)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn LastMonthInThisYear(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastMonthInThisYear)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn NumberOfMonthsInThisYear(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NumberOfMonthsInThisYear)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Month(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Month)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMonth(&self, value: i32) -> Result<(), windows_result::HRESULT> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetMonth)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AddMonths(&self, months: i32) -> Result<(), windows_result::HRESULT> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).AddMonths)(
                windows_core::Interface::as_raw(this),
                months,
            )
            .ok()
        }
    }
    pub fn MonthAsFullString(&self) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MonthAsFullString)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn MonthAsString(
        &self,
        ideallength: i32,
    ) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MonthAsString)(
                windows_core::Interface::as_raw(this),
                ideallength,
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn MonthAsFullSoloString(&self) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MonthAsFullSoloString)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn MonthAsSoloString(
        &self,
        ideallength: i32,
    ) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MonthAsSoloString)(
                windows_core::Interface::as_raw(this),
                ideallength,
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn MonthAsNumericString(&self) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MonthAsNumericString)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn MonthAsPaddedNumericString(
        &self,
        mindigits: i32,
    ) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MonthAsPaddedNumericString)(
                windows_core::Interface::as_raw(this),
                mindigits,
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn AddWeeks(&self, weeks: i32) -> Result<(), windows_result::HRESULT> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).AddWeeks)(
                windows_core::Interface::as_raw(this),
                weeks,
            )
            .ok()
        }
    }
    pub fn FirstDayInThisMonth(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirstDayInThisMonth)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn LastDayInThisMonth(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastDayInThisMonth)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn NumberOfDaysInThisMonth(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NumberOfDaysInThisMonth)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Day(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Day)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetDay(&self, value: i32) -> Result<(), windows_result::HRESULT> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetDay)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AddDays(&self, days: i32) -> Result<(), windows_result::HRESULT> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).AddDays)(
                windows_core::Interface::as_raw(this),
                days,
            )
            .ok()
        }
    }
    pub fn DayAsString(&self) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DayAsString)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn DayAsPaddedString(
        &self,
        mindigits: i32,
    ) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DayAsPaddedString)(
                windows_core::Interface::as_raw(this),
                mindigits,
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn DayOfWeekAsFullString(&self) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DayOfWeekAsFullString)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn DayOfWeekAsString(
        &self,
        ideallength: i32,
    ) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DayOfWeekAsString)(
                windows_core::Interface::as_raw(this),
                ideallength,
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn DayOfWeekAsFullSoloString(
        &self,
    ) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DayOfWeekAsFullSoloString)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn DayOfWeekAsSoloString(
        &self,
        ideallength: i32,
    ) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DayOfWeekAsSoloString)(
                windows_core::Interface::as_raw(this),
                ideallength,
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn FirstPeriodInThisDay(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirstPeriodInThisDay)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn LastPeriodInThisDay(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastPeriodInThisDay)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn NumberOfPeriodsInThisDay(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NumberOfPeriodsInThisDay)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Period(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Period)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetPeriod(&self, value: i32) -> Result<(), windows_result::HRESULT> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetPeriod)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AddPeriods(&self, periods: i32) -> Result<(), windows_result::HRESULT> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).AddPeriods)(
                windows_core::Interface::as_raw(this),
                periods,
            )
            .ok()
        }
    }
    pub fn PeriodAsFullString(&self) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PeriodAsFullString)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn PeriodAsString(
        &self,
        ideallength: i32,
    ) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PeriodAsString)(
                windows_core::Interface::as_raw(this),
                ideallength,
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn FirstHourInThisPeriod(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirstHourInThisPeriod)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn LastHourInThisPeriod(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastHourInThisPeriod)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn NumberOfHoursInThisPeriod(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NumberOfHoursInThisPeriod)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Hour(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Hour)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetHour(&self, value: i32) -> Result<(), windows_result::HRESULT> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetHour)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AddHours(&self, hours: i32) -> Result<(), windows_result::HRESULT> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).AddHours)(
                windows_core::Interface::as_raw(this),
                hours,
            )
            .ok()
        }
    }
    pub fn HourAsString(&self) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HourAsString)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn HourAsPaddedString(
        &self,
        mindigits: i32,
    ) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HourAsPaddedString)(
                windows_core::Interface::as_raw(this),
                mindigits,
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn Minute(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Minute)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMinute(&self, value: i32) -> Result<(), windows_result::HRESULT> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetMinute)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AddMinutes(&self, minutes: i32) -> Result<(), windows_result::HRESULT> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).AddMinutes)(
                windows_core::Interface::as_raw(this),
                minutes,
            )
            .ok()
        }
    }
    pub fn MinuteAsString(&self) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinuteAsString)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn MinuteAsPaddedString(
        &self,
        mindigits: i32,
    ) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinuteAsPaddedString)(
                windows_core::Interface::as_raw(this),
                mindigits,
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn Second(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Second)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetSecond(&self, value: i32) -> Result<(), windows_result::HRESULT> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetSecond)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AddSeconds(&self, seconds: i32) -> Result<(), windows_result::HRESULT> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).AddSeconds)(
                windows_core::Interface::as_raw(this),
                seconds,
            )
            .ok()
        }
    }
    pub fn SecondAsString(&self) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SecondAsString)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SecondAsPaddedString(
        &self,
        mindigits: i32,
    ) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SecondAsPaddedString)(
                windows_core::Interface::as_raw(this),
                mindigits,
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn Nanosecond(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Nanosecond)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetNanosecond(&self, value: i32) -> Result<(), windows_result::HRESULT> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetNanosecond)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn AddNanoseconds(&self, nanoseconds: i32) -> Result<(), windows_result::HRESULT> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).AddNanoseconds)(
                windows_core::Interface::as_raw(this),
                nanoseconds,
            )
            .ok()
        }
    }
    pub fn NanosecondAsString(&self) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NanosecondAsString)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn NanosecondAsPaddedString(
        &self,
        mindigits: i32,
    ) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NanosecondAsPaddedString)(
                windows_core::Interface::as_raw(this),
                mindigits,
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn Compare<P0>(&self, other: P0) -> Result<i32, windows_result::HRESULT>
    where
        P0: windows_core::Param<Calendar>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compare)(
                windows_core::Interface::as_raw(this),
                other.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn CopyTo<P0>(&self, other: P0) -> Result<(), windows_result::HRESULT>
    where
        P0: windows_core::Param<Calendar>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).CopyTo)(
                windows_core::Interface::as_raw(this),
                other.param().abi(),
            )
            .ok()
        }
    }
    pub fn FirstMinuteInThisHour(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirstMinuteInThisHour)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn LastMinuteInThisHour(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastMinuteInThisHour)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn NumberOfMinutesInThisHour(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NumberOfMinutesInThisHour)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn FirstSecondInThisMinute(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirstSecondInThisMinute)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn LastSecondInThisMinute(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastSecondInThisMinute)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn NumberOfSecondsInThisMinute(&self) -> Result<i32, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NumberOfSecondsInThisMinute)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ResolvedLanguage(&self) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResolvedLanguage)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn IsDaylightSavingTime(&self) -> Result<bool, windows_result::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDaylightSavingTime)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetTimeZone(&self) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<ITimeZoneOnCalendar>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTimeZone)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn ChangeTimeZone(
        &self,
        timezoneid: &windows_core::HSTRING,
    ) -> Result<(), windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<ITimeZoneOnCalendar>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).ChangeTimeZone)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(timezoneid),
            )
            .ok()
        }
    }
    pub fn TimeZoneAsFullString(&self) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<ITimeZoneOnCalendar>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeZoneAsFullString)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn TimeZoneAsString(
        &self,
        ideallength: i32,
    ) -> Result<windows_core::HSTRING, windows_result::HRESULT> {
        let this = &windows_core::Interface::cast::<ITimeZoneOnCalendar>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimeZoneAsString)(
                windows_core::Interface::as_raw(this),
                ideallength,
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    fn ICalendarFactory<R, F: FnOnce(&ICalendarFactory) -> Result<R, windows_result::HRESULT>>(
        callback: F,
    ) -> Result<R, windows_result::HRESULT> {
        static SHARED: windows_core::imp::FactoryCache<Calendar, ICalendarFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ICalendarFactory2<R, F: FnOnce(&ICalendarFactory2) -> Result<R, windows_result::HRESULT>>(
        callback: F,
    ) -> Result<R, windows_result::HRESULT> {
        static SHARED: windows_core::imp::FactoryCache<Calendar, ICalendarFactory2> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Calendar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICalendar>();
}
unsafe impl windows_core::Interface for Calendar {
    type Vtable = <ICalendar as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICalendar as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Calendar {
    const NAME: &'static str = "Windows.Globalization.Calendar";
}
unsafe impl Send for Calendar {}
unsafe impl Sync for Calendar {}
windows_core::imp::define_interface!(
    ICalendar,
    ICalendar_Vtbl,
    0xca30221d_86d9_40fb_a26b_d44eb7cf08ea
);
impl windows_core::RuntimeType for ICalendar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICalendar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Clone: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetToMin: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetToMax: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    Languages: usize,
    pub NumeralSystem: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetNumeralSystem: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetCalendarSystem: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ChangeCalendarSystem: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetClock: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ChangeClock: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    GetDateTime: usize,
    SetDateTime: usize,
    pub SetToNow: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FirstEra:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastEra:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NumberOfEras:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Era: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEra: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AddEras: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EraAsFullString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub EraAsString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FirstYearInThisEra:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastYearInThisEra:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NumberOfYearsInThisEra:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Year: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetYear: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AddYears: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub YearAsString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub YearAsTruncatedString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub YearAsPaddedString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FirstMonthInThisYear:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastMonthInThisYear:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NumberOfMonthsInThisYear:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Month: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMonth: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AddMonths: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MonthAsFullString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub MonthAsString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub MonthAsFullSoloString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub MonthAsSoloString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub MonthAsNumericString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub MonthAsPaddedNumericString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub AddWeeks: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub FirstDayInThisMonth:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastDayInThisMonth:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NumberOfDaysInThisMonth:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Day: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDay: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AddDays: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DayAsString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DayAsPaddedString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    DayOfWeek: usize,
    pub DayOfWeekAsFullString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DayOfWeekAsString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DayOfWeekAsFullSoloString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DayOfWeekAsSoloString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FirstPeriodInThisDay:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastPeriodInThisDay:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NumberOfPeriodsInThisDay:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Period:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPeriod: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AddPeriods: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub PeriodAsFullString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub PeriodAsString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FirstHourInThisPeriod:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastHourInThisPeriod:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NumberOfHoursInThisPeriod:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Hour: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetHour: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AddHours: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub HourAsString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub HourAsPaddedString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Minute:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMinute: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AddMinutes: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MinuteAsString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub MinuteAsPaddedString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Second:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSecond: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AddSeconds: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SecondAsString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SecondAsPaddedString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Nanosecond:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetNanosecond:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AddNanoseconds:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub NanosecondAsString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub NanosecondAsPaddedString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Compare: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i32,
    ) -> windows_core::HRESULT,
    CompareDateTime: usize,
    pub CopyTo: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FirstMinuteInThisHour:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastMinuteInThisHour:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NumberOfMinutesInThisHour:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub FirstSecondInThisMinute:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastSecondInThisMinute:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NumberOfSecondsInThisMinute:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ResolvedLanguage: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub IsDaylightSavingTime:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICalendarFactory,
    ICalendarFactory_Vtbl,
    0x83f58412_e56b_4c75_a66e_0f63d57758a6
);
impl windows_core::RuntimeType for ICalendarFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICalendarFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    CreateCalendarDefaultCalendarAndClock: usize,
    CreateCalendar: usize,
}
windows_core::imp::define_interface!(
    ICalendarFactory2,
    ICalendarFactory2_Vtbl,
    0xb44b378c_ca7e_4590_9e72_ea2bec1a5115
);
impl windows_core::RuntimeType for ICalendarFactory2 {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICalendarFactory2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    CreateCalendarWithTimeZone: usize,
}
windows_core::imp::define_interface!(
    ITimeZoneOnCalendar,
    ITimeZoneOnCalendar_Vtbl,
    0xbb3c25e5_46cf_4317_a3f5_02621ad54478
);
impl windows_core::RuntimeType for ITimeZoneOnCalendar {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeZoneOnCalendar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetTimeZone: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ChangeTimeZone: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub TimeZoneAsFullString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub TimeZoneAsString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
