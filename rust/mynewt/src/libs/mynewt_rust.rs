/* automatically generated by rust-bindgen */

use
super::*;

#[repr(C)]
#[derive(Copy, Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    storage: Storage,
    align: [Align; 0],
}
impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn new(storage: Storage) -> Self {
        Self { storage, align: [] }
    }
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
pub type __uint8_t = ::cty::c_uchar;
pub type __int16_t = ::cty::c_short;
pub type __uint16_t = ::cty::c_ushort;
pub type __int32_t = ::cty::c_long;
pub type __uint32_t = ::cty::c_ulong;
pub type __int64_t = ::cty::c_longlong;
pub type __uint64_t = ::cty::c_ulonglong;
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = "  Initialise the Mynewt system.  Start the Mynewt drivers and libraries.  Equivalent to `sysinit()` macro in C."]
    pub fn rust_sysinit();
}
pub type os_stack_t = u32;
pub type os_time_t = u32;
pub type os_event_fn = ::core::option::Option<unsafe extern "C" fn(ev: *mut os_event)>;
#[repr(C)]
pub struct os_event__bindgen_ty_1 {
    pub stqe_next: *mut os_event,
}
impl Default for os_event__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = " Initialize a device."]
#[doc = ""]
#[doc = " - __`dev`__: The device to initialize."]
#[doc = " - __`arg`__: User defined argument to pass to the device initalization"]
#[doc = ""]
#[doc = " Return: 0 on success, non-zero error code on failure."]
pub type os_dev_init_func_t = ::core::option::Option<
    unsafe extern "C" fn(arg1: *mut os_dev, arg2: *mut ::cty::c_void) -> ::cty::c_int,
>;
pub type os_dev_open_func_t = ::core::option::Option<
    unsafe extern "C" fn(arg1: *mut os_dev, arg2: u32, arg3: *mut ::cty::c_void) -> ::cty::c_int,
>;
pub type os_dev_suspend_func_t = ::core::option::Option<
    unsafe extern "C" fn(arg1: *mut os_dev, arg2: os_time_t, arg3: ::cty::c_int) -> ::cty::c_int,
>;
pub type os_dev_resume_func_t =
    ::core::option::Option<unsafe extern "C" fn(arg1: *mut os_dev) -> ::cty::c_int>;
pub type os_dev_close_func_t =
    ::core::option::Option<unsafe extern "C" fn(arg1: *mut os_dev) -> ::cty::c_int>;
#[repr(C)]
pub struct os_dev__bindgen_ty_1 {
    pub stqe_next: *mut os_dev,
}
impl Default for os_dev__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct os_mutex__bindgen_ty_1 {
    pub slh_first: *mut os_task,
}
impl Default for os_mutex__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type os_sanity_check_func_t = ::core::option::Option<
    unsafe extern "C" fn(arg1: *mut os_sanity_check, arg2: *mut ::cty::c_void) -> ::cty::c_int,
