use crate::*;

pub unsafe trait ComInterface {
    const GUID: Guid;
}
