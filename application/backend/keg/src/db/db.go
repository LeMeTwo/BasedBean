package db

import (
    "context"
    "fmt"
    "github.com/redis/go-redis/v9"
)

type KeyDb struct {
    client *redis.Client
}

func (kdb KeyDb) ScanKeys(ctx context.Context) (keys []string, err error) {
    var cursor uint64
    for {
        var kdbKeys []string
        kdbKeys, cursor, err = kdb.client.Scan(ctx, cursor, AVAIL_WILDCARD, 100).Result()
        if err != nil {
            panic(err)
        }
        keys = append(keys, kdbKeys[:]...)
        if cursor == 0 || len(keys) >= 10{
            break
        }
    }

    fmt.Println(keys)
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
