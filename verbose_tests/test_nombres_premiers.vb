A vaut 2
B vaut 2
EstPremier vaut non
Tant que oui alors
  Afficher "A vaut: "+A
  A vaut A+1
  Afficher "A vaut ([36m A vaut A+1 [0m): "+A
  B vaut 1
  Afficher "A vaut ([36m B vaut 1 [0m): "+A
  EstPremier vaut oui
  Afficher "A vaut ([36m EstPremier vaut oui [0m): "+A
  Tant que B+1 est plus petit que A alors
    Afficher "A vaut ([36m Tant que B+1 est plus petit que A alors [0m): "+A
    B vaut B+1
    Afficher "A vaut ([36m B vaut B+1 [0m): "+A
    Si (A/B)*B est égal à A alors
      Afficher "A vaut ([36m Si (A/B)*B est égal à A alors [0m): "+A
      EstPremier vaut non
      Afficher "A vaut ([36m EstPremier vaut non [0m): "+A
    Fin
    Afficher "A vaut ([36m Fin [35msi A/B*B==A [0m): "+A
  Fin
  Afficher "A vaut ([36m Fin [35mboucle B+1<A [0m): "+A
  Si EstPremier alors
    Afficher "A vaut ([36m Si EstPremier est égal à 1 alors [0m): "+A
    Afficher "Nombre premier : "+A
    Afficher "A vaut ([36m Afficher 'Nombre premier : '+A [0m): "+A
  Fin
  Afficher "[31m???[0m"
  Afficher "A vaut ([36m Fin [35msi EstPremier=1 [0m): "+A
Fin
Afficher "A vaut ([36m Fin [35mdes temps [0m): "+A