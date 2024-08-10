use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;
use std::os::raw::c_char;
use std::path::Path;
use std::ptr;

use lasso::Rodeo;
use libc::c_void;
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::Deref;
use std::error::Error;

use llvm_sys::core::{LLVMCreateMessage, LLVMDisposeMessage};
pub const UNNAMED: *const c_char = b"\0".as_ptr() as *const c_char;
#[derive(Eq)]
#[repr(transparent)]
pub struct LLVMString {
    ptr: *const c_char,
}

impl LLVMString {
    pub unsafe fn new(ptr: *const c_char) -> Self {
        LLVMString { ptr }
    }

    pub(crate) fn create_from_str(string: &str) -> LLVMString {
        let msg = CString::new(string).unwrap();
        unsafe { LLVMString::new(LLVMCreateMessage(msg.as_ptr())) }
    }

    pub fn create_from_c_str(string: &CStr) -> LLVMString {
        unsafe { LLVMString::new(LLVMCreateMessage(string.as_ptr())) }
    }
}

impl Deref for LLVMString {
    type Target = CStr;

    fn deref(&self) -> &Self::Target {
        unsafe { CStr::from_ptr(self.ptr) }
    }
}

impl Debug for LLVMString {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.deref())
    }
}

impl Display for LLVMString {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.deref().to_string_lossy())
    }
}

impl PartialEq for LLVMString {
    fn eq(&self, other: &LLVMString) -> bool {
        **self == **other
    }
}

impl Error for LLVMString {
    fn description(&self) -> &str {
        self.to_str().expect("Could not convert LLVMString to str (likely invalid unicode)")
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

impl Drop for LLVMString {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeMessage(self.ptr as *mut _);
        }
    }
}

use llvm_sys::target_machine::{LLVMCodeGenOptLevel,LLVMGetHostCPUFeatures, LLVMGetHostCPUName
};
    use llvm_sys::core::{
    LLVMGetDiagInfoDescription, LLVMGetDiagInfoSeverity
};
use target::spec::Target;

mod builder;
mod context;
mod declarations;
mod intrinsics;
mod types;

mod callbacks;
#[cfg(test)]
mod tests;

pub use builder::{Builder, BuilderVal, MemLoc};
pub use callbacks::CallbackFun;
pub use context::CodegenCx;
pub struct LLVMBackend<'t> {
    target: &'t Target,
    target_cpu: String,
    features: String,
}

impl<'t> LLVMBackend<'t> {
    pub fn new(
        cg_opts: &[String],
        target: &'t Target,
        mut target_cpu: String,
        target_features: &[String],
    ) -> LLVMBackend<'t> {
        if target_cpu == "generic" {
            target_cpu = target.options.cpu.clone();
        }

        let mut features = vec![];
        if target_cpu == "native" {
            let features_string = unsafe {
                let ptr = LLVMGetHostCPUFeatures();
                let features_string = if !ptr.is_null() {
                    CStr::from_ptr(ptr)
                        .to_str()
                        .unwrap_or_else(|e| {
                            unreachable!("LLVM returned a non-utf8 features string: {}", e);
                        })
                        .to_owned()
                } else {
                    unreachable!(
                        "could not allocate host CPU features, LLVM returned a `null` string"
                    );
                };

                LLVMDisposeMessage(ptr as *mut c_char);

                features_string
            };
            features.extend(features_string.split(',').map(String::from));

            target_cpu = unsafe {
                let ptr = LLVMGetHostCPUName();
                let cpu = if !ptr.is_null() {
                    CStr::from_ptr(ptr)
                        .to_str()
                        .unwrap_or_else(|e| {
                            unreachable!("LLVM returned a non-utf8 features string: {}", e);
                        })
                        .to_owned()
                } else {
                    unreachable!(
                        "could not allocate host CPU features, LLVM returned a `null` string"
                    );
                };

                LLVMDisposeMessage(ptr as *mut c_char);

                cpu
            };
        }

        features
            .extend(target.options.features.split(',').filter(|v| !v.is_empty()).map(String::from));
        features.extend(target_features.iter().cloned());

