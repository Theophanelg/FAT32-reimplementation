# FAT32-reimplementation
Projet de RUST de 1ere année de mastère

e projet est une petite bibliothèque qui permet de lire un système de fichiers **FAT32** de manière très simple.  
L’objectif est d’apprendre à manipuler des structures bas niveau (secteurs, clusters) sans utiliser la bibliothèque standard (`no_std`).  

## Ce que le projet sait faire

Le projet permet de :

- Lister les fichiers d’un répertoire, comme la commande `ls`.
- Lire le contenu d’un fichier, comme `cat` ou `more`.
- Changer de répertoire et connaître le répertoire courant, comme `cd`.

Tout cela est fait en lisant directement les secteurs d’un périphérique abstrait (`BlockDevice`) et en interprétant la structure FAT32 (secteur de boot, clusters, entrées de répertoire).

## Organisation du code

- `src/lib.rs` : point d’entrée de la bibliothèque (déclare `no_std`, les erreurs, les types publics).
- `src/bootsector.rs` : code qui lit et représente le secteur de démarrage FAT32.
- `src/fat.rs` : code principal qui manipule le volume FAT32 (lecture de clusters, répertoires, fichiers).
- `examples/commandline.rs` : petit programme d’exemple qui simule un périphérique et teste `ls`, `cd` et `cat`.
- `tests/` : tests qui vérifient le parsing du secteur de boot et le fonctionnement des opérations sur le volume.

## Comment lancer l’exemple

Pour lancer le petit programme de démonstration (CLI simple) :

```bash
cargo run --example commandline
```

## Ce programme :

- Simule un périphérique avec quelques secteurs en mémoire.  
- Crée un `FatVolume` à partir de ce périphérique.  
- Affiche :
  - le nombre de fichiers trouvés dans le répertoire courant (`ls`) ;
  - le changement de répertoire (`cd`) ;
  - la lecture d’un cluster de fichier (`cat` simplifié).

## Comment lancer les tests

Les tests se lancent avec :

```bash
cargo test
```

## Contraintes du projet

Le code de la bibliothèque est écrit en mode `no_std`.

L'accès au disque est abstrait via un trait `BlockDevice`, ce qui permet :
- de tester facilement avec un périphérique simulé
- de réutiliser la bibliothèque dans un environnement sans OS
