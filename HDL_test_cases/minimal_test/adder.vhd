library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

entity adder is
    port (
        a    : in  std_logic_vector(7 downto 0);
        b    : in  std_logic_vector(7 downto 0);
        sum  : out std_logic_vector(8 downto 0)
    );
end entity adder;

architecture rtl of adder is
begin
    sum <= std_logic_vector(resize(unsigned(a), 9) + resize(unsigned(b), 9));
end architecture rtl;
