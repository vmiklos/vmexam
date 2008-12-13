transform pagecoords;
pagecoords:=identity;
beginfig (1)
fill ((0,0)--(10,0)--(10,10)--(0,10)--cycle)
		transformed pagecoords withcolor (0.75, 0.75, 1);
fill ((3.33,0)--(10,3.33)--(6.66,10)--(0,6.66)--cycle)
		transformed pagecoords withcolor (0.5, 0.5, 1);
fill ((2.5,2.5)--(7.5,2.5)--(7.5,7.5)--(2.5,7.5)--cycle)
		transformed pagecoords withcolor (0.25, 0.25, 1);
fill ((3,3)--(3,7)--(6,7)--(6,6)--(4,6)--(4,5)--(5.5,5)--(5.5,4)--(4,4)--(4,3)--cycle)
		transformed pagecoords withcolor (0.75, 0.75, 1);
endfig;
bye
