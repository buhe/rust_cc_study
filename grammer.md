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
```
A = Aa | Ab | r
消除后
A = rT
T = aT | bT | <
```
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
#### ast
```
expression
    : unary
    | expression + expression
    | expression * expression

```
#### ir
```
_T0 = 1
_T1 = 3
_T2 = ADD _T0, _T1
_T3 = SUB _T0, _T1
_T4 = MUL _T0, _T1
_T5 = DIV _T0, _T1
_T6 = MOD _T0, _T1
```

### step4
```
expression
    : logical_or

logical_or
    : logical_and
    | logical_or '||' logical_and

logical_and
    : equality
    | logical_and '&&' equality

equality
    : relational
    | equality ('=='|'!=') relational

relational
    : additive
    | relational ('<'|'>'|'<='|'>=') additive
```
#### step4 消除左递归
```
A = Aa | Ab | r
消除后
A = rT
T = aT | bT | <
```
```
logical_or
    : logical_and rest3

rest3
    : '||' logical_and rest3
    | <

logical_and
    : equality rest4

rest4
    : '&&' equality rest4
    | <

equality
    : relational rest5

rest5
    : ('=='|'!=') relational rest5
    | <

relational
    : additive rest6

rest6
    : ('<'|'>'|'<='|'>=') additive rest6
    | rest6
```
#### ir
```
_T0 = 1
_T1 = 3
_T2 = GT _T0, _T1
_T3 = GET _T0, _T1
_T4 = LT _T0, _T1
_T5 = LET _T0, _T1
_T6 = AND _T0, _T1
_T7 = OR _T0, _T1
_T8 = EQUAL _T0, _T1
_T9 = NOT_EQUAL _T0, _T1
```
#### asm
```

```

### step5
```
program
    : function

function
    : type Identifier '(' ')' '{' statement* '}'

type
    : 'int'

statement
    : 'return' expression ';'
    | expression? ';'
    | declaration

declaration
    : type Identifier ('=' expression)? ';'

expression
    : assignment

assignment
    : logical_or
    | Identifier '=' expression

logical_or
    : logical_and
    | logical_or '||' logical_and

logical_and
    : equality
    | logical_and '&&' equality

equality
    : relational
    | equality ('=='|'!=') relational

relational
    : additive
    | relational ('<'|'>'|'<='|'>=') additive

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
    | Identifier
```

### step6
```
program
    : function

function
    : type Identifier '(' ')' '{' block_item* '}'

type
    : 'int'

block_item
    : statement
    | declaration

statement
    : 'return' expression ';'
    | expression? ';'
    | 'if' '(' expression ')' statement ('else' statement)?

declaration
    : type Identifier ('=' expression)? ';'

expression
    : assignment

assignment
    : conditional
    | Identifier '=' expression

conditional
    : logical_or
    | logical_or '?' expression ':' conditional

logical_or
    : logical_and
    | logical_or '||' logical_and

logical_and
    : equality
    | logical_and '&&' equality

equality
    : relational
    | equality ('=='|'!=') relational

relational
    : additive
    | relational ('<'|'>'|'<='|'>=') additive

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
    | Identifier
```
#### ir
```
main:
    _T1 = 1
    _T0 = _T1
    BEQZ _T0, _L1
    _T2 = 2
    _T0 = _T2
    JUMP _L2
_L1:
    _T3 = 3
    _T0 = _T3
_L2:
    return _T0
```
#### asm
```
step6:             # RISC-V 汇编标签
    beqz t1, step6 # 如果t1为0，跳转到 step6 标签处
    j step6        # 无条件跳转到 step6 标签处
```
### step7
```
function
    : type Identifier '(' ')' compound_statement

compound_statement
    : '{' block_item* '}'

statement
    : 'return' expression ';'
    | expression? ';'
    | 'if' '(' expression ')' statement ('else' statement)?
    | compound_statement
```

### 控制流图分析

ref: https://decaf-lang.github.io/minidecaf-tutorial/docs/step7/dataflow.html

#### 构建

