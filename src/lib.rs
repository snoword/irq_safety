//! Irq-safe locking via Mutex and RwLock. 
//! Identical behavior to the regular `spin` crate's 
//! Mutex and RwLock, with the added behavior of holding interrupts
//! for the duration of the Mutex guard. 

#![cfg_attr(feature = "const_fn", feature(const_fn))]

#![feature(asm)]
#![feature(manually_drop)]
#![no_std]

extern crate spin;
extern crate owning_ref;
extern crate stable_deref_trait;

pub use mutex_irqsafe::*;
pub use rwlock_irqsafe::*;
pub use held_interrupts::*;

mod mutex_irqsafe;
mod rwlock_irqsafe;
mod held_interrupts;
