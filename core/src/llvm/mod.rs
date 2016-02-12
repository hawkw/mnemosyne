//
//  0 1 0  Mnemosyne: a functional systems programming language.
//  0 0 1  (c) 2015 Hawk Weisman
//  1 1 1  hi@hawkweisman.me
//
//  Mnemosyne is released under the MIT License. Please refer to
//  the LICENSE file at the top-level directory of this distribution
//  or at https://github.com/hawkw/mnemosyne/.
//
use rustc::lib::llvm;
use rustc::lib::llvm::*;

use std::ffi::CString;
use std::mem::transmute;

use libc::{c_char, c_uint};

use ::errors::{ExpectICE, UnwrapICE};

// pub struct Context(llvm::ContextRef);
//
// pub struct Builder(llvm::BuilderRef);

/// because we are in the Raw Pointer Sadness Zone (read: unsafe),
/// it is necessary that we assert that everything exists.
macro_rules! not_null {
    ($target:expr) => ({
        let e = $target;
        if e.is_null() {
            ice!( "assertion failed: {} returned null!"
                , stringify!($target)
                );
        } else { e }
    })
}

trait LLVMWrapper {
    type Ref;
    fn to_ref(&self) -> Self::Ref;
    fn from_ref(r: Self::Ref) -> Self;
}

macro_rules! llvm_wrapper {
    ( $($name:ident wrapping $wrapped:path),*) => {$(
        pub struct $name ($wrapped);

        impl LLVMWrapper for $name {
            type Ref = $wrapped;

            fn to_ref(&self) -> Self::Ref {
                not_null!(self.0)
            }

            fn from_ref(r: $wrapped) -> Self {
                $name( not_null!(r) )
            }
        }
    )*};
    ( $($name:ident wrapping $wrapped:path, dropped by $drop:path),*) => {$(
        llvm_wrapper!{ $name wrapping $wrapped }

        impl Drop for $name {
            fn drop(&mut self) {
                unsafe { $drop(self.0) }
            }
        }
    )*}
}

llvm_wrapper! {
    Context wrapping llvm::ContextRef,
        dropped by llvm::LLVMContextDispose,
    Builder wrapping llvm::BuilderRef,
        dropped by llvm::LLVMDisposeBuilder,
    OperandBundle wrapping llvm::OperandBundleDefRef,
        dropped by llvm::LLVMRustFreeOperandBundleDef
}

llvm_wrapper! {
    BasicBlock wrapping llvm::BasicBlockRef,
    Value wrapping llvm::ValueRef
}

impl Context {
    /// Construct a new `Builder` within this `Context`.
    pub fn create_builder(&self) -> Builder {
        unsafe {
            Builder::from_ref(LLVMCreateBuilderInContext(self.to_ref()))
        }
    }
}

impl OperandBundle {
    /// Construct a new `OperandBundle`
    pub fn new(name: &str, vals: &mut [Value]) -> Self {
        let cname = CString::new(name).unwrap_ice();
        unsafe {
            let vals_ref = transmute::<&mut [Value], &mut [ValueRef]>(vals);
            let def = LLVMRustBuildOperandBundleDef( cname.as_ptr()
                                                   , vals_ref.as_ptr()
                                                   , vals_ref.len() as c_uint);
            OperandBundle(def)
        }
    }
}

impl Builder {
    //---- positioning --------------------------------------------------------
    /// Wrapper for `LLVMPositionBuilder`.
    pub fn position(&mut self, block: &mut BasicBlock, inst: &Value)
                   -> &mut Self {
        unsafe {
            LLVMPositionBuilder(self.to_ref(), block.to_ref(), inst.to_ref());
        }
        self
    }

    /// Wrapper for `LLVMPositionBuilderBefore`.
    pub fn position_before(&mut self, inst: &Value) -> &mut Self {
        unsafe { LLVMPositionBuilderBefore(self.to_ref(), inst.to_ref()) }
        self
    }

    /// Wrapper for `LLVMPositionBuilderAtEnd`.
    pub fn position_at_end(&mut self, block: &mut BasicBlock) -> &mut Self {
        unsafe { LLVMPositionBuilderAtEnd(self.to_ref(), block.to_ref()) }
        self
    }

    //---- insertion ----------------------------------------------------------
    pub fn get_insert_block(&mut self) -> BasicBlock {
        unsafe { BasicBlock::from_ref(LLVMGetInsertBlock(self.to_ref())) }
    }

    pub fn clear_insertion_position(&mut self) -> &mut Self {
        unsafe { LLVMClearInsertionPosition(self.to_ref()) }
        self
    }

    pub fn insert(&mut self, inst: &mut Value) -> &mut Self {
        unsafe { LLVMInsertIntoBuilder(self.to_ref(), inst.to_ref()) }
        self
    }

    pub fn insert_with_name(&mut self, inst: &mut Value, name: &str)
                            -> &mut Self {
        let cname = CString::new(name).expect_ice(
                    format!("Couldn't create CString for {}", name).as_ref());
        unsafe {
            LLVMInsertIntoBuilderWithName( self.to_ref()
                                         , inst.to_ref()
                                         , cname.as_ptr() as *const c_char)
        }
        self
    }
    //---- building ----------------------------------------------------------
    /// Create a `ret void` return instruction.
    pub fn build_ret_void(&mut self) -> Value {
        unsafe { Value::from_ref( LLVMBuildRetVoid(self.to_ref()) ) }
    }

