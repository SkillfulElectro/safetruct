/*!
  this crate just provides simple macro to implement safe struct which 
  drops a value when it goes out of the scope
  */

#![no_std]


#[macro_export]
/// call this macro this way : 
/// safetruct!(visibility of your struct , name_of_that_value : its type , name_of_the_allocator_func , name_of_the_deallocator_func); 
///
/// allocator func must get no para and return the type which is setted 
/// deallocator must get *mut *mut pointer to that type
///
/// if you deref the struct , it will use the inner value 
macro_rules! safetruct {

    ($visibility:ident , $name:ident, $valuename:ident: $type:ty, $user_defined_alloc_func:ident, $user_defined_dealloc_func:ident) => {

        $visibility struct $name {
            $visibility $valuename: $type,
        }


        impl $name {

            $visibility fn new() -> Self {
                let mut value : $type ; 
                unsafe {
                    value = $user_defined_alloc_func();
                }
                $name {
                    $valuename: value,
                }
            }
        }


        impl Drop for $name {
            fn drop(&mut self) {
                unsafe {
                    $user_defined_dealloc_func(&mut self.$valuename);
                }
            }
        }

        impl core::ops::Deref for $name {
            type Target = $type;

            fn deref(&self) -> &Self::Target {
                &self.$valuename
            }
        }

        impl core::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$valuename
            }
        }

    };
}
