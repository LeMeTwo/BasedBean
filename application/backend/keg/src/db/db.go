package db

import (
    "context"
    "github.com/redis/go-redis/v9"
)

type KeyDatabase interface {
    ScanKeys(ctx context.Context, size int) (keys []string, err error)
    ReserveKeys(ctx context.Context, keys []string) (err error)
}

type KeyDb struct {
    client *redis.Client
}

func (kdb KeyDb) ScanKeys(ctx context.Context, size int) (keys []string, err error) {
    var cursor uint64
    for {
        var kdbKeys []string
        kdbKeys, cursor, err = kdb.client.Scan(ctx, cursor, AVAIL_WILDCARD, 100).Result()
        if err != nil {
            panic(err)
        }
        keys = append(keys, kdbKeys[:]...)
        if cursor == 0 || len(keys) >= size {
            break
        }
    }

    return keys, nil
}

func (kdb KeyDb) ReserveKeys(ctx context.Context, keys []string) (err error) {
    for _, key := range keys {
        _, err = kdb.client.Rename(ctx, key, getKeyUsedName(key)).Result()
        if err != nil {
            panic(err)
        }
    }
    return
}
