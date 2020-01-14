pub mod Puissance_4{

    extern crate rust_hepia_lib;

    //Enum
    #[derive(Debug)]
    #[derive(PartialEq, Eq)]
    #[derive(Copy, Clone)]
    pub enum Case {
        Vide,
        Croix,
        Inter,
        Rond
    }

    //Creation de la Grille de jeux
    const SIZE: usize = 8;
    const SIZE2: usize = 12;
    type Grille = [[Case;SIZE];SIZE2];


    //Renvoie la line choisie par le joueur (pos_h)
    pub fn Input_player() -> usize{

        let mut x = false;
        let mut input_num_line:usize = 0;
            while x == false{
                input_num_line = rust_hepia_lib::read_int();
                    if input_num_line < 1 || input_num_line  > 7{
                    println!("The line must be between 1 and 7 !");
                }
                else{
                    x = true;
                }
            }
        return input_num_line;   
    }

    //Output de la Grille
    pub fn Output_Grille(grille: &mut Grille, case1: Case, case2: Case, case3: Case, case4: Case){
        println!("┌─┬─┬─┬─┬─┬─┬─┐"); 
        for i in 0..grille.len() {
            for j in 0..grille[0].len() {
                match grille[i][j] {
                    Case::Vide => print!("| "),
                    Case::Croix => print!("|X"),
                    Case::Inter => print!("├─"),
                    Case::Rond => print!("|O"),
                }
            }
            println!("");
        }
        //println!("└─┴─┴─┴─┴─┴─┴─┘ ");
        println!(" 1 2 3 4 5 6 7");
        println!("");
        println!("*************************************");
    }

    //Check si la line choisie est jouable
    pub fn Check_case_jouable(grille: &mut Grille, pos_h: usize, mut tour_joueur1: &mut bool, caseVide: Case, caseInter: Case, caseJ1: Case, casej2: Case, mut pos_v: &mut usize, mut compteur_tour: &mut i32){
        let mut y = 0;
        let mut w = 0;
        let mut joueur = Case::Croix;

        if  *tour_joueur1 == true{
            joueur = Case::Croix;
        }
        else{
            joueur = Case::Rond;
        }
            loop{
                if y < 11{
                    if grille[10 - y][pos_h - 1] != Case::Croix && grille[10 - y][pos_h - 1] != Case::Rond {
                        *pos_v = 6 - w;  //Récupère la position verticale de la case jouée
                        grille[10 - y][pos_h - 1] = joueur;
                        y = 0;
                        break;       
                    }
                    else{
                        y = y + 2;
                        w = w + 1;
                    }
                    //println!("y {}", y);
                }
                else{
                    *compteur_tour = *compteur_tour - 1;
                    println!("****************");
                    println!("Column full !");
                    y = 0;
                    if *tour_joueur1 == true{
                        *tour_joueur1 = false;
                        println!("player 1 play again.");
                        println!("*******************");
                    }
                    else{
                        *tour_joueur1 = true;
                        println!("player 2 play again.");
                        println!("*******************");
                    }
                    break;
                }
            } 
        Output_Grille(grille, Case::Croix, Case::Inter, Case::Vide, Case::Rond);
    }

    //Creation de la Grille vide
    pub fn Creation_Grille_vide(grille: &mut Grille, caseVide: Case, caseInter: Case, caseJ1: Case, casej2: Case){ 
        for i in 0..12{
            for j in 0..8{
                if i == 1 || i == 3 || i == 5 || i == 7 || i == 9 || i == 11 {
                    if j == 7{
                        grille[i][j] = Case::Vide;
                    }
                    else{
                        grille[i][j] = Case::Inter;
                    }
                }
                else{
                    grille[i][j] = Case::Vide;
                }
            }
            println!("");
        }
        Output_Grille(grille, caseJ1, caseInter, caseVide, casej2);
    }

    //Check si un joueur a gagné
    pub fn Check_jeu_gagnant (grille: &mut Grille, last_pos_h : usize, mut tour_joueur1: &mut bool, caseJ1: Case, casej2: Case, posv: usize, mut partie_finie: &mut bool, case_vide: Case, mut pos_ai_h: &mut usize, mut case_gagnante: &mut bool ){
        let mut compteur = 0; // compteur verifiant une suite de 4 case identique
        let mut pos_v = posv; // position de v de la dernière case
        let mut pos_h = last_pos_h; // position de h de la dernière case
        let mut case_joueur = Case::Croix; // la dernière case jouée par joueur1 ou joueur2 
        let mut joueur = "joueur"; // le nom du joueur qui a joué la dernière case
        let mut limite_diag = 6; // la limite d'une diagonale
        let mut start_pos = 0; // la position de départ d'une diagonale

        if *tour_joueur1{
            *tour_joueur1 = false;
            case_joueur = Case::Croix;
            joueur = "joueur 1";
        }
        else{
            *tour_joueur1 = true;
            case_joueur = Case::Rond;
            joueur = "joueur 2";
        }

        //Check Horizontale
        for i in 0..8{
            if grille[(pos_v-1) * 2][i] == case_joueur{
                compteur = compteur + 1;
                if compteur >= 4{  // joueur a gagné
                    *partie_finie = true;
                    println!("{} win", joueur);
                    compteur = 0;
                    break;
                }
            }
            else{
                compteur = 0;
            }
        }
        //Check Verticale
        for i in 0..6{
            if grille[10 - i*2][pos_h - 1] == case_joueur{
                compteur = compteur + 1;
                if compteur >= 4{
                    *partie_finie = true;
                    println!("{} win", joueur);
                    compteur = 0;
                    break;
                }
            }
            else{
                compteur = 0;
            }
   
        }
        //Calcal limite de la diagonale et start position pour les diagonales positives
        let mut i = 0; // compteur pour la limite des diagonales
        start_pos = (pos_v) + (pos_h);
        if start_pos > 7{
            start_pos = start_pos - 7;
            limite_diag = 7 - start_pos;
        }
        else if start_pos < 7{
            start_pos = 7 - start_pos;
            limite_diag = 7 - start_pos - 1;
        }
        else{
            start_pos = 0;
            limite_diag = 6;
        }
        compteur = 0;
        while i < limite_diag { // boucle diagonales positives
            //start_pos en_bas de la grille
            if pos_v + pos_h >= 7{
                if grille[10 - i*2][start_pos + i] == case_joueur{
                    compteur = compteur + 1;
                    if compteur >= 4{
                        *partie_finie = true;
                        println!("{} win", joueur);
                        compteur = 0;
                        limite_diag = 6;
                        break;
                    }
                }
                else{
                    compteur = 0;
                }
                i = i + 1;
            }
            //start_pos à gauche de la grille
            else if pos_v + pos_h < 7{
                if start_pos >= 3{
                    break;
                }
                if grille[(10 - start_pos*2) - i*2][i] == case_joueur{
                    compteur = compteur + 1;
                    if compteur >= 4{
                        *partie_finie = true;
                        println!("{} win", joueur);
                        compteur = 0;
                        limite_diag = 6;
                        break;
                    }
                }
                else{
                    compteur = 0;
                }
            i = i + 1;
            }
        }

        //Calcal limite de la diagonale et start position pour les diagonales negatives
        limite_diag = 6;
        i = 0;
        if pos_v > pos_h{
            limite_diag = limite_diag - (pos_v - pos_h); 
            start_pos = pos_v - pos_h;
        }
        else if pos_h > pos_v{
            limite_diag = limite_diag - (pos_h - pos_v) + 1;
            start_pos = pos_h - pos_v;
        }
        else{
            start_pos = 0;
            limite_diag = 6;
        }
        compteur = 0;
        while i < limite_diag{  // boucle diagonales negatives

            //start_pos à gauche de la grille
            if pos_v >= pos_h{
                if grille[start_pos*2 + i*2][i] == case_joueur{
                    compteur = compteur + 1;
                    if compteur >= 4{
                        *partie_finie = true;
                        println!("{} win", joueur);
                        compteur = 0;
                        break;
                    }
                }
                else{
                    compteur = 0;
                }
            i = i + 1;
            }
        
        //Partie en_haut
        else if pos_h > pos_v{
             if grille[i*2][start_pos + i] == case_joueur{
                compteur = compteur + 1;
                if compteur >= 4{
                    *partie_finie = true;
                    println!("{} win", joueur);
                    compteur = 0;
                    break;
                }
            }
            else{
                compteur = 0;
            }
        i = i + 1; 
        }
    }
    
    }

    //Renvoie la line choisie par le computer (pos_ai_h)
    pub fn Input_rdm_computer() -> usize{
    let mut rand = rust_hepia_lib::gen(1,8);

    return rand;
    }

    //Check les cases gagnantes pour l'IA
    pub fn IA(grille: &mut Grille, mut pos_ai_h: &mut usize, mut case_gagnante: &mut bool, tour_joueur1: bool, caseJ1: Case, casej2: Case, case_vide: Case, posv: usize, mut pos_ai_v: &mut usize ){
    let mut compteur = 0;
    let mut pos_v = posv;
    let mut case_joueur = Case::Croix;

    if tour_joueur1{
        case_joueur = Case::Croix;
    }
    else{
        case_joueur = Case::Rond;
    }

    //check horizontalement les cases du joueur                                <---------------------------------- #1 HORIZ
    for j in 0..6{
        for i in 0..7{
            if grille[j*2][i] == case_joueur{ 
                compteur = compteur + 1;

                if compteur == 3{   // pour 3 cases de suites ex: _XXX_
                    // println!("{} i c", i);
                    // println!("{} j c", j);

                    if j*2 < 9{  //check si la rangée verticale n'est pas la dernière
                        if i < 6{ //max à droite
                            if grille[(j*2+2)][i + 1] != Case::Vide && grille[(j*2)][i + 1] == Case::Vide{ //check la case en bas à droite n'est pas vide et la suivante à droite vide
                                *pos_ai_h = i + 2;
                                *case_gagnante = true;
                                break;
                            }
                        }
                        if i > 2 {  //min à droite
                            if grille[(j*2+2)][i - 3] != Case::Vide  && grille[(j*2)][i - 3] == Case::Vide{ //check la case en bas à gauche n'est pas vide et la précèdente à gauche vide
                                *pos_ai_h = i - 2;
                                *case_gagnante = true;
                                break;
                            }
                        }

                    }
                    else{ // sinon la rangée verticale est la dernière
                        if i < 6{  //max à droite
                            if grille[(j*2)][i + 1] == Case::Vide{  //check case suivante à droite
                                *pos_ai_h = i + 2;
                                *case_gagnante = true;
                                break;
                            }
                        }
                        if i > 2 {  //min à droite
                            if grille[(j*2)][i - 3] == Case::Vide{  //check case suivante à gauche
                                *pos_ai_h = i - 2;
                                *case_gagnante = true;
                                break;
                            }
                        }
                    }
                } 
                if compteur == 2{  //pour 2 cases joueur de suite suivient d'une case vide puis d'une cases joueur ex: XX_X ou X_XX
                    if j*2 < 9{   //check si la rangée verticale n'est pas la dernière (EN HAUTEUR)
                        if i < 6{  //max à droite
                            if grille[(j*2)][i + 2] == case_joueur && grille[(j*2) + 2][i + 1] != case_vide && grille[(j*2)][i + 1] == case_vide{ // XX_X  EN HAUTEUR
                                *pos_ai_h = i + 2;
                                *case_gagnante = true;
                                break;
                            }
                        }
                        if i > 2 {
                            if grille[(j*2)][i - 3] == case_joueur && grille[(j*2) + 2][i - 2] != case_vide && grille[(j*2)][i - 2] == case_vide{ // X_XX  EN HAUTEUR
                                *pos_ai_h = i - 1;
                                *case_gagnante = true;
                                break;
                            }
                        }
                    }
                    else{ // dernière rangée
                        if i < 6{  //max à droite
                            if grille[(j*2)][i + 2] == case_joueur && grille[(j*2)][i + 1] == case_vide{  // XX_X  
                                *pos_ai_h = i + 2;
                                *case_gagnante = true;
                                break;
                            }
                        }
                        if i > 2 {
                            if grille[(j*2)][i - 3] == case_joueur && grille[(j*2)][i - 2] == case_vide{  // X_XX  
                                *pos_ai_h = i - 1;
                                *case_gagnante = true;
                                break;
                            }
                        }
                    }
                } 
            }
            else{
                compteur = 0;
            }
        }
        compteur = 0;
    }

    //check verticalement les cases du joueur                                   <---------------------------------- #2 VERT
    compteur = 0;
    for j in 0..7{
        for i in 0..6{
            if grille[(10 - i*2)][j] == case_joueur{
                compteur = compteur + 1;
                if compteur == 3{
                    if i < 5{
                        if grille[(10 - i*2) - 2][j] == case_vide{
                            *pos_ai_h = j + 1;
                            *case_gagnante = true;
                            break;
                        }
                    }
                }
            }
            else{
                compteur = 0;
            }
        }
        compteur = 0;
    }

    //check diagonales négatives de gauche (cases verticales 1, 2 et 3 seulement) <------------------------------ #3 DIAG - GAUCHE

    //check si la prochaine case à droite de la diagonale est gagnante
    let mut limite_diago_ia = 6;
    compteur = 0;

    for i in 0..3{
        for j in 0..limite_diago_ia{
            if limite_diago_ia - 3 >= j{ // check si la rangée de la case est plus grande de 2 rangée de la dernière
                if grille[(i*2) + j*2][j] == case_joueur{
                    compteur = compteur + 1;
                    if compteur == 3{    // Pour une diagonale negative à droite XXX_
                        if grille[(i*2) + j*2 +2][j + 1] == case_vide && grille[(i*2) + j*2 +4][j + 1] != case_vide{  //Check si la prochaine case en diagonale est vide et celle juste en bas de la vide est pleine 
                            *pos_ai_h = j + 2;
                            *case_gagnante = true;
                            break;
                        }
                    }
                    if compteur == 2{  // Pour une diagonale negative à droite XX_X
                        if j < 4{
                            if grille[(i*2) + j*2 +2][j + 1] == case_vide && grille[(i*2) + j*2 +4][j + 2] == case_joueur && grille[(i*2) + j*2 +4][j + 1] != case_vide{  //Check si la prochaine case en diagonale est vide et celle juste en bas de la vide est pleine 
                                *pos_ai_h = j + 2;
                                *case_gagnante = true;
                                break;
                            }
                        }
                    }
                }
                else{
                    compteur = 0;
                }
            }
            else{ // si dernière rangée
                if grille[(i*2) + j*2][j] == case_joueur{
                    compteur = compteur + 1;
                    if compteur == 3{
                        if limite_diago_ia - 1 == j{  // check si la case est sur la dernière rangée
                            break;
                        }
                        if grille[(i*2) + j*2 +2][j + 1] == case_vide {  //Check si la prochaine case en diagonale est vide
                            *pos_ai_h = j + 2;
                            *case_gagnante = true;
                            break;
                        }
                    }
                }
                else{
                    compteur = 0;
                }
            }
        }
        limite_diago_ia = limite_diago_ia - 1;
        compteur = 0;
    }

    //check si la prochaine case à gauche de la diagonale est gagnante
    let mut limite_diago_ia = 6;
    compteur = 0;

    for i in 0..3{
        for j in 0..limite_diago_ia{
            if grille[(i*2) + j*2][j] == case_joueur{
                compteur = compteur + 1;
                if j + 2 > 4{ // check si la rangée de la dernière case est plus grande de 2 rangée de la dernière
                    if compteur == 3{   // Pour une diagonale negative  à gauche _XXX
                        if grille[(i*2) + j*2 -6][j  -3] == case_vide && grille[(i*2) + j*2 -4][j -3] != case_vide{  //Check si la prochaine case en diagonale est vide et celle juste en bas de la vide est pleine 
                            *pos_ai_h = j - 2;
                            *case_gagnante = true;
                            break;
                        }
                    }
                    if compteur == 2{  // Pour une diagonale negative à gauche X_XX
                        if j > 1{
                            if grille[(i*2) + j*2 -4][j -2] == case_vide && grille[(i*2) + j*2 -6][j -3] == case_joueur && grille[(i*2) + j*2 -2][j -2] != case_vide{  //Check si la prochaine case en diagonale est vide et celle juste en bas de la vide est pleine 
                                *pos_ai_h = j -1;
                                *case_gagnante = true;
                                break;
                            }
                        }
                    }
                }
            }
            else{
                compteur = 0;
            }
        }
        limite_diago_ia = limite_diago_ia - 1;
        compteur = 0;
    }

    //check diagonales négatives du haut (cases horizontale 2, 3 et 4 seulement)    <------------------------------ #4 DIAG - TOP

    //check si la prochaine case à droite de la diagonale est gagnante
    let mut limite_diago_ia = 6;
    compteur = 0;

    for i in 0..3{
        for j in 0..limite_diago_ia{
            if limite_diago_ia - 3 >= j{ // check si la rangée de la dernière case est plus grande de 2 rangée de la dernière
                if grille[j*2][i + j + 1] == case_joueur{
                    compteur = compteur + 1;
                    if compteur == 3{
                        if grille[j*2 +2][i + j + 1 + 1] == case_vide && grille[j*2 +4][i + j + 1 + 1] != case_vide{  //Check si la prochaine case en diagonale est vide et celle juste en bas de la vide est pleine 
                            *pos_ai_h = i + j + 1 + 2;
                            *case_gagnante = true;
                            break;
                        }
                    }
                    if compteur == 2{  // Pour une diagonale negative du haut XX_X
                        if j < 4{
                            if grille[j*2 +2][i + j + 1 +1] == case_vide && grille[j*2 +4][i + j + 1 +2] == case_joueur && grille[j*2 +4][i + j + 1 +1] != case_vide{  //Check si la prochaine case en diagonale est vide et celle juste en bas de la vide est pleine 
                                *pos_ai_h = i + j + 1 +2;
                                *case_gagnante = true;
                                break;
                            }
                        }
                    }
                }
                else{
                    compteur = 0;
                }
            }
            else{ // si dernière rangée
                if grille[j*2][i + j + 1] == case_joueur{
                    compteur = compteur + 1;
                    if compteur == 3{
                        if limite_diago_ia - 1 == j{  // check si la case est sur la dernière rangée
                            break;
                        }
                        if grille[j*2 +2][i + j + 1 + 1] == case_vide {  //Check si la prochaine case en diagonale est vide
                            *pos_ai_h = i + j + 1 + 2;
                            *case_gagnante = true;
                            break;
                        }
                    }
                    // if compteur == 2{  // Pour une diagonale negative du haut XX_X
                    //     if j < 4{
                    //         if grille[j*2 +2][i + j + 1 +1] == case_vide && grille[j*2 +4][i + j + 1 +2] == case_joueur && grille[j*2 +4][i + j + 1 +1] != case_vide{  //Check si la prochaine case en diagonale est vide et celle juste en bas de la vide est pleine 
                    //             *pos_ai_h = i + j + 1 +2;
                    //             *case_gagnante = true;
                    //             break;
                    //         }
                    //     }
                    // }
                }
                else{
                    compteur = 0;
                }
            }
        }
        limite_diago_ia = limite_diago_ia - 1;
        compteur = 0;
    }

    //check si la prochaine case à gauche de la diagonale est gagnante
    let mut limite_diago_ia = 6;
    compteur = 0;

    for i in 0..3{
        for j in 0..limite_diago_ia{
            if grille[j*2][i + j + 1] == case_joueur{
                compteur = compteur + 1;
                if j + 2 > 4{ // check si la rangée de la dernière case est plus grande de 2 rangée de la dernière
                    if compteur == 3{
                        if grille[j*2 -6][i + j + 1  -3] == case_vide && grille[j*2 -4][i + j + 1 -3] != case_vide{  //Check si la prochaine case en diagonale est vide et celle juste en bas de la vide est pleine 
                            *pos_ai_h = i + j + 1 - 2;
                            *case_gagnante = true;
                            break;
                        }
                    }
                    if compteur == 2{  // Pour une diagonale negative du haut X_XX
                        if j > 1{
                            if grille[j*2 -4][i + j + 1 -2] == case_vide && grille[j*2 -6][i + j + 1 -3] == case_joueur && grille[j*2 -2][i + j + 1 -2] != case_vide{  //Check si la prochaine case en diagonale est vide et celle juste en bas de la vide est pleine 
                                *pos_ai_h = i + j + 1 - 1;
                                *case_gagnante = true;
                                break;
                            }
                        }
                    }
                }
            }
            else{
                compteur = 0;
            }
        }
        limite_diago_ia = limite_diago_ia - 1;
        compteur = 0;
    }

    //check diagonales positives de gauche (cases verticales 6, 5 et 4 seulement)    <---------------------------- #5 DIAG + GAUCHE

    //check si la prochaine case à droite de la diagonale est gagnante
    let mut limite_diago_ia = 6;
    compteur = 0;

    for i in 0..3{
        for j in 0..limite_diago_ia{
            if limite_diago_ia - 2 >= j{
                if grille[(10 - j*2) - i*2][j] == case_joueur{
                    compteur = compteur + 1;
                    if compteur == 3{
                        if grille[(10 - j*2) - i*2 -2][j + 1] == case_vide && grille[(10 - j*2) - i*2][j + 1] != case_vide{  //Check si la prochaine case en diagonale est vide et celle juste en bas de la vide est pleine 
                            *pos_ai_h = j + 2;
                            *case_gagnante = true;
                            break;
                        }
                    }
                    // if compteur == 2{  // Pour une diagonale positives de gauche XX_X
                    //     if j < 4{
                    //         if grille[(10 - j*2) - i*2 -2][j + 1] == case_vide && grille[(10 - j*2) - i*2 -4][j + 2] == case_joueur && grille[(10 - j*2) - i*2 ][j + 1] != case_vide{  //Check si la prochaine case en diagonale est vide et celle juste en bas de la vide est pleine 
                    //             *pos_ai_h = j +2;
                    //             *case_gagnante = true;
                    //             break;
                    //         }
                    //     }
                    // }
                    
                }
                else{
                    compteur = 0;
                }
            }
        }
        limite_diago_ia = limite_diago_ia - 1;
        compteur = 0;
    }

    //check si la prochaine case à gauche de la diagonale est gagnante
    let mut limite_diago_ia = 6;
    compteur = 0;

    for i in 0..3{
        for j in 0..limite_diago_ia{
            if j >= 1{
                if grille[(10 - j*2) - i*2][j] == case_joueur{
                    compteur = compteur + 1;
                    if j + 3 == limite_diago_ia{  //cas spéciale pour la diagonale du milieu (3,4)
                        if compteur == 3{
                                if grille[(10 - j*2) - i*2 +6][j -3] == case_vide{  //Check si la prochaine case en diagonale est vide et celle juste en bas de la vide est pleine 
                                    *pos_ai_h = j -2;
                                    *case_gagnante = true;
                                break;
                            }
                        }
                    }
                    if compteur == 3{
                        if grille[(10 - j*2) - i*2 +6][j -3] == case_vide && grille[(10 - j*2) - i*2 +8][j -3+ 1] != case_vide{  //Check si la prochaine case en diagonale est vide et celle juste en bas de la vide est pleine 
                            *pos_ai_h = j -2;
                            *case_gagnante = true;
                            break;
                        }
                    }
                    if compteur == 2{  // Pour une diagonale positives de gauche X_XX
                        if j > 2{
                            if grille[(10 - j*2) - i*2 +4][j -2] == case_vide && grille[(10 - j*2) - i*2 +6][j -3] == case_joueur && grille[(10 - j*2) - i*2 +6][j -2] != case_vide{  //Check si la prochaine case en diagonale est vide et celle juste en bas de la vide est pleine 
                                *pos_ai_h = j - 1;
                                *case_gagnante = true;
                                break;
                            }
                        }
                    }
                }
                else{
                    compteur = 0;
                }
            }
        }
        limite_diago_ia = limite_diago_ia - 1;
        compteur = 0;
    }

    //check diagonales positives du bas (cases horizontales 2, 3 et 4 seulement)    <---------------------------- #6 DIAG + EN BAS

    //check si la prochaine case à droite de la diagonale est gagnante
    let mut limite_diago_ia = 6;
    compteur = 0;

    for i in 0..3{
        for j in 0..limite_diago_ia{
            if limite_diago_ia - 2 >= j{
                if grille[(10 - j*2)][i + j + 1] == case_joueur{
                    compteur = compteur + 1;
                    if compteur == 3{
                        if grille[(10 - j*2) -2][i + j + 1 + 1] == case_vide && grille[(10 - j*2)][i + j + 1 + 1] != case_vide{  //Check si la prochaine case en diagonale est vide et celle juste en bas de la vide est pleine 
                            *pos_ai_h = i + j + 1 + 2;
                            *case_gagnante = true;
                            break;
                        }
                    }
                    if compteur == 2{  // Pour une diagonale positives du bas XX_X
                        if j < 4{
                            if grille[(10 - j*2) -2][i + j + 1 + 1] == case_vide && grille[(10 - j*2) -4][i + j + 1 + 2] == case_joueur && grille[(10 - j*2) ][i + j + 1 + 1] != case_vide{  //Check si la prochaine case en diagonale est vide et celle juste en bas de la vide est pleine 
                                *pos_ai_h = i + j + 1  +2;
                                *case_gagnante = true;
                                break;
                            }
                        }
                    }
                }
                else{
                    compteur = 0;
                }
            }
        }
        limite_diago_ia = limite_diago_ia - 1;
        compteur = 0;
    }

    //check si la prochaine case à gauche de la diagonale est gagnante (pour l'avant dernière rangée)
    let mut limite_diago_ia = 6;
    compteur = 0;

    for i in 0..3{
        for j in 0..limite_diago_ia{
            if j >= 1 && j < 4{       //Check que la rangée est l'avant dernière et suit une suite de 3
                if grille[(10 - j*2)][i + j + 1] == case_joueur{
                    compteur = compteur + 1;
                    //println!("{} root 1..",  compteur);
                    if compteur == 3{
                        if grille[(10 - j*2) +6][i + j + 1 -3] == case_vide{  //Check si la prochaine case en diagonale est vide et celle juste en bas de la vide est pleine 
                            *pos_ai_h = i + j + 1 -2;
                            *case_gagnante = true;
                            break;
                        }
                    }
                }
                else{
                    compteur = 0;
                }
            }
        }
        limite_diago_ia = limite_diago_ia - 1;
        compteur = 0;
    }

    //check si la prochaine case à gauche de la diagonale est gagnante (pour une rangée plus grande que l'avant dernière)
    let mut limite_diago_ia = 6;
    compteur = 0;

    for i in 0..3{
        for j in 0..limite_diago_ia{
            if j >= 2{             //Check que la rangée est plus grande l'avant dernière
                if grille[(10 - j*2) ][i + j + 1] == case_joueur{
                    //println!("{} root 2..",  compteur); 
                    compteur = compteur + 1;
                    if compteur == 3{
                        if grille[(10 - j*2) +6][i + j + 1 -3] == case_vide && grille[(10 - j*2)  +8][i + j + 1 -3] != case_vide{  //Check si la prochaine case en diagonale est vide et celle juste en bas de la vide est pleine 
                            *pos_ai_h = i + j + 1 -2;
                            *case_gagnante = true;
                            break;
                        }
                    }
                }
                else{
                    compteur = 0;
                }
            }
        }
        limite_diago_ia = limite_diago_ia - 1;
        compteur = 0;
    }
    
    }

    //Check si la rangée a une seule case entre 3 et 5 pour une suite _XXX_  
    pub fn Check_centre(grille: &mut Grille, mut pos_ai_h: &mut usize, mut case_gagnante: &mut bool, tour_joueur1: bool, caseJ1: Case, casej2: Case, case_vide: Case, posv: usize, mut pos_ai_v: &mut usize  ){
    let mut compteur = 0;
    let mut pos_v = posv;
    let mut case_joueur = Case::Croix;

    if tour_joueur1{
        case_joueur = Case::Croix;
    }
    else{
        case_joueur = Case::Rond;
    }

    for i in 0..6{
        for j in 0..7{
            if grille[6][3] == case_vide && grille[6][4] == case_vide{
                if j < 2 || j > 4{
                    if grille[i*2][0] == case_vide || grille[i*2][0] != case_vide && grille[i*2][1] == case_vide && grille[i*2][6] == case_vide || grille[i*2][6] != case_vide && grille[i*2][5] == case_vide{
                    compteur = compteur + 1;
                    if compteur == 4{
                        if grille[i*2][2] == case_joueur && grille[i*2][3] == case_joueur{
                            *pos_ai_h = 5;
                            *case_gagnante = true;
                            break;
                        }
                        else if grille[i*2][3] == case_joueur && grille[i*2][4] == case_joueur{
                            *pos_ai_h = 3;
                            *case_gagnante = true;
                            break;
                        }
                        else if grille[i*2][2] == case_joueur {
                            *pos_ai_h = 4;
                            *case_gagnante = true;
                            break;
                        }
                        else if grille[i*2][3] == case_joueur{
                            *pos_ai_h = 5;
                            *case_gagnante = true;
                            break;
                        }
                        else if grille[i*2][4] == case_joueur{
                            *pos_ai_h = 4;
                            *case_gagnante = true;
                            break;
                        }
                    }
                }
                else{
                    compteur = 0;
                }
                }
            }
        }
        compteur = 0;
    }
    }

    //Menu du jeu
    pub fn Menu(){
    use std::time::Duration;
    use std::thread;

    let mut grille: Grille = [[Case::Vide;SIZE];SIZE2];

    let joueur1 = Case::Croix;
    let joueur2 = Case::Rond;
    let case_vide = Case::Vide;
    let case_inter = Case::Inter;

    let mut pos_v = 0; // position v de la dernière case jouée
    let mut pos_h = 0; //Input du joueur et position h de la dernière case jouée
    let mut pos_ai_h = 0;
    let mut pos_ai_v = 0;

    let mut tour_joueur1 = true;
    let mut partie_finie = false;
    let mut case_gagnante = false;

    let mut Choix_menu = 1;
    let mut compteur_tour = 0;

    loop{  //LOOP DES MODES DU JEU
        print!("{}[2J", 27 as char);

        println!("        .%%%%%...%%..%%..%%%%%%...%%%%....%%%%....%%%%...%%..%%...%%%%...%%%%%%..........%%..%%.
        .%%..%%..%%..%%....%%....%%......%%......%%..%%..%%%.%%..%%..%%..%%..............%%..%%.
        .%%%%%...%%..%%....%%.....%%%%....%%%%...%%%%%%..%%.%%%..%%......%%%%............%%%%%%.
        .%%......%%..%%....%%........%%......%%..%%..%%..%%..%%..%%..%%..%%..................%%.
        .%%.......%%%%...%%%%%%...%%%%....%%%%...%%..%%..%%..%%...%%%%...%%%%%%..............%%.
        ........................................................................................");
        println!("");
        println!("                                     1 -> Play       2 -> Quit");
        println!("");
        Choix_menu = rust_hepia_lib::read_int();
        print!("{}[2J", 27 as char);

        if Choix_menu == 2 {

        }
        else{
            println!("Choose game mode");
            println!("");
            println!("1 -> Player vs player"); 
            println!("2 -> Player vs Computer(easy)"); 
            println!("3 -> Player vs computer(hard)"); 
            println!("");
            Choix_menu = rust_hepia_lib::read_int();
            print!("{}[2J", 27 as char);

            if Choix_menu == 1{  //                                             <----------------------- Joueur vs joueur
                partie_finie = false;
                Creation_Grille_vide(&mut grille, case_vide, case_inter, joueur1, joueur2);
                loop{
                    compteur_tour = compteur_tour + 1;
                    if compteur_tour == 43{
                        partie_finie = true;
                        println!("Draw");
                        compteur_tour = 0;
                    }
                    if partie_finie{
                        compteur_tour = 0;
                        println!("**********************");
                        println!("1 -> Menu    2-> Quit");
                        println!("");
                        Choix_menu = rust_hepia_lib::read_int();
                        if Choix_menu == 2{

                        }
                        else if Choix_menu == 1{
                            break;
                        }
                    }
                    pos_h = Input_player();
                    print!("{}[2J", 27 as char);

                    Check_case_jouable(&mut grille, pos_h, &mut tour_joueur1, case_vide, case_inter, joueur1, joueur2, &mut pos_v, &mut compteur_tour);
                    Check_jeu_gagnant(&mut grille, pos_h, &mut tour_joueur1, joueur1, joueur2, pos_v, &mut partie_finie, case_vide, &mut pos_ai_h, &mut case_gagnante);
                }       
            }
            else if Choix_menu == 2{  //                                            <----------------------- Joueur vs Computer(easy)
                partie_finie = false;
                Creation_Grille_vide(&mut grille, case_vide, case_inter, joueur1, joueur2);
                loop{
                    compteur_tour = compteur_tour + 1;
                    if compteur_tour == 43{
                        partie_finie = true;
                        println!("Draw");
                        compteur_tour = 0;
                    }
                    if partie_finie{
                        compteur_tour = 0;
                        println!("**********************");
                        println!("1 -> Menu    2-> Quit");
                        println!("");
                        Choix_menu = rust_hepia_lib::read_int();
                        if Choix_menu == 2{
                        
                        }
                        else if Choix_menu == 1{
                            break;
                        }
                    }
                    if tour_joueur1{  //Joueur
                        pos_h = Input_player();
                        print!("{}[2J", 27 as char);
                    }
                    else{  //Computer
                        pos_h = Input_rdm_computer();
                        thread::sleep(Duration::from_secs(1));
                        print!("{}[2J", 27 as char);
                    }
                
                Check_case_jouable(&mut grille, pos_h, &mut tour_joueur1, case_vide, case_inter, joueur1, joueur2, &mut pos_v,  &mut compteur_tour);
                Check_jeu_gagnant(&mut grille, pos_h, &mut tour_joueur1, joueur1, joueur2, pos_v, &mut partie_finie, case_vide, &mut pos_ai_h, &mut case_gagnante);
                }
            }
            else if Choix_menu == 3{  //                                            <-----------------------  Joueur vs Computer(hard)
                partie_finie = false;
                Creation_Grille_vide(&mut grille, case_vide, case_inter, joueur1, joueur2);
                loop{
                    compteur_tour = compteur_tour + 1;
                    if compteur_tour == 43{
                        partie_finie = true;
                        println!("Draw");
                        compteur_tour = 0;
                    }
                    if partie_finie{
                        compteur_tour = 0;
                        println!("**********************");
                        println!("1 -> Menu    2-> Quit");
                        println!("");
                        Choix_menu = rust_hepia_lib::read_int();
                        if Choix_menu == 2{
                        
                        }
                        else if Choix_menu == 1{
                            break;
                        }
                    }
                    if tour_joueur1{  //Joueur
                        pos_h = Input_player();
                        print!("{}[2J", 27 as char);
                    }
                    else{  //IA
                         // IA trouve une case gagnante pour lui
                        IA(&mut grille, &mut pos_ai_h, &mut case_gagnante, tour_joueur1, joueur1, joueur2, case_vide, pos_v, &mut pos_ai_v);

                        if case_gagnante{ //Si IA a trouvé case gagnante pour lui
                            pos_h = pos_ai_h;
                            case_gagnante = false;
                        }
                        else{  //Si pas de case gagnante pour lui
                            //IA trouve une case gagnante du joueur
                            tour_joueur1 = !tour_joueur1;  //#1
                            IA(&mut grille, &mut pos_ai_h, &mut case_gagnante, tour_joueur1, joueur1, joueur2, case_vide, pos_v, &mut pos_ai_v );

                            if case_gagnante{  //Si IA a trouvé case gagnante pour le joueur
                                pos_h = pos_ai_h;
                                //println!("{} pos_ai_h ", pos_ai_h);
                                case_gagnante = false;
                            }
                            else{  ////Si IA ne trouve pas de case gagnante pour lui et pour le joueur
                                //Check une posibilitée de faire une suite au centre pour lui
                                tour_joueur1 = !tour_joueur1; //#2
                                Check_centre(&mut grille, &mut pos_ai_h, &mut case_gagnante, tour_joueur1, joueur1, joueur2, case_vide, pos_v, &mut pos_ai_v);

                                if case_gagnante{
                                    pos_h = pos_ai_h;
                                    case_gagnante = false;
                                }
                                else{ //Check une posibilitée de faire une suite au centre pour le joueur
                                    tour_joueur1 = !tour_joueur1; //#3
                                    Check_centre(&mut grille, &mut pos_ai_h, &mut case_gagnante, tour_joueur1, joueur1, joueur2, case_vide, pos_v, &mut pos_ai_v);

                                    if case_gagnante{
                                        pos_h = pos_ai_h;
                                        case_gagnante = false;
                                    }
                                    else{  // aucune posibilitée... random
                                        pos_h = Input_rdm_computer();
                                        case_gagnante = false;
                                    }
                                    tour_joueur1 = !tour_joueur1;  //#3
                                }
                                tour_joueur1 = !tour_joueur1;  //#2
                            }

                            tour_joueur1 = !tour_joueur1;  //#1
                        }
                        thread::sleep(Duration::from_secs(1));
                        print!("{}[2J", 27 as char);
                    }
                    Check_case_jouable(&mut grille, pos_h, &mut tour_joueur1, case_vide, case_inter, joueur1, joueur2, &mut pos_v ,&mut compteur_tour);
                    Check_jeu_gagnant(&mut grille, pos_h, &mut tour_joueur1, joueur1, joueur2, pos_v, &mut partie_finie, case_vide, &mut pos_ai_h, &mut case_gagnante);
                }
            }
        }
    }
    }
}