>;
#[repr(C)]
pub struct os_sanity_check__bindgen_ty_1 {
    pub sle_next: *mut os_sanity_check,
}
impl Default for os_sanity_check__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type os_task_func_t = ::core::option::Option<unsafe extern "C" fn(arg1: *mut ::cty::c_void)>;
#[repr(C)]
pub struct os_task__bindgen_ty_1 {
    pub stqe_next: *mut os_task,
}
impl Default for os_task__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct os_task__bindgen_ty_2 {
    pub tqe_next: *mut os_task,
    pub tqe_prev: *mut *mut os_task,
}
impl Default for os_task__bindgen_ty_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct os_task__bindgen_ty_3 {
    pub sle_next: *mut os_task,
}
impl Default for os_task__bindgen_ty_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const sensor_type_t_SENSOR_TYPE_NONE: sensor_type_t = 0;
pub const sensor_type_t_SENSOR_TYPE_ACCELEROMETER: sensor_type_t = 1;
pub const sensor_type_t_SENSOR_TYPE_MAGNETIC_FIELD: sensor_type_t = 2;
pub const sensor_type_t_SENSOR_TYPE_GYROSCOPE: sensor_type_t = 4;
pub const sensor_type_t_SENSOR_TYPE_LIGHT: sensor_type_t = 8;
pub const sensor_type_t_SENSOR_TYPE_TEMPERATURE: sensor_type_t = 16;
pub const sensor_type_t_SENSOR_TYPE_AMBIENT_TEMPERATURE: sensor_type_t = 32;
pub const sensor_type_t_SENSOR_TYPE_PRESSURE: sensor_type_t = 64;
pub const sensor_type_t_SENSOR_TYPE_PROXIMITY: sensor_type_t = 128;
pub const sensor_type_t_SENSOR_TYPE_RELATIVE_HUMIDITY: sensor_type_t = 256;
pub const sensor_type_t_SENSOR_TYPE_ROTATION_VECTOR: sensor_type_t = 512;
pub const sensor_type_t_SENSOR_TYPE_ALTITUDE: sensor_type_t = 1024;
pub const sensor_type_t_SENSOR_TYPE_WEIGHT: sensor_type_t = 2048;
pub const sensor_type_t_SENSOR_TYPE_LINEAR_ACCEL: sensor_type_t = 4096;
pub const sensor_type_t_SENSOR_TYPE_GRAVITY: sensor_type_t = 8192;
pub const sensor_type_t_SENSOR_TYPE_EULER: sensor_type_t = 16384;
pub const sensor_type_t_SENSOR_TYPE_COLOR: sensor_type_t = 32768;
pub const sensor_type_t_SENSOR_TYPE_USER_DEFINED_1: sensor_type_t = 67108864;
pub const sensor_type_t_SENSOR_TYPE_USER_DEFINED_2: sensor_type_t = 134217728;
pub const sensor_type_t_SENSOR_TYPE_USER_DEFINED_3: sensor_type_t = 268435456;
pub const sensor_type_t_SENSOR_TYPE_USER_DEFINED_4: sensor_type_t = 536870912;
pub const sensor_type_t_SENSOR_TYPE_USER_DEFINED_5: sensor_type_t = 1073741824;
pub const sensor_type_t_SENSOR_TYPE_USER_DEFINED_6: sensor_type_t = -2147483648;
pub const sensor_type_t_SENSOR_TYPE_ALL: sensor_type_t = 4294967295;
pub type sensor_type_t = i64;
pub const sensor_event_type_t_SENSOR_EVENT_TYPE_DOUBLE_TAP: sensor_event_type_t = 1;
pub const sensor_event_type_t_SENSOR_EVENT_TYPE_SINGLE_TAP: sensor_event_type_t = 2;
pub const sensor_event_type_t_SENSOR_EVENT_TYPE_FREE_FALL: sensor_event_type_t = 4;
pub const sensor_event_type_t_SENSOR_EVENT_TYPE_SLEEP_CHANGE: sensor_event_type_t = 8;
pub const sensor_event_type_t_SENSOR_EVENT_TYPE_WAKEUP: sensor_event_type_t = 16;
pub const sensor_event_type_t_SENSOR_EVENT_TYPE_SLEEP: sensor_event_type_t = 32;
pub const sensor_event_type_t_SENSOR_EVENT_TYPE_ORIENT_CHANGE: sensor_event_type_t = 64;
pub const sensor_event_type_t_SENSOR_EVENT_TYPE_ORIENT_X_CHANGE: sensor_event_type_t = 128;
pub const sensor_event_type_t_SENSOR_EVENT_TYPE_ORIENT_Y_CHANGE: sensor_event_type_t = 256;
pub const sensor_event_type_t_SENSOR_EVENT_TYPE_ORIENT_Z_CHANGE: sensor_event_type_t = 512;
pub const sensor_event_type_t_SENSOR_EVENT_TYPE_ORIENT_X_L_CHANGE: sensor_event_type_t = 1024;
pub const sensor_event_type_t_SENSOR_EVENT_TYPE_ORIENT_Y_L_CHANGE: sensor_event_type_t = 2048;
pub const sensor_event_type_t_SENSOR_EVENT_TYPE_ORIENT_Z_L_CHANGE: sensor_event_type_t = 4096;
pub const sensor_event_type_t_SENSOR_EVENT_TYPE_ORIENT_X_H_CHANGE: sensor_event_type_t = 8192;
pub const sensor_event_type_t_SENSOR_EVENT_TYPE_ORIENT_Y_H_CHANGE: sensor_event_type_t = 16384;
pub const sensor_event_type_t_SENSOR_EVENT_TYPE_ORIENT_Z_H_CHANGE: sensor_event_type_t = 32768;
pub type sensor_event_type_t = u32;
#[doc = " Callback for handling sensor data, specified in a sensor listener."]
#[doc = ""]
#[doc = " - __`sensor`__: The sensor for which data is being returned"]
#[doc = " - __`arg`__: The argument provided to sensor_read() function."]
#[doc = " - __`data`__: A single sensor reading for that sensor listener"]
#[doc = " - __`type`__: The sensor type for the data function"]
#[doc = ""]
#[doc = " Return: 0 on success, non-zero error code on failure."]
pub type sensor_data_func_t = ::core::option::Option<
    unsafe extern "C" fn(
        arg1: *mut sensor,
        arg2: *mut ::cty::c_void,
        arg3: *mut ::cty::c_void,
        arg4: sensor_type_t,
    ) -> ::cty::c_int,
