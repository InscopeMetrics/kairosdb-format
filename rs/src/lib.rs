pub mod inscopemetrics {
    pub mod kairosdb {
        pub mod protos {
            include!(concat!(
                env!("OUT_DIR"),
                "/io.inscopemetrics.kairosdb.proto.v2.rs"
            ));
        }
    }
}
