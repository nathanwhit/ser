use crate::record::StackChange;
use std::fmt::{Debug, Formatter};

use super::smt::*;
use crate::traits::MachineComponent;
use smallvec::SmallVec;
use z3_ext::{
    ast::{Ast, BV},
    Config,
};
#[derive(Default, Debug, Clone)]
pub struct Stack<const SZ: u32> {
    stack: SmallVec<[BitVec<SZ>; 1024]>,
    size: usize,
}

impl<const SZ: u32> Stack<SZ> {
    pub fn push(&mut self, val: BitVec<SZ>) {
        self.size += 1;
        self.stack.push(val);
    }

    pub fn pop(&mut self) -> BitVec<SZ> {
        self.size -= 1;
        self.stack.pop().unwrap()
    }

    pub fn peek(&self) -> Option<&BitVec<SZ>> {
        self.stack.last()
    }

    pub fn size(&self) -> usize {
        assert!(self.stack.len() == self.size);
        self.stack.len()
    }

    pub fn peek_nth(&self, n: usize) -> Option<&BitVec<SZ>> {
        self.stack.get(self.size - n - 1)
    }
}

impl<const SZ: u32> MachineComponent for Stack<SZ> {
    type Record = StackChange<SZ>;

    fn apply_change(&mut self, rec: Self::Record) {
        let StackChange {
            pop_qty,
            push_qty,
            ops,
        } = rec;

        let mut new_stack = self.clone();

        ops.iter().for_each(|op| match op {
            crate::record::StackOp::Push(v) => new_stack.push(v.clone()),
            crate::record::StackOp::Pop => {
                new_stack.pop();
            }
        });

        self.stack = new_stack.stack;
        self.size = new_stack.size;
    }
}
