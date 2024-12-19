package app

import (
    "net/http"
)

func (app Application) internalServerError(w http.ResponseWriter, r *http.Request, err error) {
    app.Logger.Errorw("internal error", "method", r.Method, "path", r.URL.Path, "error", err.Error())

    writeJsonError(w, http.StatusInternalServerError, "the server encountered a problem")
}

func (app Application) notFoundResponse(w http.ResponseWriter, r *http.Request, err error) {
    app.Logger.Warnf("not found error", "method", r.Method, "path", r.URL.Path, "error", err.Error())

    writeJsonError(w, http.StatusNotFound, "not found")
}
