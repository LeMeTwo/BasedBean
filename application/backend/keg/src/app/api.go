package app

import (
    "fmt"
    "net/http"
)

func (app *Application) getKey(w http.ResponseWriter, r *http.Request) {
    fmt.Fprintf(w, "GetKey GET\n")
}

func (app *Application) deleteKey(w http.ResponseWriter, r *http.Request) {
    fmt.Fprintf(w, "DeleteKey DELETE\n")
}
