#!/bin/sh

# Ustawienie zmiennych środowiskowych, jeżeli są dostępne
export VITE_SERVER_IP="${VITE_SERVER_IP:-backend-service}"
export VITE_SERVER_PORT="${VITE_SERVER_PORT:-8090}"

# Uruchomienie Nginx (serwowanie aplikacji)
nginx -g 'daemon off;'
