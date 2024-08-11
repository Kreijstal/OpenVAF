use std::ffi::CString;

use libc::c_char;
use llvm_sys::core::LLVMTypeOf;
use llvm_sys::prelude::LLVMBool;
use llvm_sys::LLVMType as Type;
use llvm_sys::LLVMValue as Value;

const False: LLVMBool = 0;

use crate::CodegenCx;

/// Declare a function.
///
/// If there’s a value with the same name already declared, the function will
/// update the declaration and return existing Value instead.
pub fn declare_raw_fn<'ll>(
    cx: &CodegenCx<'_, 'll>,
    name: &str,
    callconv: llvm_sys::LLVMCallConv,
    unnamed: llvm_sys::LLVMUnnamedAddr,
    ty: &'ll Type,
) -> &'ll Value {
    let name = CString::new(name).unwrap();
    unsafe {
        let llfn = llvm_sys::core::LLVMAddFunction(cx.llmod, name.as_ptr() as *const c_char, ty);

        llvm_sys::core::LLVMSetFunctionCallConv(llfn, callconv);
        llvm_sys::core::LLVMSetUnnamedAddress(llfn, unnamed);
        llfn
    }
}

impl<'a, 'll> CodegenCx<'a, 'll> {
    // pub fn target_cpu_attr(&self) -> &'ll Attribute {
    //     create_attr_string_value(self.llcx, "target-cpu", self.target_cpu)
    // }

