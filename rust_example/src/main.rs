include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn print_data(app_time_s: f64, output: &AppDataOutput) {
    println!(
        "app_time_s: {:.1}, output_ffi: {:.3}",
        app_time_s, output.output_ffi
    );
}

fn main() {
    println!("Starting app");
    let app_iface = unsafe { app_interface_new() };
    let mut input = AppDataInput { input_ffi: 0.0 };
    let timestep_s = 0.1;
    let max_time_s = 1.0;

    for app_time_s in (0..(max_time_s / timestep_s) as i32 + 1).map(|i| i as f64 * timestep_s) {
        input.input_ffi = app_time_s;
        let input_ptr = &mut input as *mut AppDataInput;
        let output = unsafe { app_interface_update(app_iface, app_time_s, input_ptr) };
        print_data(app_time_s, &output);
    }

    unsafe {
        app_interface_free(app_iface);
    }

    println!("Done running app");
}
