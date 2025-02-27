localhost/
├── assets/
│   └── header.txt            
├── src/
│   ├── main.rs                      # Entrée principale de l'application
│   ├── server/
│   │   ├── mod.rs                   # Module principal du serveur
│   │   ├── epoll.rs                 # Gestion des E/S non bloquantes avec epoll
│   │   ├── config.rs                # Analyse et gestion de la configuration
│   │   ├── listener.rs              # Gestion des sockets (écoute sur plusieurs ports)
│   │   └── router.rs                # Routage des requêtes vers les bons gestionnaires
│   ├── http/
│   │   ├── mod.rs                   # Module principal pour HTTP
│   │   ├── request.rs               # Analyse des requêtes HTTP
│   │   ├── response.rs              # Construction des réponses HTTP
│   │   ├── status.rs                # Définition des codes de statut HTTP
│   │   └── headers.rs               # Gestion des en-têtes HTTP (Cookies, Content-Type, etc.)
│   ├── handlers/
│   │   ├── mod.rs                   # Module principal pour les gestionnaires
│   │   ├── static_files.rs          # Gestion des fichiers statiques
│   │   ├── cgi.rs                   # Exécution des scripts CGI
│   │   ├── upload.rs                # Gestion des téléchargements de fichiers
│   │   └── error_pages.rs           # Gestion des pages d'erreur personnalisées
│   └── utils/
│       ├── mod.rs                   # Module principal pour les utilitaires
│       ├── error.rs                 # Gestion des erreurs génériques
│       ├── timeout.rs               # Gestion des timeouts
│       └── file.rs                  # Fonctions utilitaires pour la manipulation de fichiers
├── tests/
│   ├── config_tests.rs              # Tests unitaires pour la configuration
│   ├── http_tests.rs                # Tests unitaires pour les fonctionnalités HTTP
│   ├── epoll_tests.rs               # Tests unitaires pour epoll
│   ├── handlers_tests.rs            # Tests unitaires pour les gestionnaires
│   └── integration_tests.rs         # Tests d'intégration pour l'ensemble du serveur
├── config/
│   └── server.conf                  # Fichier de configuration du serveur (hôte, ports, routes, etc.)
├── static/                          # Dossier pour les fichiers statiques
│   ├── index.html                   # Page d'accueil par défaut
│   ├── 404.html                     # Page d'erreur 404
│   └── 500.html                     # Page d'erreur 500
├── cgi/                             # Dossier pour les scripts CGI
│   ├── example.py                   # Exemple de script Python pour CGI
│   └── example.php                  # Exemple de script PHP pour CGI
├── logs/                            # Dossier pour les journaux
│   └── access.log                   # Fichier de journal des accès
├── uploads/                         # Dossier pour les fichiers téléchargés
│   └── README.md                    # Fichier README pour expliquer le dossier 
├── Cargo.toml                       # Fichier de configuration Cargo
└── README.md                        # Documentation et instructions
