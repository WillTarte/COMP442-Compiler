Table: POLYNOMIAL
|	name	|	kind	|	type	|	link	|
| --- | --- | --- | --- |
|	evaluate	|	function	|	[float]->float	|	[table](#evaluate)	|
Table: LINEAR
|	name	|	kind	|	type	|	link	|
| --- | --- | --- | --- |
|	a	|	variable	|	float	|	X	|
|	b	|	variable	|	float	|	X	|
|	build	|	function	|	[float, float]->LINEAR	|	[table](#build)	|
|	evaluate	|	function	|	[float]->float	|	[table](#evaluate)	|
Table: QUADRATIC
|	name	|	kind	|	type	|	link	|
| --- | --- | --- | --- |
|	a	|	variable	|	float	|	X	|
|	b	|	variable	|	float	|	X	|
|	c	|	variable	|	float	|	X	|
|	build	|	function	|	[float, float, float]->QUADRATIC	|	[table](#build)	|
|	evaluate	|	function	|	[float]->float	|	[table](#evaluate)	|
Table: main
|	name	|	kind	|	type	|	link	|
| --- | --- | --- | --- |
|	f1	|	variable	|	linear	|	X	|
|	f2	|	variable	|	quadratic	|	X	|
|	counter	|	variable	|	integer	|	X	|
