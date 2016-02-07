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

use std::ops::DerefMut;

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

macro_rules! llvm_wrappers {
    ( $($name:ident wrapping $wrapped:path),*) =>
    {$(
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
    )*}
}

llvm_wrappers! {
    Context wrapping llvm::ContextRef,
    Builder wrapping llvm::BuilderRef,
    BasicBlock wrapping llvm::BasicBlockRef,
    Value wrapping llvm::ValueRef
}

impl Context {
    /// Construct a new Builder within this `Context`.
    pub fn create_builder(&self) -> Builder {
        unsafe {
            Builder::from_ref(LLVMCreateBuilderInContext(self.to_ref()))
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
        let name = CString::new(name).unwrap_ice();
        unsafe {
            LLVMInsertIntoBuilderWithName(&self.to_ref(), inst.to_ref(), name)
        }
        self
    }
}
