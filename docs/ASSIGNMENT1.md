# Assignment 1 Document
___
## Atomic lexical elements of the language
| Element  	|    	| Expression                         	|
|----------	|-----	|------------------------------------	|
| **id**    | ::= 	| letter alphanum*                   	|
| alphanum 	| ::= 	| letter &#124; digit &#124; _          |
|**integer**| ::= 	| nonzero digit* &#124; 0               |
| **float** | ::= 	| integer fraction [**e**[+&#124;−] integer]|
| fraction 	| ::= 	| .digit* nonzero &#124; **.0**         |
| letter   	| ::= 	| **a..z &#124; A..Z**                  |
| digit    	| ::= 	| **0..9**                               	|
| nonzero  	| ::= 	| **1..9**                               	|
| **string**| ::= 	| “ alphanum “                       	|
| character | ::=   | alphanum | space
## Operators, punctuation and reserved words
|	|	|	|	|	|	|	||
|----	|---	|----	|---	|----	|---------	|---------	|----------	|
| == 	| + 	| &#124;| ( 	| ;  	| if      	| public  	| read     	|
| <> 	| - 	| &  	| ) 	| ,  	| then    	| private 	| write    	|
| <  	| * 	| !  	| { 	| .  	| else    	| func    	| return   	|
| \>  	| / 	| ?  	| } 	| :  	| integer 	| var     	| main     	|
| <= 	| = 	|    	| \[ 	| :: 	| float   	| class   	| inherits 	|
| >= 	|   	|    	| ] 	| "  	| string  	| while   	| break    	|
|    	|   	|    	|   	|    	| void    	|         	| continue 	|
## Comments
  - Block comments start with /* and end with */ and may span over multiple lines.
  - Inline comments start with // and end with the end of the line they appear in.
___


## Lexical Specification
 - **id**: [a..zA..Z]([a..zA..Z] | [0..9] | _ )*
 - **integer**: ([1..9]\([0..9])*) | **0**
 - **string**: "([a..zA..Z] | [0..9] | _ | space)*"
 - For operators, punctuation and reserved keywords: match as is.
 - Single line comments: //.? 
    - This is assuming that the "." (dot) operator matches everything but newlines
 - Multiline  comments: /\*.?\*/