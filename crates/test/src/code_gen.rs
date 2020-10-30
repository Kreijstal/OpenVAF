use crate::{middle_test, PrettyError};
use openvaf_codegen_llvm::inkwell::context::Context;
use openvaf_codegen_llvm::inkwell::module::Linkage;
use openvaf_codegen_llvm::inkwell::targets::{
    CodeModel, InitializationConfig, RelocMode, Target, TargetMachine,
};
use openvaf_codegen_llvm::inkwell::OptimizationLevel;
use openvaf_codegen_llvm::LlvmCodegen;
use openvaf_derivatives::generate_derivatives;
use openvaf_diagnostics::{MultiDiagnostic, StandardPrinter};
use openvaf_middle::cfg::serde_dump::CfgDump;
use openvaf_middle::cfg::{ControlFlowGraph, START_BLOCK};
use openvaf_middle::const_fold::ConstantPropagation;
use openvaf_transformations::{Simplify, SimplifyBranches, Verify};
use std::path::PathBuf;
use tracing::info_span;

fn codegen_test(model: &'static str) -> Result<(), PrettyError> {
    let tspan = info_span!(target: "test", "HIR_LOWERING", model = model);
    let _enter = tspan.enter();
    let mut main_file = PathBuf::new();

    main_file.push("integration");
    main_file.push(model);

    let mut file_name = model.to_lowercase();
    file_name.push_str(".va");

    middle_test(main_file.join(file_name), |mir| {
        for module in &mir.modules {
            let mut cfg = module.analog_cfg.borrow_mut();

            let mut errors = MultiDiagnostic(Vec::new());
            generate_derivatives(&mut cfg, &mir, &mut errors);

            if !errors.is_empty() {
                return Err(errors.user_facing::<StandardPrinter>().into());
            }

            cfg.run_pass(ConstantPropagation::default());

            let file_name = format!("{}_{}_cfg.yaml", model, module.ident);
            std::fs::write(
                main_file.join(file_name),
                serde_yaml::to_string(&CfgDump {
                    mir: &mir,
                    cfg: &cfg,
                    blocks_in_resverse_postorder: true,
                })
                .expect("Serialization failed!"),
            )?;

            cfg.run_pass(SimplifyBranches);
            cfg.run_pass(Simplify);

            let malformations = cfg.run_pass(Verify(&mir));

            let file_name = format!("{}_{}_cfg_malformations.yaml", model, module.ident);
            std::fs::write(
                main_file.join(file_name),
                serde_yaml::to_string(&malformations).expect("Serialization failed!"),
            )?;

            let file_name = format!("{}_{}_cfg_simplified.yaml", model, module.ident);
            std::fs::write(
                main_file.join(file_name),
                serde_yaml::to_string(&CfgDump {
                    mir: &mir,
                    cfg: &cfg,
                    blocks_in_resverse_postorder: true,
                })
                .expect("Serialization failed!"),
            )?;

            assert!(malformations.is_empty());
        }

        let span = info_span!("Codegen begins!");
        let _enter = span.enter();

        let llvm_context = Context::create();

        Target::initialize_all(&InitializationConfig::default());

        let default_triple = TargetMachine::get_default_triple();
        let target = Target::from_triple(&default_triple)
            .unwrap()
            .create_target_machine(
                &default_triple,
                "x86-64",
                "",
                OptimizationLevel::Aggressive,
                RelocMode::Default,
                CodeModel::Default,
            )
            .unwrap();
        let target_data = target.get_target_data();
        let mut codegen_ctx = LlvmCodegen::new(&mir, &llvm_context, false, &target_data, "test");
        for module in &mir.modules {
            let cfg = module.analog_cfg.borrow();
            let function = codegen_ctx.module.add_function(
                &module.ident.name.as_str(),
                llvm_context.void_type().fn_type(&[], false),
                Some(Linkage::External),
            );
            let mut codegen = codegen_ctx.cfg_codegen(&cfg, function);
            codegen
                .ctx
                .builder
                .position_at_end(codegen.blocks[START_BLOCK]);
            codegen.alloc_vars_and_branches(|cg, access, branch| {
                cg.ctx.builder.build_alloca(
                    cg.ctx.context.f64_type(),
                    &format!("{}({})", access, mir[branch].ident),
                )
            });

            codegen.build_blocks();
            codegen_ctx.builder.build_return(None);
        }

        codegen_ctx.module.verify().unwrap();

        Ok(())
    })
}

macro_rules! code_gen_tests {
    ($model:ident) => {
        paste::item! {
            #[test]
            pub fn [< $model _CODEGEN>]()-> Result<(), PrettyError> {
                codegen_test(stringify!($model))
            }
        }
    };
}

code_gen_tests!(HICUML2);
//middle_tests!(HICUML0);
code_gen_tests!(BSIMSOI);
code_gen_tests!(BSIMBULK);
code_gen_tests!(BSIMCMG);
code_gen_tests!(BSIMIMG);
code_gen_tests!(VBIC_4T_IT_XF_HO);
code_gen_tests!(DIODE);
