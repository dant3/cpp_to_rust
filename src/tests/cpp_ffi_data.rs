use cpp_ffi_data::*;
use cpp_type::*;
use caption_strategy::*;

#[test]
fn argument_meaning() {
  let a1 = CppFfiArgumentMeaning::This;
  assert!(!a1.is_argument());

  let a2 = CppFfiArgumentMeaning::Argument(2);
  assert!(a2.is_argument());

  let a3 = CppFfiArgumentMeaning::ReturnValue;
  assert!(!a3.is_argument());
}

#[test]
fn argument_int() {
  let arg = CppFfiFunctionArgument {
    name: "arg1".to_string(),
    argument_type: CppFfiType {
      original_type: CppType {
        indirection: CppTypeIndirection::None,
        is_const: false,
        base: CppTypeBase::BuiltInNumeric(CppBuiltInNumericType::Int),
      },
      ffi_type: CppType {
        indirection: CppTypeIndirection::None,
        is_const: false,
        base: CppTypeBase::BuiltInNumeric(CppBuiltInNumericType::Int),
      },
      conversion: IndirectionChange::NoChange,
    },
    meaning: CppFfiArgumentMeaning::Argument(0),
  };
  assert_eq!(arg.caption(ArgumentCaptionStrategy::NameOnly), "arg1");
  assert_eq!(arg.caption(ArgumentCaptionStrategy::TypeOnly(TypeCaptionStrategy::Short)),
             "int");
  assert_eq!(arg.caption(ArgumentCaptionStrategy::TypeOnly(TypeCaptionStrategy::Full)),
             "int");
  assert_eq!(arg.caption(ArgumentCaptionStrategy::TypeAndName(TypeCaptionStrategy::Short)),
             "int_arg1");
  assert_eq!(arg.caption(ArgumentCaptionStrategy::TypeAndName(TypeCaptionStrategy::Full)),
             "int_arg1");
  assert_eq!(arg.to_cpp_code().unwrap(), "int arg1");
}

#[test]
fn argument_int_ptr() {
  let arg = CppFfiFunctionArgument {
    name: "arg1".to_string(),
    argument_type: CppFfiType {
      original_type: CppType {
        indirection: CppTypeIndirection::Ptr,
        is_const: false,
        base: CppTypeBase::BuiltInNumeric(CppBuiltInNumericType::Int),
      },
      ffi_type: CppType {
        indirection: CppTypeIndirection::Ptr,
        is_const: false,
        base: CppTypeBase::BuiltInNumeric(CppBuiltInNumericType::Int),
      },
      conversion: IndirectionChange::NoChange,
    },
    meaning: CppFfiArgumentMeaning::Argument(0),
  };
  assert_eq!(arg.caption(ArgumentCaptionStrategy::NameOnly), "arg1");
  assert_eq!(arg.caption(ArgumentCaptionStrategy::TypeOnly(TypeCaptionStrategy::Short)),
             "int");
  assert_eq!(arg.caption(ArgumentCaptionStrategy::TypeOnly(TypeCaptionStrategy::Full)),
             "int_ptr");
  assert_eq!(arg.caption(ArgumentCaptionStrategy::TypeAndName(TypeCaptionStrategy::Short)),
             "int_arg1");
  assert_eq!(arg.caption(ArgumentCaptionStrategy::TypeAndName(TypeCaptionStrategy::Full)),
             "int_ptr_arg1");
  assert_eq!(arg.to_cpp_code().unwrap(), "int* arg1");
}

