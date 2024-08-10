use std::ffi::CString;

use libc::c_uint;
use llvm_sys::core::LLVMInt8TypeInContext;
use llvm_sys::LLVMType as Type;
use llvm_sys::LLVMValue as Value;
use llvm_sys::prelude::LLVMBool;

const False: LLVMBool = 0;
const True: LLVMBool = 1;
use mir::Const;

use crate::CodegenCx;

pub struct Types<'ll> {
    pub double: &'ll Type,
    pub char: &'ll Type,
    pub int: &'ll Type,
    pub size: &'ll Type,
    pub ptr: &'ll Type,
    pub fat_ptr: &'ll Type,
    pub bool: &'ll Type,
    pub void: &'ll Type,
    pub null_ptr_val: &'ll Value,
}

impl<'ll> Types<'ll> {
    pub fn new(llcx: &'ll llvm_sys::LLVMContext, pointer_width: u32) -> Types<'ll> {
        unsafe {
            let char = LLVMInt8TypeInContext(llcx);
            // we are using opaque pointers, with old llvm version that plain
            // means always using char pointers, with newer llvm version the
            // type is ignored anyway
            //let ptr = llvm_sys::LLVMPointerType(char, llvm_sys::AddressSpace::DATA);
            let ptr = llvm_sys::LLVMPointerType(char, 0); // 0 represents the default (DATA) address space
            let size = llvm_sys::LLVMIntTypeInContext(llcx, pointer_width);
            Types {
                double: llvm_sys::LLVMDoubleTypeInContext(llcx),
                char,
                int: llvm_sys::LLVMInt32TypeInContext(llcx),
                size,
                ptr,
                fat_ptr: ty_struct(llcx, "fat_ptr", &[ptr, llvm_sys::LLVMInt64TypeInContext(llcx)]),
                bool: llvm_sys::LLVMInt1TypeInContext(llcx),
                void: llvm_sys::LLVMVoidTypeInContext(llcx),
                null_ptr_val: llvm_sys::LLVMConstPointerNull(ptr),
            }
        }
    }
}

fn ty_struct<'ll>(llcx: &'ll llvm_sys::LLVMContext, name: &str, elements: &[&'ll Type]) -> &'ll Type {
    let name = CString::new(name).unwrap();
    unsafe {
        let ty = llvm_sys::LLVMStructCreateNamed(llcx, name.as_ptr());
        llvm_sys::LLVMStructSetBody(ty, elements.as_ptr(), elements.len() as u32, False);
        ty
    }
}

impl<'a, 'll> CodegenCx<'a, 'll> {
    #[inline(always)]
    pub fn ty_double(&self) -> &'ll Type {
        self.tys.double
    }
    #[inline(always)]
    pub fn ty_int(&self) -> &'ll Type {
        self.tys.int
    }
    #[inline(always)]
    pub fn ty_char(&self) -> &'ll Type {
        self.tys.char
    }
    #[inline(always)]
    pub fn ty_size(&self) -> &'ll Type {
        self.tys.size
    }
    #[inline(always)]
    pub fn ty_bool(&self) -> &'ll Type {
        self.tys.bool
    }
    #[inline(always)]
    pub fn ty_c_bool(&self) -> &'ll Type {
        self.tys.char
    }
    #[inline(always)]
    pub fn ty_ptr(&self) -> &'ll Type {
        self.tys.ptr
    }
    #[inline(always)]
    pub fn ty_void(&self) -> &'ll Type {
        self.tys.void
    }
    #[inline(always)]
    pub fn ty_fat_ptr(&self) -> &'ll Type {
        self.tys.fat_ptr
    }
    pub fn ty_aint(&self, bits: u32) -> &'ll Type {
        unsafe { llvm_sys::LLVMIntTypeInContext(self.llcx, bits) }
    }

    pub fn ty_struct(&self, name: &str, elements: &[&'ll Type]) -> &'ll Type {
        ty_struct(self.llcx, name, elements)
    }

    pub fn ty_func(&self, args: &[&'ll Type], ret: &'ll Type) -> &'ll Type {
        unsafe { llvm_sys::LLVMFunctionType(ret, args.as_ptr(), args.len() as c_uint, False) }
    }

    pub fn ty_variadic_func(&self, args: &[&'ll Type], ret: &'ll Type) -> &'ll Type {
        unsafe { llvm_sys::LLVMFunctionType(ret, args.as_ptr(), args.len() as c_uint, True) }
    }

    pub fn ty_array(&self, ty: &'ll Type, len: u32) -> &'ll Type {
        unsafe { llvm_sys::LLVMArrayType(ty, len) }
    }

    pub fn const_val(&self, val: &Const) -> &'ll Value {
        match *val {
            Const::Float(val) => self.const_real(val.into()),
            Const::Int(val) => self.const_int(val),
            Const::Bool(val) => self.const_bool(val),
            // Const::Complex(ref val) => self.const_cmplx(val),
            Const::Str(val) => self.const_str(val),
        }
    }

    /// # Safety
    /// indices must be valid and inbounds for the provided ptr
    /// The pointer must be a constant address
    pub unsafe fn const_gep(
        &self,
        elem_ty: &'ll Type,
        ptr: &'ll Value,
        indices: &[&'ll Value],
    ) -> &'ll Value {
        llvm_sys::LLVMConstInBoundsGEP2(elem_ty, ptr, indices.as_ptr(), indices.len() as u32)
    }

    pub fn const_int(&self, val: i32) -> &'ll Value {
        unsafe { llvm_sys::LLVMConstInt(self.ty_int(), val as u64, True) }
    }

    pub fn const_unsigned_int(&self, val: u32) -> &'ll Value {
        unsafe { llvm_sys::LLVMConstInt(self.ty_int(), val as u64, True) }
    }

    pub fn const_isize(&self, val: isize) -> &'ll Value {
        unsafe { llvm_sys::LLVMConstInt(self.ty_size(), val as u64, True) }
    }

    pub fn const_usize(&self, val: usize) -> &'ll Value {
        unsafe { llvm_sys::LLVMConstInt(self.ty_size(), val as u64, False) }
    }

    pub fn const_bool(&self, val: bool) -> &'ll Value {
        unsafe { llvm_sys::LLVMConstInt(self.ty_bool(), val as u64, False) }
    }

    pub fn const_c_bool(&self, val: bool) -> &'ll Value {
        unsafe { llvm_sys::LLVMConstInt(self.ty_c_bool(), val as u64, False) }
    }

    pub fn const_u8(&self, val: u8) -> &'ll Value {
        unsafe { llvm_sys::LLVMConstInt(self.ty_c_bool(), val as u64, False) }
    }

    pub fn const_real(&self, val: f64) -> &'ll Value {
        unsafe { llvm_sys::LLVMConstReal(self.ty_double(), val) }
    }

    pub fn const_arr(&self, elem_ty: &'ll Type, vals: &[&'ll Value]) -> &'ll Value {
        unsafe { llvm_sys::LLVMConstArray(elem_ty, vals.as_ptr(), vals.len() as u32) }
    }

    pub fn const_struct(&self, ty: &'ll Type, vals: &[&'ll Value]) -> &'ll Value {
        unsafe { llvm_sys::LLVMConstNamedStruct(ty, vals.as_ptr(), vals.len() as u32) }
    }

    pub fn const_null(&self, t: &'ll Type) -> &'ll Value {
        unsafe { llvm_sys::LLVMConstNull(t) }
    }

    pub fn const_null_ptr(&self) -> &'ll Value {
        self.tys.null_ptr_val
    }

    pub fn const_undef(&self, t: &'ll Type) -> &'ll Value {
        unsafe { llvm_sys::LLVMGetUndef(t) }
    }

    pub fn val_ty(&self, v: &'ll Value) -> &'ll Type {
        unsafe { llvm_sys::core::LLVMTypeOf(v) }
    }
}
