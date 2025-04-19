
# FRO Calculator #

FRO - fast reliable only.

The calculator is designed to simplify working with simple expressions. It is fast, practical, and beautiful. Unlike other calculators, it speeds up the user's input of the equation or example that needs to be calculated. This is achieved through a universal syntax. We suggest using the keyboard in ENG layout only for inputting data into the calculator.

# Syntax #

1. Missing `*` sign.

    Such input from the user will not panic and will provide the correct answer:
    ```
    >>> 43(23 -  2)
    903
    ```

    Another example:
    ```
    >>> (23 -  2)  (54 - 52)
    42
    ```

    The missing multiplication sign works not only with brackets but also with numbers:
    ```
    >>> 34 5
    170
    ```

    So be careful with such notation:
    ```
    >>> 34 -5
    -170
    ```
    Note that the calculator did not subtract the elements, but multiplied! After all, `-5` is a number.

2. Any number of spaces.

    The calculator ignores all spaces: whether at the beginning, the end, or in the middle. Therefore, the following examples do not panic but give the correct result:
    ```
    >>>      4 5
    20
    >>> 4     5
    20
    >>> 4   5            9
    180
    >>> (4 - 5)     (4)
    -4
    ```

3. Using commands:

    - The command `/help` will display this manual.
    - The command `/end` will terminate the program execution and save your variables and your history.
    - The command `/history` will display the last 10 lines of the table from the columns "Input" "Output". The table does not include any input that resulted in an error instead of a result. You can display all your entries by adding the word 'all' to the command. For example: "/history all". You can specify how many recent records you want to see by adding a number after the command like this: "/history 5".

# Constants and Variables #

1. The calculator has a small number of constants. They are:

    - PI - the number pi.
    - E - the Euler number.
    - c - the speed of light.
    - g - the free fall acceleration.
    - G - the gravitational constant.

2. Constants can be used in your input:

    ```
    >>> PI
    3.141592653589793
    >>> PI * E - 5
    3.539734222673566
    ```

3. You can create a variable by first entering the variable name (by which it will respond) and through the equals sign, its value:
    ```
    >>> a  = 10
    >>> d = a - 3
    >>> a
    10
    >>> d
    7
    ```

4. Variables can be redefined:
    ```
    >>> home = 1
    >>> home = home - 1
    >>> home
    0
    ```

5. Variables can also be used in expressions:
    ```
    >>> (home - a + PI) (23 - 4 -4)
    -267.4778865099981
    ```

# Functions #

1. The calculator has several functions that can be called by writing its name and putting its values in parentheses. For example:
    ```
    >>>sin(2)
    0.03489949670250097
    ```

2. There are several functions in the calculator:

    - sin(degrees)
    - cos(degrees)
    - tg(degrees)
    - ctg(degrees)

    - sqrt(number, from which to compute the square root; base on which the root is calculated)
    - rt(number, from which to compute the square root; base on which the root is calculated)
    - exp(number, which needs to be raised to a power; the power to which the number will be raised)

3. Some functions have optional values. Such functions are "exp", "sqrt", "rt":
    ```
    >>> exp(2)
    4
    >>> exp(2; 4)
    16
    >>> sqrt(9)
    3
    ```
    For such functions, the second value is optional, by default it is equal to "2".

4. Function arguments are flexible and can contain an example:
    ```
    >>> exp(2 - 4)
    4
    >>> exp(2 - 5 - 1; 2 + 2)
    256
    ```

5. Functions can be applied in examples:
    ```
    >>> 123 - 1 - 1 + sin(cos(a))
    121.01718729148051
    >>> exp(sin(a)) + exp(cos(a))
    0.9999999999999999
    ```
