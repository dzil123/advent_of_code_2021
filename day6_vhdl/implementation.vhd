LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
USE ieee.numeric_std.ALL;

entity AdventOfCode2021Day6 is
    port ( Clk : in STD_LOGIC;
           Total: out unsigned(63 DOWNTO 0) );
end AdventOfCode2021Day6;

architecture Behavioral of AdventOfCode2021Day6 is

type FishAgeType is array (0 to 8) of unsigned(63 DOWNTO 0);

type StateType is (READY_STATE, DONE_STATE);

-- sample input
SIGNAL fish_ages: FishAgeType := (to_unsigned(0, 64), to_unsigned(1, 64), to_unsigned(1, 64), to_unsigned(2, 64),
                                  to_unsigned(1, 64), to_unsigned(0, 64), to_unsigned(0, 64), to_unsigned(0, 64), to_unsigned(0, 64));

-- my actual input
--SIGNAL fish_ages: FishAgeType := (to_unsigned(0, 64), to_unsigned(81, 64), to_unsigned(46, 64), to_unsigned(60, 64),
--                                  to_unsigned(63, 64), to_unsigned(50, 64), to_unsigned(0, 64), to_unsigned(0, 64), to_unsigned(0, 64));

SIGNAL total_fishes: unsigned(63 DOWNTO 0) := to_unsigned(0, 64);
SIGNAL current_day: unsigned(15 downto 0) := to_unsigned(0, 16);

BEGIN

    CombinationalLogic: PROCESS(fish_ages)
        variable sum : unsigned(63 DOWNTO 0) := to_unsigned(0, 64);
    BEGIN
        sum := to_unsigned(0, 64);
            for I in fish_ages'low to fish_ages'high loop
                sum := sum + fish_ages(I);
            end loop;
        total_fishes <= sum;
    END PROCESS CombinationalLogic; 

    ClockProcess: PROCESS ( Clk )
        
        variable new_fishes: unsigned(63 DOWNTO 0) := to_unsigned(0, 64);
    BEGIN
    
        If (Clk = '1' AND Clk'EVENT) THEN     
             
            new_fishes := fish_ages(0);
            
            fish_ages(0) <= fish_ages(1);
            fish_ages(1) <= fish_ages(2);
            fish_ages(2) <= fish_ages(3);
            fish_ages(3) <= fish_ages(4);
            fish_ages(4) <= fish_ages(5);
            fish_ages(5) <= fish_ages(6);
            fish_ages(6) <= fish_ages(7) + new_fishes;
            fish_ages(7) <= fish_ages(8);
            fish_ages(8) <= new_fishes;
            
            current_day <= current_day + 1;
            
        END IF;
    
    END PROCESS ClockProcess; 

    Total <= total_fishes;

END Behavioral;