        // TODO add target options here if we ever have any
        // https://reviews.llvm.org/D145043
        //llvm_sys::initialization::init(cg_opts, &[]);
        //https://github.com/llvm/llvm-project/commit/62ef97e0631ff41ad53436477cecc7d3eb244d1b
        LLVMBackend { target, target_cpu, features: features.join(",") }
    }

    /// # Safety
    ///
    /// This function calls the LLVM-C Api which may not be entirely safe.
    /// Exercise caution!
    pub unsafe fn new_module(
        &self,
        name: &str,
        opt_lvl: LLVMCodeGenOptLevel,
    ) -> Result<ModuleLlvm, LLVMString> {
        ModuleLlvm::new(name, self.target, &self.target_cpu, &self.features, opt_lvl)
    }

    /// # Safety
    ///
    /// This function calls the LLVM-C Api which may not be entirely safe.
    /// Exercise caution!
    pub unsafe fn new_ctx<'a, 'll>(
        &'a self,
        literals: &'a Rodeo,
        module: &'ll ModuleLlvm,
    ) -> CodegenCx<'a, 'll> {
        CodegenCx::new(literals, module, self.target)
    }

    pub fn target(&self) -> &'t Target {
        self.target
    }
}


impl Drop for LLVMBackend<'_> {
    fn drop(&mut self) {}
}

extern "C" fn diagnostic_handler(info: &llvm_sys::LLVMDiagnosticInfo, _: *mut c_void) {
    let severity = unsafe { LLVMGetDiagInfoSeverity(info) };
    let msg = unsafe { LLVMString::new(LLVMGetDiagInfoDescription(info)) };
    match severity {
        llvm_sys::LLVMDiagnosticSeverity::LLVMDSError => log::error!("{msg}"),
        llvm_sys::LLVMDiagnosticSeverity::LLVMDSWarning => log::warn!("{msg}"),
        llvm_sys::LLVMDiagnosticSeverity::LLVMDSRemark => log::debug!("{msg}"),
        llvm_sys::LLVMDiagnosticSeverity::LLVMDSNote => log::trace!("{msg}"),
    }
}
pub struct ModuleLlvm {
    llcx: &'static mut llvm_sys::LLVMContext,
    // must be a raw pointer because the reference must not outlife self/the context
    llmod_raw: *const llvm_sys::LLVMModule,
    tm: &'static mut llvm_sys::LLVMTargetMachine,
    opt_lvl: LLVMCodeGenOptLevel,
}

impl ModuleLlvm {
    unsafe fn new(
        name: &str,
        target: &Target,
        target_cpu: &str,
        features: &str,
        opt_lvl: LLVMCodeGenOptLevel,
    ) -> Result<ModuleLlvm, LLVMString> {
        let llcx = llvm_sys::core::LLVMContextCreate();
        let target_data_layout = target.data_layout.clone();

        llvm_sys::core::LLVMContextSetDiagnosticHandler(llcx, Some(diagnostic_handler), ptr::null_mut());

        let name = CString::new(name).unwrap();
        let llmod = llvm_sys::core::LLVMModuleCreateWithNameInContext(name.as_ptr(), llcx);

        let data_layout = CString::new(&*target_data_layout).unwrap();
        llvm_sys::core::LLVMSetDataLayout(llmod, data_layout.as_ptr());
        set_normalized_target(llmod, &target.llvm_target);

        let tm = create_target(
            &target.llvm_target,
            target_cpu,
            features,
            opt_lvl,
            llvm_sys::target_machine::LLVMRelocMode::LLVMRelocPIC,
            llvm_sys::target_machine::LLVMCodeModel::LLVMCodeModelDefault,
        )?;
        let llmod_raw = llmod as _;

        Ok(ModuleLlvm { llcx, llmod_raw, tm, opt_lvl })
    }

    pub fn to_str(&self) -> LLVMString {
        unsafe { LLVMString::new(llvm_sys::core::LLVMPrintModuleToString(self.llmod())) }
    }