#[test]
fn argument_func() {
  let type1 = CppType {
    is_const: false,
    indirection: CppTypeIndirection::None,
    base: CppTypeBase::FunctionPointer {
      allows_variable_arguments: false,
      return_type: Box::new(CppType {
        indirection: CppTypeIndirection::None,
        is_const: false,
        base: CppTypeBase::BuiltInNumeric(CppBuiltInNumericType::Int),
      }),
      arguments: vec![CppType {
                        indirection: CppTypeIndirection::None,
                        is_const: false,
                        base: CppTypeBase::BuiltInNumeric(CppBuiltInNumericType::Int),
                      },
                      CppType {
                        indirection: CppTypeIndirection::Ptr,
                        is_const: false,
                        base: CppTypeBase::BuiltInNumeric(CppBuiltInNumericType::Bool),
                      }],
    },
  };

  let arg = CppFfiFunctionArgument {
    name: "arg1".to_string(),
    argument_type: CppFfiType {
      original_type: type1.clone(),
      ffi_type: type1.clone(),
      conversion: IndirectionChange::NoChange,
    },
    meaning: CppFfiArgumentMeaning::Argument(0),
  };
  assert_eq!(arg.caption(ArgumentCaptionStrategy::NameOnly), "arg1");
  assert_eq!(arg.caption(ArgumentCaptionStrategy::TypeOnly(TypeCaptionStrategy::Short)),
             "func");
  assert_eq!(arg.caption(ArgumentCaptionStrategy::TypeOnly(TypeCaptionStrategy::Full)),
             "func");
  assert_eq!(arg.caption(ArgumentCaptionStrategy::TypeAndName(TypeCaptionStrategy::Short)),
             "func_arg1");
  assert_eq!(arg.caption(ArgumentCaptionStrategy::TypeAndName(TypeCaptionStrategy::Full)),
             "func_arg1");
  assert_eq!(arg.to_cpp_code().unwrap(), "int (*arg1)(int, bool*)");
}

#[test]
fn signature_two_numbers() {
  let sig = CppFfiFunctionSignature {
    arguments: vec![CppFfiFunctionArgument {
                      name: "arg1".to_string(),
                      argument_type: CppFfiType {
                        original_type: CppType {
                          indirection: CppTypeIndirection::None,
                          is_const: false,
                          base: CppTypeBase::BuiltInNumeric(CppBuiltInNumericType::Int),
                        },
                        ffi_type: CppType {
                          indirection: CppTypeIndirection::None,
                          is_const: false,
                          base: CppTypeBase::BuiltInNumeric(CppBuiltInNumericType::Int),
                        },
                        conversion: IndirectionChange::NoChange,
                      },
                      meaning: CppFfiArgumentMeaning::Argument(0),
                    },
                    CppFfiFunctionArgument {
                      name: "arg2".to_string(),
                      argument_type: CppFfiType {
                        original_type: CppType {
                          indirection: CppTypeIndirection::None,
                          is_const: false,
                          base: CppTypeBase::BuiltInNumeric(CppBuiltInNumericType::Double),
                        },
                        ffi_type: CppType {
                          indirection: CppTypeIndirection::None,
                          is_const: false,
                          base: CppTypeBase::BuiltInNumeric(CppBuiltInNumericType::Double),
                        },
                        conversion: IndirectionChange::NoChange,
                      },
                      meaning: CppFfiArgumentMeaning::Argument(0),
                    }],
    return_type: CppFfiType::void(),
  };
  assert_eq!(sig.caption(ArgumentCaptionStrategy::NameOnly), "arg1_arg2");
  assert_eq!(sig.caption(ArgumentCaptionStrategy::TypeOnly(TypeCaptionStrategy::Short)),
             "int_double");
  assert_eq!(sig.caption(ArgumentCaptionStrategy::TypeOnly(TypeCaptionStrategy::Full)),
             "int_double");
  assert_eq!(sig.caption(ArgumentCaptionStrategy::TypeAndName(TypeCaptionStrategy::Short)),
             "int_arg1_double_arg2");
  assert_eq!(sig.caption(ArgumentCaptionStrategy::TypeAndName(TypeCaptionStrategy::Full)),
             "int_arg1_double_arg2");
  assert_eq!(sig.arguments_to_cpp_code().unwrap(), "int arg1, double arg2");
}

#[test]
fn cpp_ffi_type_void() {
  let t = CppFfiType::void();
  assert!(t.cpp_type.is_void());
  assert!(t.ffi_type.is_void());
  assert_eq!(t.conversion, IndirectionChange::NoChange);
}

#[test]
fn c_base_name1() {
  let method =





}