# add_subdirectory(test)

lumen_cc_library(
  NAME
    Transforms
  HDRS
    "Passes.h"
  SRCS
    "Passes.cpp"
  DEPS
    lumen::compiler::Dialect::EIR::Conversion::EIRToLLVM
    lumen::compiler::Dialect::EIR::IR
    lumen::compiler::Target
    LLVMSupport
    MLIRIR
    MLIRPass
    MLIRSupport
    MLIRTransformUtils
    MLIRTransforms
    MLIRStandardToLLVM
  ALWAYSLINK
  PUBLIC
)
