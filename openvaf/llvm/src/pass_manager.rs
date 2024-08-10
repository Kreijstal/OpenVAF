use libc::c_uint;

use crate::module::function_iter;
use crate::util::InvariantOpaque;
use crate::{Bool, Module, OptLevel, Value};

#[repr(C)]
pub struct PassBuilder(InvariantOpaque<'static>);

#[repr(C)]
pub struct ModulePassManager(InvariantOpaque<'static>);

#[repr(C)]
pub struct FunctionPassManager<'a>(InvariantOpaque<'a>);

#[repr(C)]
pub struct ModuleAnalysisManager(InvariantOpaque<'static>);

#[repr(C)]
pub struct FunctionAnalysisManager(InvariantOpaque<'static>);

#[repr(C)]
pub struct LoopAnalysisManager(InvariantOpaque<'static>);

#[repr(C)]
pub struct CGSCCAnalysisManager(InvariantOpaque<'static>);
extern "C" {

        pub fn LLVMCreatePassBuilder() -> *mut PassBuilder;
    pub fn LLVMDisposePassBuilder(PB: *mut PassBuilder);

    pub fn LLVMPassBuilderCreate() -> &'static mut PassBuilder;
    pub fn LLVMPassBuilderDispose(PB: &'static mut PassBuilder);

    pub fn LLVMPassBuilderSetOptLevel(PB: &PassBuilder, OptLevel: OptLevel);
    pub fn LLVMPassBuilderSetSizeLevel(PB: &PassBuilder, SizeLevel: c_uint);

    pub fn LLVMCreateModulePassManager() -> &'static mut ModulePassManager;
    pub fn LLVMCreateFunctionPassManager(M: &Module) -> &'static mut FunctionPassManager<'_>;
    pub fn LLVMDisposePassManager(PM: &'static mut ModulePassManager);

    pub fn LLVMCreateModuleAnalysisManager() -> &'static mut ModuleAnalysisManager;
    pub fn LLVMCreateFunctionAnalysisManager() -> &'static mut FunctionAnalysisManager;
    pub fn LLVMCreateLoopAnalysisManager() -> &'static mut LoopAnalysisManager;
    pub fn LLVMCreateCGSCCAnalysisManager() -> &'static mut CGSCCAnalysisManager;

    pub fn LLVMPassBuilderRegisterModuleAnalyses(
        PB: &PassBuilder,
        MAM: &ModuleAnalysisManager,
    );
    pub fn LLVMPassBuilderRegisterFunctionAnalyses(
        PB: &PassBuilder,
        FAM: &FunctionAnalysisManager,
    );
    pub fn LLVMPassBuilderRegisterLoopAnalyses(
        PB: &PassBuilder,
        LAM: &LoopAnalysisManager,
    );
    pub fn LLVMPassBuilderRegisterCGSCCAnalyses(
        PB: &PassBuilder,
        CGAM: &CGSCCAnalysisManager,
    );

    pub fn LLVMPassBuilderCrossRegisterProxies(
        PB: &PassBuilder,
        LAM: &LoopAnalysisManager,
        FAM: &FunctionAnalysisManager,
        CGAM: &CGSCCAnalysisManager,
        MAM: &ModuleAnalysisManager,
    );

    pub fn LLVMPassBuilderBuildPerModuleDefaultPipeline(
        PB: &PassBuilder,
        OptLevel: OptLevel,
    ) -> &'static mut ModulePassManager;

    pub fn LLVMRunPassManager(MPM: &ModulePassManager, M: &Module);

    // Additional functions for disposal
    pub fn LLVMDisposeModuleAnalysisManager(MAM: &'static mut ModuleAnalysisManager);
    pub fn LLVMDisposeFunctionAnalysisManager(FAM: &'static mut FunctionAnalysisManager);
    pub fn LLVMDisposeCGSCCAnalysisManager(CGAM: &'static mut CGSCCAnalysisManager);
    pub fn LLVMDisposeLoopAnalysisManager(LAM: &'static mut LoopAnalysisManager);
}

/// # Safety
/// This function calls the LLVM C API.
/// If the module or its contents have been incorrectly constructed this can cause UB
pub unsafe fn build_and_run_optimization_pipeline(module: &Module, opt_level: OptLevel) {
    let pb = LLVMPassBuilderCreate();
    LLVMPassBuilderSetOptLevel(pb, opt_level);

    let mam = LLVMCreateModuleAnalysisManager();
    let fam = LLVMCreateFunctionAnalysisManager();
    let lam = LLVMCreateLoopAnalysisManager();
    let cgam = LLVMCreateCGSCCAnalysisManager();

    LLVMPassBuilderRegisterModuleAnalyses(pb, mam);
    LLVMPassBuilderRegisterFunctionAnalyses(pb, fam);
    LLVMPassBuilderRegisterLoopAnalyses(pb, lam);
    LLVMPassBuilderRegisterCGSCCAnalyses(pb, cgam);

    LLVMPassBuilderCrossRegisterProxies(pb, lam, fam, cgam, mam);

    let mpm = LLVMPassBuilderBuildPerModuleDefaultPipeline(pb, opt_level);

    LLVMRunPassManager(mpm, module);

    LLVMDisposePassManager(mpm);
    LLVMPassBuilderDispose(pb);
}
