extern crate qt_core;
use qt_core::core_application::CoreApplication;
use qt_core::variant::Variant;
use qt_core::variant_animation::VariantAnimation;
use qt_core::cpp_utils::*;
use qt_core::connections::Signal;

use qt_core::libc::c_void;
use qt_core::slots::ExternSlotVariantVariantRef;

extern fn value_changed(_data: *mut c_void, value: *const Variant) {
  let value = unsafe { value.as_ref() }.expect("value must not be null");
  println!("value_changed: {}", value.to_string(AsStruct).to_std_string());
}

#[test]
fn variant_animation() {
  CoreApplication::create_and_exit(|app| {
    let mut slot1 = ExternSlotVariantVariantRef::new();
    slot1.set(value_changed, std::ptr::null_mut());

    let mut animation = VariantAnimation::new(AsBox);
    animation.signals().value_changed().connect(slot1.as_ref());
    animation.signals().finished().connect(&app.slots().quit());
    animation.set_start_value(&Variant::new((1, AsStruct)));
    animation.set_end_value(&Variant::new((5, AsStruct)));
    animation.set_duration(5000);
    animation.start(());
    CoreApplication::exec()
  })
}