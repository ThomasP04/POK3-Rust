#[derive(Debug)]

struct Tableau<T> {
    elements: Vec<T>,
    taille: usize,
    capacite: usize,
}

impl<T> Tableau<T> {
    fn nouveau(capacite: usize) -> Self {
        let elements = Vec::with_capacity(capacite);
        Tableau {
            elements,
            taille: 0,
            capacite,
        }
    }
  
    fn ajouter_element(&mut self, element : T) {
        if self.taille == self.capacite {
            panic!("Le tableau n'a plus de place");
        }
        self.elements.push(element);
        self.taille += 1;
    }

    fn supprimer_element(&mut self){
        if self.taille == 0 {
            panic!("Le tableau est vide");
        }
        self.elements.pop();
        self.taille-=1;
    }

}

fn affiche_element<E>(tableau: &Tableau<E>, i: usize) -> &E {
    if i >= tableau.taille {
        panic!("Attention, l'indice sélectionné est plus grand que le tableau.");
    }
    &tableau.elements[i]
}

fn somme_tableau(tab1 : Tableau<i32>, tab2 : Tableau<i32>) -> Tableau<i32> {        
    let mut i: usize = tab1.capacite;
    if tab1.capacite<tab2.capacite{
        i=tab2.taille;
    }
    let mut tab_final :Tableau<i32>= Tableau::nouveau(i);

    let mut k=tab1.taille;
    if tab1.taille<tab2.taille{
        k=tab2.taille;
    }

    for j in 0..k {
        let mut elem1=0;
        let mut elem2=0;
        if j < tab1.taille {
            elem1=tab1.elements[j]
        };
        if j < tab2.taille {
            elem2=tab2.elements[j]
        };
        tab_final.ajouter_element(elem1 + elem2);
    }
    tab_final
}

fn main() {
    let mut tab: Tableau<i32> = Tableau::nouveau(10);
    println!("{:?}", tab);
    tab.ajouter_element(1);
    println!("{:?}", tab);

    let mut T = Tableau {
        elements:vec![1,2,3],
        taille: 3,
        capacite: 10,
    };

    println!("{:?}", somme_tableau(tab, T));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nouveau() {
        let tab: Tableau<i32> = Tableau::nouveau(10);
        assert_eq!(tab.capacite, 10);
        assert_eq!(tab.taille, 0);
        assert_eq!(tab.elements.len(), 0);
    }

    #[test]
    fn test_ajout() {
        let mut T = Tableau {
            elements:vec![1,2,3],
            taille: 3,
            capacite: 10,
        };

        T.ajouter_element(4);
        assert_eq!(T.taille, 4);
        assert_eq!(T.elements, vec![1,2,3,4])
    }

    #[test]
    fn test_supprime() {
        let mut T = Tableau {
            elements:vec![1,2,3],
            taille: 3,
            capacite: 10,
        };

        T.supprimer_element();
        assert_eq!(T.taille, 2);
        assert_eq!(T.elements, vec![1,2])
    }

    #[test]
    fn test_lire_element() {
        let mut T = Tableau {
            elements:vec![1,2,3],
            taille: 3,
            capacite: 10,
        };
        assert_eq!(affiche_element(&T, 2), &3);
    }
}
