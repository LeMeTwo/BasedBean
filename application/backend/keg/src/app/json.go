package app

import (
    "encoding/json"
    "net/http"
)

func writeJsonError(w http.ResponseWriter, status int, message string) error {
    type jsonErr struct {
        Error string `json:"error"`
    }

    return writeJson(w, status, &jsonErr{Error: message})
}

func (app Application) jsonResponse(w http.ResponseWriter, status int, data any) error {
    type jsonData struct {
        Data any `json:"data"`
    }

    return writeJson(w, status, &jsonData{Data: data})
}

func writeJson(w http.ResponseWriter, status int, data any) error {
    w.Header().Set("Content-Type", "application/json")
    w.WriteHeader(status)
    return json.NewEncoder(w).Encode(data)
}
