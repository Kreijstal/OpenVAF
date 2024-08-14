//! Generated by `gen_ffi`, do not edit by hand.

use super::{FatPtr, NativePath};
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum LLVMCodeGenOptLevel {
    None = 0,
    Less = 1,
    Default = 2,
    Aggressive = 3,
}
pub type ParamFlags = u8;
pub type ModelcardInit = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f64,
        arg2: *mut i32,
        arg3: *mut *const ::std::os::raw::c_char,
        arg4: *mut f64,
        arg5: *mut i32,
        arg6: *mut f64,
        arg7: *mut i32,
        arg8: *mut ParamFlags,
    ),
>;
pub type VaeFun = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: usize,
        arg2: *mut FatPtr<f64>,
        arg3: *mut FatPtr<f64>,
        arg4: *mut FatPtr<f64>,
        arg5: *mut FatPtr<i32>,
        arg6: *mut *const ::std::os::raw::c_char,
        arg7: *mut FatPtr<f64>,
        arg8: *mut FatPtr<i32>,
        arg9: *mut FatPtr<f64>,
        arg10: *mut ::std::os::raw::c_void,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Slice<T> {
    pub ptr: *mut T,
    pub len: usize,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VfsEntry {
    pub name: Slice<u8>,
    pub data: Slice<u8>,
}
pub type Vfs = Slice<VfsEntry>;
#[repr(C)]
pub struct Opts {
    pub model: Slice<u8>,
    pub cache_dir: NativePath,
    pub include_dirs: Slice<NativePath>,
    pub macro_flags: Slice<Slice<u8>>,
    pub allow_lints: Slice<Slice<u8>>,
    pub warn_lints: Slice<Slice<u8>>,
    pub deny_lints: Slice<Slice<u8>>,
    pub opt_lvl: LLVMCodeGenOptLevel,
    pub target_cpu: Slice<u8>,
    pub target: Slice<u8>,
    pub cg_flags: Slice<Slice<u8>>,
    pub vfs: Vfs,
}
extern "C" {
    #[doc = "This function returns a pointer to the `functions` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_functions(
        lib: *const ::std::os::raw::c_void,
    ) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = "This function returns a pointer to the `functions.sym` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_function_symbols(
        lib: *const ::std::os::raw::c_void,
    ) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = "This function returns a pointer to the `opvars` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_opvars(
        lib: *const ::std::os::raw::c_void,
    ) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = "This function returns a pointer to the `params.real` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_real_params(
        lib: *const ::std::os::raw::c_void,
    ) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = "This function returns a pointer to the `params.unit.real` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_real_param_units(
        lib: *const ::std::os::raw::c_void,
    ) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = "This function returns a pointer to the `params.desc.real` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_real_param_descriptions(
        lib: *const ::std::os::raw::c_void,
    ) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = "This function returns a pointer to the `params.group.real` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_real_param_groups(
        lib: *const ::std::os::raw::c_void,
    ) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = "This function returns a pointer to the `params.integer` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_int_params(
        lib: *const ::std::os::raw::c_void,
    ) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = "This function returns a pointer to the `params.unit.integer` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_int_param_units(
        lib: *const ::std::os::raw::c_void,
    ) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = "This function returns a pointer to the `params.desc.integer` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_int_param_descriptions(
        lib: *const ::std::os::raw::c_void,
    ) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = "This function returns a pointer to the `params.group.integer` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_int_param_groups(
        lib: *const ::std::os::raw::c_void,
    ) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = "This function returns a pointer to the `params.string` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_str_params(
        lib: *const ::std::os::raw::c_void,
    ) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = "This function returns a pointer to the `params.unit.string` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_str_param_units(
        lib: *const ::std::os::raw::c_void,
    ) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = "This function returns a pointer to the `params.desc.string` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_str_param_descriptions(
        lib: *const ::std::os::raw::c_void,
    ) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = "This function returns a pointer to the `params.group.string` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_str_param_groups(
        lib: *const ::std::os::raw::c_void,
    ) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = "This function returns a pointer to the `nodes` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_nodes(
        lib: *const ::std::os::raw::c_void,
    ) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = "This function returns the value stored in the `functions.cnt` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_function_cnt(lib: *const ::std::os::raw::c_void) -> usize;
}
extern "C" {
    #[doc = "This function returns the value stored in the `opvars.cnt` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_opvars_cnt(lib: *const ::std::os::raw::c_void) -> usize;
}
extern "C" {
    #[doc = "This function returns the value stored in the `params.real.cnt` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_real_param_cnt(lib: *const ::std::os::raw::c_void) -> usize;
}
extern "C" {
    #[doc = "This function returns the value stored in the `params.integer.cnt` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_int_param_cnt(lib: *const ::std::os::raw::c_void) -> usize;
}
extern "C" {
    #[doc = "This function returns the value stored in the `params.string.cnt` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_str_param_cnt(lib: *const ::std::os::raw::c_void) -> usize;
}
extern "C" {
    #[doc = "This function returns the value stored in the `nodes.cnt` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_node_cnt(lib: *const ::std::os::raw::c_void) -> usize;
}
extern "C" {
    #[doc = "This function returns a pointer to the `params.real` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    #[doc = "`sym_name` must batch the schema fun.{NUM}params.real"]
    pub fn verilogae_real_fun_params(
        lib: *const ::std::os::raw::c_void,
        fun: *const ::std::os::raw::c_char,
    ) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = "This function returns a pointer to the `params.integer` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    #[doc = "`sym_name` must batch the schema fun.{NUM}params.integer"]
    pub fn verilogae_int_fun_params(
        lib: *const ::std::os::raw::c_void,
        fun: *const ::std::os::raw::c_char,
    ) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = "This function returns a pointer to the `params.string` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    #[doc = "`sym_name` must batch the schema fun.{NUM}params.string"]
    pub fn verilogae_str_fun_params(
        lib: *const ::std::os::raw::c_void,
        fun: *const ::std::os::raw::c_char,
    ) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = "This function returns a pointer to the `depbreak.real` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    #[doc = "`sym_name` must batch the schema fun.{NUM}depbreak.real"]
    pub fn verilogae_real_fun_depbreak(
        lib: *const ::std::os::raw::c_void,
        fun: *const ::std::os::raw::c_char,
    ) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = "This function returns a pointer to the `depbreak.integer` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    #[doc = "`sym_name` must batch the schema fun.{NUM}depbreak.integer"]
    pub fn verilogae_int_fun_depbreak(
        lib: *const ::std::os::raw::c_void,
        fun: *const ::std::os::raw::c_char,
    ) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = "This function returns a pointer to the `voltages` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    #[doc = "`sym_name` must batch the schema fun.{NUM}voltages"]
    pub fn verilogae_fun_voltages(
        lib: *const ::std::os::raw::c_void,
        fun: *const ::std::os::raw::c_char,
    ) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = "This function returns a pointer to the `currents` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    #[doc = "`sym_name` must batch the schema fun.{NUM}currents"]
    pub fn verilogae_fun_currents(
        lib: *const ::std::os::raw::c_void,
        fun: *const ::std::os::raw::c_char,
    ) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = "This function returns a pointer to the `voltages.default` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    #[doc = "`sym_name` must batch the schema fun.{NUM}voltages.default"]
    pub fn verilogae_fun_voltage_defaults(
        lib: *const ::std::os::raw::c_void,
        fun: *const ::std::os::raw::c_char,
    ) -> *const f64;
}
extern "C" {
    #[doc = "This function returns a pointer to the `currents.default` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    #[doc = "`sym_name` must batch the schema fun.{NUM}currents.default"]
    pub fn verilogae_fun_current_defaults(
        lib: *const ::std::os::raw::c_void,
        fun: *const ::std::os::raw::c_char,
    ) -> *const f64;
}
extern "C" {
    #[doc = "This function returns a pointer to the `params.real.cnt` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_real_fun_param_cnt(
        lib: *const ::std::os::raw::c_void,
        fun: *const ::std::os::raw::c_char,
    ) -> usize;
}
extern "C" {
    #[doc = "This function returns a pointer to the `params.integer.cnt` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_int_fun_param_cnt(
        lib: *const ::std::os::raw::c_void,
        fun: *const ::std::os::raw::c_char,
    ) -> usize;
}
extern "C" {
    #[doc = "This function returns a pointer to the `params.string.cnt` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_str_fun_param_cnt(
        lib: *const ::std::os::raw::c_void,
        fun: *const ::std::os::raw::c_char,
    ) -> usize;
}
extern "C" {
    #[doc = "This function returns a pointer to the `depbreak.real.cnt` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_real_fun_depbreak_cnt(
        lib: *const ::std::os::raw::c_void,
        fun: *const ::std::os::raw::c_char,
    ) -> usize;
}
extern "C" {
    #[doc = "This function returns a pointer to the `depbreak.integer.cnt` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_int_fun_depbreak_cnt(
        lib: *const ::std::os::raw::c_void,
        fun: *const ::std::os::raw::c_char,
    ) -> usize;
}
extern "C" {
    #[doc = "This function returns a pointer to the `voltages.cnt` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_fun_voltage_cnt(
        lib: *const ::std::os::raw::c_void,
        fun: *const ::std::os::raw::c_char,
    ) -> usize;
}
extern "C" {
    #[doc = "This function returns a pointer to the `currents.cnt` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_fun_current_cnt(
        lib: *const ::std::os::raw::c_void,
        fun: *const ::std::os::raw::c_char,
    ) -> usize;
}
extern "C" {
    #[doc = "This function returns a pointer to the `voltages.default.cnt` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_fun_voltage_default_cnt(
        lib: *const ::std::os::raw::c_void,
        fun: *const ::std::os::raw::c_char,
    ) -> usize;
}
extern "C" {
    #[doc = "This function returns a pointer to the `currents.default.cnt` global"]
    #[doc = " of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_fun_current_default_cnt(
        lib: *const ::std::os::raw::c_void,
        fun: *const ::std::os::raw::c_char,
    ) -> usize;
}
extern "C" {
    #[doc = " Obtains a pointer to the modelcard initialization function of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_init_modelcard(lib: *const ::std::os::raw::c_void) -> ModelcardInit;
}
extern "C" {
    #[doc = " Obtains a pointer to a model functions of a VerilogAE model loaded with `load`."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " `lib` must be a valid pointer returned by the `load` functions or `dlopen`"]
    pub fn verilogae_fun_ptr(
        lib: *const ::std::os::raw::c_void,
        fun: *const ::std::os::raw::c_char,
    ) -> VaeFun;
}
extern "C" {
    #[doc = " # Safety"]
    #[doc = " handle must be a valid model compiled with VerilogAE"]
    pub fn verilogae_module_name(
        lib: *const ::std::os::raw::c_void,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " All required parameters must be initialized appropriately"]
    pub fn verilogae_call_fun_parallel(
        fun: VaeFun,
        cnt: usize,
        voltages: *mut FatPtr<f64>,
        currents: *mut FatPtr<f64>,
        real_params: *mut FatPtr<f64>,
        int_params: *mut FatPtr<i32>,
        str_params: *mut *const ::std::os::raw::c_char,
        real_dep_break: *mut FatPtr<f64>,
        int_dep_break: *mut FatPtr<i32>,
        temp: *mut FatPtr<f64>,
        out: *mut ::std::os::raw::c_void,
    ) -> i32;
}
extern "C" {
    pub fn verilogae_new_opts() -> *mut Opts;
}
extern "C" {
    #[doc = " # Safety"]
    #[doc = " `opts` must be a valid pointer created with `new_opts`"]
    pub fn verilogae_free_opts(opts: *mut Opts);
}
extern "C" {
    #[doc = " # Safety"]
    #[doc = " * path must be valid for reads"]
    #[doc = " * opts must be valid for reads or null"]
    #[doc = " * opts must only contain valid data"]
    pub fn verilogae_export_vfs(path: NativePath, opts: *mut Opts) -> Vfs;
}
extern "C" {
    #[doc = " # Safety"]
    #[doc = " * path must be valid for reads"]
    #[doc = " * opts must be valid for reads or null"]
    #[doc = " * opts must only contain valid data"]
    pub fn verilogae_free_vfs(vfs: Vfs);
}
extern "C" {
    #[doc = " # Safety"]
    #[doc = " * path must be valid for reads"]
    #[doc = " * opts must be valid for reads or null"]
    #[doc = " * opts must only contain valid data"]
    pub fn verilogae_load(
        path: NativePath,
        full_compile: bool,
        opts: *const Opts,
    ) -> *const ::std::os::raw::c_void;
}
pub const PARAM_FLAGS_MIN_INCLUSIVE: ParamFlags = 1;
pub const PARAM_FLAGS_MAX_INCLUSIVE: ParamFlags = 2;
pub const PARAM_FLAGS_INVALID: ParamFlags = 4;
pub const PARAM_FLAGS_GIVEN: ParamFlags = 8;
