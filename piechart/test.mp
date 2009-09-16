input piechartmp ;
prologues:=3;
SetupPercent(this, "%") ;
Segment(80.0, "lamers");
Segment(20.0, "leets");
beginfig(1);
PieChart(3cm, 0.1, 65, 0, 0);
Label(0)(percent)(inwards,0);
Label.auto(0)(name)(outwards,(-0.5,0)) ;
endfig;
end
