//=======================================================
//  Avalon-MM slave: switch input and interrupt-pending register
//
//  Register map (base = 0x8144 in this Platform Designer system):
//    Byte offsets 0x0 - 0x3 (0x8144 - 0x8147):
//        Read : returns current SW[9:0] state in bits [9:0]
//        Write: write 1 to bit 0 to clear the interrupt-pending flag
//=======================================================
module switches (
	// Avalon clock / reset
	input  wire        clk,
	input  wire        reset_n,

	// Avalon-MM slave interface
	input  wire [1:0]  avs_address, // byte address (4-byte span)
	input  wire        avs_read,
	input  wire        avs_write,
	input  wire [31:0] avs_writedata,
	output reg  [31:0] avs_readdata,
	output wire        irq_out,

	// Conduit from the board switches
	input  wire [9:0]  switches_in
);
	// Byte offsets 0x0-0x3 => avs_address = 0x0 with 0x0 offset 
	wire sw_sel = (avs_address[1:0] == 2'h0);
	// IRQ0 stays asserted while bit 0 is pending. Software acknowledges
	// the interrupt by writing a 1 to bit 0 of this register.
	reg [31:0] pending_reg;
	assign irq_out = pending_reg[0];

	// Switches are asynchronous to clk, so pass them through a
	// two-flop synchronizer to avoid metastability.
	reg [9:0] sw_meta;
	reg [9:0] sw_sync;
	reg [9:0] sw_prev;
	reg       sw_initialized;

	always @(posedge clk or negedge reset_n) begin
		if (!reset_n) begin
			sw_meta <= 10'b0;
			sw_sync <= 10'b0;
			sw_prev <= 10'b0;
			sw_initialized <= 1'b0;
			pending_reg <= 32'b0;
		end else begin
			sw_meta <= switches_in;
			sw_sync <= sw_meta;

			// Establish the initial switch state without generating an IRQ.
			if (!sw_initialized) begin
				sw_prev <= sw_sync;
				sw_initialized <= 1'b1;
			end else if (sw_sync != sw_prev) begin
				// Any switch transition raises a single latched IRQ.
				sw_prev <= sw_sync;
				pending_reg[0] <= 1'b1;
			end

			// Write-one-to-clear. A new switch event takes priority over a
			// simultaneous acknowledgement so no event is lost.
			if (sw_sel && avs_write && avs_writedata[0])
				pending_reg[0] <= 1'b0;
			if (sw_initialized && sw_sync != sw_prev)
				pending_reg[0] <= 1'b1;
		end
	end

	// Read logic (zero read latency).
	always @(*) begin
		avs_readdata = 32'b0;
		if (sw_sel && avs_read)
			avs_readdata = {22'b0, sw_sync};
	end
endmodule