>;
#[doc = " Callback for trigger compare functions."]
#[doc = ""]
#[doc = " - __`type`__: Type of sensor"]
#[doc = " - __`low_thresh`__: The sensor low threshold"]
#[doc = " - __`high_thresh`__: The sensor high threshold"]
#[doc = " - __`arg`__: Ptr to data"]
pub type sensor_trigger_cmp_func_t = ::core::option::Option<
    unsafe extern "C" fn(
        arg1: sensor_type_t,
        arg2: *mut sensor_data_t,
        arg3: *mut sensor_data_t,
        arg4: *mut ::cty::c_void,
    ) -> ::cty::c_int,
>;
#[doc = " Callback for event notifications."]
#[doc = ""]
#[doc = " - __`sensor`__: The sensor that observed the event"]
#[doc = " - __`arg`__: The opaque argument provided during registration"]
#[doc = " - __`event`__: The sensor event type that was observed"]
pub type sensor_notifier_func_t = ::core::option::Option<
    unsafe extern "C" fn(
        arg1: *mut sensor,
        arg2: *mut ::cty::c_void,
        arg3: sensor_event_type_t,
    ) -> ::cty::c_int,
>;
#[doc = " Callback for reporting a sensor read error."]
#[doc = ""]
#[doc = " - __`sensor`__: The sensor for which a read failed."]
#[doc = " - __`arg`__: The optional argument registered with the callback."]
#[doc = " - __`status`__: Indicates the cause of the read failure.  Determined by the"]
#[doc = "               underlying sensor driver."]
pub type sensor_error_func_t = ::core::option::Option<
    unsafe extern "C" fn(sensor: *mut sensor, arg: *mut ::cty::c_void, status: ::cty::c_int),
>;
#[repr(C)]
pub struct sensor_listener__bindgen_ty_1 {
    pub sle_next: *mut sensor_listener,
}
impl Default for sensor_listener__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct sensor_notifier__bindgen_ty_1 {
    pub sle_next: *mut sensor_notifier,
}
impl Default for sensor_notifier__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct sensor_type_traits__bindgen_ty_1 {
    pub sle_next: *mut sensor_type_traits,
}
impl Default for sensor_type_traits__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = " Read a single value from a sensor, given a specific sensor type"]
#[doc = " (e.g. SENSOR_TYPE_PROXIMITY)."]
#[doc = ""]
#[doc = " - __`sensor`__: The sensor to read from"]
#[doc = " - __`type`__: The type(s) of sensor values to read.  Mask containing that type, provide"]
#[doc = "        all, to get all values."]
#[doc = " - __`data_func`__: The function to call with each value read.  If NULL, it calls all"]
#[doc = "        sensor listeners associated with this function."]
#[doc = " - __`arg`__: The argument to pass to the read callback."]
#[doc = " - __`timeout`__: Timeout. If block until result, specify OS_TIMEOUT_NEVER, 0 returns"]
#[doc = "        immediately (no wait.)"]
#[doc = ""]
#[doc = " Return: 0 on success, non-zero error code on failure."]
pub type sensor_read_func_t = ::core::option::Option<
    unsafe extern "C" fn(
        arg1: *mut sensor,
        arg2: sensor_type_t,
        arg3: sensor_data_func_t,
        arg4: *mut ::cty::c_void,
        arg5: u32,
    ) -> ::cty::c_int,
>;
#[doc = " Get the configuration of the sensor for the sensor type.  This includes"]
#[doc = " the value type of the sensor."]
#[doc = ""]
#[doc = " - __`sensor`__: Ptr to the sensor"]
#[doc = " - __`type`__: The type of sensor value to get configuration for"]
#[doc = " - __`cfg`__: A pointer to the sensor value to place the returned result into."]
#[doc = ""]
#[doc = " Return: 0 on success, non-zero error code on failure."]
pub type sensor_get_config_func_t = ::core::option::Option<
    unsafe extern "C" fn(
        arg1: *mut sensor,
        arg2: sensor_type_t,
        arg3: *mut sensor_cfg,
    ) -> ::cty::c_int,
