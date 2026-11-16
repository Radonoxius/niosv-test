//=======================================================
//  Avalon-MM slave: Switch input register (byte-addressed, read-only)
//
//  Register map (base = 0xFF200000 on lightweight bridge):
//    Byte offsets 0x0 - 0x3 (0xFF200000 - 0xFF200003):
//        Read : returns current SW[9:0] state in bits [9:0]
//        Write: ignored (read-only)
//=======================================================
module switches (
	// Avalon clock / reset
	input  wire        clk,
	input  wire        reset_n,

	// Avalon-MM slave interface
	input  wire [1:0]  avs_address, // byte address (4-byte span)
	input  wire        avs_read,
	output reg  [31:0] avs_readdata,

	// Conduit from the board switches
	input  wire [9:0]  switches_in
);
	// Byte offsets 0x0-0x3 => avs_address = 0x0 with 0x0 offset 
	wire sw_sel = (avs_address[1:0] == 2'h0);
	// Writes are not supported since its meaningless for this module

	// Switches are asynchronous to clk, so pass them through a
	// two-flop synchronizer to avoid metastability.
	reg [9:0] sw_meta;
	reg [9:0] sw_sync;

	always @(posedge clk or negedge reset_n) begin
		if (!reset_n) begin
			sw_meta <= 10'b0;
			sw_sync <= 10'b0;
		end else begin
			sw_meta <= switches_in;
			sw_sync <= sw_meta;
		end
	end

	// Read logic (zero read latency).
	// No default assignment: avs_readdata is intentionally latched,
	// holding its last value when not being read.
	always @(*) begin
		if (sw_sel && avs_read)
			avs_readdata = {22'b0, sw_sync};
	end
endmodule