from typing import Tuple


def swap_vars(var_1: int, var_2: int) -> Tuple[int, int]:
    """Swap two variables

    Args:
        var_1 (int): first variable
        var_2 (int): second variable
    """

    print(f"Variable two is {var_2}")
    var_2 = var_1
    var_1 = int(input("What was the number again?"))

    return (var_1, var_2)


def fizz_buzz(number: int):
    """A program that prints the numbers from 1 to 100. But for multiples of three print “Fizz” instead of the number and for the multiples of five print “Buzz”. For numbers which are multiples of both three and five print “FizzBuzz”.

    Args:
        number (int): Current number
    """
    if (number%3 ==0) and (number%5 == 0):
        print("FizzBuzz")
    elif  number%3 == 0:
        print("Fizz")
    elif number%5 == 0:
        print("Buzz")
    else:
        print(number)


if __name__=="__main__":

    print(swap_vars(3, 6))

    for n in range(1,101):
        fizz_buzz(n)
