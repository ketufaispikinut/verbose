//Merci à Astroide!
A vaut 2
B vaut 2
C vaut 0
EstPremier vaut oui
Tant que oui alors
  C vaut C+1
  Afficher "C: "+C
  Afficher "A.1: "+A
  A vaut A+1
  Afficher "A.2: "+A
  B vaut 1
  EstPremier vaut oui
  Tant que B+1 est plus petit que A alors
    B vaut B+1
    Si (A/B)*B est égal à A alors
      EstPremier vaut non
    Fin
  Fin
  Si EstPremier alors
    Afficher "Nombre premier : "+A
  Fin
Fin