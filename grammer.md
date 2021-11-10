### step1
```
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
```

### step2
```
expression
    : unary

unary
    : Integer
    | ('-'|'!'|'~') unary
```
- 一元运算符是右结合. 在 IR 阶段实现
#### ir

_T0 = 1
_T1 = NEG _T0