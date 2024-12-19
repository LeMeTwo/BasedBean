package cache

import (
    "errors"
)

type KeyCache interface {
    IsEmpty() bool
    GetKey() (key string, err error)
    PushKeys(keys []string)
}

type LocalKeyCache struct {
    keys []string
}

func (lkc LocalKeyCache) IsEmpty() bool {
    return len(lkc.keys) == 0
}

func (lkc LocalKeyCache) GetKey() (key string, err error) {
    if len(lkc.keys) == 0 {
        return "", errors.New("Key Cache is empty; cannot return valid key.")
    }
    key = lkc.keys[len(lkc.keys)-1]
    lkc.keys = lkc.keys[:len(lkc.keys)-1]
    return key, nil
}

func (lkc LocalKeyCache) PushKeys(keys []string) {
    lkc.keys = append(lkc.keys, keys[:]...)
}