    /// Create a `ret <value>` instruction.
    ///
    /// # Arguments
    ///
    ///  + `v`: the `Value` to return
    pub fn build_ret(&mut self, v: &Value) -> Value {
        unsafe {
            Value::from_ref( LLVMBuildRet(self.to_ref(), v.to_ref()) )
        }
    }

    /// Create an unconditional branch `br label X` instruction.
    ///
    /// # Arguments
    ///
    ///  + `dest` the `BasicBlock` to branch to.
    ///
    /// # Return Value
    ///  An unconditional branch `br` instruction.
    pub fn build_br(&mut self, dest: &BasicBlock) -> Value {
        unsafe {
            Value::from_ref( LLVMBuildBr(self.to_ref(), dest.to_ref()) )
        }
    }

    /// Create a conditional branch instruction.
    ///
    /// # Arguments
    ///
    ///  + `condition`: the condition to test
    ///  + `then_block`: the block to branch to if the condition is true
    ///  + `else_block`: the block to branch to if the condition is false
    ///
    ///  # Returns Value
    ///
    /// A conditional branch instruction of the form
    /// `br $condition, $then_block, $else_block`.
    pub fn build_cond_br( &mut self
                        , condition: Value
                        , then_block: &BasicBlock
                        , else_block: &BasicBlock )
                        -> Value {
        unsafe {
            let val = LLVMBuildCondBr( self.to_ref()
                                     , condition.to_ref()
                                     , then_block.to_ref()
                                     , else_block.to_ref() );
            Value::from_ref(val)
        }
    }

    /// Create a switch instruction.
    ///
    /// # Arguments
    ///
    ///  + `on`: the `Value` to switch on
    ///  + `else_block`: a `BasicBlock` representing the default destination
    ///  + `num_cases`: a hint towards the number of cases in the switch
    ///                 expression (for more efficient allocation)
    ///
    /// # Return Value
    ///
    /// A switch instruction with the specified value and default destination.
    pub fn build_switch_br( &mut self
                          , on: Value
                          , else_block: &BasicBlock
                          , num_cases: u32 )
                          -> Value {
        unsafe {
            Value::from_ref(LLVMBuildSwitch( self.to_ref()
                                            , on.to_ref()
                                            , else_block.to_ref()
                                            , num_cases))
        }
    }

    /// Create a function invocation instruction.
    ///
    /// # Arguments
    ///
    ///  + `function`: the function to invoke
    ///  + `args': the arguments to `function`
    ///  + `then_block`:
    ///  + `catch_block`:
    ///  + `name`: the name of the instruction.
    ///  + `bundle`:
    ///
    /// # Return Value
    ///
    /// A function invocation instruction.
    pub fn build_invoke( &mut self
                       , function: Value
                       , args: &mut [Value]
                       , then_block: &BasicBlock
                       , catch_block: &BasicBlock
                       , bundle: &OperandBundle
                       , name: &str )
                       -> Value {
        let cname = CString::new(name).unwrap_ice().as_ptr();
        unsafe {
            let args_ref = transmute::<&mut [Value], &mut [ValueRef]>(args)
                            .as_ptr();
            let val = LLVMRustBuildInvoke( self.to_ref()
                                         , function.to_ref()
                                         , args_ref, args.len() as c_uint
                                         , then_block.to_ref()
                                         , catch_block.to_ref()
                                         , bundle.to_ref()
                                         , cname as *const c_char);
            Value::from_ref(val)
        }
    }

    /// Create landing pad
    ///
    // TODO: do we even want to support this? We don't have checked exns
    pub fn build_landing_pad( &mut self
                            , ty: TypeRef
                            , pers_fn: Value
                            , num_clauses: usize
                            , name: &str
                            , f: Value)
                            -> Value {
        let cname = CString::new(name).unwrap_ice().as_ptr();
        unsafe {
            Value::from_ref(
                LLVMRustBuildLandingPad( self.to_ref()
                                       , ty
                                       , pers_fn.to_ref()
                                       , num_clauses as c_uint
                                       , cname as *const c_char
                                       , f.to_ref() )
            )
        }
    }


    pub fn build_cleanup_pad( &mut self
                            , parent_pad: Value
                            , args: &mut [Value]
                            , name: &str )
                            -> Value {
        let cname = CString::new(name).unwrap_ice().as_ptr();
        unsafe {
            let args_ref = transmute::<&mut [Value], &mut [ValueRef]>(args);
            Value::from_ref(
                LLVMRustBuildCleanupPad( self.to_ref()
                                       , parent_pad.to_ref()
                                       , args_ref.len() as c_uint
                                       , args_ref.as_ptr()
                                       , cname as *const c_char ))
        }
    }

    pub fn build_cleanup_ret( &mut self
                            , cleanup_pad: Value
                            , unwind_blk: BasicBlock )
                            -> Value {
        unsafe {
            Value::from_ref(LLVMRustBuildCleanupRet( self.to_ref()
                                                   , cleanup_pad.to_ref()
                                                   , unwind_blk.to_ref() ))
        }
    }


    pub fn build_resume(&mut self, ex: Value) -> Value {
        unsafe { Value::from_ref(LLVMBuildResume(self.to_ref(), ex.to_ref())) }
    }

    pub fn build_unreachable(&mut self) -> Value {
        unsafe { Value::from_ref(LLVMBuildUnreachable(self.to_ref())) }
    }


}
