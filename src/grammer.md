program
    : function

function
    : type Identifier '(' ')' '{' statement '}'

type
    : 'int'

statement
    : 'return' expression ';'

expression
    : Integer


### step2

expression
    : unary

unary
    : Integer
    | ('-'|'!'|'~') unary
- 一元运算符是右结合
#### ir

_T0 = 1
_T1 = NEG _T0