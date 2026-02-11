module gpio_reg (
    input  wire clk,
    input  wire write_en,
    input  wire data_in,
    output reg  gpio_out
);
always @(posedge clk)
begin
    if (write_en)
        gpio_out <= data_in;
end
endmodule
