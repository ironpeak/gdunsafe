#[macro_export]
macro_rules! load_resource {
    ($path:tt) => {
        unsafe {
            ResourceLoader::godot_singleton()
                .load($path, "", false)
                .unwrap()
                .assume_safe()
        }
    };
    ($path:tt, $type_hint:tt) => {
        unsafe {
            ResourceLoader::godot_singleton()
                .load($path, $type_hint, false)
                .unwrap()
                .assume_safe()
        }
    };
    ($path:tt, $type_hint:tt, $p_no_cache:tt) => {
        unsafe {
            ResourceLoader::godot_singleton()
                .load($path, $type_hint, $p_no_cache)
                .unwrap()
                .assume_safe()
        }
    };
}
