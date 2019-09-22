use support::{decl_storage, decl_module, StorageValue, dispatch::Result, StorageMap};
use system::ensure_signed;
pub trait Trait: system::Trait {
   
}

decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage {
          // Declare storage and getter functions here
        FakeBalance: u32;
        MyValue: map T::AccountId => u64 ;
  }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Declare public functions here
        fn put_balance(origin, input_u32:u32) -> Result{
            let _sender = ensure_signed(origin)?;
            
            <FakeBalance<T>>::put(input_u32);

            Ok(())
        }
    }
}