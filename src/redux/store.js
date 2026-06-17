import { configureStore } from '@reduxjs/toolkit';
import { FLUSH, PAUSE, PERSIST, persistReducer, persistStore, PURGE, REGISTER, REHYDRATE } from 'redux-persist';
import rootReducer from './reducer';

// Inline localStorage adapter. This avoids a CJS/ESM default-export interop issue with
// `redux-persist/lib/storage` under Vite/Rolldown (where the import can resolve to the
// module namespace instead of the storage object). Behaviour is identical to the original:
// state is persisted to window.localStorage under the 'finance' key.
const storage = {
    getItem: (key) => Promise.resolve(window.localStorage.getItem(key)),
    setItem: (key, value) => Promise.resolve(window.localStorage.setItem(key, value)),
    removeItem: (key) => Promise.resolve(window.localStorage.removeItem(key)),
};

const persistConfig = {
    key: 'finance',
    storage,
};

const persistedReducer = persistReducer(persistConfig, rootReducer);

const store = configureStore({
    reducer: persistedReducer,
    middleware: (getDefaultMiddleware) =>
        getDefaultMiddleware({
            serializableCheck: {
                ignoredActions: [FLUSH, REHYDRATE, PAUSE, PERSIST, PURGE, REGISTER],
            },
        }),
});

const persistor = persistStore(store);

export {
    store,
    persistor
};