    /// Declare a C ABI function.
    ///
    /// Only use this for foreign function ABIs and glue. For Rust functions use
    /// `declare_fn` instead.
    ///
    /// If there’s a value with the same name already declared, the function will
    /// update the declaration and return existing Value instead.
    pub fn declare_ext_fn(
        &self,
        name: &str,
        // unnamed: llvm_sys::LLVMUnnamedAddr,
        fn_type: &'ll Type,
    ) -> &'ll Value {
        declare_raw_fn(
            self,
            name,
            llvm_sys::LLVMCallConv::CCallConv,
            llvm_sys::LLVMUnnamedAddr::No,
            fn_type,
        )
    }

    /// Declare a internal function.
    pub fn declare_int_fn(&self, name: &str, fn_type: &'ll Type) -> &'ll Value {
        // Function addresses are never significant, allowing functions to be merged.
        let fun = declare_raw_fn(
            self,
            name,
            llvm_sys::LLVMCallConv::FastCallConv,
            llvm_sys::LLVMUnnamedAddr::Global,
            fn_type,
        );
        unsafe { llvm_sys::core::LLVMSetLinkage(fun, llvm_sys::LLVMLinkage::Internal) }
        fun
    }

    /// Declare a internal function.
    pub fn declare_int_c_fn(&self, name: &str, fn_type: &'ll Type) -> &'ll Value {
        // Function addresses are never significant, allowing functions to be merged.
        let fun = declare_raw_fn(
            self,
            name,
            llvm_sys::LLVMCallConv::CCallConv,
            llvm_sys::LLVMUnnamedAddr::Global,
            fn_type,
        );
        unsafe { llvm_sys::core::LLVMSetLinkage(fun, llvm_sys::LLVMLinkage::Internal) }
        fun
    }

    /// Declare a global with an intention to define it.
    ///
    /// Use this function when you intend to define a global. This function will
    /// return `None` if the name already has a definition associated with it.
    pub fn define_global(&self, name: &str, ty: &'ll Type) -> Option<&'ll Value> {
        if self.get_defined_value(name).is_some() {
            None
        } else {
            let name = CString::new(name).unwrap();
            let global = unsafe { llvm_sys::core::LLVMAddGlobal(self.llmod, ty, name.as_ptr()) };
            Some(global)
        }
    }

    /// Declare a private global
    ///
    /// Use this function when you intend to define a global without a name.
    pub fn define_private_global(&self, ty: &'ll Type) -> &'ll Value {
        unsafe {
            let global = llvm_sys::core::LLVMAddGlobal(self.llmod, ty, crate::UNNAMED);
            llvm_sys::core::LLVMSetLinkage(global, llvm_sys::LLVMLinkage::PrivateLinkage);
            global
        }
    }

    /// Gets declared value by name.
    pub fn get_declared_value(&self, name: &str) -> Option<&'ll Value> {
        let name = CString::new(name).unwrap();
        unsafe { llvm_sys::core::LLVMGetNamedGlobal(self.llmod, name.as_ptr()) }
    }

    /// Gets defined or externally defined (AvailableExternally linkage) value by
    /// name.
    pub fn get_defined_value(&self, name: &str) -> Option<&'ll Value> {
        self.get_declared_value(name).and_then(|val| {
            let declaration = unsafe { llvm_sys::core::LLVMIsDeclaration(val) != False };
            if !declaration {
                Some(val)
            } else {
                None
            }
        })
    }

    pub fn export_val(
        &self,
        name: &str,
        ty: &'ll Type,
        val: &'ll Value,
        is_const: bool,
    ) -> &'ll Value {
        unsafe {
            let res = self
                .define_global(name, ty)
                .unwrap_or_else(|| unreachable!("symbol '{}' already defined", name));

            llvm_sys::core::LLVMSetInitializer(res, val);
            llvm_sys::core::LLVMSetLinkage(res, llvm_sys::LLVMLinkage::ExternalLinkage);
            llvm_sys::core::LLVMSetUnnamedAddress(res, llvm_sys::LLVMUnnamedAddr::No);
            llvm_sys::core::LLVMSetDLLStorageClass(
                res,
                llvm_sys::LLVMDLLStorageClass::LLVMDLLExportStorageClass,
            );

            if is_const {
                llvm_sys::core::LLVMSetGlobalConstant(res, 1);
            }

            res
        }
    }

    pub fn global_const(&self, ty: &'ll Type, val: &'ll Value) -> &'ll Value {
        unsafe {
            let res = self.define_private_global(ty);
            llvm_sys::core::LLVMSetInitializer(res, val);
            llvm_sys::core::LLVMSetUnnamedAddress(res, llvm_sys::LLVMUnnamedAddr::No);
            llvm_sys::core::LLVMSetGlobalConstant(res, 1);
            res
        }
    }

    pub fn const_arr_ptr(&self, elem_ty: &'ll Type, vals: &[&'ll Value]) -> &'ll Value {
        for (i, val) in vals.iter().enumerate() {
            assert_eq!(
                unsafe { LLVMTypeOf(val) } as *const Type,
                elem_ty as *const Type,
                "val {i} not eq"
            )
        }

        let val = self.const_arr(elem_ty, vals);
        let ty = self.ty_array(elem_ty, vals.len() as u32);

        let sym = self.generate_local_symbol_name("arr");
        let global = self
            .define_global(&sym, ty)
            .unwrap_or_else(|| unreachable!("symbol {} already defined", sym));

        unsafe {
            llvm_sys::core::LLVMSetInitializer(global, val);
            llvm_sys::core::LLVMSetGlobalConstant(global, 1);
            llvm_sys::core::LLVMSetLinkage(global, llvm_sys::LLVMLinkage::Internal);
        }
        global
    }

    pub fn export_array(
        &self,
        name: &str,
        elem_ty: &'ll Type,
        vals: &[&'ll Value],
        is_const: bool,
        add_cnt: bool,
    ) -> &'ll Value {
        let arr = self.export_val(
            name,
            self.ty_array(elem_ty, vals.len() as u32),
            self.const_arr(elem_ty, vals),
            is_const,
        );

        if add_cnt {
            let name = format!("{}.cnt", name);
            self.export_val(&name, self.ty_size(), self.const_usize(vals.len()), true);
        }

        arr
    }

    pub fn export_zeroed_array(
        &self,
        name: &str,
        elem_ty: &'ll Type,
        len: usize,
        add_cnt: bool,
    ) -> &'ll Value {
        let ty = self.ty_array(elem_ty, len as u32);
        let arr = self
            .define_global(name, ty)
            .unwrap_or_else(|| unreachable!("symbol '{}' already defined", name));

        unsafe {
            let init = llvm_sys::core::LLVMConstNull(ty);
            llvm_sys::core::LLVMSetInitializer(arr, init);
            llvm_sys::core::LLVMSetLinkage(arr, llvm_sys::LLVMLinkage::ExternalLinkage);
        }

        if add_cnt {
            let name = format!("{}.cnt", name);
            let arr_len = self
                .define_global(&name, self.ty_size())
                .unwrap_or_else(|| unreachable!("symbol '{}' already defined", name));

            unsafe {
                let init = self.const_usize(len);
                llvm_sys::core::LLVMSetInitializer(arr_len, init);
                llvm_sys::core::LLVMSetGlobalConstant(arr_len, 1);
                llvm_sys::core::LLVMSetLinkage(arr_len, llvm_sys::LLVMLinkage::ExternalLinkage);
            }
        }

        arr
    }
}
