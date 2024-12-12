use test_bindgen::ref_params::*;
use windows::core::*;

#[implement(IDynamicConceptProviderConcept)]
#[derive(Default)]
struct Test(std::sync::RwLock<Option<IUnknown>>);

impl IDynamicConceptProviderConcept_Impl for Test_Impl {
    fn GetConcept(
        &self,
        _: Ref<IModelObject>,
        _: *const GUID,
        concept: OutRef<IUnknown>,
        _: OutRef<IKeyStore>,
        _: *mut bool,
    ) -> Result<()> {
        let this = self.0.read().unwrap();
        concept.write(this.clone())?;
        Ok(())
    }
    fn SetConcept(
        &self,
        _: Ref<IModelObject>,
        _: *const GUID,
        concept: Ref<IUnknown>,
        _: Ref<IKeyStore>,
    ) -> Result<()> {
        let mut this = self.0.write().unwrap();
        *this = concept.cloned();
        Ok(())
    }
    fn NotifyParent(&self, _: Ref<IModelObject>) -> Result<()> {
        todo!()
    }
    fn NotifyParentChange(&self, _: Ref<IModelObject>) -> Result<()> {
        todo!()
    }
    fn NotifyDestruct(&self) -> Result<()> {
        todo!()
    }
}

#[test]
fn test() {
    unsafe {
        let identity: IUnknown = Test::default().into();

        let test: IDynamicConceptProviderConcept = Test::default().into();

        test.SetConcept(None, std::ptr::null(), &identity, None)
            .unwrap();

        let mut concept = None;
        test.GetConcept(
            None,
            std::ptr::null(),
            &mut concept,
            None,
            std::ptr::null_mut(),
        )
        .unwrap();

        assert_eq!(identity, concept.unwrap());
    }
}
