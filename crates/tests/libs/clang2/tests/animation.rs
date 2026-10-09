use windows_metadata::{
    Type,
    reader::{HasAttributes, Index, Item, TypeCategory},
};

#[allow(dead_code)]
#[path = "../../../../tools/bindings/src/animation.rs"]
mod animation;
#[allow(dead_code)]
#[path = "../sdk.rs"]
mod sdk;

#[cfg(target_env = "msvc")]
#[allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    clippy::upper_case_acronyms,
    clippy::missing_transmute_annotations
)]
mod consumer {
    mod bindings {
        include!(concat!(env!("OUT_DIR"), "/animation.rs"));
    }
    mod manager {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../libs/animation/src/manager.rs"
        ));
    }
    mod storyboard {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../libs/animation/src/storyboard.rs"
        ));
    }
    mod transition {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../libs/animation/src/transition.rs"
        ));
    }
    mod variable {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../libs/animation/src/variable.rs"
        ));
    }
    pub use bindings::*;
    pub use manager::Manager;
    pub use storyboard::{Keyframe, Storyboard};
    pub use transition::{Transition, TransitionLibrary};
    pub use variable::Variable;
    pub use windows_core::Result;
    use windows_core::{Interface, create_instance};
}

#[test]
fn consumer_closure_agrees_across_targets_and_translation_units() {
    assert_eq!(animation::roots().len(), 8);
    let output = std::path::Path::new(env!("OUT_DIR")).join("animation-profile");
    std::fs::create_dir_all(&output).unwrap();
    for target in [
        "i686-pc-windows-msvc",
        "x86_64-pc-windows-msvc",
        "aarch64-pc-windows-msvc",
    ] {
        let mut previous = None;
        for reversed in [false, true] {
            let mut inputs = animation::inputs();
            if reversed {
                inputs.reverse();
            }
            let plan = animation::project(
                target,
                &sdk::include(),
                &sdk::tools()
                    .join("..")
                    .join("libs")
                    .join("clang2")
                    .join("src")
                    .join("sal.h"),
                inputs,
            )
            .unwrap();
            assert!(plan.omitted().is_empty(), "{:?}", plan.omitted());
            let rdl = plan.rdl();
            if let Some(previous) = &previous {
                assert_eq!(&rdl, previous);
            } else {
                previous = Some(rdl.clone());
            }
            let winmd = output.join(format!("{target}.winmd"));
            std::fs::write(output.join(format!("{target}.rdl")), &rdl).unwrap();
            windows_rdl::reader()
                .input_text(&rdl)
                .input(sdk::projection_metadata())
                .reference_default()
                .output(&winmd)
                .write()
                .unwrap();
            let index = Index::read(&winmd).unwrap();
            for name in [
                "UI_ANIMATION_UPDATE_RESULT",
                "UI_ANIMATION_SCHEDULING_RESULT",
            ] {
                assert_eq!(
                    index.expect("Animation", name).category(),
                    TypeCategory::Enum
                );
            }
            assert!(
                index
                    .expect("Animation", "UI_ANIMATION_SECONDS")
                    .has_attribute("NativeTypedefAttribute")
            );
            assert!(
                index
                    .expect("Animation", "UI_ANIMATION_KEYFRAME")
                    .has_attribute("NativeTypedefAttribute")
            );
            let manager = index.expect("Animation", "IUIAnimationManager2");
            let update = manager
                .methods()
                .find(|method| method.name() == "Update")
                .unwrap();
            assert_eq!(
                update.signature(&[]).types[0],
                Type::value_named("Animation", "UI_ANIMATION_SECONDS")
            );
            let storyboard = index.expect("Animation", "IUIAnimationStoryboard2");
            let add = storyboard
                .methods()
                .find(|method| method.name() == "AddTransitionAtKeyframe")
                .unwrap();
            assert_eq!(
                add.signature(&[]).types[2],
                Type::value_named("Animation", "UI_ANIMATION_KEYFRAME")
            );
            for root in animation::roots() {
                let item = index.expect_item("Animation", root);
                if root.starts_with("UIAnimation") {
                    assert!(matches!(item, Item::Const(_)));
                } else {
                    assert!(matches!(item, Item::Type(_)));
                }
            }
            sdk::animation_bindings(&winmd, &output.join(format!("{target}.rs")));
        }
    }
}

fn parity_type(index: &Index, ty: &Type) -> Type {
    match ty {
        Type::PtrConst(target, depth) => {
            Type::PtrConst(Box::new(parity_type(index, target)), *depth)
        }
        Type::PtrMut(target, depth) => Type::PtrMut(Box::new(parity_type(index, target)), *depth),
        Type::ValueName(name) | Type::ClassName(name) => {
            let value = matches!(ty, Type::ValueName(_));
            if name.name == "HRESULT"
                || (name.namespace == "Windows.Foundation" && name.name == "HResult")
            {
                return Type::value_named("Contract", "HRESULT");
            }
            if !name.name.starts_with("UI_ANIMATION_")
                && let Some(definition) = index.get(&name.namespace, &name.name).next()
                && definition.has_attribute("NativeTypedefAttribute")
            {
                return parity_type(index, &definition.fields().next().unwrap().ty());
            }
            let namespace = if name.namespace == "Animation"
                || name.name.starts_with("IUIAnimation")
                || name.name.starts_with("UI_ANIMATION_")
            {
                "Animation"
            } else {
                &name.namespace
            };
            if value {
                Type::value_named(namespace, &name.name)
            } else {
                Type::class_named(namespace, &name.name)
            }
        }
        _ => ty.clone(),
    }
}

