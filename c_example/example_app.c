#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>

#include "pictorus.h"


void print_data(double app_time_s, AppDataOutput *data) {
    printf("Time: %f, output_ffi: %f\n", app_time_s, data->output_ffi);
}

int main(void) {
    printf("Starting app\n");

    AppInterface *app_iface = app_interface_new();

    AppDataInput input = {
        .input_ffi= 0.0,
    };

    double timestep_s = 0.1;
    double app_time_s = 0.0;
    double max_time_s = 1.0;

    for(double app_time_s=0.0; app_time_s < max_time_s; app_time_s += timestep_s) {
        input.input_ffi = app_time_s;
        AppDataOutput output = app_interface_update(app_iface, app_time_s, &input);
        print_data(app_time_s, &output);
    }

    app_interface_free(app_iface);

    printf("Done running app\n");
}
