i  = 0
somme = 0
imax = 100000000
while i < imax :
    val = (4 * i + 1) * (4*i + 3)
    somme = somme + 1.0 / val 
    i += 1;    
print(somme * 8.0);