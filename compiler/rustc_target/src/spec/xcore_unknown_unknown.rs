use crate::spec::{LinkerFlavor, LldFlavor, PanicStrategy, RelocModel};
use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    Target {
        data_layout: "e-m:e-p:32:32-i1:8:32-i8:8:32-i16:16:32-i64:64-f64:64-v64:32-v128:32-a:0:32-i128:64-n32".to_string(),
<<<<<<< HEAD
        llvm_target: "xcore-unknown-unknown".to_string(),
        pointer_width: 32,
        arch: "xs2a".to_string(),
=======
        llvm_target: "xcore".to_string(),
        pointer_width: 32,
        arch: "xcore".to_string(),
>>>>>>> add-abi

        options: TargetOptions {
            linker_flavor: LinkerFlavor::Lld(LldFlavor::Ld),
            linker: Some("xmap".to_string()),
            cpu: "xs2a-generic".to_string(),
<<<<<<< HEAD
            max_atomic_width: Some(32),
            features: "+m".to_string(),
            executables: true,
=======
            max_atomic_width: Some(0),
            atomic_cas: false,
            executables: true,
            singlethread: false,
>>>>>>> add-abi
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            emit_debug_gdb_scripts: false,
            eh_frame_header: false,
            ..Default::default()
        },
    }
}
