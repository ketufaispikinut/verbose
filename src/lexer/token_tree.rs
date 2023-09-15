use std::collections::HashMap;

use super::Tokens;

pub struct TokenTree {
    pub map_firstchar: HashMap<char, Vec<(String, Tokens)>>,
}

impl TokenTree {
    pub fn new() -> TokenTree {
        let mut d = TokenTree {
            map_firstchar: HashMap::new(),
        };
        d.map_firstchar
            .insert('v', vec![(String::from("vaut"), Tokens::ASSIGN)]);
        d.map_firstchar.insert(
            's',
            vec![
                (String::from("sinon"), Tokens::ELSE),
                (String::from("si"), Tokens::IF),
            ],
        );

        d.map_firstchar.insert(
            //é
            'a',
            vec![
                (String::from("alors"), Tokens::BEGIN),
                (String::from("afficher"), Tokens::PRINT),
                (String::from("affiche"), Tokens::PRINT), //)
                (String::from("avec"), Tokens::PARAM),    //fficher//PRINT
                (String::from("ajouter"),Tokens::ADD_L)
            ],
        );
        d.map_firstchar.insert('à', vec![(String::from("à"),Tokens::ADD_R)]);//k//v//L
        d.map_firstchar
            .insert('f', vec![(String::from("fin"), Tokens::END)]);
        d.map_firstchar.insert(
            'e',
            vec![
                (String::from("et"), Tokens::COMMA),
                (String::from("est égal à"), Tokens::EQUAL),
                (
                    String::from("est plus grand que ou égal à"),
                    Tokens::GREATER_EQ,
                ), //égal à//EQUAL
                (String::from("est plus grand que"), Tokens::GREATER), //égal à//EQUAL
                (
                    String::from("est plus petit que ou égal à"),
                    Tokens::SMALLER_EQ,
                ), //égal à//EQUAL
                (String::from("est plus petit que"), Tokens::SMALLER), //égal à//EQUAL//grand//GREATER
            ],
        );
        d.map_firstchar.insert(
            'n',
            vec![
                (String::from("n'est pas égal a"), Tokens::NOTEQUAL),
                (String::from("non"), Tokens::CONST),
            ], //'est pas egal a//NOTEQUAL
        );
        d.map_firstchar.insert(
            'é',vec![
                (String::from("élément dans"),Tokens::INDEX),
                (String::from("élément de"),Tokens::INDEX)//ans
            ]
        );
        d.map_firstchar //e//est egal a//EQUAL
            .insert('p', vec![(String::from("procedure"), Tokens::PROCE)]);
        d.map_firstchar //e//est egal a//EQUAL
            .insert('o', vec![(String::from("oui"), Tokens::CONST)]); //p//PROCE//procedure

        d.map_firstchar //e//est egal a//EQUAL
            .insert('t', vec![(String::from("tant que"), Tokens::LOOP)]); //p//PROCE//procedure//o
                                                                          //  d.map_firstchar//oui//CONST
                                                                          //      .insert('a', vec![(String::from("vaut"), Tokens::ASSIGN)]);
                                                                          //
        d.map_firstchar //e//est egal a//EQUAL//t//tant que//LOOP
            .insert('l', vec![(String::from("le temps"), Tokens::CLOCK)]); //p//PROCE//procedure//o
        d
    }
}
