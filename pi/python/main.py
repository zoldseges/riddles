import subprocess

from fractions import Fraction
from math import factorial

directory = "out"
fpath = "{}/out.tex".format(directory)

class PTerm():
    
    def __init__(self, n, x, alpha, power_num, power_denom):
        self.n = n
        self.x = x
        self.alpha = alpha
        self.power_num = power_num
        self.power_denom = power_denom

        self.num = 0
        self.denom = 0
        self.get_num()
        self.get_denom()
        self.get_power()
        self.arrange_fraction()

    def get_num(self):
        self.num = 1
        for i in range(self.n):
            self.num *= (self.alpha - i)

    def get_denom(self):
        self.denom = factorial(self.n)

    def get_power(self):
        power = self.n + Fraction(self.power_num, self.power_denom)
        self.power_num = power.numerator
        self.power_denom = power.denominator

    def arrange_fraction(self):
        num = self.num
        denom = self.denom
        q = Fraction(num, denom)
        self.num = q.numerator
        self.denom = q.denominator

    def print_term(self):
        global fpath
        with open(fpath, "a") as f:
            # fraction
            f.write(r"\frac{")
            f.write("{}".format(self.num))
            f.write(r"}")

            f.write(r"{")
            f.write("{}".format(self.denom))
            f.write(r"}")

            f.write(r"x^\frac{")
            f.write("{}".format(self.power_num))
            f.write(r"}")
            f.write(r"{")
            f.write("{}".format(self.power_denom))
            f.write(r"}")
            
def start():
    global fpath
    with open(fpath, "a") as f:
        f.write(r"\documentclass{article}")
        f.write("\n")
        f.write(r"\begin{document}")
        f.write("\n")
        f.write(r"\[")
        f.write("\n")

def end():
    global fpath
    with open(fpath, "a") as f:
        f.write(r"\]")
        f.write("\n")
        f.write(r"\end{document}")
        f.write("\n")

def body(to_n):
    for n in range(to_n):
        if (n != 0):
            with open(fpath, "a") as f:
                f.write(" - \n")
        PTerm(n, n, Fraction(1, 2), 1, 2).print_term()

if __name__ == "__main__":
    open(fpath,"w").close()

    start()
    body(6)
    end()
    subprocess.run(["pdflatex",
                    "-output-directory={}".format(directory),
                    "-halt-on-error",
                    fpath])
                    
