### Constraint
1. 只在终结符号移动下标，非终结不移动。
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

### step2.5
```
unary
    : primary
    | ('-'|'~'|'!') unary

primary
    : Integer
    | '(' expression ')'
```
### step3
```
expression
    : additive

additive
    : multiplicative
    | additive ('+'|'-') multiplicative

multiplicative
    : unary
    | multiplicative ('*'|'/'|'%') unary

unary
    : primary
    | ('-'|'~'|'!') unary

primary
    : Integer
    | '(' expression ')'
```

#### step3 消除左递归后
A = Aa | Ab | r
消除后
A = rT
T = aT | bT | <
```
expression
    : additive

additive
    : multiplicative rest
rest
    : ('+'|'-') multiplicative rest
    | <
multiplicative
    : unary rest2
rest2
    : ('*'|'/'|'%') unary rest2
    | <
unary
    : primary
    | ('-'|'~'|'!') unary

primary
    : Integer
    | '(' expression ')'
```