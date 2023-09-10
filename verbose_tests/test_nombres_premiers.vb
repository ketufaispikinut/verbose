A vaut 2
B vaut 2
EstPremier vaut non
Tant que oui alors
  A vaut A+1
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