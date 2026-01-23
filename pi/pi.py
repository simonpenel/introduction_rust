import argparse
parser = argparse.ArgumentParser()
parser.add_argument('-i', '--iterations', type=int, required=True, help='nb of iteration')
args = parser.parse_args()
i  = 0
somme = 0
imax = args.iterations
while i < imax :
    val = (4 * i + 1) * (4*i + 3)
    somme = somme + 1.0 / val 
    i += 1;    
print(somme * 8.0);