>;
#[doc = " Send a new configuration register set to the sensor."]
#[doc = ""]
#[doc = " - __`sensor`__: Ptr to the sensor-specific stucture"]
#[doc = " - __`arg`__: Ptr to the sensor-specific configuration structure"]
#[doc = ""]
#[doc = " Return: 0 on success, non-zero error code on failure."]
pub type sensor_set_config_func_t = ::core::option::Option<
    unsafe extern "C" fn(arg1: *mut sensor, arg2: *mut ::cty::c_void) -> ::cty::c_int,
>;
#[doc = " Set the trigger and threshold values for a specific sensor for the sensor"]
#[doc = " type."]
#[doc = ""]
#[doc = " - __`sensor`__: Ptr to the sensor"]
#[doc = " - __`type`__: type of sensor"]
#[doc = " - __`stt`__: Ptr to teh sensor traits"]
#[doc = ""]
#[doc = " Return: 0 on success, non-zero error code on failure."]
pub type sensor_set_trigger_thresh_t = ::core::option::Option<
    unsafe extern "C" fn(
        arg1: *mut sensor,
        arg2: sensor_type_t,
        stt: *mut sensor_type_traits,
    ) -> ::cty::c_int,
>;
#[doc = " Clear the high/low threshold values for a specific sensor for the sensor"]
#[doc = " type."]
#[doc = ""]
#[doc = " - __`sensor`__: Ptr to the sensor"]
#[doc = " - __`type`__: Type of sensor"]
#[doc = ""]
#[doc = " Return: 0 on success, non-zero error code on failure."]
pub type sensor_clear_trigger_thresh_t = ::core::option::Option<
    unsafe extern "C" fn(sensor: *mut sensor, type_: sensor_type_t) -> ::cty::c_int,
>;
#[doc = " Set the notification expectation for a targeted set of events for the"]
#[doc = " specific sensor. After this function returns successfully, the implementer"]
#[doc = " shall post corresponding event notifications to the sensor manager."]
#[doc = ""]
#[doc = " - __`sensor`__: The sensor to expect notifications from."]
#[doc = " - __`event`__: The mask of event types to expect notifications from."]
#[doc = ""]
#[doc = " Return: 0 on success, non-zero error code on failure."]
pub type sensor_set_notification_t = ::core::option::Option<
    unsafe extern "C" fn(arg1: *mut sensor, arg2: sensor_event_type_t) -> ::cty::c_int,
>;
#[doc = " Unset the notification expectation for a targeted set of events for the"]
#[doc = " specific sensor."]
#[doc = ""]
#[doc = " - __`sensor`__: The sensor."]
#[doc = " - __`event`__: The mask of event types."]
#[doc = ""]
#[doc = " Return: 0 on success, non-zero error code on failure."]
pub type sensor_unset_notification_t = ::core::option::Option<
    unsafe extern "C" fn(arg1: *mut sensor, arg2: sensor_event_type_t) -> ::cty::c_int,
>;
#[doc = " Let driver handle interrupt in the sensor context"]
#[doc = ""]
#[doc = " - __`sensor`__: Ptr to the sensor"]
#[doc = ""]
#[doc = " Return: 0 on success, non-zero error code on failure."]
pub type sensor_handle_interrupt_t =
    ::core::option::Option<unsafe extern "C" fn(sensor: *mut sensor) -> ::cty::c_int>;
#[doc = " Reset Sensor function Ptr"]
#[doc = ""]
#[doc = " - __`Ptr`__: to the sensor"]
#[doc = ""]
#[doc = " Return: 0 on success, non-zero on failure"]
pub type sensor_reset_t =
    ::core::option::Option<unsafe extern "C" fn(arg1: *mut sensor) -> ::cty::c_int>;
