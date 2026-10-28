use super::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EffectKey(Key);

impl<T: Into<Key>> From<T> for EffectKey {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

pub(super) type EffectCleanup = Box<dyn FnOnce()>;

type EffectSetup = Box<dyn FnOnce() -> Option<EffectCleanup>>;

struct EffectRegistration {
    dependency: Box<dyn Any>,
    equals: fn(&dyn Any, &dyn Any) -> bool,
    key: EffectKey,
    setup: EffectSetup,
}

pub(super) struct EffectSlot {
    pub(super) cleanup: Option<EffectCleanup>,
    dependency: Box<dyn Any>,
    equals: fn(&dyn Any, &dyn Any) -> bool,
    key: EffectKey,
}

#[derive(Default)]
pub(super) struct EffectDraft {
    registrations: Vec<EffectRegistration>,
}

pub(super) struct PreparedEffects(Vec<PreparedEffect>);

enum PreparedEffect {
    Retained(EffectSlot),
    Setup(EffectRegistration),
}

impl EffectDraft {
    pub(super) fn register<D>(
        &mut self,
        key: EffectKey,
        dependency: D,
        setup: impl FnOnce() -> Option<EffectCleanup> + 'static,
    ) where
        D: PartialEq + 'static,
    {
        assert!(
            self.registrations
                .iter()
                .all(|registration| registration.key != key),
            "effect {key:?} declared more than once"
        );
        self.registrations.push(EffectRegistration {
            dependency: Box::new(dependency),
            equals: |left, right| {
                left.downcast_ref::<D>()
                    .zip(right.downcast_ref::<D>())
                    .is_some_and(|(left, right)| left == right)
            },
            key,
            setup: Box::new(setup),
        });
    }

    pub(super) fn prepare(self, slots: &mut Vec<EffectSlot>) -> PreparedEffects {
        let mut previous = std::mem::take(slots);
        let mut prepared = Vec::with_capacity(self.registrations.len());
        for registration in self.registrations {
            let retained = previous
                .iter()
                .position(|slot| slot.key == registration.key)
                .and_then(|index| {
                    (previous[index].equals)(
                        previous[index].dependency.as_ref(),
                        registration.dependency.as_ref(),
                    )
                    .then(|| previous.remove(index))
                });
            if let Some(slot) = retained {
                prepared.push(PreparedEffect::Retained(slot));
            } else {
                if let Some(index) = previous
                    .iter()
                    .position(|slot| slot.key == registration.key)
                    && let Some(cleanup) = previous.remove(index).cleanup
                {
                    cleanup();
                }
                prepared.push(PreparedEffect::Setup(registration));
            }
        }
        for slot in previous.iter_mut().rev() {
            if let Some(cleanup) = slot.cleanup.take() {
                cleanup();
            }
        }
        PreparedEffects(prepared)
    }

    pub(super) fn commit(self, slots: &mut Vec<EffectSlot>) {
        self.prepare(slots).commit(slots);
    }
}

impl PreparedEffects {
    pub(super) fn commit(self, slots: &mut Vec<EffectSlot>) {
        for effect in self.0 {
            match effect {
                PreparedEffect::Retained(slot) => slots.push(slot),
                PreparedEffect::Setup(registration) => slots.push(EffectSlot {
                    cleanup: (registration.setup)(),
                    dependency: registration.dependency,
                    equals: registration.equals,
                    key: registration.key,
                }),
            }
        }
    }
}

pub(super) fn cleanup_effects(effects: &mut [EffectSlot]) {
    for effect in effects.iter_mut().rev() {
        if let Some(cleanup) = effect.cleanup.take() {
            cleanup();
        }
    }
}
