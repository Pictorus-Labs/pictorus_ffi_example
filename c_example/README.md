# C Example

This directory contains a simple example that shows how to call a Pictorus app from a C program. In order to accomplish this we need to link against the shared static library and header file provided in the [lib/](../lib/) directory.

To build and run the example, run the following commands from this directory:

```bash
gcc example_app.c ../lib/libpictorus.a -I../lib/ -o output/example_app -lpthread -Wl,--no-as-needed -ldl -lm
./output/example_app
```

You should see an output similar to the following:

```bash
Starting app
Time: 0.000000, output_ffi: -0.000000
Time: 0.100000, output_ffi: -0.025728
Time: 0.200000, output_ffi: 0.509879
Time: 0.300000, output_ffi: -0.036878
Time: 0.400000, output_ffi: 0.678048
Time: 0.500000, output_ffi: 0.713464
Time: 0.600000, output_ffi: 0.087214
Time: 0.700000, output_ffi: -0.078597
Time: 0.800000, output_ffi: -0.115386
Time: 0.900000, output_ffi: 0.571152
Time: 1.000000, output_ffi: -0.771117
```
