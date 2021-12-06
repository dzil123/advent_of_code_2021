LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
USE ieee.numeric_std.ALL;

ENTITY AdventOfCode2021Day6_tb IS
END AdventOfCode2021Day6_tb;

architecture Behavioral of AdventOfCode2021Day6_tb is

    COMPONENT AdventOfCode2021Day6
        port ( Clk : in STD_LOGIC );
    END COMPONENT;
    
    signal Clk : std_logic := '0';
    
    constant Clk_period : time := 10 ns;
    
BEGIN

    uut: AdventOfCode2021Day6 PORT MAP (
           Clk => Clk
         );
    
   Clk_process: process
   begin
		Clk <= '0';
		wait for Clk_period/2;
		Clk <= '1';
		wait for Clk_period/2;
   end process;

END Behavioral;
