Table: POLYNOMIAL<a name="POLYNOMIAL"></a>
|	name	|	kind	|	type	|	link	|
| --- | --- | --- | --- |
|	evaluate	|	function	|	[float]->float	|	[table](#POLYNOMIAL::evaluate)	|

____
Table: evaluate<a name="POLYNOMIAL::evaluate"></a>
|	name	|	kind	|	type	|	link	|
| --- | --- | --- | --- |
|	x	|	parameter	|	float	|	X	|

____
Table: LINEAR<a name="LINEAR"></a>
|	name	|	kind	|	type	|	link	|
| --- | --- | --- | --- |
|	a	|	variable	|	float	|	X	|
|	b	|	variable	|	float	|	X	|
|	build	|	function	|	[float, float]->LINEAR	|	[table](#LINEAR::build)	|
|	evaluate	|	function	|	[float]->float	|	[table](#LINEAR::evaluate)	|

____

____

____
Table: build<a name="LINEAR::build"></a>
|	name	|	kind	|	type	|	link	|
| --- | --- | --- | --- |
|	A	|	parameter	|	float	|	X	|
|	B	|	parameter	|	float	|	X	|
|	new_function	|	variable	|	LINEAR	|	X	|

____
Table: evaluate<a name="LINEAR::evaluate"></a>
|	name	|	kind	|	type	|	link	|
| --- | --- | --- | --- |
|	x	|	parameter	|	float	|	X	|
|	result	|	variable	|	float	|	X	|

____
Table: QUADRATIC<a name="QUADRATIC"></a>
|	name	|	kind	|	type	|	link	|
| --- | --- | --- | --- |
|	a	|	variable	|	float	|	X	|
|	b	|	variable	|	float	|	X	|
|	c	|	variable	|	float	|	X	|
|	build	|	function	|	[float, float, float]->QUADRATIC	|	[table](#QUADRATIC::build)	|
|	evaluate	|	function	|	[float]->float	|	[table](#QUADRATIC::evaluate)	|

____

____

____

____
Table: build<a name="QUADRATIC::build"></a>
|	name	|	kind	|	type	|	link	|
| --- | --- | --- | --- |
|	A	|	parameter	|	float	|	X	|
|	B	|	parameter	|	float	|	X	|
|	C	|	parameter	|	float	|	X	|
|	new_function	|	variable	|	QUADRATIC	|	X	|

____
Table: evaluate<a name="QUADRATIC::evaluate"></a>
|	name	|	kind	|	type	|	link	|
| --- | --- | --- | --- |
|	x	|	parameter	|	float	|	X	|
|	result	|	variable	|	float	|	X	|

____
Table: main<a name="main"></a>
|	name	|	kind	|	type	|	link	|
| --- | --- | --- | --- |
|	f1	|	variable	|	linear	|	X	|
|	f2	|	variable	|	quadratic	|	X	|
|	counter	|	variable	|	integer	|	X	|

____
