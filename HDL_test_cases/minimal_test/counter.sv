module counter (
    input  logic        clk,
    input  logic        rst_n,
    input  logic [7:0]  max_val,
    output logic [7:0]  count
);
    always_ff @(posedge clk or negedge rst_n) begin
        if (!rst_n)
            count <= 8'd0;
        else if (count >= max_val)
            count <= 8'd0;
        else
            count <= count + 8'd1;
    end
endmodule
