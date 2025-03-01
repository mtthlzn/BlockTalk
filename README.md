# BlockTalk

BlockTalk est une API blockchain minimaliste développée en Rust avec Axum pour le backend. Ce projet vise à fournir une base pour une messagerie décentralisée où chaque message est stocké dans une blockchain.

## 📌 Technologies utilisées

- **Langage** : Rust
- **Framework Web** : Axum
- **Base de données** : LevelDB (à venir)
- **Réseau P2P** : Libp2p (à venir)
- **Frontend** : Solid.js (à venir)

## 🚀 Installation et lancement

### 1️⃣ Cloner le dépôt

```sh
git clone <URL_DU_REPO>
cd BlockTalk
```

### 2️⃣ Initialiser et vérifier l’environnement Rust

Assurez-vous que Rust est installé :

```sh
rustc --version
cargo --version
```

Si ce n'est pas le cas, installez Rust via :

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 3️⃣ Installation des dépendances

Dans le dossier `backend/`, ajoutez les dépendances requises :

```sh
cargo check
```

### 4️⃣ Lancer le serveur API

Dans le dossier `backend/`, exécutez :

```sh
cargo run
```

✅ Le serveur sera accessible à l’adresse :

```
http://127.0.0.1:3000
```

### 5️⃣ Tester l’API

Vous pouvez tester l'API avec `curl` :

```sh
curl http://127.0.0.1:3000
```

Vous devriez obtenir une réponse JSON confirmant que l’API fonctionne :

```json
{ "status": "OK", "message": "API BlockTalk is running!" }
```

## 🔧 Routes disponibles

| Méthode | Route           | Description                                  |
| ------- | --------------- | -------------------------------------------- |
| GET     | `/`             | Vérifie si l’API est en ligne                |
| POST    | `/messages`     | (À venir) Ajouter un message à la blockchain |
| GET     | `/transactions` | (À venir) Récupérer les transactions         |

## 🔜 Prochaines étapes

✅ **Intégration de LevelDB** pour stocker les blocs. ✅ **Définition de la structure d’un bloc.** ✅ **Ajout des routes pour envoyer et récupérer des messages.** ✅ **Connexion en P2P avec Libp2p.**
