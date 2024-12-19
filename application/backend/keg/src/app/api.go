package app

import (
	"net/http"

	"github.com/go-chi/chi/v5"
)

type GetResponse struct {
    Key string `json:"key"`
}

func (app *Application) getKey(w http.ResponseWriter, r *http.Request) {
    ctx := r.Context()
    if app.Cache.IsEmpty() {
        keys, err := app.Storage.FetchBatchReservedKeys(ctx, KEYS_BATCH_SIZE)
        if err != nil {
            app.internalServerError(w, r, err)
            return
        }
        app.Cache.PushKeys(keys)
    }

    key, err := app.Cache.GetKey()
    if err != nil {
        app.internalServerError(w, r, err)
        return
    }

    response := GetResponse{
        Key: key,
    }
    if err = app.jsonResponse(w, http.StatusCreated, response); err != nil {
        app.internalServerError(w, r, err)
        return
    }
}

func (app *Application) deleteKey(w http.ResponseWriter, r *http.Request) {
    key := chi.URLParam(r, "key")
    ctx := r.Context()

    if err := app.Storage.ExpireKey(ctx, key); err != nil {
        // todo
        app.internalServerError(w, r, err)
        return
    }

    w.WriteHeader(http.StatusNoContent)
}
