package db

import (
	"context"
	"fmt"

	"github.com/redis/go-redis/v9"
)

type TimeoutExceeded struct {
    operation string
}

func (e *TimeoutExceeded) Error() string {
    return fmt.Sprintf("Timed out while performing operation: %s", e.operation)
}

type DbConfig struct {
    Addr string
    Password string
}

const ACQUIRE_TIMEOUT int = 5
const LOCK_TIMEOUT int = 5

type KeyStorage interface {
    FetchBatchReservedKeys(ctx context.Context, size int) (keys []string, err error)
    ExpireKey(ctx context.Context, key string) (err error)
    Health(ctx context.Context) error 
}

type KeyDbStorage struct {
    db KeyDatabase
    lock Lock
}

func NewKeyDbStorage(client *redis.Client) KeyDbStorage {
    return KeyDbStorage{
        db: KeyDb{client},
        lock: NewRedisExpiringLock(client, ACQUIRE_TIMEOUT, LOCK_TIMEOUT),
    }
}

func (kds KeyDbStorage) FetchBatchReservedKeys(ctx context.Context, size int) (keys []string, err error) {
    acquired, deadline, err := kds.lock.Acquire(ctx)
    if acquired {
        defer kds.lock.Release(ctx)

        dctx, cancel := context.WithDeadline(ctx, deadline)
        defer cancel()

        ckeys := make(chan []string)
        cerr := make(chan error)

        go kds.fetchKeys(dctx, size, ckeys, cerr)

        select {
        case <-dctx.Done():
            return nil, &TimeoutExceeded{operation: "FetchKeys"}
        case err := <-cerr:
            return nil, err
        case keys = <-ckeys:
            return parseAvailKeys(keys), nil
        }
    } else {
        return nil, fmt.Errorf("Could not acquire lock: %v", err)
    }
}

func (kds KeyDbStorage) fetchKeys(ctx context.Context, size int, ckeys chan<- []string, cerr chan<- error) {
    keys, err := kds.db.ScanKeys(ctx, size)
    if err != nil {
        cerr <- err
        return
    }
    err = kds.db.ReserveKeys(ctx, keys)
    if err != nil {
        cerr <- err
    } else {
        ckeys <- keys
    }
    return
}

func (kds KeyDbStorage) ExpireKey(ctx context.Context, key string) (err error) {
    acquired, deadline, err := kds.lock.Acquire(ctx)
    key = appendUsedKeySuffix(key)
    if acquired {
        defer kds.lock.Release(ctx)
        dctx, cancel := context.WithDeadline(ctx, deadline)
        defer cancel()

        cerr := make(chan error)
        go func() {
            cerr <- kds.db.ExpireKey(dctx, key)
        }()

        select {
        case <-dctx.Done():
            return &TimeoutExceeded{operation: "ExpireKey"}
        case err = <- cerr:
            return
        }
    } else {
        return fmt.Errorf("Could not acquire lock: %v", err)
    }
}

func (kds KeyDbStorage) Health(ctx context.Context) error {
    err := kds.db.Ping(ctx)
    if err != nil {
        return err
    }
    return nil
}
