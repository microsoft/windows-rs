mod Things {
    interface IThing {
        fn Method(&self, p1: u8) -> f32;
    }
    mod More {
        interface IOtherThing {
            fn OtherMethod(&self, p1: u8) -> f32;
        }
    }
}
