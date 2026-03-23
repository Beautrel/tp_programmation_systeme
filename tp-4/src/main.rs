#[derive(Debug, Clone, PartialEq)]
enum EtatProcessus {
    Prêt,
    EnExécution { cpu_id: u8 },
    Bloqué { raison: String },
    Terminé { code_retour: i32 },
    Zombie,
}

#[derive(Debug, Clone)]
enum Priorité {
    TrèsFaible,
    Faible,
    Normale,
    Haute,
    TrèsHaute,
    Temps_Réel(u8), // niveau 0-99
}

#[derive(Debug)]
struct Processus {
    pid: u32,
    nom: String,
    état: EtatProcessus,
    priorité: Priorité,
    mémoire_ko: u64,
    pid_parent: Option<u32>,
}

#[derive(Debug)]
struct GestionnaireProcessus {
    processus: Vec<Processus>,
    prochain_pid: u32,
}

impl GestionnaireProcessus {
    // Crée un nouveau gestionnaire
    fn nouveau() -> Self {
        GestionnaireProcessus {
            processus: Vec::new(),
            prochain_pid: 1,
        }
    }

    // Créer un processus et retourne son PID
    fn créer_processus(
        &mut self,
        nom: String,
        priorité: Priorité,
        mémoire_ko: u64,
        pid_parent: Option<u32>,
    ) -> u32 {
        let pid = self.prochain_pid;
        let proc = Processus {
            pid,
            nom,
            état: EtatProcessus::Prêt,
            priorité,
            mémoire_ko,
            pid_parent,
        };
        self.processus.push(proc);
        self.prochain_pid += 1;
        pid
    }

    // Trouver un processus par PID
    fn trouver(&self, pid: u32) -> Option<&Processus> {
        self.processus.iter().find(|p| p.pid == pid)
    }

    // Changer l'état d'un processus
    fn changer_état(&mut self, pid: u32, nouvel_état: EtatProcessus) -> Result<(), String> {
        match self.processus.iter_mut().find(|p| p.pid == pid) {
            Some(proc) => {
                proc.état = nouvel_état;
                Ok(())
            }
            None => Err(format!("PID {} introuvable", pid)),
        }
    }

    // Mémoire totale utilisée
    fn mémoire_totale_utilisée(&self) -> u64 {
        self.processus.iter().map(|p| p.mémoire_ko).sum()
    }

    // Liste des processus par état
    fn processus_par_état(&self, état: &EtatProcessus) -> Vec<&Processus> {
        self.processus
            .iter()
            .filter(|p| &p.état == état)
            .collect()
    }

    // Tuer un processus : change son état en Terminé { code_retour: 0 }
    fn tuer_processus(&mut self, pid: u32) -> Result<i32, String> {
        match self.processus.iter_mut().find(|p| p.pid == pid) {
            Some(proc) => {
                proc.état = EtatProcessus::Terminé { code_retour: 0 };
                Ok(0)
            }
            None => Err(format!("PID {} introuvable", pid)),
        }
    }

    // Afficher un résumé des processus
    fn afficher_résumé(&self) {
        println!("--- Résumé des processus ---");
        for p in &self.processus {
            println!(
                "PID: {}, Nom: {}, État: {:?}, Priorité: {:?}, Mémoire: {} ko, Parent: {:?}",
                p.pid, p.nom, p.état, p.priorité, p.mémoire_ko, p.pid_parent
            );
        }
        println!("Mémoire totale utilisée : {} ko", self.mémoire_totale_utilisée());
    }
}

// ----------------- Programme principal -----------------

fn main() {
    let mut gp = GestionnaireProcessus::nouveau();

    // Créer init (PID 1)
    let init = gp.créer_processus(
        String::from("init"),
        Priorité::Haute,
        1024,
        None,
    );

    // Créer des processus fils
    let bash = gp.créer_processus(
        String::from("bash"),
        Priorité::Normale,
        4096,
        Some(init),
    );

    // Changer les états
    gp.changer_état(bash, EtatProcessus::EnExécution { cpu_id: 0 }).unwrap();

    // Afficher le résumé
    gp.afficher_résumé();

    // Tuer un processus
    match gp.tuer_processus(bash) {
        Ok(code) => println!("bash terminé avec code {}", code),
        Err(e) => eprintln!("Erreur : {}", e),
    }

    // Afficher le résumé après avoir tué bash
    gp.afficher_résumé();
}