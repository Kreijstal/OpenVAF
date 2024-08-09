#include "llvm/IR/Instructions.h"
#include "llvm/Support/CrashRecoveryContext.h"
#include "llvm/IR/Attributes.h"
#include "llvm/IR/Function.h"
#include "llvm/Passes/PassBuilder.h"
#include "llvm/IR/PassManager.h"
#include "llvm/Transforms/Vectorize/SLPVectorizer.h"
#include "llvm/Transforms/IPO/PassManagerBuilder.h"

#include <iostream>
#include <mutex>
#include <stdlib.h>

using namespace llvm;

extern "C" {

void LLVMSetPartialFastMath(LLVMValueRef V) {
  if (auto I = dyn_cast<Instruction>(unwrap<Value>(V))) {
    I->setFast(true);
    I->setHasAllowReassoc(true);
    I->setHasAllowReciprocal(true);
    I->setHasAllowContract(true);
  }
}

void LLVMSetFastMath(LLVMValueRef V) {
  if (auto I = dyn_cast<Instruction>(unwrap<Value>(V))) {
    I->setFast(true);
  }
}

void LLVMPurgeAttrs(LLVMValueRef V) {
  if (auto func = dyn_cast<Function>(unwrap<Value>(V))) {
    func->setAttributes(AttributeList());
  }
}

// Keep this function for backwards compatibility
void LLVMPassManagerBuilderSLPVectorize(LLVMPassManagerBuilderRef PMB) {
  PassManagerBuilder *Builder = unwrap(PMB);
  Builder->SLPVectorize = true;
}

// New function to add SLPVectorizer pass using the new pass manager
void addSLPVectorizerPass(ModulePassManager &MPM) {
  FunctionPassManager FPM;
  FPM.addPass(SLPVectorizerPass());
  MPM.addPass(createModuleToFunctionPassAdaptor(std::move(FPM)));
}

// New function to set up the optimization pipeline using the new pass manager
void setupOptimizationPipeline(Module &M) {
  LoopAnalysisManager LAM;
  FunctionAnalysisManager FAM;
  CGSCCAnalysisManager CGAM;
  ModuleAnalysisManager MAM;

  PassBuilder PB;

  PB.registerModuleAnalyses(MAM);
  PB.registerCGSCCAnalyses(CGAM);
  PB.registerFunctionAnalyses(FAM);
  PB.registerLoopAnalyses(LAM);
  PB.crossRegisterProxies(LAM, FAM, CGAM, MAM);

  ModulePassManager MPM = PB.buildPerModuleDefaultPipeline(OptimizationLevel::O2);

  addSLPVectorizerPass(MPM);

  MPM.run(M, MAM);
}

} // extern "C"
