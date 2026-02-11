= Example of `littst`

This example shows a firmware program calling into a memory-mapped hardware
register implemented in Verilog.

== Firmware

We would like to toggle a register:

_⟨Toggle the register⟩_≡ <chunk-f50d7ea136b2fbfe> \
#raw(lang: "c", "GPIO_REG ^= 1;") \

Let's write a C program that does defines and toggles this register continually
in a loop:

_⟨main.c⟩_≡ <chunk-9440535e48a92b92> \
#raw(lang: "c", "#include <stdint.h>") \
#raw(lang: "c", "#define GPIO_REG (*(volatile uint32_t*)0x40000000)") \
#raw(lang: "c", "") \
#raw(lang: "c", "int main(void)") \
#raw(lang: "c", "{") \
#raw(lang: "c", "    GPIO_REG = 1;") \
#raw(lang: "c", "    while (1)") \
#raw(lang: "c", "        ")#link(<chunk-f50d7ea136b2fbfe>)[_⟨Toggle the register _*#link(<chunk-f50d7ea136b2fbfe>)[#context counter(page).at(<chunk-f50d7ea136b2fbfe>).first()]*⟩]\
#raw(lang: "c", "}") \

== Hardware

The following Verilog code describes the hardware implementation of the same
register:

_⟨Output data on clock⟩_≡ <chunk-31c5b7c6c3389ed8> \
#raw(lang: "verilog", "always @(posedge clk)") \
#raw(lang: "verilog", "begin") \
#raw(lang: "verilog", "    if (write_en)") \
#raw(lang: "verilog", "        gpio_out <= data_in;") \
#raw(lang: "verilog", "end") \

Of course we have to put the code into a module:

_⟨gpio.v⟩_≡ <chunk-34a3b312d92fa57> \
#raw(lang: "verilog", "module gpio_reg (") \
#raw(lang: "verilog", "    input  wire clk,") \
#raw(lang: "verilog", "    input  wire write_en,") \
#raw(lang: "verilog", "    input  wire data_in,") \
#raw(lang: "verilog", "    output reg  gpio_out") \
#raw(lang: "verilog", ");") \
#link(<chunk-31c5b7c6c3389ed8>)[_⟨Output data on clock _*#link(<chunk-31c5b7c6c3389ed8>)[#context counter(page).at(<chunk-31c5b7c6c3389ed8>).first()]*⟩]\
#raw(lang: "verilog", "endmodule") \
