use crate::core::GUID;
use std::collections::HashMap;

pub(crate) struct GuidConst {
    pub name: &'static str,
    pub guid: GUID,
}

inventory::collect!(GuidConst);

pub fn build_guid_hashmap() -> HashMap<GUID, &'static str> {
    inventory::iter::<GuidConst>().map(|gc| (gc.guid, gc.name)).collect()
}