#[test]
fn animation_cutover_matches_committed_interface_abi_and_public_type_names() {
    let output = std::path::Path::new(env!("OUT_DIR")).join("animation-parity");
    std::fs::create_dir_all(&output).unwrap();
    let baseline_path = output.join("committed.winmd");
    windows_rdl::reader()
        .input(
            sdk::tools()
                .join("..")
                .join("..")
                .join("metadata")
                .join("win32")
                .join("uianimation.rdl"),
        )
        .reference_default()
        .output(&baseline_path)
        .write()
        .unwrap();
    let baseline = Index::read(&baseline_path).unwrap();
    for target in [
        "i686-pc-windows-msvc",
        "x86_64-pc-windows-msvc",
        "aarch64-pc-windows-msvc",
    ] {
        let plan = animation::project(
            target,
            &sdk::include(),
            &sdk::tools()
                .join("..")
                .join("libs")
                .join("clang2")
                .join("src")
                .join("sal.h"),
            animation::inputs(),
        )
        .unwrap();
        let actual_path = output.join(format!("{target}.winmd"));
        windows_rdl::reader()
            .input_text(&plan.rdl())
            .input(sdk::projection_metadata())
            .reference_default()
            .output(&actual_path)
            .write()
            .unwrap();
        let actual = Index::read(actual_path).unwrap();
        for name in [
            "UI_ANIMATION_SECONDS",
            "UI_ANIMATION_UPDATE_RESULT",
            "UI_ANIMATION_SCHEDULING_RESULT",
        ] {
            let expected = baseline.expect("Windows.Win32", name);
            let projected = actual.expect("Animation", name);
            assert_eq!(expected.category(), projected.category(), "{target} {name}");
            let fields = |index: &Index, definition: windows_metadata::reader::TypeDef<'_>| {
                definition
                    .fields()
                    .map(|field| {
                        (
                            field.name().to_string(),
                            parity_type(index, &field.ty()),
                            field.constant().map(|constant| constant.value()),
                        )
                    })
                    .collect::<Vec<_>>()
            };
            assert_eq!(
                fields(&baseline, expected),
                fields(&actual, projected),
                "{target} {name}"
            );
        }
        let expected = baseline.expect("Windows.Win32", "UI_ANIMATION_KEYFRAME");
        let projected = actual.expect("Animation", "UI_ANIMATION_KEYFRAME");
        assert_eq!(
            expected.fields().next().unwrap().ty(),
            Type::PtrMut(Box::new(Type::Void), 1)
        );
        let Type::PtrMut(target_type, 1) = projected.fields().next().unwrap().ty() else {
            panic!("source keyframe typedef must retain one native pointer")
        };
        let Type::ValueName(record) = *target_type else {
            panic!("source keyframe typedef must point to its captured record")
        };
        assert_eq!(
            actual.expect(&record.namespace, &record.name).category(),
            TypeCategory::Struct
        );
        for name in animation::roots()
            .into_iter()
            .filter(|name| name.starts_with("IUIAnimation"))
        {
            let expected = baseline.expect("Windows.Win32", name);
            let projected = actual.expect("Animation", name);
            assert_eq!(
                expected.find_attribute("GuidAttribute").unwrap().value(),
                projected.find_attribute("GuidAttribute").unwrap().value(),
                "{target} {name}"
            );
            let expected_methods: Vec<_> = expected.methods().collect();
            let actual_methods: Vec<_> = projected.methods().collect();
            assert_eq!(
                expected_methods.len(),
                actual_methods.len(),
                "{target} {name}"
            );
            for (expected, actual_method) in expected_methods.iter().zip(actual_methods) {
                assert_eq!(expected.name(), actual_method.name(), "{target} {name}");
                let expected = expected.signature(&[]);
                let projected = actual_method.signature(&[]);
                assert_eq!(
                    parity_type(&baseline, &expected.return_type),
                    parity_type(&actual, &projected.return_type),
                    "{target} {name} {}",
                    actual_method.name()
                );
                let expected: Vec<_> = expected
                    .types
                    .iter()
                    .map(|ty| parity_type(&baseline, ty))
                    .collect();
                let projected: Vec<_> = projected
                    .types
                    .iter()
                    .map(|ty| parity_type(&actual, ty))
                    .collect();
                assert_eq!(
                    expected,
                    projected,
                    "{target} {name} {}",
                    actual_method.name()
                );
            }
        }
    }
}

