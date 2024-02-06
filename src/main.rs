use std::ptr::null_mut;

use autocxx::prelude::*; // use all the main autocxx functions

include_cpp! {
    #include "DeckLinkAPI.h"
    safety!(unsafe) // see details of unsafety policies described in the 'safety' section of the book
    generate!("CreateDeckLinkAPIInformationInstance")
    generate!("IDeckLinkAPIInformation")
    generate!("_BMDDeckLinkAPIInformationID")

    generate!("IDeckLinkIterator")
    generate!("CreateDeckLinkIteratorInstance")

}

fn main() {
    let api_info = ffi::CreateDeckLinkAPIInformationInstance();

    let mut api_info = unsafe { std::pin::Pin::new_unchecked(&mut *api_info) };

    let mut val: i64 = 0;

    unsafe {
        api_info.as_mut().GetInt(
            ffi::_BMDDeckLinkAPIInformationID::BMDDeckLinkAPIVersion as u32,
            &mut val as *mut i64,
        );
        println!("{}", val);
    }

    let iterator_instance = ffi::CreateDeckLinkIteratorInstance();
    let mut iterator_instance = unsafe { std::pin::Pin::new_unchecked(&mut *iterator_instance) };
    
    //let mut device = null_mut();

    unsafe {
        ffi::IDeckLinkIterator::Next(iterator_instance.as_mut(), null_mut());
        //iterator_instance.as_mut().Next(null_mut());
         //ffi::IDeckLinkIterator::Next(iterator_instance.as_mut(), device);
       // println!("{}", val);
    }
}
