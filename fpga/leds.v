//=======================================================
//  Avalon-MM slave: LED control register (byte-addressed)
//
//  Register map (base = 0xFF200000 on lightweight bridge):
//    Byte offsets 0x4 - 0x7 (0xFF200004 - 0xFF200007):
//        Write: bits [9:0] drive LEDR[9:0]
//        Read : returns current LED value in bits [9:0]
//=======================================================

module leds (
	// Avalon clock / reset
	input  wire        clk,
	input  wire        reset_n,

	// Avalon-MM slave interface
	input  wire [1:0]  avs_address, // byte address (4-byte span)
	input  wire        avs_write,
	input  wire        avs_read,
	input  wire [31:0] avs_writedata,
	output reg  [31:0] avs_readdata,

	// Conduit to the board LEDs
	output wire [9:0]  leds_out
);

	// Byte offsets 0x4-0x7 => avs_address = 0x00 with 0x4 offset
	wire led_sel = (avs_address[1:0] == 2'h0);

	reg [9:0] led_reg;

	// Write logic (address decoded, byte-enable aware)
	always @(posedge clk or negedge reset_n) begin
		if (!reset_n) begin
			led_reg <= 10'b0;
		end else if (avs_write && led_sel) begin
				led_reg[9:0] <= avs_writedata[9:0];
		end
	end

	// Read logic (zero read latency)
	always @(*) begin
		if (led_sel && avs_read)
			avs_readdata = {22'b0, led_reg};
	end

	assign leds_out = led_reg;

endmodule