#[cfg(target_env = "msvc")]
#[test]
fn consumer_identity_slots_and_opaque_keyframe_match_native_headers() {
    use consumer::*;
    use windows_core::Interface;
    unsafe extern "C" {
        fn AnimationIdentities(values: *const windows_core::GUID) -> bool;
        fn AnimationSlot(index: u32) -> usize;
        fn AnimationKeyframeSize() -> usize;
        fn AnimationKeyframeAlign() -> usize;
    }
    let identities = [
        UIAnimationManager2,
        UIAnimationTransitionLibrary2,
        IUIAnimationManager2::IID,
        IUIAnimationStoryboard2::IID,
        IUIAnimationTransition2::IID,
        IUIAnimationTransitionLibrary2::IID,
        IUIAnimationVariable2::IID,
        IDCompositionAnimation::IID,
    ];
    assert!(unsafe { AnimationIdentities(identities.as_ptr()) });
    let slots = [
        core::mem::offset_of!(IUIAnimationManager2_Vtbl, CreateAnimationVariable),
        core::mem::offset_of!(IUIAnimationManager2_Vtbl, CreateStoryboard),
        core::mem::offset_of!(IUIAnimationManager2_Vtbl, Update),
        core::mem::offset_of!(IUIAnimationManager2_Vtbl, ScheduleTransition),
        core::mem::offset_of!(IUIAnimationStoryboard2_Vtbl, AddTransition),
        core::mem::offset_of!(IUIAnimationStoryboard2_Vtbl, AddKeyframeAfterTransition),
        core::mem::offset_of!(IUIAnimationStoryboard2_Vtbl, AddTransitionAtKeyframe),
        core::mem::offset_of!(IUIAnimationStoryboard2_Vtbl, Schedule),
        core::mem::offset_of!(
            IUIAnimationTransitionLibrary2_Vtbl,
            CreateAccelerateDecelerateTransition
        ),
        core::mem::offset_of!(IUIAnimationTransitionLibrary2_Vtbl, CreateLinearTransition),
        core::mem::offset_of!(
            IUIAnimationTransitionLibrary2_Vtbl,
            CreateInstantaneousTransition
        ),
        core::mem::offset_of!(IUIAnimationVariable2_Vtbl, GetValue),
        core::mem::offset_of!(IUIAnimationVariable2_Vtbl, GetCurve),
    ];
    for (index, offset) in slots.into_iter().enumerate() {
        assert_eq!(offset, unsafe { AnimationSlot(index as u32) });
    }
    assert_eq!(size_of::<Keyframe>(), unsafe { AnimationKeyframeSize() });
    assert_eq!(align_of::<Keyframe>(), unsafe { AnimationKeyframeAlign() });
}

#[cfg(target_env = "msvc")]
#[test]
fn real_animation_lifecycle_and_native_curve_handoff() {
    use consumer::*;
    use windows_core::Interface;
    #[repr(C)]
    #[derive(Default)]
    struct CurveStats {
        reset: u32,
        begin: u32,
        cubic: u32,
        sinusoidal: u32,
        repeat: u32,
        end: u32,
    }
    unsafe extern "C" {
        fn AnimationCurve(stats: *mut CurveStats) -> *mut core::ffi::c_void;
    }
    windows_core::init_mta().unwrap();
    let manager = Manager::new().unwrap();
    let variable = manager.create_variable(0.0).unwrap();
    assert_eq!(variable.value().unwrap(), 0.0);
    let library = TransitionLibrary::new().unwrap();
    let linear = library.linear(2.0, 10.0).unwrap();
    manager
        .schedule_transition(&variable, &linear, 0.0)
        .unwrap();
    manager.update(0.0).unwrap();
    let mut stats = CurveStats::default();
    let curve = unsafe { IDCompositionAnimation::from_raw(AnimationCurve(&mut stats)) };
    variable.copy_curve(&curve).unwrap();
    drop(curve);
    assert!(stats.cubic > 0);
    assert!(stats.end > 0);
    manager.update(1.0).unwrap();
    assert!((variable.value().unwrap() - 5.0).abs() < 1e-8);
    manager.update(2.0).unwrap();
    assert!((variable.value().unwrap() - 10.0).abs() < 1e-8);
    let storyboard = manager.create_storyboard().unwrap();
    let first = library.accelerate_decelerate(1.0, 20.0, 0.2, 0.8).unwrap();
    let keyframe = storyboard.add_transition(&variable, &first).unwrap();
    let second = library.linear(1.0, 30.0).unwrap();
    storyboard
        .add_transition_at_keyframe(&variable, &second, keyframe)
        .unwrap();
    storyboard.schedule(3.0).unwrap();
    manager.update(3.0).unwrap();
    manager.update(4.0).unwrap();
    assert!((variable.value().unwrap() - 20.0).abs() < 1e-8);
    manager.update(5.0).unwrap();
    assert!((variable.value().unwrap() - 30.0).abs() < 1e-8);
    let instantaneous = library.instantaneous(99.0).unwrap();
    manager
        .schedule_transition(&variable, &instantaneous, 6.0)
        .unwrap();
    manager.update(6.0).unwrap();
    assert_eq!(variable.value().unwrap(), 99.0);
}