### step8
```
statement
    : 'return' expression ';'
    | expression? ';'
    | 'if' '(' expression ')' statement ('else' statement)?
    | compound_statement

    | 'for' '(' expression? ';' expression? ';' expression? ')' statement
    | 'for' '(' declaration expression? ';' expression? ')' statement
    | 'while' '(' expression ')' statement
    | 'do' statement 'while' '(' expression ')' ';'
    | 'break' ';'
    | 'continue' ';'
```
for 语句需要提取公因式
#### TAC
```
    _T1 = 0
    _T0 = _T1                 # int i = 0;
_L1:                          # begin label
    _T2 = 5
    _T3 = LT _T0, _T2
    BEQZ _T3, _L3              # i < 5;
    JUMP _L3                   # 循环体
_L2:                          # loop label
    _T4 = 1
    _T5 = ADD _T0, _T4
    _T0 = _T5                 # i = i + 1;
    JUMP _L1
_L3:                          # break label
    # 后续指令 ...
```
### step9
```
program
    : function*

function
    : type Identifier '(' parameter_list ')' compound_statement

type
    : 'int'

parameter_list
    : (type Identifier (',' type Identifier)*)?

compound_statement
    : '{' block_item* '}'

block_item
    : statement
    | declaration

statement
    : 'return' expression ';'
    | expression? ';'
    | 'if' '(' expression ')' statement ('else' statement)?
    | compound_statement
    | 'for' '(' expression? ';' expression? ';' expression? ')' statement
    | 'for' '(' declaration expression? ';' expression? ')' statement
    | 'while' '(' expression ')' statement
    | 'do' statement 'while' '(' expression ')' ';'
    | 'break' ';'
    | 'continue' ';'

declaration
    : type Identifier ('=' expression)? ';'

expression_list
    : (expression (',' expression)*)?

expression
    : assignment

assignment
    : conditional
    | Identifier '=' expression

conditional
    : logical_or
    | logical_or '?' expression ':' conditional

logical_or
    : logical_and
    | logical_or '||' logical_and

logical_and
    : equality
    | logical_and '&&' equality

equality
    : relational
    | equality ('=='|'!=') relational

relational
    : additive
    | relational ('<'|'>'|'<='|'>=') additive

additive
    : multiplicative
    | additive ('+'|'-') multiplicative

multiplicative
    : unary
    | multiplicative ('*'|'/'|'%') unary

unary
    : postfix
    | ('-'|'~'|'!') unary

postfix
    : primary
    | Identifier '(' expression_list ')'

primary
    : Integer
    | '(' expression ')'
    | Identifier
```
#### 消除左公因式
```
unary
    : postfix
    | ('-'|'~'|'!') unary

postfix
    : primary
    | Identifier '(' expression_list ')'

primary
    : Integer
    | '(' expression ')'
    | Identifier
```
```
unary
    | ('-'|'~'|'!') unary
    | Identifier '(' expression_list ')'
    : Integer
    | '(' expression ')'
    | Identifier
```
- A: ay | ab
  A: aM
  M: y | b

```
unary
    | ('-'|'~'|'!') unary
    | Identifier rest
    : Integer
    | '(' expression ')'

rest
    '(' expression_list ')' | <
```
#### TAC
```
func:
    _T2 = ADD _T0, _T1
    return _T2        # 参数 x 和 y 分别对应 _T0, _T1
main:
    _T0 = 1
    PARAM _T0         # 将 _T0 的值作为参数 x
    _T1 = 2
    PARAM _T1         # 将 _T1 的值作为参数 y
    _T3 = CALL func   # 调用函数
    return _T3
```
#### asm
```
    .text
    .global main

func:
    # start of prologue
    addi sp, sp, -56
    # end of prologue

    # start of body
    sw a0, 0(sp)
    sw a1, 4(sp)
    lw t0, 0(sp)
    lw t1, 4(sp)
    add t2, t0, t1
    mv t0, t2
    mv a0, t0
    j func_exit
    # end of body

func_exit:
    # start of epilogue
    addi sp, sp, 56
    # end of epilogue

    ret

main:
    # start of prologue
    addi sp, sp, -56
    sw ra, 52(sp)
    # end of prologue

    # start of body
    li t0, 1
    li t1, 2
    mv a0, t0
    mv a1, t1
    call func
    mv t0, a0
    mv a0, t0
    j main_exit
    # end of body

main_exit:
    # start of epilogue
    lw ra, 52(sp)
    addi sp, sp, 56
    # end of epilogue

    ret
```

#### 调用约定

![reg](https://tva1.sinaimg.cn/large/008i3skNgy1gy41ar3j7xj30fl0e3jsf.jpg)

使用 gcc 的约定，为什么要用约定呢？有了约定就知道从 a0,a1 等寄存器加载参数。为了实现简化，第一版 bugu-lang 实现暂定不传超过 8 个参数，也就是不通过内存来传递参数。



外层函数的寄存器保存在外层函数栈里。