impl ::core::cmp::PartialEq for UserDataAccount {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataAccount {}
impl ::core::fmt::Debug for UserDataAccount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccount").field(&self.0).finish()
    }
}
impl ::core::default::Default for UserDataAccountContentKinds {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UserDataAccountContentKinds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountContentKinds").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for UserDataAccountContentKinds {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for UserDataAccountContentKinds {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for UserDataAccountContentKinds {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for UserDataAccountContentKinds {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for UserDataAccountContentKinds {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::cmp::PartialEq for UserDataAccountManagerForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataAccountManagerForUser {}
impl ::core::fmt::Debug for UserDataAccountManagerForUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountManagerForUser").field(&self.0).finish()
    }
}
impl ::core::default::Default for UserDataAccountOtherAppReadAccess {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UserDataAccountOtherAppReadAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountOtherAppReadAccess").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for UserDataAccountStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataAccountStore {}
impl ::core::fmt::Debug for UserDataAccountStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountStore").field(&self.0).finish()
    }
}
impl ::core::default::Default for UserDataAccountStoreAccessType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UserDataAccountStoreAccessType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountStoreAccessType").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for UserDataAccountStoreChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataAccountStoreChangedEventArgs {}
impl ::core::fmt::Debug for UserDataAccountStoreChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountStoreChangedEventArgs").field(&self.0).finish()
    }
}
