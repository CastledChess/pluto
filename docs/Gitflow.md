# Gitflow

## Branches Principales

- **main** : Cette branche contient le code en production. Il doit toujours être stable et ne doit pas contenir de version non prête à être déployée.
- **develop** : Cette branche sert à intégrer les fonctionnalités. Elle contient le code qui est prêt à être testé avant son intégration dans `main`.

## Branches de Support

- **feat/** : Utilisées pour le développement de nouvelles fonctionnalités. Chaque fonctionnalité a sa propre branche, par exemple `feat/ajout-authentification`.
- **fix/** : Utilisées pour corriger des bugs non critiques identifiés en phase de développement. Chaque correction a sa propre branche, par exemple `fix/correction-bug-login`. Ces branches sont créées à partir de `develop` et fusionnées dans `develop` une fois la correction terminée.
- **hotfix/** : Utilisées pour les corrections urgentes sur la branche `main`. Elles permettent de résoudre rapidement des problèmes critiques en production. Les branches `hotfix/` sont créées à partir de `main` et, une fois la correction apportée, sont fusionnées dans `main` et `develop` pour assurer la synchronisation des correctifs.

- **release/** : Utilisées pour préparer une nouvelle version. Elles permettent de finaliser les tests et les corrections avant de fusionner dans `main`.

## Workflow

1. **Développement de Fonctionnalités** :
   - Créez une branche `feat/` à partir de `develop`.
   - Développez la fonctionnalité et testez-la.
   - Fusionnez la branche `feat/` dans `develop` une fois terminée.

2. **Correction de Bugs** :
   - Créez une branche `fix/` à partir de `develop`.
   - Corrigez le bug et testez la correction.
   - Fusionnez la branche `fix/` dans `develop` une fois terminée.

3. **Hotfix** :
   - Créez une branche `hotfix/` à partir de `main`.
   - Appliquez la correction urgente.
   - Fusionnez la branche `hotfix/` dans `main` et `develop`.

4. **Préparation d'une Version** :
   - Créez une branche `release/` à partir de `develop`.
   - Finalisez les tests et les corrections.
   - Fusionnez la branche `release/` dans `main` et `develop`.

## Utilisation d'Outils pour Simplifier le Workflow

Pour faciliter l'adoption et l'application cohérente du Gitflow, il est recommandé d'utiliser des outils dédiés :

- **Extensions Gitflow** : Des extensions comme `git-flow` automatisent la création et la gestion des branches selon les conventions Gitflow. Elles ajoutent des commandes simplifiées pour initier des fonctionnalités, des releases ou des hotfixes.

**Installation de `git-flow`** :
- **macOS** : `brew install git-flow`
- **Windows** : Inclus dans certaines distributions Git pour Windows ou disponible via des installateurs tiers.
- **Ubuntu/Debian** : `apt-get install git-flow`

**Initialisation du dépôt avec `git-flow`**:
Après installation, initialisez `git-flow` dans votre dépôt :

```bash
git flow init
```

Cette commande configure les branches par défaut (`main` et `develop`) et définit les préfixes pour les branches de support (`feat/`, `release/`, `hotfix/`).

## Bonnes Pratiques

- Assurez-vous que le code est bien testé avant de fusionner dans `develop` ou `main`.
- Utilisez des **pull requests** pour faciliter la revue de code.
- Gardez les branches `main` et `develop` à jour avec les dernières modifications.
- Utilisez des messages de commit explicites et conformes aux conventions de votre équipe.
- Mettez en place des pipelines d'intégration continue pour automatiser les tests et les déploiements.

- **Convention de Nommage des Branches**:
    Adoptez une convention claire pour le nommage des branches, par exemple :
    - Fonctionnalités : `feat/nom-fonctionnalité`
    - Corrections : `fix/description-correction`
    - Releases : `release/x.y.z` (où `x.y.z` représente le numéro de version)
    - Hotfixes : `hotfix/description-correction`
