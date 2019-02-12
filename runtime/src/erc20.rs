use rstd::prelude::*;
use support::{dispatch::Result, StorageMap, StorageValue, decl_storage, decl_module, decl_event, ensure};
use runtime_primitives::traits::{CheckedSub, CheckedAdd};
use {balances, system::ensure_signed};

// the module trait
// contains type definitions
pub trait Trait: balances::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// public interface for this runtime module
decl_module! {
  pub struct Module<T: Trait> for enum Call where origin: T::Origin {
      // initialize the default event for this module
      fn deposit_event<T>() = default;

      // initialize the token
      // transfers the total_supply amout to the caller
      // the token becomes usable
      // not part of ERC20 standard interface
      // replicates the ERC20 smart contract constructor functionality
      fn init(_origin) -> Result {
          let sender = ensure_signed(_origin)?;
          ensure!(Self::is_init() == false, "Already Initialized");

          <BalanceOf<T>>::insert(sender.clone(), Self::total_supply());
          <Init<T>>::put(true);

          Ok(())
      }

      // transfer tokens from one account to another
      fn transfer(_origin, to: T::AccountId, value: T::Balance) -> Result {
          let sender = ensure_signed(_origin)?;
          Self::_transfer(sender, to, value)
      }

      // approve token transfer from one account to another
      // if this is done, then transfer_from can be called with corresponding values
      fn approve(_origin, spender: T::AccountId, value: T::Balance) -> Result {
          let sender = ensure_signed(_origin)?;
          ensure!(<BalanceOf<T>>::exists(&sender), "Account does not own this token");
          Self::deposit_event(RawEvent::Approval(sender.clone(), spender.clone(), value));

          <Allowance<T>>::mutate((sender, spender), |allowance| {
              // using checked_add (safe math) to avoid overflow
              if let Some(updated_allowance) = allowance.checked_add(&value) {
                  *allowance = updated_allowance;
                }
          });

          Ok(())
      }

      // if approved, transfer from an account to another account without needed owner's signature
      // marked public so that it can be called from other modules
      pub fn transfer_from(_origin, from: T::AccountId, to: T::AccountId, value: T::Balance) -> Result {
        ensure!(<Allowance<T>>::exists((from.clone(), to.clone())), "Allowance does not exist.");
        ensure!(Self::allowance((from.clone(), to.clone())) >= value, "Not enough allowance.");

        <Allowance<T>>::mutate((from.clone(), to.clone()), |allowance| {
              // using checked_sub (safe math) to avoid overflow
              if let Some(updated_allowance) = allowance.checked_sub(&value) {
                  *allowance = updated_allowance;
                }
          });

        Self::deposit_event(RawEvent::Approval(from.clone(), to.clone(), value));

        Self::_transfer(from, to, value)
      }
  }
}

// storage for this runtime module
decl_storage! {
  trait Store for Module<T: Trait> as Erc20 {
    // bool flag to allow init to be called only once
    Init get(is_init): bool;

    // total supply of the token
    // set in the genesis config
    // see ../../src/chain_spec.rs - line 105
    TotalSupply get(total_supply) config(): T::Balance;
    
    // not really needed - name and ticker, but why not?
    Name get(name) config(): Vec<u8>;
    Ticker get (ticker) config(): Vec<u8>;

    // standard balances and allowances mappings for ERC20 implementation
    BalanceOf get(balance_of): map T::AccountId => T::Balance;
    Allowance get(allowance): map (T::AccountId, T::AccountId) => T::Balance;
  }
}

// events
decl_event!(
    pub enum Event<T> where AccountId = <T as system::Trait>::AccountId, Balance = <T as balances::Trait>::Balance {
        // event for transfer of tokens
        // from, to, value
        Transfer(AccountId, AccountId, Balance),
        // event when an approval is made
        // owner, spender, value
        Approval(AccountId, AccountId, Balance),
    }
);

// module implementation block
// utility and private functions
// if marked public, accessible by other modules
impl<T: Trait> Module<T> {
    fn _transfer(
        from: T::AccountId,
        to: T::AccountId,
        value: T::Balance,
    ) -> Result {
        ensure!(<BalanceOf<T>>::exists(from.clone()), "Account does not own this token");

        let sender_balance = Self::balance_of(from.clone());
        ensure!(sender_balance > value, "Not enough balance.");

        Self::deposit_event(RawEvent::Transfer(from.clone(), to.clone(), value));
        
        // reduce sender's balance
        <BalanceOf<T>>::mutate(from, |from_balance| {
              // using checked_sub (safe math) to avoid overflow
              if let Some(updated_from_balance) = from_balance.checked_sub(&value) {
                  *from_balance = updated_from_balance;
                }
          });

        // increase receiver's balance
        <BalanceOf<T>>::mutate(to, |to_balance| {
              // using checked_add (safe math) to avoid overflow
              if let Some(updated_to_balance) = to_balance.checked_add(&value) {
                  *to_balance = updated_to_balance;
                }
          });

        Ok(())
    }
}