#[repr(C)]
pub struct sensor__bindgen_ty_1 {
    pub slh_first: *mut sensor_listener,
}
impl Default for sensor__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct sensor__bindgen_ty_2 {
    pub slh_first: *mut sensor_notifier,
}
impl Default for sensor__bindgen_ty_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct sensor__bindgen_ty_3 {
    pub slh_first: *mut sensor_type_traits,
}
impl Default for sensor__bindgen_ty_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct sensor__bindgen_ty_4 {
    pub sle_next: *mut sensor,
}
impl Default for sensor__bindgen_ty_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "  Represents a single temperature sensor raw value"]
#[repr(C, packed)]
#[derive(Default)]
pub struct sensor_temp_raw_data {
    #[doc = "  Raw temp from STM32 Internal Temp Sensor is 0 to 4095"]
    pub strd_temp_raw: u32,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
}
impl sensor_temp_raw_data {
    #[inline]
    pub fn strd_temp_raw_is_valid(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_strd_temp_raw_is_valid(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(strd_temp_raw_is_valid: u8) -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize], u8> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let strd_temp_raw_is_valid: u8 =
                unsafe { ::core::mem::transmute(strd_temp_raw_is_valid) };
            strd_temp_raw_is_valid as u64
        });
        __bindgen_bitfield_unit
    }
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = "  Interpret `sensor_data` as a `sensor_temp_raw_data` struct that contains raw temp."]
    #[doc = "  Copy the sensor data into `dest`.  Return 0 if successful."]
    pub fn get_temp_raw_data(
        sensor_data: *mut ::cty::c_void,
        dest: *mut sensor_temp_raw_data,
    ) -> ::cty::c_int;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = "  Interpret `sensor_data` as a `sensor_temp_data` struct that contains computed temp."]
    #[doc = "  Copy the sensor data into `dest`.  Return 0 if successful."]
    pub fn get_temp_data(
        sensor_data: *mut ::cty::c_void,
        dest: *mut sensor_temp_data,
    ) -> ::cty::c_int;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = "  Return the Mynewt device for the Mynewt sensor."]
    pub fn sensor_get_device(s: *mut sensor) -> *mut os_dev;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = "  Return the name for the Mynewt device.  Assumes name is non-null."]
    pub fn device_get_name(device: *mut os_dev) -> *const ::cty::c_char;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = "  Return the NULL sensor."]
    pub fn null_sensor() -> *mut sensor;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = "  Return non-zero if sensor is NULL."]
    pub fn is_null_sensor(p: *mut sensor) -> ::cty::c_int;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = "  Return non-zero if sensor data is NULL."]
    pub fn is_null_sensor_data(p: *mut ::cty::c_void) -> ::cty::c_int;
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = "  Assume we are writing an object now.  Write the key name and start a child array."]
    #[doc = "  ```"]
    #[doc = "  {a:b --> {a:b, key:["]
    #[doc = "  ```"]
    pub fn json_helper_set_array(object: *mut ::cty::c_void, key: *const ::cty::c_char);
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = "  End the child array and resume writing the parent object."]
    #[doc = "  ```"]
    #[doc = "  {a:b, key:[... --> {a:b, key:[...]"]
    #[doc = "  ```"]
    pub fn json_helper_close_array(object: *mut ::cty::c_void, key: *const ::cty::c_char);
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = "  Assume we have called `set_array`.  Start an array item, assumed to be an object."]
    #[doc = "  ```"]
    #[doc = "  [... --> [...,"]
    #[doc = "  ```"]
    pub fn json_helper_object_array_start_item(key: *const ::cty::c_char);
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = "  End an array item, assumed to be an object."]
    #[doc = "  ```"]
    #[doc = "  [... --> [...,"]
    #[doc = "  ```"]
    pub fn json_helper_object_array_end_item(key: *const ::cty::c_char);
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = "  Encode an int value into the current JSON encoding value `coap_json_value`"]
    pub fn json_helper_set_int(object: *mut ::cty::c_void, key: *const ::cty::c_char, value: u64);
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = "  Encode an unsigned int value into the current JSON encoding value `coap_json_value`"]
    pub fn json_helper_set_uint(object: *mut ::cty::c_void, key: *const ::cty::c_char, value: u64);
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = "  Encode a float value into the current JSON encoding value `coap_json_value`"]
    pub fn json_helper_set_float(object: *mut ::cty::c_void, key: *const ::cty::c_char, value: f32);
}
#[mynewt_macros::safe_wrap(attr)] extern "C" {
    #[doc = "  Encode a text value into the current JSON encoding value `coap_json_value`"]
    pub fn json_helper_set_text_string(
        object: *mut ::cty::c_void,
        key: *const ::cty::c_char,
        value: *const ::cty::c_char,
    );
}