    pub fn llmod(&self) -> &llvm_sys::LLVMModule {
        unsafe { &*self.llmod_raw }
    }

pub fn optimize(&self) {
    let llmod = self.llmod();

    unsafe {
        // Create the pass builder
        let pb = llvm_sys::LLVMCreatePassBuilder();

        // Create the analysis managers
        let mam = llvm_sys::LLVMCreateModuleAnalysisManager();
        let fam = llvm_sys::LLVMCreateFunctionAnalysisManager();
        let cgam = llvm_sys::LLVMCreateCGSCCAnalysisManager();
        let lam = llvm_sys::LLVMCreateLoopAnalysisManager();

        // Register the analysis passes
        llvm_sys::LLVMPassBuilderRegisterModuleAnalyses(&*pb, &*mam);
        llvm_sys::LLVMPassBuilderRegisterFunctionAnalyses(&*pb, &*fam);
        llvm_sys::LLVMPassBuilderRegisterCGSCCAnalyses(&*pb, &*cgam);
        llvm_sys::LLVMPassBuilderRegisterLoopAnalyses(&*pb, &*lam);
        llvm_sys::LLVMPassBuilderCrossRegisterProxies(&*pb, &*lam, &*fam, &*cgam, &*mam);

        // Create the optimization pipeline
        let mpm = llvm_sys::LLVMPassBuilderBuildPerModuleDefaultPipeline(&*pb, self.opt_lvl);

        // Run the passes
        llvm_sys::core::LLVMRunPassManager(mpm, llmod);

        // Clean up
        llvm_sys::LLVMDisposePassManager(mpm);
        llvm_sys::LLVMDisposeLoopAnalysisManager(lam);
        llvm_sys::LLVMDisposeCGSCCAnalysisManager(cgam);
        llvm_sys::LLVMDisposeFunctionAnalysisManager(fam);
        llvm_sys::LLVMDisposeModuleAnalysisManager(mam);
        llvm_sys::LLVMDisposePassBuilder(pb);
    }
}



    /// Verifies this module and prints out  any errors
    ///
    /// # Returns
    /// Whether this module is valid (true if valid)
    pub fn verify_and_print(&self) -> bool {
        unsafe {
            llvm_sys::analysis::LLVMVerifyModule(self.llmod(), llvm_sys::analysis::LLVMVerifierFailureAction::LLVMPrintMessageAction, None)
                == 0
        }
    }

    /// Verifies this module and prints out an error for any errors
    ///
    /// # Returns
    /// An error messages in case the module invalid
    pub fn verify(&self) -> Option<LLVMString> {
        unsafe {
            let mut res = MaybeUninit::uninit();
            if llvm_sys::analysis::LLVMVerifyModule(
                self.llmod(),
                llvm_sys::analysis::LLVMVerifierFailureAction::LLVMReturnStatusAction,
                Some(&mut res),
            ) == 1
            {
                Some(res.assume_init())
            } else {
                None
            }
        }
    }

    pub fn emit_object(&self, dst: &Path) -> Result<(), LLVMString> {
        let path = CString::new(dst.to_str().unwrap()).unwrap();

        let mut err_string = MaybeUninit::uninit();
        let return_code = unsafe {
            // REVIEW: Why does LLVM need a mutable ptr to path...?

            llvm_sys::target_machine::LLVMTargetMachineEmitToFile(
                self.tm,
                self.llmod(),
                path.as_ptr(),
                llvm_sys::target_machine::LLVMCodeGenFileType::LLVMObjectFile,
                err_string.as_mut_ptr(),
            )
        };

        if return_code == 1 {
            unsafe {
                return Err(LLVMString::new(err_string.assume_init()));
            }
        }

        Ok(())
    }
}

impl Drop for ModuleLlvm {
    fn drop(&mut self) {
        unsafe {
            llvm_sys::target_machine::LLVMDisposeTargetMachine(&mut *(self.tm as *mut _));
            llvm_sys::core::LLVMContextDispose(&mut *(self.llcx as *mut _));
        }
    }
}
