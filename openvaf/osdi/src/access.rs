use core::ptr::NonNull;
use llvm_sys::core::{
    LLVMAddCase, LLVMAppendBasicBlockInContext, LLVMBuildAnd, LLVMBuildBr, LLVMBuildCondBr,
    LLVMBuildICmp, LLVMBuildRet, LLVMBuildSwitch, LLVMCreateBuilderInContext, LLVMDisposeBuilder,
    LLVMGetParam, LLVMPositionBuilderAtEnd,
};
use llvm_sys::LLVMIntPredicate::LLVMIntNE;
use mir_llvm::UNNAMED;

use crate::compilation_unit::OsdiCompilationUnit;
use crate::metadata::osdi_0_3::{ACCESS_FLAG_INSTANCE, ACCESS_FLAG_SET};

impl<'ll> OsdiCompilationUnit<'_, '_, 'll> {
    pub fn access_function_prototype(&self) -> &'ll llvm_sys::LLVMValue {
        let cx = &self.cx;
        let void_ptr = cx.ty_ptr();
        let uint32_t = cx.ty_int();
        let fun_ty = cx.ty_func(&[void_ptr, void_ptr, uint32_t, uint32_t], void_ptr);
        let name = &format!("access_{}", &self.module.sym);
        cx.declare_ext_fn(name, fun_ty)
    }

    pub fn access_function(&self) -> &'ll llvm_sys::LLVMValue {
        let llfunc = self.access_function_prototype();
        let OsdiCompilationUnit { inst_data, model_data, cx, .. } = &self;

        unsafe {
            let entry = LLVMAppendBasicBlockInContext(
                NonNull::from(cx.llcx).as_ptr(),
                NonNull::from(llfunc).as_ptr(),
                UNNAMED,
            );
            let err_exit = LLVMAppendBasicBlockInContext(
                NonNull::from(cx.llcx).as_ptr(),
                NonNull::from(llfunc).as_ptr(),
                UNNAMED,
            );
            let model_bb = LLVMAppendBasicBlockInContext(
                NonNull::from(cx.llcx).as_ptr(),
                NonNull::from(llfunc).as_ptr(),
                UNNAMED,
            );
            let inst_bb = LLVMAppendBasicBlockInContext(
                NonNull::from(cx.llcx).as_ptr(),
                NonNull::from(llfunc).as_ptr(),
                UNNAMED,
            );
            let opvar_bb = LLVMAppendBasicBlockInContext(
                NonNull::from(cx.llcx).as_ptr(),
                NonNull::from(llfunc).as_ptr(),
                UNNAMED,
            );
            let llbuilder = LLVMCreateBuilderInContext(NonNull::from(cx.llcx).as_ptr());

            LLVMPositionBuilderAtEnd(llbuilder, entry);

            // get params
            let inst = LLVMGetParam(NonNull::from(llfunc).as_ptr(), 0);
            let model = LLVMGetParam(NonNull::from(llfunc).as_ptr(), 1);
            let param_id = LLVMGetParam(NonNull::from(llfunc).as_ptr(), 2);
            let flags = LLVMGetParam(NonNull::from(llfunc).as_ptr(), 3);

            let access_flag_instance =
                NonNull::from(cx.const_unsigned_int(ACCESS_FLAG_INSTANCE)).as_ptr();
            let access_flag_set = NonNull::from(cx.const_unsigned_int(ACCESS_FLAG_SET)).as_ptr();
            let zero = NonNull::from(cx.const_unsigned_int(0)).as_ptr();

            // check various flags
            let flags_and_instance = LLVMBuildAnd(llbuilder, flags, access_flag_instance, UNNAMED);
            let instance_flag_set =
                LLVMBuildICmp(llbuilder, LLVMIntNE, flags_and_instance, zero, UNNAMED);

            let flags_and_set = LLVMBuildAnd(llbuilder, flags, access_flag_set, UNNAMED);
            let write_flag_set = LLVMBuildICmp(llbuilder, LLVMIntNE, flags_and_set, zero, UNNAMED);

            LLVMBuildCondBr(llbuilder, instance_flag_set, inst_bb, model_bb);

            // inst params
            LLVMPositionBuilderAtEnd(llbuilder, inst_bb);
            let switch_inst =
                LLVMBuildSwitch(llbuilder, param_id, opvar_bb, inst_data.params.len() as u32);

            for param_idx in 0..inst_data.params.len() {
                let bb = LLVMAppendBasicBlockInContext(
                    NonNull::from(cx.llcx).as_ptr(),
                    NonNull::from(llfunc).as_ptr(),
                    UNNAMED,
                );
                LLVMPositionBuilderAtEnd(llbuilder, bb);
                let case = NonNull::from(cx.const_unsigned_int(param_idx as u32)).as_ptr();
                LLVMAddCase(switch_inst, case, bb);

                let (ptr, _) = inst_data.nth_param_ptr(param_idx as u32, &*inst, &*llbuilder);

                // set the write flag if given
                let write = LLVMAppendBasicBlockInContext(
                    NonNull::from(cx.llcx).as_ptr(),
                    NonNull::from(llfunc).as_ptr(),
                    UNNAMED,
                );
                let ret = LLVMAppendBasicBlockInContext(
                    NonNull::from(cx.llcx).as_ptr(),
                    NonNull::from(llfunc).as_ptr(),
                    UNNAMED,
                );
                LLVMBuildCondBr(llbuilder, write_flag_set, write, ret);
                LLVMPositionBuilderAtEnd(llbuilder, write);
                inst_data.set_nth_param_given(cx, param_idx as u32, &*inst, &*llbuilder);
                LLVMBuildBr(llbuilder, ret);

                // return the pointer
                LLVMPositionBuilderAtEnd(llbuilder, ret);
                LLVMBuildRet(llbuilder, NonNull::from(ptr).as_ptr());
            }

            LLVMPositionBuilderAtEnd(llbuilder, model_bb);
            let switch_model =
                LLVMBuildSwitch(llbuilder, param_id, opvar_bb, model_data.params.len() as u32);

            // inst param model default values
            for param_idx in 0..inst_data.params.len() {
                let bb = LLVMAppendBasicBlockInContext(
                    NonNull::from(cx.llcx).as_ptr(),
                    NonNull::from(llfunc).as_ptr(),
                    UNNAMED,
                );
                LLVMPositionBuilderAtEnd(llbuilder, bb);
                let case = cx.const_unsigned_int(param_idx as u32);
                LLVMAddCase(switch_model, NonNull::from(case).as_ptr(), bb);

                let (ptr, _) = model_data.nth_inst_param_ptr(
                    inst_data,
                    param_idx as u32,
                    &*model,
                    &*llbuilder,
                );

                // set the write flag if given
                let write = LLVMAppendBasicBlockInContext(
                    NonNull::from(cx.llcx).as_ptr(),
                    NonNull::from(llfunc).as_ptr(),
                    UNNAMED,
                );
                let ret = LLVMAppendBasicBlockInContext(
                    NonNull::from(cx.llcx).as_ptr(),
                    NonNull::from(llfunc).as_ptr(),
                    UNNAMED,
                );
                LLVMBuildCondBr(llbuilder, write_flag_set, write, ret);
                LLVMPositionBuilderAtEnd(llbuilder, write);
                model_data.set_nth_inst_param_given(cx, param_idx as u32, &*model, &*llbuilder);
                LLVMBuildBr(llbuilder, ret);

                // return the pointer
                LLVMPositionBuilderAtEnd(llbuilder, ret);
                LLVMBuildRet(llbuilder, NonNull::from(ptr).as_ptr());
            }

            // model params
            for param_idx in 0..model_data.params.len() {
                let bb = LLVMAppendBasicBlockInContext(
                    NonNull::from(cx.llcx).as_ptr(),
                    NonNull::from(llfunc).as_ptr(),
                    UNNAMED,
                );
                LLVMPositionBuilderAtEnd(llbuilder, bb);
                let case = cx.const_unsigned_int((inst_data.params.len() + param_idx) as u32);
                LLVMAddCase(switch_model, NonNull::from(case).as_ptr(), bb);

                let (ptr, _) = model_data.nth_param_ptr(param_idx as u32, &*model, &*llbuilder);

                // set the write flag if given
                let write = LLVMAppendBasicBlockInContext(
                    NonNull::from(cx.llcx).as_ptr(),
                    NonNull::from(llfunc).as_ptr(),
                    UNNAMED,
                );
                let ret = LLVMAppendBasicBlockInContext(
                    NonNull::from(cx.llcx).as_ptr(),
                    NonNull::from(llfunc).as_ptr(),
                    UNNAMED,
                );
                LLVMBuildCondBr(llbuilder, write_flag_set, write, ret);
                LLVMPositionBuilderAtEnd(llbuilder, write);
                model_data.set_nth_param_given(cx, param_idx as u32, &*model, &*llbuilder);
                LLVMBuildBr(llbuilder, ret);

                // return the pointer
                LLVMPositionBuilderAtEnd(llbuilder, ret);
                LLVMBuildRet(llbuilder, NonNull::from(ptr).as_ptr());
            }

            let null_ptr = cx.const_null_ptr();

            // opvars
            LLVMPositionBuilderAtEnd(llbuilder, opvar_bb);
            let switch_opvar =
                LLVMBuildSwitch(llbuilder, param_id, err_exit, inst_data.opvars.len() as u32);
            for opvar_idx in 0..inst_data.opvars.len() {
                let OsdiCompilationUnit { inst_data, model_data, cx, .. } = &self;
                let bb = LLVMAppendBasicBlockInContext(
                    NonNull::from(cx.llcx).as_ptr(),
                    NonNull::from(llfunc).as_ptr(),
                    UNNAMED,
                );
                LLVMPositionBuilderAtEnd(llbuilder, bb);
                let case = cx.const_unsigned_int(
                    (model_data.params.len() + inst_data.params.len() + opvar_idx) as u32,
                );
                LLVMAddCase(switch_opvar, NonNull::from(case).as_ptr(), bb);
                let (ptr, _) = self.nth_opvar_ptr(opvar_idx as u32, &*inst, &*model, &*llbuilder);
                LLVMBuildRet(llbuilder, NonNull::from(ptr).as_ptr());
            }

            //return NULL on unknown id
            LLVMPositionBuilderAtEnd(llbuilder, err_exit);
            LLVMBuildRet(llbuilder, NonNull::from(null_ptr).as_ptr());

            LLVMDisposeBuilder(llbuilder);
        }

        llfunc
    }
}
