#!/bin/sh

# Ustawienie zmiennych środowiskowych, jeżeli są dostępne
export VITE_SERVER_IP="${VITE_SERVER_IP:-backend-service}"
export VITE_SERVER_PORT="${VITE_SERVER_PORT:-8090}"

# Jeżeli chcesz uruchomić dodatkowe procesy (np. frontend Vite), możesz dodać to tutaj
# np. uruchomienie serwera Vite, jeśli nie używasz Nginx:
#npm run dev

# Uruchomienie Nginx (serwowanie aplikacji)
nginx -g 'daemon off;'
